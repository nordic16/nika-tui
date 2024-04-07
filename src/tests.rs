use crate::helpers::{get_manga_from_name, search_manga};

#[tokio::test]
async fn test_get_manga_from_name() {
    let val = get_manga_from_name(String::from("one piece")).await;

    match val {
        Ok(opt) => {
            // everything fine.
            if let Some(comic) = opt {
                println!("Comic found!");
                println!("{}", comic);
            } else {
                println!("Comic not found!");
            }
        }
        Err(e) => println!("Error! {}", e),
    }
}

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
