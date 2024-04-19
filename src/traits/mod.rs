use async_trait::async_trait;

use crate::models::{comic::{Chapter, Comic, ComicInfo}, sources::mangapill::MangapillSource};

#[async_trait]
pub trait Source : Send + Sync {
    /// Returns a list of search results based on query
    async fn search(
        &self,
        query: &str,
    ) -> reqwest::Result<Vec<Comic>>;

    fn base_url(&self) -> &'static str;

    /// Returns the chapters for a given comic
    async fn get_chapters(
        &self,
        comic: &Comic,
    ) -> reqwest::Result<Vec<Chapter>>;

    async fn get_info(
        &self,
        comic: &Comic,
    ) -> reqwest::Result<Option<ComicInfo>>;
}

impl Default for Box<dyn Source>  {
    fn default() -> Self {
        Box::new(MangapillSource::new())
    }
}