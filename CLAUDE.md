# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`blivedm_rs` is a Rust workspace implementing a Bilibili live room danmaku WebSocket client with automatic browser cookie detection, TTS capabilities, and plugin architecture.

## Workspace Architecture

- **Root (src/)** - Main CLI executable (`blivedm`)
- **`client/`** - Core library (WebSocket, auth, browser cookies)
- **`plugins/`** - Plugin system (terminal display, TTS)
- **`examples/`** - Usage examples

## Development Commands

```bash
# Build
cargo build
cargo build --release
cargo build -p client  # specific package

# Install globally
cargo install --locked --path .

# Run main client (auto-detects browser cookies)
cargo run -- --room-id 24779526

# With manual cookies or TTS
cargo run -- --room-id 24779526 --cookies "SESSDATA=..."
cargo run -- --room-id 24779526 --tts-server http://localhost:8000

# Configuration file support
cargo run -- --config config.toml
cargo run -- --print-config

# Other binaries
cargo run --bin tts_example
cargo run --bin integration_bili_live_client

# Test
cargo test
cargo test -p client
```

## Key Components

- **WebSocket Client** (`client/src/websocket.rs`) - `BiliLiveClient` handles connection and message parsing
- **Authentication** (`client/src/auth.rs`) - Room auth and token management
- **Browser Cookies** (`client/src/browser_cookies.rs`) - Auto-detects SESSDATA from major browsers
- **Plugin System** (`plugins/src/`) - Terminal display and TTS functionality
- **Scheduler** (`client/src/scheduler.rs`) - Event-driven message processing

## Configuration

TOML configuration files supported with precedence: CLI args > env vars > config file > defaults.

Config locations: `--config path`, `config.toml`, `~/.config/blivedm_rs/config.toml`

```toml
[connection]
room_id = "24779526"

[tts]
server = "http://localhost:8000"
volume = 0.8

debug = false
```

## System Dependencies

**Linux**: `sudo apt-get install libasound2-dev pkg-config libssl-dev espeak-ng`
**macOS**: Uses built-in `say` command
**External TTS**: Clone `https://github.com/jiahaoxiang2000/danmu-tts.git`

## Entry Points

- **CLI**: `src/main.rs`
- **Library**: `client/src/lib.rs`
- **Plugins**: `plugins/src/lib.rs`

## Plugin Development

1. Implement handlers in `plugins/src/`
2. Follow patterns from `terminal_display.rs` and `tts.rs`
3. Register with scheduler in `src/main.rs`
4. Use async event-driven architecture