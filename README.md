# blivedm_rs

Bilibili live room DM (Danmaku) websocket client library for Rust.

[中文版本 README](README.zh.md)

## ✨ New Feature: Automatic Browser Cookie Detection

**No more manual Cookie extraction!** The client now automatically detects bilibili cookies from your browser.

```bash
# Just run without cookies - it will auto-detect from your browser!
cargo run --bin danmu -- --room-id 24779526
# Still works with manual cookies if needed
cargo run --bin danmu -- --room-id 24779526
# Or, with explicit argument:
cargo run --bin danmu -- --room-id 24779526 --cookies "SESSDATA=your_sessdata; other_cookie=..."
```

Supports Chrome, Firefox, Edge, Chromium, and Opera on Linux, macOS, and Windows. See [Browser Cookie Documentation](docs/browser-cookies.md) for details.

## Quick Start

The detailed used guide on the [Danmu](docs/danmu.md) page.

### Building from Source

```bash
# Clone the repository
git clone https://github.com/jiahaoxiang2000/blivedm_rs.git
cd blivedm_rs

# Build the project
cargo build --release

# Run the danmu client (auto-detects browser cookies)
./target/release/danmu --room-id 24779526

# Or with manual cookies (must include SESSDATA)
./target/release/danmu --cookies "SESSDATA=your_sessdata; other_cookie=..." --room-id 24779526
```

### System Requirements

- **Rust**: Latest stable version
- **Linux**: 
  - Audio support: `sudo apt-get install libasound2-dev`
  - Build tools: `sudo apt-get install pkg-config libssl-dev`
  - Optional TTS: `sudo apt-get install espeak-ng`
- **macOS**: No additional dependencies (uses built-in `say` command for TTS)

### Pre-built Binaries

Pre-built binaries will be available in future releases. Currently, please build from source using the instructions above.

### TTS Server Setup (Optional)

For advanced TTS functionality, you can set up the danmu-tts server:

```bash
# Clone and setup the TTS server
git clone https://github.com/jiahaoxiang2000/danmu-tts.git
cd danmu-tts
# Follow the setup instructions in the repository
```

The TTS server provides high-quality neural voices and multiple TTS backends. See the [danmu-tts repository](https://github.com/jiahaoxiang2000/danmu-tts) for detailed setup instructions.

### Usage Examples

```bash
# NEW: Auto-detect browser cookies (recommended)
./target/release/danmu --room-id 12345

# Manual cookies (must include SESSDATA)
./target/release/danmu --cookies "SESSDATA=your_sessdata; other_cookie=..." --room-id 12345

# With TTS REST API server
./target/release/danmu --room-id 12345 --tts-server http://localhost:8000 --tts-volume 0.7

# With local TTS (macOS)
./target/release/danmu --room-id 12345 --tts-command say --tts-args "-v,Mei-Jia"

# With local TTS (Linux)
./target/release/danmu --room-id 12345 --tts-command espeak-ng --tts-args "-v,cmn"

# Show all available options
./target/release/danmu --help
```


## Documentation

The full documentation is available in the [`docs/`](docs/) folder. Here are the main sections:

- [Getting Started](docs/README.md): Introduction and setup instructions.
- [Browser Cookie Auto-Detection](docs/browser-cookies.md): **NEW!** How automatic cookie detection works.
- [Usage Guide](docs/usage.md): How to use the library in your projects.
- [Architecture](docs/architecture.md): Project architecture and design.
- [Client Module](docs/client.md): Details about the client implementation.
- [Danmu Module](docs/danmu.md): Information on the danmu (bullet chat) module.
- [Scheduler](docs/scheduler.md): Overview of the scheduler component.
- [Plugins](docs/plugins.md): Available plugins and how to use them.

The Library Documentation is also available at [pages](https://jiahaoxiang2000.github.io/blivedm_rs/).

## Reference

- [blivedm](https://github.com/xfgryujk/blivedm): Original Python implementation of the Bilibili live DM protocol.
- [blivedm-rs](https://github.com/yanglul/blivedm_rs): Rust port of the blivedm library.
- [bililive-rs](https://github.com/LightQuantumArchive/bililive-rs): Another Rust implementation for Bilibili live streaming.
- [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect): Bilibili API collection by SocialSisterYi.