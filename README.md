# blivedm_rs

Bilibili live room DM (Danmaku) websocket client library for Rust.

## Quick Start

### Download Pre-built Binaries

Download the latest release for your platform from the [Releases page](https://github.com/jiahaoxiang2000/blivedm_rs/releases):

- **Linux x86_64**: `blivedm-linux-x86_64.tar.gz`
- **macOS Intel**: `blivedm-macos-x86_64.tar.gz`
- **macOS Apple Silicon**: `blivedm-macos-aarch64.tar.gz`

```bash
# Extract and run
tar -xzf blivedm-<platform>.tar.gz
./danmu <SESSDATA> <ROOM_ID>
```

### System Requirements

- **Linux**: Install `espeak-ng` for TTS support: `sudo apt-get install espeak-ng`
- **macOS**: No additional dependencies (uses built-in `say` command)

### TTS Server Setup (Optional)

For advanced TTS functionality, you can set up the danmu-tts server:

```bash
# Clone and setup the TTS server
git clone https://github.com/jiahaoxiang2000/danmu-tts.git
cd danmu-tts
# Follow the setup instructions in the repository
```

The TTS server provides high-quality neural voices and multiple TTS backends. See the [danmu-tts repository](https://github.com/jiahaoxiang2000/danmu-tts) for detailed setup instructions.

### Building from Source

```bash
git clone https://github.com/jiahaoxiang2000/blivedm_rs.git
cd blivedm_rs
cargo build --release
```

## Documentation

The full documentation is available in the [`docs/`](docs/) folder. Here are the main sections:

- [Getting Started](docs/README.md): Introduction and setup instructions.
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