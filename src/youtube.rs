use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub duration: Option<u64>,
    pub channel: Option<String>,
}
