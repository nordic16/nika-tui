use async_trait::async_trait;
use soup::{NodeExt, QueryBuilderExt, Soup};

use crate::{
    helpers,
    models::comic::{Chapter, Comic, ComicInfo, ComicType},
    traits::Source,
};

#[derive(Clone, Default)]
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

                    mangas.push(Comic {
                        name,
                        source: format!("{base_url}{tmp}"),
                        comic_type: ComicType::Manga,
                        manga_info: None,
                        chapters: Vec::new(),
                    })
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
            .map(|f| Chapter {
                name: f.text(),
                source: format!("{base_url}{}", f.get("href").unwrap()),
            })
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
            let year = values[6].text().parse::<u16>().unwrap();

            return Ok(Some(ComicInfo {
                status,
                year,
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
}

impl MangapillSource {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{
            comic::{Comic, ComicType},
            sources::mangapill::MangapillSource,
        },
        traits::Source,
    };

    #[tokio::test]
    async fn test_scrape_manga_info() {
        let source = MangapillSource;

        let comic = Comic {
            name: String::from("One Piece"),
            source: String::from("https://mangapill.com/manga/2/one-piece"),
            comic_type: ComicType::Manga,
            manga_info: None,
            chapters: Vec::new(),
        };

        let result = source.get_info(&comic).await.unwrap();

        match result {
            Some(val) => {
                println!("Comic info secured!");
                println!(
                    "Year: {}\nStatus: {}\nGenres: {}",
                    val.year,
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
            manga_info: None,
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
}
