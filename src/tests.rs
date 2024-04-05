use crate::helpers::get_manga_from_name;

#[tokio::test]
async fn test_get_manga_from_name() {
    let val = get_manga_from_name(String::from("one piece")).await;
    
    match val {
        Ok(opt) => { // everything fine.
            if let Some(comic) = opt {
                println!("Comic found!");
                println!("{}", comic);
            
            } else { 
                println!("Comic not found!");
            }  
        },
        Err(e) => println!("Error! {}", e),
    }
}