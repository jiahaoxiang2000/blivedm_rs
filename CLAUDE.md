# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`blivedm_rs` is a single-package Rust project implementing a Bilibili live room danmaku WebSocket client with automatic browser cookie detection, TTS capabilities, and plugin architecture.

## Project Architecture

This is a **single package** with both library and binary targets:

- **Library** (`src/lib.rs`) - Exports all client and plugin functionality
- **Binary** (`src/main.rs`) - CLI executable `blivedm`
- **Client Module** (`src/client/`) - WebSocket, auth, browser cookies
- **Plugins Module** (`src/plugins/`) - Terminal display, TTS, auto-reply
- **Examples** (`examples/`) - Usage examples (tts_example, integration_bili_live_client)

## Development Commands

```bash
# Build
cargo build
cargo build --release

# Build examples
cargo build --examples

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

# Run examples
cargo run --example tts_example
cargo run --example integration_bili_live_client

# Test
cargo test
```

## Key Components

- **WebSocket Client** (`src/client/websocket.rs`) - `BiliLiveClient` handles connection and message parsing
- **Authentication** (`src/client/auth.rs`) - Room auth and token management
- **Browser Cookies** (`src/client/browser_cookies.rs`) - Auto-detects SESSDATA from major browsers
- **Plugin System** (`src/plugins/`) - Terminal display, TTS, and auto-reply functionality
- **Scheduler** (`src/client/scheduler.rs`) - Event-driven message processing

## Entry Points

- **Library**: `src/lib.rs` - Exports all modules (client, plugins)
- **Binary**: `src/main.rs` - CLI application
- **Examples**: `examples/*.rs` - Usage examples

## Release Process

To publish a new version to crates.io:

1. **Update version in Cargo.toml**: `version = "x.y.z"`
2. **Commit the version change**: `git commit -am "chore: bump version to x.y.z"`
3. **Create and push a tag**: `git tag vx.y.z && git push origin vx.y.z`

The GitHub Actions workflow will automatically:

- Verify the tag version matches Cargo.toml version
- Publish the package to crates.io
- Create a GitHub release with generated notes

**Important**: The tag version (e.g., `v0.4.2`) must match the Cargo.toml version (e.g., `0.4.2`) or the workflow will fail.
