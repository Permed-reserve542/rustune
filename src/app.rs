use ratatui::widgets::ListState;

use crate::youtube::SearchResult;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Browse,
    Input,
}

#[derive(Debug, Clone)]
pub enum Status {
    Idle,
    Searching(String),
    Loading(String),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct PlaybackState {
    pub title: String,
    pub duration_secs: u64,
    pub elapsed_secs: u64,
    pub paused: bool,
}

pub struct App {
    pub mode: Mode,
    pub results: Vec<SearchResult>,
    pub list_state: ListState,
    pub page: usize,
    pub input_text: String,
    pub input_cursor: usize, // char index within input_text
    pub input_history: Vec<String>,
    pub history_index: usize,
    pub playback: Option<PlaybackState>,
    pub status: Status,
    pub should_quit: bool,
    pub mpv_kill: Option<tokio::sync::oneshot::Sender<()>>,
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            mode: Mode::Browse,
            results: Vec::new(),
            list_state,
            page: 0,
            input_text: String::new(),
            input_cursor: 0,
            input_history: Vec::new(),
            history_index: 0,
            playback: None,
            status: Status::Idle,
            should_quit: false,
            mpv_kill: None,
        }
    }

    pub fn select_next(&mut self) {
        if self.results.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) if i >= self.results.len() - 1 => 0,
            Some(i) => i + 1,
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn select_prev(&mut self) {
        if self.results.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(0) => self.results.len() - 1,
            Some(i) => i - 1,
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn select_first(&mut self) {
        if !self.results.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn select_last(&mut self) {
        if !self.results.is_empty() {
            self.list_state.select(Some(self.results.len() - 1));
        }
    }

    pub fn selected_result(&self) -> Option<&SearchResult> {
        self.list_state.selected().map(|i| &self.results[i])
    }

    pub fn kill_mpv(&mut self) {
        if let Some(kill) = self.mpv_kill.take() {
            let _ = kill.send(());
        }
        self.playback = None;
    }

    pub fn format_duration(secs: u64) -> String {
        let m = secs / 60;
        let s = secs % 60;
        format!("{m}:{s:02}")
    }
}
