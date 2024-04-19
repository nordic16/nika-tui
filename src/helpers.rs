use ratatui::widgets::ListDirection;
use soup::{NodeExt, QueryBuilderExt, Soup};

use crate::{
    constants,
    models::comic::{Comic, ComicType},
};

async fn get_search_response_body(query: &str) -> reqwest::Result<String> {
    let base_url = constants::MANGA_URL;
    let tmp = query.replace(' ', "+");

    let url = format!("{base_url}/search?q={tmp}");

    reqwest::get(url).await?.text().await
}

pub async fn search_manga(query: &str) -> reqwest::Result<Vec<Comic>> {
    let body = get_search_response_body(query)
        .await
        .unwrap_or(String::from(""));
    let manga_url = constants::MANGA_URL;
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

        match tmp {
            Some(e) => {
                let tmp = e.get("href").unwrap();

                mangas.push(Comic {
                    name,
                    source: format!("{manga_url}{tmp}"),
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

pub fn get_new_selection_index(val: usize, len: usize, direction: ListDirection) -> usize {
    match direction {
        ListDirection::TopToBottom => {
            if val == len - 1 {
                // Prevent user from selecting elements below the list
                val
            } else {
                val + 1
            }
        }
        ListDirection::BottomToTop => {
            if val > 0 {
                val - 1
            } else {
                val
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::search_manga;

    #[tokio::test]
    async fn test_search_manga() {
        let query = "one piece";

        let result = search_manga(query).await;

        match result {
            Ok(vec) => {
                for comic in vec {
                    println!("{}", comic)
                }
            }
            Err(e) => println!("Wtf happened? {}", e),
        }
    }
}
