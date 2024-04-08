use soup::{NodeExt, QueryBuilderExt, Soup};

use crate::{
    constants,
    models::comic::{Comic, ComicType},
};

// This code probably sucks...
pub async fn get_manga_from_name(query: &str) -> reqwest::Result<Option<Comic>> {
    let base_url = constants::MANGA_URL;
    let mut tmp = query.replace(" ", "+");

    let url = format!("{base_url}/search?q={tmp}");

    let body = reqwest::get(url).await?.text().await?;
    let soup = Soup::new(&body);

    let src: Vec<_> = soup
        .class("lg:grid-cols-5")
        .find()
        .expect("Not found")
        .children()
        .filter(|x| x.display().to_lowercase().contains(&query.to_lowercase()))
        .collect();

    // In case it finds something.
    if let Some(first) = src.first() {
        let name = first
            .class("leading-tight")
            .find()
            .expect("Couldn't find name")
            .text();

        tmp = first
            .tag("a")
            .find()
            .expect("Not found")
            .get("href")
            .unwrap();
        let source = format!("{base_url}{tmp}");

        Ok(Some(Comic {
            name,
            source,
            comic_type: ComicType::Manga,
        }))
    } else {
        Ok(None)
    }
}

pub async fn search_manga(query: &str) -> reqwest::Result<Vec<Comic>> {
    let manga_url = constants::MANGA_URL;
    let tmp = query.replace(" ", "+");

    let url = format!("{manga_url}/search?q={tmp}");

    let body = reqwest::get(url).await?.text().await?;
    let soup = Soup::new(&body);

    let tmp = soup
        .class("lg:grid-cols-5")
        .find();

    if let None = tmp { // Couldn't find anything.
        return Ok(Vec::new())
    }

    let manga_src: Vec<_> = tmp.unwrap().children()
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
                })
            },
            None => continue,
        }
    }

    Ok(mangas)
}
