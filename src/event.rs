use anyhow::Result;

use crate::youtube::SearchResult;

#[derive(Debug)]
pub enum AppEvent {
    SearchComplete(Result<Vec<SearchResult>>),
    StreamUrlReady(Result<(String, String)>), // (url, title)
    PlaybackProgress {
        elapsed_secs: u64,
        duration_secs: u64,
    },
    PlaybackComplete,
    PlaybackError(String),
}
