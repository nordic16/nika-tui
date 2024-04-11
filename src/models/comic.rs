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
}

#[derive(Debug, Clone, Default)]
pub struct ComicInfo {
    pub year: u16,
    pub status: String,
    pub genres: Vec<String>,
}

impl Display for Comic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nSource: {}\nType: {}",
            self.name, self.source, self.comic_type
        )
    }
}

impl Display for ComicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComicType::Manga => write!(f, "Manga"),
            ComicType::Western => write!(f, "Western Comic"),
        }
    }
}
