# blivedm_rs

A powerful Bilibili live room DM (Danmaku) WebSocket client library for Rust, supporting real-time danmaku monitoring, Text-to-Speech (TTS), and automatic browser cookie detection.

[中文版本 README](README.md)

## 🚀 Key Features

- **🔍 Smart Cookie Detection** - Automatically detect login status from popular browsers (Chrome, Firefox, Edge, Opera)
- **💬 Real-time Danmaku Monitoring** - Connect to Bilibili live rooms and receive real-time messages including danmaku, gifts, and room entries
- **🔊 Multi-platform TTS Support** - Support local TTS (Windows PowerShell, macOS say, Linux espeak-ng) and remote TTS servers
- **🎛️ Plugin Architecture** - Modular design with support for custom plugin extensions
- **🖥️ Cross-platform Support** - Native support for Windows, macOS, Linux with pre-compiled binaries
- **⚡ High-performance Async** - Built on Tokio async architecture for low resource usage and high concurrency
- **🔧 Flexible Configuration** - Support command-line parameter configuration with customizable TTS volume, voice parameters

## 🎯 Use Cases

- **Live Stream Monitoring** - Real-time monitoring of streamer-audience interactions and feedback
- **Voice Broadcasting** - Convert danmaku content to speech via TTS, freeing your eyes
- **Data Analysis** - Collect live room interaction data for user behavior analysis
- **Auto Response** - Trigger automated responses based on danmaku content (requires custom plugins)
- **Content Moderation** - Monitor live room content and identify inappropriate information
- **Fan Interaction** - Enhance interaction experience between streamers and audiences

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

### Pre-built Binaries (Recommended)

Pre-built binaries are now available! Download the appropriate version for your system from the [Releases page](https://github.com/jiahaoxiang2000/blivedm_rs/releases):

- **Windows**: `danmu-windows-x86_64.exe`
- **Linux**: `danmu-linux-x86_64`
- **macOS Intel**: `danmu-macos-x86_64`
- **macOS Apple Silicon**: `danmu-macos-arm64`

After downloading, run directly:

```bash
# Windows
danmu-windows-x86_64.exe --room-id 24779526

# Linux/macOS (add execute permission first)
chmod +x danmu-linux-x86_64
./danmu-linux-x86_64 --room-id 24779526

# macOS
chmod +x danmu-macos-x86_64
./danmu-macos-x86_64 --room-id 24779526
```

### Usage Examples

```bash
# NEW: Auto-detect browser cookies (recommended)
./danmu-linux-x86_64 --room-id 12345

# Manual cookies (must include SESSDATA)
./danmu-linux-x86_64 --cookies "SESSDATA=your_sessdata; other_cookie=..." --room-id 12345

# With TTS REST API server
./danmu-linux-x86_64 --room-id 12345 --tts-server http://localhost:8000 --tts-volume 0.7

# With local TTS (macOS)
./danmu-macos-x86_64 --room-id 12345 --tts-command say --tts-args "-v,Mei-Jia"

# With local TTS (Linux)
./danmu-linux-x86_64 --room-id 12345 --tts-command espeak-ng --tts-args "-v,cmn"

# ⚠️ Windows users recommendation: Use TTS server for better voice experience
# Local PowerShell TTS has technical limitations, recommend using remote TTS server:
./danmu-windows-x86_64.exe --room-id 12345 --tts-server http://localhost:8000

# Show all available options
./danmu-linux-x86_64 --help
```

### TTS Server Setup (Recommended for Windows Users)

**Windows users especially recommend using TTS server!** Compared to limited local PowerShell TTS, the server provides better voice quality and functionality.

```bash
# Clone and setup the TTS server
git clone https://github.com/jiahaoxiang2000/danmu-tts.git
cd danmu-tts
# Follow the setup instructions in the repository
```

**TTS Server Advantages:**
- 🎙️ **High-quality Voice** - Support neural network TTS and multiple voice engines
- 🌐 **Multi-language Support** - Support Chinese, English and other languages
- ⚙️ **Flexible Configuration** - Customizable voice parameters, pitch, speed
- 🔧 **Easy Deployment** - Run independently without complex configuration

See the [danmu-tts repository](https://github.com/jiahaoxiang2000/danmu-tts) for detailed setup instructions.

## Building from Source

If you prefer to build from source or for development purposes, follow these steps:

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
- **Windows**: No additional dependencies (uses built-in PowerShell TTS via System.Speech)

The detailed usage guide is available on the [Danmu](docs/danmu.md) page.


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