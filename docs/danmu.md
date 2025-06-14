# Danmu Crate

This crate provides a standalone binary for integrating and testing plugins with the BiliLiveClient system. It is intended for end-to-end and integration testing of plugin functionality in a real Bilibili live room environment.

## Features
- Connects to a Bilibili live room using WebSocket and SESSDATA authentication.
- Receives danmaku (chat), gift, and other live messages.
- Passes received messages to a plugin event handler (e.g., terminal display, TTS) via the scheduler system.
- Demonstrates real-time event-driven plugin integration.
- **Text-to-Speech (TTS) Support**: Convert danmaku messages to speech with configurable volume control.
- **Multiple TTS Backends**: Support for REST API TTS servers and local command-line TTS programs.
- **Terminal Display**: Real-time display of incoming messages in the terminal.

## Usage

You can provide `SESSDATA` and `room_id` either as command-line arguments or environment variables.

### Command-line Usage

```sh
# Basic usage
cargo run -p danmu -- [SESSDATA] [ROOM_ID]

# With TTS configuration
cargo run -p danmu -- [SESSDATA] [ROOM_ID] [TTS_OPTIONS]

# Show all available options
cargo run -p danmu -- --help
```

### Environment Variables Fallback

```sh
export SESSDATA=your_sessdata
export ROOM_ID=your_room_id
cargo run -p danmu
```

If command-line arguments are not provided, the binary will use environment variables. If environment variables are also not set, it will use default values (`dummy_sessdata` for SESSDATA and `24779526` for ROOM_ID).

### Build Release Binary

To build an optimized release binary:

```sh
cargo build -p danmu --release
```

The resulting binary will be in `target/release/danmu`.

## TTS (Text-to-Speech) Configuration

The danmu binary supports TTS functionality to convert incoming danmaku messages to speech. You can configure TTS in multiple ways:

### REST API TTS Server

If you have a TTS REST API server running (like edge-tts server):

```sh
# Default Chinese voice with medium quality
./danmu <SESSDATA> <room_id> --tts-server http://localhost:8000

# Custom voice and quality
./danmu <SESSDATA> <room_id> --tts-server http://localhost:8000 --tts-voice "zh-CN-XiaoxiaoNeural" --tts-quality high

# Custom volume (0.0 to 1.0)
./danmu <SESSDATA> <room_id> --tts-server http://localhost:8000 --tts-volume 0.7
```

### Local Command-line TTS

For local TTS programs:

**macOS (using built-in `say` command):**
```sh
./danmu <SESSDATA> <room_id> --tts-command say --tts-args "-v,Mei-Jia"
```

**Linux (using espeak-ng):**
```sh
# Install espeak-ng first
sudo apt-get install espeak-ng

# Run with Chinese voice
./danmu <SESSDATA> <room_id> --tts-command espeak-ng --tts-args "-v,cmn"
```

### TTS Configuration Options

All TTS options are optional and can be combined as needed:

- `--tts-server <URL>`: TTS REST API server URL
- `--tts-voice <VOICE>`: Voice ID (e.g., "zh-CN-XiaoxiaoNeural")
- `--tts-backend <BACKEND>`: TTS backend ("edge", "xtts", "piper")
- `--tts-quality <QUALITY>`: Audio quality ("low", "medium", "high")
- `--tts-format <FORMAT>`: Audio format (e.g., "wav")
- `--tts-sample-rate <RATE>`: Sample rate (e.g., 22050, 44100)
- `--tts-volume <VOLUME>`: Audio volume (0.0 to 1.0, default: 1.0)
- `--tts-command <COMMAND>`: Local TTS command (e.g., "say", "espeak-ng")
- `--tts-args <ARGS>`: Comma-separated arguments for TTS command
- `--debug`: Enable debug logging

**Note**: Use either `--tts-server` for REST API mode OR `--tts-command` for local command mode, not both.

## System Requirements

- **Rust**: Latest stable version (tested with Rust 2024 edition)
- **Operating System**: Linux, macOS (Windows support may vary)
- **Audio System**: 
  - Linux: ALSA support required
  - macOS: Built-in Core Audio
- **Dependencies**:

### Linux Dependencies
```bash
# Audio support
sudo apt-get install libasound2-dev

# SSL and build tools
sudo apt-get install pkg-config libssl-dev

# Optional: Local TTS with espeak-ng
sudo apt-get install espeak-ng
```

### macOS Dependencies
```bash
# No additional dependencies required
# macOS includes built-in 'say' command for TTS
```

### Network Requirements
- Internet connection for Bilibili WebSocket API
- Optional: Local or remote TTS REST API server access

Incoming messages will be processed by the registered plugin (e.g., printed to the terminal).

## Extending
- To test other plugins, register them in `main.rs` using the scheduler.
- See the `plugins` crate for available plugins and how to implement your own.
- **Available Plugins**:
  - `TerminalDisplayHandler`: Displays messages in the terminal
  - `TtsHandler`: Converts messages to speech with configurable options
- **Plugin Development**: Create custom event handlers by implementing the `EventHandler` trait.

## Examples

### Basic Usage (Terminal Display Only)
```sh
cargo run -p danmu -- your_sessdata 12345
```

### Help and Available Options
```sh
cargo run -p danmu -- --help
```

### With TTS REST API
```sh
cargo run -p danmu -- your_sessdata 12345 --tts-server http://localhost:8000 --tts-volume 0.8
```

### With Local TTS (macOS)
```sh
cargo run -p danmu -- your_sessdata 12345 --tts-command say --tts-args "-v,Mei-Jia"
```

### With Local TTS (Linux)
```sh
cargo run -p danmu -- your_sessdata 12345 --tts-command espeak-ng --tts-args "-v,cmn,-s,150"
```

### Advanced TTS Configuration
```sh
# Full REST API configuration with debug logging
cargo run -p danmu -- your_sessdata 12345 \
  --tts-server http://localhost:8000 \
  --tts-voice "zh-CN-XiaoxiaoNeural" \
  --tts-backend "edge" \
  --tts-quality "high" \
  --tts-volume 0.6 \
  --debug

# Using built release binary (faster startup)
./target/release/danmu your_sessdata 12345 \
  --tts-server http://localhost:8000 \
  --tts-volume 0.5

# Command-line TTS with custom speech rate (Linux)
cargo run -p danmu -- your_sessdata 12345 \
  --tts-command espeak-ng \
  --tts-args "-v,cmn,-s,120,-p,50"

# Quiet mode (no TTS, terminal display only)
cargo run -p danmu -- your_sessdata 12345
```

## Troubleshooting

### Common Issues

**TTS not working:**
- Ensure audio system is properly configured
- Check if TTS server is running (for REST API mode)
- Verify TTS command is installed (for command mode)

**Connection issues:**
- Verify SESSDATA is valid and not expired
- Check room ID is correct and accessible
- Ensure internet connection is stable

**Audio issues on Linux:**
- Install ALSA development libraries: `sudo apt-get install libasound2-dev`
- Check audio permissions and device availability

**Build issues:**
- Update Rust to latest stable version: `rustup update`
- Clear cargo cache: `cargo clean`

---
