use std::borrow::BorrowMut;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::{env, fs};

use async_trait::async_trait;
use rand::distributions::Alphanumeric;
use rand::Rng;
use soup::{NodeExt, QueryBuilderExt, Soup};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::helpers;
use crate::models::comic::{Chapter, Comic, ComicInfo, ComicType};
use crate::traits::Source;

pub struct MangapillSource;

#[async_trait]
impl Source for MangapillSource {
    async fn search(&self, query: &str) -> reqwest::Result<Vec<Comic>> {
        let body = helpers::get_search_response_body(query, self)
            .await
            .unwrap_or(String::from(""));
        let soup = Soup::new(&body);

        let tmp = soup.class("lg:grid-cols-5").find();

        if tmp.is_none() {
            // Couldn't find anything.
            return Ok(Vec::new());
        }

        let manga_src: Vec<_> = tmp
            .unwrap()
            .children()
            .filter(|x| x.display().to_lowercase().contains(&query.to_lowercase()))
            .collect();

        let mut mangas: Vec<Comic> = Vec::with_capacity(manga_src.len());

        for i in manga_src {
            // didn't use functional programming here because code was too long
            let name = i
                .class("leading-tight")
                .find()
                .expect("Couldn't find name")
                .text();

            let tmp = i.tag("a").find();
            let base_url = self.base_url();

            match tmp {
                Some(e) => {
                    let tmp = e.get("href").unwrap();
                    let source = format!("{base_url}{tmp}");

                    mangas.push(Comic::new(&name, &source, ComicType::Manga, Vec::new()));
                }
                None => continue,
            }
        }

        Ok(mangas)
    }

    async fn get_chapters(&self, comic: &Comic) -> reqwest::Result<Vec<Chapter>> {
        let base_url = self.base_url();

        let manga_page = reqwest::get(&comic.source).await?.text().await?;
        let soup = Soup::new(&manga_page);

        let chapter_urls: Vec<_> = soup.tag("a").class("border-border").find_all().collect();
        let chapters: Vec<Chapter> = chapter_urls
            .into_iter()
            .map(|f| Chapter::new(&f.text(), &format!("{base_url}{}", f.get("href").unwrap())))
            .collect();

        Ok(chapters)
    }

    async fn get_info(&self, comic: &Comic) -> reqwest::Result<Option<ComicInfo>> {
        let manga_page = reqwest::get(&comic.source).await?.text().await?;
        let soup = Soup::new(&manga_page);

        let info_div = soup.class("md:grid-cols-3").find();

        if let Some(container) = info_div {
            let values: Vec<_> = container.tag("div").find_all().collect();

            if values.is_empty() {
                return Ok(None);
            }

            let status = values[4].text();
            let date = values[6].text();

            return Ok(Some(ComicInfo {
                status,
                date,
                genres: vec![String::from("fantasy")],
            }));
        }

        Ok(None)
    }

    fn base_url(&self) -> &'static str {
        "https://mangapill.com"
    }

    fn name(&self) -> &'static str {
        "mangapill"
    }

    async fn download_chapter(&self, chapter: &Chapter) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let body = client
            .get(&chapter.source)
            .header("Referer", self.base_url())
            .send()
            .await?
            .text()
            .await?;

        println!("{body}");

        let path = {
            let rng = rand::thread_rng();
            let str: String = rng
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect();

            Path::join(&env::temp_dir(), str)
        };

        // TODO: handle this.
        fs::create_dir(&path)?;

        // Has to be inside a code block to make this function Send (soup isn't Send).
        let urls: Vec<String> = {
            let soup = Soup::new(&body);
            let images: Vec<_> = soup.tag("img").find_all().collect();
            images
                .into_iter()
                .map(|f| f.get("data-src").unwrap())
                .collect()
        };

        for (i, url) in urls.iter().enumerate() {
            let mut data = client
                .get(url)
                .header("Referer", self.base_url())
                .send()
                .await?;

            let fname = format!("page-{i}.jpeg");
            let path = path.join(fname);
            let mut f = File::create(path).await?;

            while let Some(chunk) = data.chunk().await? {
                f.write_all(&chunk).await?;
            }
        }

        Ok(String::from(path.to_str().unwrap()))
    }
}

impl MangapillSource {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::models::comic::{Comic, ComicType};
    use crate::models::sources::mangapill::MangapillSource;
    use crate::traits::Source;

    #[tokio::test]
    async fn test_scrape_manga_info() {
        let source = MangapillSource;

        let comic = Comic {
            name: String::from("One Piece"),
            source: String::from("https://mangapill.com/manga/2/one-piece"),
            comic_type: ComicType::Manga,
            chapters: Vec::new(),
        };

        let result = source.get_info(&comic).await.unwrap();

        match result {
            Some(val) => {
                println!("Comic info secured!");
                println!(
                    "Year: {}\nStatus: {}\nGenres: {}",
                    val.date,
                    val.status,
                    val.genres.join(",")
                )
            }

            None => panic!("Couldn't get info."),
        }
    }

    #[tokio::test]
    async fn test_get_chapters() {
        let source = MangapillSource;
        let comic = Comic {
            name: String::from("One Piece"),
            source: String::from("https://mangapill.com/manga/2/one-piece"),
            comic_type: ComicType::Manga,
            chapters: Vec::new(),
        };

        if let Ok(chapters) = source.get_chapters(&comic).await {
            println!("Printing first 25 chapters....");

            // Print only the last 25 chapters lol.
            for i in 0..25 {
                let chapter = &chapters[i];
                println!("{} ({})", chapter.name, chapter.source)
            }
        }
    }

    #[tokio::test]
    async fn test_download_chapter() -> anyhow::Result<()> {
        let source = MangapillSource::new();
        let comic = Comic::new(
            "One Piece",
            "https://mangapill.com/manga/2/one-piece",
            ComicType::Manga,
            Vec::new(),
        );
        let chapters = source.get_chapters(&comic).await?;

        let chapter = &chapters[0];
        let path = source.download_chapter(&chapter).await?;

        println!("Path: {path}");

        Ok(())
    }
}
