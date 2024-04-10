use crate::{
    helpers::{self, search_manga},
    models::comic::{Comic, ComicType},
};

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

#[tokio::test]
async fn test_scrape_manga_info() {
    let comic = Comic {
        name: String::from("One Piece"),
        source: String::from("https://mangapill.com/manga/2/one-piece"),
        comic_type: ComicType::Manga,
        manga_info: None,
    };

    let result = helpers::get_comic_info(&comic).await.unwrap();

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
