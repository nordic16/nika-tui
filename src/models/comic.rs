use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub enum ComicType {
    #[default]
    Manga,
    Western,
}

#[derive(Debug, Clone, Default)]
pub struct Comic {
    pub name: String,
    pub source: String,
    pub comic_type: ComicType,
    pub manga_info: Option<ComicInfo>,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, Default)]
pub struct Chapter {
    pub name: String,
    pub source: String,
}

impl Chapter {
    pub fn new(name: &str, source: &str) -> Self {
        Self {
            name: name.to_owned(),
            source: source.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ComicInfo {
    // might change later.
    pub date: String,
    pub status: String,
    pub genres: Vec<String>,
}

impl ComicInfo {
    pub fn new(date: &str, status: &str, genres: Vec<String>) -> Self {
        Self {
            date: date.to_owned(),
            status: status.to_owned(),
            genres,
        }
    }
}

impl Comic {
    pub fn new(
        name: &str,
        source: &str,
        comic_type: ComicType,
        manga_info: Option<ComicInfo>,
        chapters: Vec<Chapter>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            source: source.to_owned(),
            comic_type,
            manga_info,
            chapters,
        }
    }
}
