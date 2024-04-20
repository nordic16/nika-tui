use async_trait::async_trait;
use soup::{NodeExt, QueryBuilderExt, Soup};

use crate::{
    models::comic::{Chapter, Comic, ComicInfo, ComicType},
    traits::Source,
};

pub struct MangareaderSource;

impl MangareaderSource {
    pub fn new() -> Self {
        Self {}
    }
}

// TODO
#[async_trait]
impl Source for MangareaderSource {
    async fn search(&self, query: &str) -> reqwest::Result<Vec<Comic>> {
        let q = query.to_lowercase().replace(' ', "+");
        let search_url = format!("{}/search?keyword={q}", self.base_url());
        let mut comics: Vec<Comic> = Vec::new();

        let body = reqwest::get(&search_url).await?.text().await?;
        let soup = Soup::new(&body);
        let items: Vec<_> = soup.class("item-spc").find_all().collect();

        for f in items {
            let tag = f.class("manga-detail").find().unwrap();
            let a = tag.tag("a").find().unwrap();

            let name = a.get("title").unwrap();
            let source = a.get("href").unwrap();

            comics.push(Comic::new(
                &name,
                &source,
                ComicType::Manga,
                None,
                Vec::new(),
            ));
        }

        Ok(comics)
    }

    fn base_url(&self) -> &'static str {
        "https://mangareader.to"
    }

    async fn get_chapters(&self, comic: &Comic) -> reqwest::Result<Vec<Chapter>> {
        let body = reqwest::get(&comic.source).await?.text().await?;
        let soup = Soup::new(&body);
        let base_url = self.base_url();

        let items: Vec<_> = soup.class("item-link").find_all().collect();
        let chapters: Vec<Chapter> = items
            .into_iter()
            .map(|f| {
                Chapter::new(
                    &f.get("title").unwrap(),
                    &format!("{base_url}{}", f.get("href").unwrap()),
                )
            })
            .collect();

        Ok(chapters)
    }

    async fn get_info(&self, comic: &Comic) -> reqwest::Result<Option<ComicInfo>> {
        let body = reqwest::get(&comic.source).await?.text().await?;
        let soup = Soup::new(&body);

        let info = soup.class("anisc-info").find().unwrap();
        let fields: Vec<_> = info.class("item").find_all().collect();
        let spans: Vec<_> = fields.into_iter().map(|f| f.class("name").find().unwrap()).collect();

        let date = spans[4].text();
        let status = spans[1].text();
        let genres = vec![String::from("todo")];

        Ok(Some(ComicInfo::new(&date, &status, genres)))
        
    }

    fn name(&self) -> &'static str {
        "mangareader"
    }
}
