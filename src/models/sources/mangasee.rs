use async_trait::async_trait;
use reqwest::Client;
use soup::{NodeExt, QueryBuilderExt, Soup};

use crate::models::comic::{Chapter, Comic, ComicInfo, ComicType};
use crate::traits::Source;

pub struct MangaseeSource;

impl MangaseeSource {
    pub fn new() -> Self {
        Self {}
    }
}

// TODO
#[async_trait]
impl Source for MangaseeSource {
    async fn search(&self, query: &str) -> reqwest::Result<Vec<Comic>> {
        let q = query.to_lowercase().replace(' ', "+");
        let search_url = format!("{}/search/?name={q}", self.base_url());
        let body = reqwest::get(&search_url).await?.text().await?;
        let soup = Soup::new(&body);

        let rows: Vec<_> = soup.tag("div").class("row").find_all().collect();
        let row = &rows[1];

        let data: Vec<_> = row
            .attr("ng-bind-html", "Series.s")
            .tag("a")
            .find_all()
            .collect();
        let comics: Vec<Comic> = data
            .into_iter()
            .map(|p| {
                Comic::new(
                    &p.text(),
                    &format!("{}{}", self.base_url(), p.get("href").unwrap()),
                    ComicType::Manga,
                    Vec::new(),
                )
            })
            .collect();

        Ok(comics)
    }

    fn base_url(&self) -> &'static str {
        "https://mangasee123.com"
    }

    async fn get_chapters(&self, comic: &Comic) -> reqwest::Result<Vec<Chapter>> {
        let client = Client::new();
        let body = client
            .get(&comic.source)
            .header("Referer", self.base_url())
            .send()
            .await?
            .text()
            .await?;

        let soup = Soup::new(&body);
        let base_url = self.base_url();

        let items: Vec<_> = soup.tag("a").class("item-link").find_all().collect();
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
        let spans: Vec<_> = fields
            .into_iter()
            .map(|f| f.class("name").find().unwrap())
            .collect();

        let date = spans[4].text();
        let status = spans[1].text();
        let genres = vec![String::from("todo")];

        Ok(Some(ComicInfo::new(&date, &status, genres)))
    }

    fn name(&self) -> &'static str {
        "mangareader"
    }

    async fn download_chapter(&self, chapter: &Chapter) -> anyhow::Result<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use crate::models::comic::{Comic, ComicType};
    use crate::models::sources::mangapill::MangapillSource;
    use crate::models::sources::mangasee::MangaseeSource;
    use crate::traits::Source;

    #[tokio::test]
    async fn test_search() {
        let source = MangaseeSource::new();
        let query = "one piece";
        let results = source.search(&query).await.unwrap();

        println!("{:?}", results);
    }

    #[tokio::test]
    async fn test_get_chapters() {
        let source = MangapillSource::new();
        let comic = Comic::new(
            "One Piece",
            "https://mangareader.to/one-piece-colored-edition-55493",
            ComicType::Manga,
            Vec::new(),
        );
        let chapters = source.get_chapters(&comic).await.unwrap();
        println!("{:?}", chapters);
    }
}
