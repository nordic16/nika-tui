use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ComicType {
    Manga,
    Western,
}

#[derive(Debug, Clone)]
pub struct Comic {
    pub name: String,
    pub source: String,
    pub comic_type: ComicType,
}

impl Display for Comic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}\nSource: {}\nType: {}", self.name, self.source, self.comic_type)
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