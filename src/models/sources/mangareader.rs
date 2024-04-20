use async_trait::async_trait;

use crate::{
    models::comic::{Chapter, Comic, ComicInfo},
    traits::Source,
};

#[derive(Clone, Default)]
pub struct Mangareader;

impl Mangareader {
    pub fn new() -> Self {
        Self {}
    }
}

// TODO
#[async_trait]
impl Source for Mangareader {
    async fn search(&self, query: &str) -> reqwest::Result<Vec<Comic>> {
        Ok(Vec::new())
    }

    fn base_url(&self) -> &'static str {
        "https://mangareader.to"
    }

    async fn get_chapters(&self, comic: &Comic) -> reqwest::Result<Vec<Chapter>> {
        todo!("Still in development")
    }

    async fn get_info(&self, comic: &Comic) -> reqwest::Result<Option<ComicInfo>> {
        todo!("Still in development")
    }

    fn name(&self) -> &'static str {
        "mangareader"
    }
}
