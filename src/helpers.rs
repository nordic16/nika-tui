use ratatui::widgets::ListDirection;
use crate::traits::Source;

pub async fn get_search_response_body(
    query: &str,
    source: &impl Source,
) -> reqwest::Result<String> {
    let base_url = source.base_url();
    let tmp = query.replace(' ', "+");

    let url = format!("{base_url}/search?q={tmp}");

    reqwest::get(url).await?.text().await
}

pub fn get_new_selection_index(val: usize, len: usize, direction: ListDirection) -> usize {
    match direction {
        ListDirection::TopToBottom => {
            if val == len - 1 {
                // Prevent user from selecting elements below the list
                val
            } else {
                val + 1
            }
        }
        ListDirection::BottomToTop => {
            if val > 0 {
                val - 1
            } else {
                val
            }
        }
    }
}

/* 
pub fn get_source(source: &Sources) -> Box<dyn Source> {
    match source {
        Sources::Mangapill => Box::new(MangapillSource::new()),
        Sources::Mangasee => Box::new(MangaseeSource::new()),
    }
}
*/
