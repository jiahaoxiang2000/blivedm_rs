# Plugins Crate

This crate provides plugin implementations for the BiliLiveDanmu system. Plugins are designed to process and respond to BiliMessage events in a modular and extensible way.

## Example Plugin: Terminal Display

The `terminal_display` plugin prints incoming BiliMessage events (such as danmaku and gifts) to the terminal. It implements the `EventHandler` trait from the `client` crate.

### Usage

Add the plugin handler to your scheduler:

```rust
use plugins::terminal_display::TerminalDisplayHandler;
use client::scheduler::Scheduler;
use std::sync::Arc;

let mut scheduler = Scheduler::new();
let handler = Arc::new(TerminalDisplayHandler);
scheduler.add_sequential_handler(handler);
```

When a message is triggered, the handler will print it to the terminal.

## Adding New Plugins

To add a new plugin, implement the `EventHandler` trait for your struct and register it with the scheduler.

---

## TTS Plugin

The TTS (Text-to-Speech) plugin enables your application to read out text messages using a TTS service. This service can be either local or remote.

### How it works

```mermaid
flowchart LR
    A[User sends Danmaku message] --> B[TTS Plugin receives Danmaku event]
    B --> C[Send message text to TTS service]
    C --> D[TTS service returns speech audio]
    D --> E[Plugin plays audio in real time]
```

- When a user sends a message (Danmaku event) in the live room, the plugin receives the event.
- The plugin sends the message text to the configured TTS service.
- The TTS service converts the text into speech audio.
- The plugin plays the generated audio in real time.

### Usage

The TTS plugin supports two modes of operation:

#### 1. REST API Mode (Recommended)

Add the TTS plugin handler using the REST API service for high-quality neural voices:

```rust
use plugins::{tts_handler_default, tts_handler};
use client::scheduler::Scheduler;
use std::sync::Arc;

let mut scheduler = Scheduler::new();

// Using default Chinese voice settings (recommended):
let tts = tts_handler_default("http://localhost:8000".to_string());
scheduler.add_sequential_handler(tts);
```

For custom voice configuration:

```rust
let tts = tts_handler(
    "http://localhost:8000".to_string(),    // TTS server URL
    Some("zh-CN-XiaoxiaoNeural".to_string()), // Voice ID
    Some("edge".to_string()),                  // Backend (edge, xtts, piper)
    Some("high".to_string()),                  // Quality (low, medium, high)
    Some("wav".to_string()),                   // Format
    Some(44100),                               // Sample rate
);
scheduler.add_sequential_handler(tts);
```

#### 2. Command Mode (Simple Setup)

Use local command-line TTS programs for simple setups without external dependencies:

```rust
use plugins::tts_handler_command;
use client::scheduler::Scheduler;

let mut scheduler = Scheduler::new();

// For macOS with Chinese voice:
let tts = tts_handler_command(
    "say".to_string(),
    vec!["-v".to_string(), "Mei-Jia".to_string()]
);
scheduler.add_sequential_handler(tts);

// For Linux with espeak-ng:
let tts = tts_handler_command(
    "espeak-ng".to_string(),
    vec!["-v".to_string(), "cmn".to_string()]
);
scheduler.add_sequential_handler(tts);

// For testing with echo (cross-platform):
let tts = tts_handler_command("echo".to_string(), vec![]);
scheduler.add_sequential_handler(tts);
```

The plugin requires a running danmu-tts server. You can start the server following the instructions in the [danmu-tts repository](https://github.com/jiahaoxiang2000/danmu-tts).

### Setting up the TTS Server

The TTS functionality depends on the [danmu-tts](https://github.com/jiahaoxiang2000/danmu-tts) project, which provides a REST API server for text-to-speech conversion.

#### Quick Setup

1. **Clone the TTS server repository:**
   ```bash
   git clone https://github.com/jiahaoxiang2000/danmu-tts.git
   cd danmu-tts
   ```

2. **Follow the setup instructions** in the danmu-tts repository to install dependencies and configure the server.

3. **Start the TTS server:**
   ```bash
   # The server typically runs on port 8000
   # See danmu-tts documentation for specific startup commands
   ```

4. **Configure your blivedm_rs application** to use the TTS server:
   ```rust
   // Use localhost if running on the same machine
   let tts = tts_handler_default("http://localhost:8000".to_string());
   
   // Or use a remote server IP
   let tts = tts_handler_default("http://192.168.1.100:8000".to_string());
   ```

The danmu-tts server supports multiple TTS backends and provides high-quality neural voices, making it ideal for live streaming applications.

### Implementation

The TTS plugin supports two distinct modes of operation, each with its own advantages:

#### Mode 1: REST API Integration
The modern approach using the danmu-tts server:

- **HTTP Communication**: Sends POST requests to `/tts` endpoint
- **JSON Protocol**: Supports structured request/response format
- **Neural Voices**: Access to high-quality voice synthesis
- **Configurable Options**: Voice, backend, quality, and format selection
- **Audio Streaming**: Base64 encoded audio data response
- **Multiple Backends**: Edge TTS, XTTS, Piper support

**Configuration Options:**
- `server_url`: Base URL of the TTS server
- `voice`: Voice ID (e.g., "zh-CN-XiaoxiaoNeural")
- `backend`: TTS backend ("edge", "xtts", "piper")
- `quality`: Audio quality ("low", "medium", "high")
- `format`: Audio format ("wav")
- `sample_rate`: Sample rate (22050, 44100, etc.)

#### Mode 2: Command-Line Integration
The traditional approach using local TTS programs:

- **Direct Execution**: Spawns local TTS processes
- **Cross-Platform**: Works with system TTS commands
- **No Network**: Completely offline operation
- **Simple Setup**: No external server required
- **Lightweight**: Minimal resource overhead

**Supported Commands:**
- **macOS**: `say` command with voice selection
- **Linux**: `espeak-ng`, `festival`, or similar TTS engines
- **Custom**: Any command-line TTS program

#### Common Features
Both modes share these implementation characteristics:

- **Sequential Processing**: Worker thread with message queue prevents audio overlap
- **Message Ordering**: Ensures proper sequence of TTS playback
- **Non-blocking**: Main event loop remains responsive during TTS processing
- **Error Handling**: Graceful handling of failures with logging
- **Event Integration**: Seamless integration with the scheduler system

#### Choosing the Right Mode

**Use REST API Mode when:**
- You want high-quality neural voices
- Multiple language/voice options are needed
- You can run the danmu-tts server
- Network connectivity is reliable

**Use Command Mode when:**
- Simple setup is preferred
- No external dependencies allowed
- Offline operation is required
- Basic TTS functionality is sufficient

This dual-mode approach allows you to add voice feedback to your live room with the flexibility to choose the implementation that best fits your needs and infrastructure.