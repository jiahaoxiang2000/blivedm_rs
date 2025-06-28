# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`blivedm_rs` is a Rust workspace implementing a Bilibili live room danmaku (bullet chat) WebSocket client library. The project features automatic browser cookie detection, TTS capabilities, and an extensible plugin architecture.

## Workspace Architecture

This is a Rust workspace with four main packages:

- **`client/`** - Core library with WebSocket client, authentication, and browser cookie detection
- **`danmu/`** - Main CLI executable for connecting to Bilibili live rooms  
- **`plugins/`** - Plugin system (terminal display, TTS functionality)
- **`examples/`** - Usage examples and demonstrations

## Development Commands

### Building
```bash
# Build entire workspace
cargo build

# Build release version
cargo build --release

# Build specific package
cargo build -p client
cargo build -p danmu
cargo build -p plugins
cargo build -p examples
```

### Running
```bash
# Main danmu client (auto-detects browser cookies)
cargo run --bin danmu -- --room-id 24779526

# With manual cookies
cargo run --bin danmu -- --room-id 24779526 --cookies "SESSDATA=your_sessdata; other_cookie=..."

# With TTS server
cargo run --bin danmu -- --room-id 24779526 --tts-server http://localhost:8000

# TTS example
cargo run --bin tts_example

# Integration test client (exists in client/src/bin/)
cargo run --bin integration_bili_live_client
```

### Testing
```bash
# Run all tests
cargo test

# Test specific package
cargo test -p client
```

### Browser Cookie Testing
The `browser_cookie_test` binary was removed from source but compiled versions may still exist in `target/`. The browser cookie functionality is tested through the main `danmu` client when no cookies are provided.

## Key Architecture Components

### WebSocket Client (`client/src/websocket.rs`)
- `BiliLiveClient` - Main WebSocket client for Bilibili live rooms
- Handles connection, authentication, and message parsing
- Supports real-time danmaku message reception

### Authentication (`client/src/auth.rs`)
- Room authentication and token management
- Integrates with browser cookie detection

### Browser Cookie Detection (`client/src/browser_cookies.rs`)
- Automatically detects SESSDATA from Chrome, Firefox, Edge, Chromium, Opera
- Cross-platform support (Linux, macOS, Windows)
- Reads from browser SQLite databases

### Plugin System (`plugins/src/`)
- `terminal_display.rs` - Displays messages in terminal
- `tts.rs` - Text-to-speech with REST API and local command support
- Extensible architecture for adding new functionality

### Scheduler (`client/src/scheduler.rs`)
- Event-driven message processing
- Supports sequential or parallel event handling
- Coordinates between WebSocket client and plugins

## System Dependencies

### Linux
```bash
sudo apt-get install libasound2-dev pkg-config libssl-dev
# Optional for local TTS
sudo apt-get install espeak-ng
```

### macOS
- Uses built-in `say` command for TTS
- No additional dependencies required

## External TTS Server
For advanced TTS functionality, set up the companion `danmu-tts` server:
```bash
git clone https://github.com/jiahaoxiang2000/danmu-tts.git
cd danmu-tts
# Follow setup instructions in that repository
```

## Entry Points

- **Main CLI**: `danmu/src/main.rs` - Primary executable with clap argument parsing
- **Library**: `client/src/lib.rs` - Exposes auth, browser_cookies, models, scheduler, websocket modules  
- **Plugins**: `plugins/src/lib.rs` - Plugin system with helper functions
- **Examples**: `examples/tts_example.rs` - TTS functionality demonstration

## Configuration Notes

- The project uses Rust edition 2021/2024
- WebSocket connections use `tungstenite` crate
- HTTP requests use `reqwest` with rustls-tls
- Browser cookie reading via `sqlite` crate
- Async runtime provided by `tokio`
- CLI parsing via `clap` with derive features

## Plugin Development

When creating new plugins:
1. Implement event handlers in `plugins/src/`
2. Use the existing pattern from `terminal_display.rs` and `tts.rs`
3. Register handlers with the scheduler in `danmu/src/main.rs`
4. Follow the async event-driven architecture
```

## GitHub Issue Management

- Use `gh issue create` to open a new issue in the repository
- Use `gh issue list` to view existing issues
- Use `gh issue view [issue-number]` to see details of a specific issue
- Use `gh issue close [issue-number]` to close an issue
- Use `gh issue reopen [issue-number]` to reopen a previously closed issue