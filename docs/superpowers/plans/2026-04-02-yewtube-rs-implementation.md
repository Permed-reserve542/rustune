# yewtube-rs Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a minimal Rust TUI YouTube player that searches via yt-dlp and plays audio via mpv.

**Architecture:** Single-crate Rust app using ratatui for rendering, crossterm for terminal events, and tokio for async subprocess management. The app has two modes (Browse/Input) with a three-zone vertical layout (results list, player bar, input bar).

**Tech Stack:** Rust 2021, ratatui 0.29, crossterm 0.28, tokio 1, serde/serde_json 1, anyhow 1

**Spec:** `docs/superpowers/specs/2026-04-02-yewtube-rs-tui-design.md`

---

## File Structure

| File | Responsibility |
|------|---------------|
| `Cargo.toml` | Package config, dependencies |
| `src/main.rs` | Entry point, terminal init/teardown, panic hook, event loop |
| `src/app.rs` | `App` state struct, `Mode`, `Status`, key handling, state transitions |
| `src/event.rs` | `AppEvent` enum |
| `src/youtube.rs` | yt-dlp subprocess wrapper (search, stream URL extraction) |
| `src/player.rs` | mpv subprocess spawning, IPC socket communication, progress tracking |
| `src/ui/mod.rs` | Top-level `draw()` function, vertical layout splitting |
| `src/ui/results.rs` | Search results `List` widget rendering |
| `src/ui/player.rs` | Now-playing bar `Paragraph` widget rendering |
| `src/ui/input.rs` | Input bar `Paragraph` widget rendering with cursor |

---

## Chunk 1: Project Scaffold + Core Types

### Task 1: Initialize Cargo project

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`

- [ ] **Step 1: Create Cargo.toml**

```toml
[package]
name = "yewtube-rs"
version = "0.1.0"
edition = "2021"

[dependencies]
ratatui = "0.29"
crossterm = "0.28"
tokio = { version = "1", features = ["rt-multi-thread", "process", "sync", "time", "macros", "io-util"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
```

- [ ] **Step 2: Create minimal src/main.rs**

```rust
fn main() {
    println!("yewtube-rs: not yet implemented");
}
```

- [ ] **Step 3: Verify it compiles**

Run: `cd /Users/macbookpro/Documents/CobaCoba/yewtube-rs && cargo build`
Expected: Compiles with no errors.

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock src/main.rs
git commit -m "feat: initialize cargo project with dependencies"
```

---

### Task 2: Define core types

**Files:**
- Create: `src/youtube.rs`
- Create: `src/event.rs`
- Create: `src/app.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Create src/youtube.rs**

```rust
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub duration: Option<u64>,
    pub channel: Option<String>,
}
```

- [ ] **Step 2: Create src/event.rs**

```rust
use anyhow::Result;

use crate::youtube::SearchResult;

#[derive(Debug)]
pub enum AppEvent {
    SearchComplete(Result<Vec<SearchResult>>),
    StreamUrlReady(Result<(String, String)>), // (url, title)
    PlaybackProgress { elapsed_secs: u64, duration_secs: u64 },
    PlaybackComplete,
    PlaybackError(String),
}
```

- [ ] **Step 3: Create src/app.rs**

```rust
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
        if self.results.is_empty() { return; }
        let i = match self.list_state.selected() {
            Some(i) if i >= self.results.len() - 1 => 0,
            Some(i) => i + 1,
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn select_prev(&mut self) {
        if self.results.is_empty() { return; }
        let i = match self.list_state.selected() {
            Some(0) => self.results.len() - 1,
            Some(i) => i - 1,
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn select_first(&mut self) {
        if !self.results.is_empty() { self.list_state.select(Some(0)); }
    }

    pub fn select_last(&mut self) {
        if !self.results.is_empty() { self.list_state.select(Some(self.results.len() - 1)); }
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
```

- [ ] **Step 4: Update src/main.rs**

```rust
mod app;
mod event;
mod youtube;

fn main() {
    println!("yewtube-rs: not yet implemented");
}
```

- [ ] **Step 5: Verify it compiles**

Run: `cd /Users/macbookpro/Documents/CobaCoba/yewtube-rs && cargo build`
Expected: Compiles with no errors.

- [ ] **Step 6: Commit**

```bash
git add src/app.rs src/event.rs src/youtube.rs src/main.rs
git commit -m "feat: define core types (App, AppEvent, SearchResult)"
```

---

## Chunk 2: YouTube Search + Stream Extraction

### Task 3: Implement yt-dlp search and stream extraction

**Files:**
- Modify: `src/youtube.rs`

- [ ] **Step 1: Replace src/youtube.rs with full implementation**

```rust
use anyhow::{Context, Result};
use serde::Deserialize;
use std::time::Duration;
use tokio::process::Command;

const YTDLP_TIMEOUT_SECS: u64 = 30;

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub duration: Option<u64>,
    pub channel: Option<String>,
}

/// Search YouTube using yt-dlp. Returns up to 10 results per page.
/// `page` is 0-indexed. Times out after 30 seconds.
pub async fn search(query: &str, page: usize) -> Result<Vec<SearchResult>> {
    let start = page * 10 + 1;
    let end = (page + 1) * 10;
    let search_term = format!("ytsearch{start}-{end}:{query}");

    let output = tokio::time::timeout(
        Duration::from_secs(YTDLP_TIMEOUT_SECS),
        Command::new("yt-dlp")
            .arg(&search_term)
            .arg("--dump-json")
            .arg("--no-download")
            .output(),
    )
    .await
    .context("Search timed out.")?
    .context("Failed to run yt-dlp. Is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("yt-dlp search failed: {}", stderr.trim());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    for line in stdout.lines() {
        if line.trim().is_empty() { continue; }
        match serde_json::from_str::<SearchResult>(line) {
            Ok(r) => results.push(r),
            Err(e) => eprintln!("Warning: skipping unparseable result: {e}"),
        }
    }

    Ok(results)
}

/// Extract the best audio stream URL for a video ID. Times out after 30 seconds.
pub async fn get_stream_url(video_id: &str) -> Result<String> {
    let url = format!("https://www.youtube.com/watch?v={video_id}");

    let output = tokio::time::timeout(
        Duration::from_secs(YTDLP_TIMEOUT_SECS),
        Command::new("yt-dlp")
            .arg("-f")
            .arg("bestaudio/best")
            .arg("--get-url")
            .arg(&url)
            .output(),
    )
    .await
    .context("Stream extraction timed out.")?
    .context("Failed to run yt-dlp for stream extraction")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Stream extraction failed: {}", stderr.trim());
    }

    let url = String::from_utf8_lossy(&output.stdout);
    let url = url.trim().to_string();

    if url.is_empty() {
        anyhow::bail!("No stream URL returned");
    }

    Ok(url)
}

/// Check if yt-dlp is available on the system.
pub async fn check_ytdlp() -> Result<()> {
    let output = Command::new("yt-dlp")
        .arg("--version")
        .output()
        .await
        .context("yt-dlp not found. Install it with: brew install yt-dlp")?;

    if !output.status.success() {
        anyhow::bail!("yt-dlp check failed. Install it with: brew install yt-dlp");
    }

    Ok(())
}
```

Note: We do NOT use `--flat-playlist` because it strips `duration` and `channel` fields from the JSON output, making all results show "LIVE". Without it, search is slightly slower but results are complete.

- [ ] **Step 2: Verify it compiles**

Run: `cd /Users/macbookpro/Documents/CobaCoba/yewtube-rs && cargo build`
Expected: Compiles with no errors.

- [ ] **Step 3: Commit**

```bash
git add src/youtube.rs
git commit -m "feat: implement yt-dlp search and stream URL extraction"
```

---

## Chunk 3: mpv Player Module

### Task 4: Implement mpv player with IPC

**Files:**
- Create: `src/player.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Create src/player.rs**

The mpv child process is wrapped in `Arc<Mutex<>>` so both the `tokio::select!` branches can access it. The IPC reader uses `request_id` to correctly distinguish `time-pos` from `duration` responses.

```rust
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::process::Child;
use tokio::sync::{mpsc, oneshot, Mutex};

use crate::event::AppEvent;

/// Check if mpv is available on the system.
pub async fn check_mpv() -> Result<()> {
    let output = tokio::process::Command::new("mpv")
        .arg("--version")
        .output()
        .await
        .context("mpv not found. Install it with: brew install mpv")?;

    if !output.status.success() {
        anyhow::bail!("mpv check failed. Install it with: brew install mpv");
    }

    Ok(())
}

/// Play a stream URL via mpv. Sends events through `tx`.
/// Stops if `kill_rx` receives a signal.
pub async fn play(
    url: String,
    title: String,
    tx: mpsc::UnboundedSender<AppEvent>,
    mut kill_rx: oneshot::Receiver<()>,
) {
    let socket_path = format!("/tmp/yewtube-mpv-{}.sock", std::process::id());
    let _ = std::fs::remove_file(&socket_path);

    let child = match tokio::process::Command::new("mpv")
        .arg("--no-video")
        .arg("--idle=no")
        .arg(format!("--input-ipc-server={socket_path}"))
        .arg(&url)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            let _ = tx.send(AppEvent::PlaybackError(format!("Failed to start mpv: {e}")));
            return;
        }
    };

    let child = Arc::new(Mutex::new(child));

    // Spawn IPC progress reader task
    let ipc_tx = tx.clone();
    let ipc_socket = socket_path.clone();
    let ipc_handle = tokio::spawn(async move {
        // Wait for socket to appear (up to 5 seconds)
        for _ in 0..50 {
            if std::path::Path::new(&ipc_socket).exists() { break; }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        let stream = match UnixStream::connect(&ipc_socket).await {
            Ok(s) => s,
            Err(_) => return,
        };

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut request_id: u64 = 0;

        loop {
            // Request time-pos
            request_id += 1;
            let time_id = request_id;
            let req = format!(r#"{{"command":["get_property","time-pos"],"request_id":{time_id}}}"#);
            if writer.write_all(format!("{req}\n").as_bytes()).await.is_err() { break; }

            // Request duration
            request_id += 1;
            let dur_id = request_id;
            let req = format!(r#"{{"command":["get_property","duration"],"request_id":{dur_id}}}"#);
            if writer.write_all(format!("{req}\n").as_bytes()).await.is_err() { break; }
            let _ = writer.flush().await;

            // Read responses, using request_id to distinguish them
            let mut elapsed: Option<f64> = None;
            let mut duration: Option<f64> = None;

            for _ in 0..4 { // read up to 4 lines to account for unsolicited events
                let mut line = String::new();
                match reader.read_line(&mut line).await {
                    Ok(0) | Err(_) => break,
                    _ => {}
                }

                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&line) {
                    let rid = val.get("request_id").and_then(|v| v.as_u64()).unwrap_or(0);
                    let data = val.get("data");

                    if rid == time_id {
                        if let Some(d) = data.and_then(|v| v.as_f64()) {
                            elapsed = Some(d);
                        }
                    } else if rid == dur_id {
                        if let Some(d) = data.and_then(|v| v.as_f64()) {
                            duration = Some(d);
                        }
                    }
                    // ignore unsolicited events (no request_id match)
                }

                if elapsed.is_some() && duration.is_some() { break; }
            }

            if let (Some(e), Some(d)) = (elapsed, duration) {
                let _ = ipc_tx.send(AppEvent::PlaybackProgress {
                    elapsed_secs: e as u64,
                    duration_secs: d as u64,
                });
            }

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });

    // Wait for mpv to exit or kill signal
    let child_clone = child.clone();
    let result = tokio::select! {
        _ = &mut kill_rx => {
            // Kill signal: kill the mpv process
            let mut guard = child_clone.lock().await;
            let _ = guard.kill().await;
            None
        }
        status = async {
            let mut guard = child.lock().await;
            guard.wait().await
        } => {
            Some(status)
        }
    };

    ipc_handle.abort();
    let _ = std::fs::remove_file(&socket_path);
    let _ = tx.send(AppEvent::PlaybackComplete);
}

/// Set pause state on the mpv IPC socket.
pub async fn set_pause(paused: bool) -> Result<()> {
    let socket_path = format!("/tmp/yewtube-mpv-{}.sock", std::process::id());

    let stream = UnixStream::connect(&socket_path)
        .await
        .context("Failed to connect to mpv IPC socket")?;

    let (_, mut writer) = stream.into_split();

    let request = format!(r#"{{"command":["set_property","pause",{paused}]}}"#);
    writer.write_all(format!("{request}\n").as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}
```

- [ ] **Step 2: Update src/main.rs to include player module**

```rust
mod app;
mod event;
mod player;
mod youtube;

fn main() {
    println!("yewtube-rs: not yet implemented");
}
```

- [ ] **Step 3: Verify it compiles**

Run: `cd /Users/macbookpro/Documents/CobaCoba/yewtube-rs && cargo build`
Expected: Compiles with no errors.

- [ ] **Step 4: Commit**

```bash
git add src/player.rs src/main.rs
git commit -m "feat: implement mpv player with IPC socket communication"
```

---

## Chunk 4: UI Rendering

### Task 5: Implement UI rendering

**Files:**
- Create: `src/ui/mod.rs`
- Create: `src/ui/results.rs`
- Create: `src/ui/player.rs`
- Create: `src/ui/input.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Create src/ui/results.rs**

```rust
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{HighlightSpacing, List, ListItem},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    if app.results.is_empty() {
        let msg = match &app.status {
            crate::app::Status::Searching(text) => text.clone(),
            crate::app::Status::Error(text) => text.clone(),
            _ => match app.input_history.is_empty() {
                true => "Press / to search YouTube".to_string(),
                false => "No results found.".to_string(),
            },
        };
        let style = match &app.status {
            crate::app::Status::Searching(_) => Style::default().fg(Color::Yellow),
            crate::app::Status::Error(_) => Style::default().fg(Color::Red),
            _ => Style::default().fg(Color::DarkGray),
        };
        let paragraph = ratatui::widgets::Paragraph::new(msg).style(style);
        frame.render_widget(paragraph, area);
        return;
    }

    let items: Vec<ListItem> = app
        .results
        .iter()
        .enumerate()
        .map(|(i, result)| {
            let duration = result
                .duration
                .map(|d| crate::app::App::format_duration(d))
                .unwrap_or_else(|| "LIVE".to_string());

            let title = Line::from(vec![
                Span::styled(
                    format!(" {:>2}. ", i + 1),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(&result.title, Style::default().fg(Color::White)),
                Span::styled(
                    format!("  {}", duration),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            ListItem::new(title)
        })
        .collect();

    let list = List::new(items)
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(list, area, &mut app.list_state);
}
```

- [ ] **Step 2: Create src/ui/player.rs**

```rust
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, PlaybackState};

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let content = match &app.playback {
        Some(PlaybackState { title, duration_secs, elapsed_secs, paused }) => {
            let elapsed_str = App::format_duration(*elapsed_secs);
            let duration_str = App::format_duration(*duration_secs);
            let state_icon = if *paused { "||" } else { ">>" };

            vec![Line::from(vec![
                Span::styled(format!(" {state_icon} "), Style::default().fg(Color::Cyan)),
                Span::styled(format!("{title}"), Style::default().fg(Color::Cyan)),
                Span::styled(format!("  {elapsed_str}/{duration_str}"), Style::default().fg(Color::Green)),
            ])]
        }
        None => match &app.status {
            crate::app::Status::Loading(text) => {
                vec![Line::from(Span::styled(
                    format!(" {text}"),
                    Style::default().fg(Color::Yellow),
                ))]
            }
            _ => vec![Line::from("")],
        },
    };

    let paragraph = Paragraph::new(content);
    frame.render_widget(paragraph, area);
}
```

- [ ] **Step 3: Create src/ui/input.rs**

```rust
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, Mode};

pub fn render(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let prompt = match app.mode {
        Mode::Browse => " > ",
        Mode::Input => " / ",
    };

    let mut spans = vec![
        Span::styled(prompt, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled(&app.input_text, Style::default().fg(Color::White)),
    ];

    if app.mode == Mode::Input {
        spans.push(Span::styled("_", Style::default().fg(Color::White)));
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}
```

- [ ] **Step 4: Create src/ui/mod.rs**

```rust
mod results;
mod player;
mod input;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),     // results area
            Constraint::Length(1),  // player bar
            Constraint::Length(1),  // input bar
        ])
        .split(area);

    results::render(frame, vertical[0], app);
    player::render(frame, vertical[1], app);
    input::render(frame, vertical[2], app);
}
```

- [ ] **Step 5: Update src/main.rs to include ui module**

```rust
mod app;
mod event;
mod player;
mod ui;
mod youtube;

fn main() {
    println!("yewtube-rs: not yet implemented");
}
```

- [ ] **Step 6: Verify it compiles**

Run: `cd /Users/macbookpro/Documents/CobaCoba/yewtube-rs && cargo build`
Expected: Compiles with no errors.

- [ ] **Step 7: Commit**

```bash
git add src/ui/ src/main.rs
git commit -m "feat: implement UI rendering (results list, player bar, input bar)"
```

---

## Chunk 5: Key Handling (Browse Mode)

### Task 6: Add browse mode key handling

**Files:**
- Modify: `src/app.rs`

- [ ] **Step 1: Add action enums and browse key handler to src/app.rs**

Append to `src/app.rs`, after the existing `impl App` block:

```rust
pub enum BrowseAction {
    None,
    Play(String, String), // video_id, title
    NextPage,
    PrevPage,
    TogglePause,
}

pub enum InputAction {
    None,
    Search(String),
}

impl App {
    pub fn handle_browse_key(&mut self, key: crossterm::event::KeyEvent) -> BrowseAction {
        use crossterm::event::{KeyCode, KeyModifiers};

        // Ctrl+C always quits regardless of other bindings
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.kill_mpv();
            self.should_quit = true;
            return BrowseAction::None;
        }

        match key.code {
            KeyCode::Char('q') => {
                self.kill_mpv();
                self.should_quit = true;
                BrowseAction::None
            }
            KeyCode::Char('/') => {
                self.mode = Mode::Input;
                self.input_text.clear();
                self.input_cursor = 0;
                BrowseAction::None
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.select_next();
                BrowseAction::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.select_prev();
                BrowseAction::None
            }
            KeyCode::Char('g') | KeyCode::Home => {
                self.select_first();
                BrowseAction::None
            }
            KeyCode::Char('G') | KeyCode::End => {
                self.select_last();
                BrowseAction::None
            }
            KeyCode::Char('n') => BrowseAction::NextPage,
            KeyCode::Char('p') => BrowseAction::PrevPage,
            KeyCode::Char(' ') => BrowseAction::TogglePause,
            KeyCode::Enter => {
                if let Some(result) = self.selected_result() {
                    BrowseAction::Play(result.id.clone(), result.title.clone())
                } else {
                    BrowseAction::None
                }
            }
            _ => BrowseAction::None,
        }
    }
}
```

Note: `Ctrl+C` is checked before the match to avoid the `Char('c')` wildcard catching it. The `Char('G')` arm works because uppercase G is a different `KeyCode::Char` value than lowercase chars. The `Home`/`End` keys don't conflict with any `Char` variants.

- [ ] **Step 2: Verify it compiles**

Run: `cd /Users/macbookpro/Documents/CobaCoba/yewtube-rs && cargo build`
Expected: Compiles with no errors.

- [ ] **Step 3: Commit**

```bash
git add src/app.rs
git commit -m "feat: add browse mode key handling"
```

---

## Chunk 6: Key Handling (Input Mode)

### Task 7: Add input mode key handling

**Files:**
- Modify: `src/app.rs`

- [ ] **Step 1: Add input key handler to the second `impl App` block in src/app.rs**

Append to the second `impl App` block (the one with `handle_browse_key`):

```rust
    pub fn handle_input_key(&mut self, key: crossterm::event::KeyEvent) -> InputAction {
        use crossterm::event::{KeyCode, KeyModifiers};

        match key.code {
            KeyCode::Esc => {
                self.mode = Mode::Browse;
                InputAction::None
            }
            KeyCode::Enter => {
                let text = self.input_text.trim().to_string();
                if text.is_empty() {
                    return InputAction::None;
                }

                // Add to history (skip duplicates)
                if self.input_history.last().map(|s| s.as_str()) != Some(text.as_str()) {
                    self.input_history.push(text.clone());
                }
                self.history_index = self.input_history.len();

                // Check for commands (: prefix)
                if let Some(cmd) = text.strip_prefix(':') {
                    match cmd.trim() {
                        "q" => {
                            self.kill_mpv();
                            self.should_quit = true;
                            return InputAction::None;
                        }
                        _ => {
                            self.status = Status::Error(format!("Unknown command: :{cmd}"));
                            return InputAction::None;
                        }
                    }
                }

                self.mode = Mode::Browse;
                self.page = 0;
                InputAction::Search(text)
            }
            KeyCode::Backspace => {
                if self.input_cursor > 0 {
                    self.input_cursor -= 1;
                    if let Some((idx, _)) = self.input_text.char_indices().nth(self.input_cursor) {
                        self.input_text.remove(idx);
                    }
                }
                InputAction::None
            }
            KeyCode::Delete => {
                let char_count = self.input_text.chars().count();
                if self.input_cursor < char_count {
                    if let Some((idx, _)) = self.input_text.char_indices().nth(self.input_cursor) {
                        self.input_text.remove(idx);
                    }
                }
                InputAction::None
            }
            KeyCode::Left => {
                if self.input_cursor > 0 {
                    self.input_cursor -= 1;
                }
                InputAction::None
            }
            KeyCode::Right => {
                if self.input_cursor < self.input_text.chars().count() {
                    self.input_cursor += 1;
                }
                InputAction::None
            }
            KeyCode::Home => {
                self.input_cursor = 0;
                InputAction::None
            }
            KeyCode::End => {
                self.input_cursor = self.input_text.chars().count();
                InputAction::None
            }
            KeyCode::Up => {
                if !self.input_history.is_empty() && self.history_index > 0 {
                    self.history_index -= 1;
                    self.input_text = self.input_history[self.history_index].clone();
                    self.input_cursor = self.input_text.chars().count();
                }
                InputAction::None
            }
            KeyCode::Down => {
                if !self.input_history.is_empty() && self.history_index < self.input_history.len() - 1 {
                    self.history_index += 1;
                    self.input_text = self.input_history[self.history_index].clone();
                    self.input_cursor = self.input_text.chars().count();
                } else if !self.input_history.is_empty() {
                    self.history_index = self.input_history.len();
                    self.input_text.clear();
                    self.input_cursor = 0;
                }
                InputAction::None
            }
            KeyCode::Char(c) => {
                // Handle Ctrl+A, Ctrl+E, Ctrl+U via modifier check
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match c {
                        'a' => { self.input_cursor = 0; }
                        'e' => { self.input_cursor = self.input_text.chars().count(); }
                        'u' => { self.input_text.clear(); self.input_cursor = 0; }
                        _ => {}
                    }
                    return InputAction::None;
                }

                // Normal character input
                if let Some((idx, _)) = self.input_text.char_indices().nth(self.input_cursor) {
                    self.input_text.insert(idx, c);
                } else {
                    self.input_text.push(c);
                }
                self.input_cursor += 1;
                InputAction::None
            }
            _ => InputAction::None,
        }
    }
```

Key design decisions:
- `Home`/`End` have their own dedicated arms (no or-pattern with guarded `Char('a')`/`Char('e')`).
- `Ctrl+A`/`Ctrl+E`/`Ctrl+U` are handled inside the `Char(c)` arm by checking `key.modifiers` first, before the normal char insertion logic. This avoids match arm overlap.
- `input_cursor` is consistently a **char index** (not byte offset). All conversions to byte offsets happen via `char_indices().nth()`.
- `history_index` underflow is prevented by checking `!self.input_history.is_empty()` before comparing `len() - 1`.

- [ ] **Step 2: Verify it compiles**

Run: `cd /Users/macbookpro/Documents/CobaCoba/yewtube-rs && cargo build`
Expected: Compiles with no errors.

- [ ] **Step 3: Commit**

```bash
git add src/app.rs
git commit -m "feat: add input mode key handling with editing and history"
```

---

## Chunk 7: Main Event Loop + Wiring

### Task 8: Wire up the main event loop

**Files:**
- Modify: `src/main.rs` (complete rewrite)

- [ ] **Step 1: Replace src/main.rs**

```rust
mod app;
mod event;
mod player;
mod ui;
mod youtube;

use std::time::Duration;

use anyhow::Result;
use crossterm::event::{Event, KeyEventKind};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

use app::{App, BrowseAction, InputAction};
use event::AppEvent;

fn main() -> Result<()> {
    // Install panic hook to restore terminal on panic
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        default_panic(info);
    }));

    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async_main())
}

async fn async_main() -> Result<()> {
    // Startup checks
    if let Err(e) = youtube::check_ytdlp().await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
    if let Err(e) = player::check_mpv().await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    let terminal = ratatui::init();
    let result = run(terminal).await;
    ratatui::restore();
    result
}

async fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    // Channel for async events (search results, playback updates)
    let (tx_app, mut rx_app) = mpsc::unbounded_channel::<AppEvent>();

    // Bridge crossterm events into a tokio channel
    let (tx_term, mut rx_term) = mpsc::channel(100);
    tokio::spawn(async move {
        loop {
            if crossterm::event::poll(Duration::from_millis(100)).is_ok() {
                if let Ok(event) = crossterm::event::read() {
                    if tx_term.send(event).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        tokio::select! {
            Some(term_event) = rx_term.recv() => {
                if let Event::Key(key) = term_event {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    handle_key(&mut app, key, &tx_app);
                }
            }
            Some(app_event) = rx_app.recv() => {
                handle_app_event(&mut app, app_event, &tx_app);
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn handle_key(app: &mut App, key: crossterm::event::KeyEvent, tx: &mpsc::UnboundedSender<AppEvent>) {
    match app.mode {
        app::Mode::Browse => {
            let action = app.handle_browse_key(key);
            match action {
                BrowseAction::Play(video_id, title) => {
                    app.kill_mpv();
                    app.status = app::Status::Loading("Loading stream...".into());
                    let tx = tx.clone();
                    tokio::spawn(async move {
                        let result = youtube::get_stream_url(&video_id).await;
                        let _ = tx.send(AppEvent::StreamUrlReady(result.map(|url| (url, title))));
                    });
                }
                BrowseAction::NextPage => {
                    if !app.results.is_empty() {
                        app.page += 1;
                        let query = app.input_history.last().cloned().unwrap_or_default();
                        search(app, query, tx);
                    }
                }
                BrowseAction::PrevPage => {
                    if app.page > 0 {
                        app.page -= 1;
                        let query = app.input_history.last().cloned().unwrap_or_default();
                        search(app, query, tx);
                    }
                }
                BrowseAction::TogglePause => {
                    if let Some(ref playback) = app.playback {
                        let new_paused = !playback.paused;
                        if let Some(ref mut pb) = app.playback {
                            pb.paused = new_paused;
                        }
                        let tx = tx.clone();
                        tokio::spawn(async move {
                            let _ = player::set_pause(new_paused).await;
                        });
                    }
                }
                BrowseAction::None => {}
            }
        }
        app::Mode::Input => {
            let action = app.handle_input_key(key);
            match action {
                InputAction::Search(query) => {
                    search(app, query, tx);
                }
                InputAction::None => {}
            }
        }
    }
}

fn search(app: &mut App, query: String, tx: &mpsc::UnboundedSender<AppEvent>) {
    app.status = app::Status::Searching("Searching...".into());
    app.results.clear();
    app.list_state.select(Some(0));
    let page = app.page;
    let tx = tx.clone();
    tokio::spawn(async move {
        let result = youtube::search(&query, page).await;
        let _ = tx.send(AppEvent::SearchComplete(result));
    });
}

fn handle_app_event(app: &mut App, event: AppEvent, tx: &mpsc::UnboundedSender<AppEvent>) {
    match event {
        AppEvent::SearchComplete(result) => {
            match result {
                Ok(results) => {
                    app.results = results;
                    app.status = app::Status::Idle;
                    if !app.results.is_empty() {
                        app.list_state.select(Some(0));
                    }
                }
                Err(e) => {
                    app.status = app::Status::Error(format!("Search failed: {e}"));
                }
            }
        }
        AppEvent::StreamUrlReady(result) => {
            match result {
                Ok((url, title)) => {
                    app.status = app::Status::Idle;
                    app.playback = Some(app::PlaybackState {
                        title: title.clone(),
                        duration_secs: 0,
                        elapsed_secs: 0,
                        paused: false,
                    });

                    let (kill_tx, kill_rx) = tokio::sync::oneshot::channel::<()>();
                    app.mpv_kill = Some(kill_tx);

                    let tx = tx.clone();
                    tokio::spawn(async move {
                        player::play(url, title, tx, kill_rx).await;
                    });
                }
                Err(e) => {
                    app.status = app::Status::Error(format!("Playback failed: {e}"));
                }
            }
        }
        AppEvent::PlaybackProgress { elapsed_secs, duration_secs } => {
            if let Some(ref mut pb) = app.playback {
                pb.elapsed_secs = elapsed_secs;
                pb.duration_secs = duration_secs;
            }
        }
        AppEvent::PlaybackComplete => {
            app.playback = None;
            app.mpv_kill = None;
        }
        AppEvent::PlaybackError(msg) => {
            app.playback = None;
            app.mpv_kill = None;
            app.status = app::Status::Error(msg);
        }
    }
}
```

Note: `handle_key` and `search` are synchronous functions (not `async fn`). `tokio::spawn` works from synchronous context because we're already inside the tokio runtime. This avoids the borrow checker issue of holding `&mut App` across an await point.

- [ ] **Step 2: Verify it compiles**

Run: `cd /Users/macbookpro/Documents/CobaCoba/yewtube-rs && cargo build`
Expected: Compiles with no errors. Fix any compilation issues before proceeding.

- [ ] **Step 3: Test the full application**

Run: `cd /Users/macbookpro/Documents/CobaCoba/yewtube-rs && cargo run`

Test scenarios:
1. Launch -> see "Press / to search YouTube"
2. Press `/` -> input mode activates (cursor visible)
3. Type "never gonna give you up" + Enter -> "Searching..." then results appear
4. `j`/`k` to navigate, `g`/`G` for top/bottom
5. `Enter` to play a result -> mpv plays audio, player bar shows title
6. `Space` -> pause/resume
7. `n` -> next page of results
8. `q` -> quits cleanly, terminal restored

- [ ] **Step 4: Commit**

```bash
git add src/main.rs
git commit -m "feat: wire up event loop, key handling, and async search/playback"
```

---

## Chunk 8: Final Polish

### Task 9: Add .gitignore and verify

**Files:**
- Create: `.gitignore`

- [ ] **Step 1: Create .gitignore**

```
/target
*.sock
```

- [ ] **Step 2: Commit**

```bash
git add .gitignore
git commit -m "chore: add .gitignore"
```

- [ ] **Step 3: Verify final release build**

Run: `cd /Users/macbookpro/Documents/CobaCoba/yewtube-rs && cargo build --release`
Expected: Clean release build with no warnings.
