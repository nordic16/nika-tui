use soup::{NodeExt, QueryBuilderExt, Soup};

use crate::{
    constants,
    models::comic::{Comic, ComicInfo, ComicType},
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
                })
            }
            None => continue,
        }
    }

    Ok(mangas)
}

pub async fn get_comic_info(comic: &Comic) -> reqwest::Result<Option<ComicInfo>> {
    let manga_page = reqwest::get(&comic.source).await?.text().await?;
    let soup = Soup::new(&manga_page);

    let info_div = soup.class("md:grid-cols-3").find();

    if let Some(container) = info_div {
        let values: Vec<_> = container.tag("div").find_all().collect();

        if values.len() == 0 {
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
