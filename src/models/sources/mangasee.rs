use async_trait::async_trait;

use crate::{
    models::comic::{Chapter, Comic},
    traits::Source,
};

#[derive(Clone, Default)]
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
        todo!()
    }

    fn base_url(&self) -> &'static str {
        todo!()
    }

    async fn get_chapters(&self, comic: &Comic) -> reqwest::Result<Vec<Chapter>> {
        todo!()
    }

    async fn get_info(
        &self,
        comic: &crate::models::comic::Comic,
    ) -> reqwest::Result<Option<crate::models::comic::ComicInfo>> {
        todo!()
    }
}
