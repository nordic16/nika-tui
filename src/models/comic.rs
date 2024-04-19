use std::fmt::Display;

use soup::{NodeExt, QueryBuilderExt, Soup};

use crate::constants;

#[derive(Debug, Clone, Default)]
pub enum ComicType {
    #[default]
    Manga,
    Western,
}

#[derive(Debug, Clone, Default)]
pub struct Comic {
    pub name: String,
    pub source: String,
    pub comic_type: ComicType,
    pub manga_info: Option<ComicInfo>,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Default)]
pub struct Chapter {
    pub name: String,
    pub source: String,
}

#[derive(Debug, Clone, Default)]
pub struct ComicInfo {
    pub year: u16,
    pub status: String,
    pub genres: Vec<String>,
}

impl Comic {
    pub async fn get_comic_info(&self) -> reqwest::Result<Option<ComicInfo>> {
        let manga_page = reqwest::get(&self.source).await?.text().await?;
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

    pub async fn get_chapters(&self) -> reqwest::Result<Vec<Chapter>> {
        let base_url = constants::MANGA_URL;

        let manga_page = reqwest::get(&self.source).await?.text().await?;
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
}

impl Display for Comic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nSource: {}\nType: {}",
            self.name, self.source, self.comic_type
        )
    }
}

impl Display for ComicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComicType::Manga => write!(f, "Manga"),
            ComicType::Western => write!(f, "Western Comic"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::comic::{Comic, ComicType};

    #[tokio::test]
    async fn test_scrape_manga_info() {
        let comic = Comic {
            name: String::from("One Piece"),
            source: String::from("https://mangapill.com/manga/2/one-piece"),
            comic_type: ComicType::Manga,
            manga_info: None,
            chapters: Vec::new(),
        };

        let result = comic.get_comic_info().await.unwrap();

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
        let comic = Comic {
            name: String::from("One Piece"),
            source: String::from("https://mangapill.com/manga/2/one-piece"),
            comic_type: ComicType::Manga,
            manga_info: None,
            chapters: Vec::new(),
        };

        if let Ok(chapters) = comic.get_chapters().await {
            println!("Printing first 25 chapters....");

            // Print only the last 25 chapters lol.
            for i in 0..25 {
                let chapter = &chapters[i];
                println!("{} ({})", chapter.name, chapter.source)
            }
        }
    }
}
