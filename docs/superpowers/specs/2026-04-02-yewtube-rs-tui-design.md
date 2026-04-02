# yewtube-rs: Rust TUI YouTube Player — Design Spec

**Date**: 2026-04-02
**Status**: Approved
**Scope**: Minimal player (search + play via mpv)

## Overview

A Rust TUI rewrite of yewtube (Python terminal YouTube player). Minimal scope: search YouTube, play audio via mpv. No download, no playlists, no integrations.

## Project Setup

- **Location**: `/Users/macbookpro/Documents/CobaCoba/yewtube-rs/`
- **Architecture**: Single crate, flat module structure
- **Edition**: Rust 2021

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `ratatui` | 0.29 | TUI rendering |
| `crossterm` | 0.28 | Terminal input/events |
| `tokio` | 1 (full features) | Async runtime, subprocess management |
| `serde` + `serde_json` | 1 | Parse yt-dlp JSON output |
| `anyhow` | 1 | Error handling |

## Module Structure

```
src/
├── main.rs          # Terminal init, event loop, teardown
├── app.rs           # App state struct + state transitions
├── ui/
│   ├── mod.rs       # Top-level draw function, layout
│   ├── results.rs   # Search results list rendering
│   ├── player.rs    # Now-playing bar rendering
│   └── input.rs     # Input bar rendering
├── youtube.rs       # yt-dlp subprocess: search + stream URL extraction
├── player.rs        # mpv subprocess management
└── event.rs         # App event types (search done, playback updates)
```

## UI Layout

Three zones stacked vertically:

```
┌──────────────────────────────────────────┐
│ Search Results (scrollable list)    ~70% │
│  1. Artist - Song Title      3:42       │
│ >>2. Artist - Song Title     4:15       │
│  3. Artist - Song Title      2:58       │
├──────────────────────────────────────────┤
│ ♪ Now Playing: Artist - Song   1:23/4:15│  <- player bar (1 line)
├──────────────────────────────────────────┤
│ /search term_                            │  <- input bar (1 line)
└──────────────────────────────────────────┘
```

### Widgets

- **Results area**: ratatui `List` widget with `ListState`. Each item shows index, title, channel, duration. Highlighted selection via `highlight_style`.
- **Player bar**: ratatui `Paragraph` showing current track title, elapsed/total time, play state.
- **Input bar**: ratatui `Paragraph` for the input field. App manages cursor position internally.

## Interaction Model

### Modes

- **Browse mode** (default): Navigate results, trigger playback
- **Input mode**: Type search queries or commands

### Key Bindings

| Key | Mode | Action |
|-----|------|--------|
| `/` | Browse | Focus input (switch to input mode) |
| `Enter` | Browse | Play selected result |
| `Enter` | Input | Execute search, return to browse |
| `Esc` | Input | Cancel input, return to browse |
| `j` / `↓` | Browse | Move selection down |
| `k` / `↑` | Browse | Move selection up |
| `n` | Browse | Next page of results |
| `p` | Browse | Previous page of results |
| `1-9` | Browse | Play result by number |
| `Space` | Browse | Toggle pause (mpv IPC) |
| `q` | Browse | Quit |
| `:q` + Enter | Input | Quit |

## Data Flow

```
User types query + Enter
  -> Spawn tokio task
  -> Run: yt-dlp "ytsearch10:query" --dump-json --flat-playlist
  -> Parse JSON lines into Vec<SearchResult>
  -> Send AppEvent::SearchComplete through mpsc channel
  -> Main loop receives event, updates app state, redraws

User selects track (Enter or number)
  -> Spawn tokio task
  -> Run: yt-dlp -f bestaudio --get-url "https://youtube.com/watch?v=ID"
  -> Get stream URL
  -> Spawn mpv subprocess: mpv --no-video --really-quiet <url>
  -> Background task waits on mpv process exit
  -> On exit: send AppEvent::PlaybackComplete, clear playback state
```

## Key Types

```rust
enum Mode {
    Browse,
    Input,
}

struct App {
    mode: Mode,
    results: Vec<SearchResult>,
    list_state: ListState,
    current_page: usize,
    input_text: String,
    playback: Option<PlaybackState>,
    should_quit: bool,
}

struct SearchResult {
    id: String,
    title: String,
    duration: String,
    channel: String,
}

struct PlaybackState {
    title: String,
    duration_secs: u64,
    started_at: Instant,
    mpv_child: Child,
}
```

## Event System

Async events communicated via `tokio::sync::mpsc`:

```rust
enum AppEvent {
    SearchComplete(Result<Vec<SearchResult>>),
    StreamUrlReady(Result<String>),
    PlaybackStarted,
    PlaybackComplete,
    PlaybackError(String),
}
```

Main event loop:
1. Poll crossterm terminal events (16ms timeout)
2. Poll mpsc receiver for async events (non-blocking)
3. Handle events, update state, redraw

## External Dependencies (System)

- **yt-dlp**: Must be installed. Used for search (`ytsearchN:` prefix + `--dump-json`) and stream URL extraction (`--get-url`).
- **mpv**: Must be installed. Used for audio playback with `--no-video --really-quiet`.

## Error Handling

- **yt-dlp not found**: Display error in results area with install instructions
- **mpv not found**: Display error in player bar with install instructions
- **No results**: Show "No results found." in list
- **Network timeout**: 30s timeout on search, show "Search timed out."
- **Stream extraction failure**: Show "Playback failed" in player bar
- **Terminal resize**: Layout adapts automatically via `frame.area()` on each draw

No crashes — all errors displayed in the TUI.
