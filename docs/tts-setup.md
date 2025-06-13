# TTS Server Setup Guide

This guide explains how to set up and use the TTS (Text-to-Speech) functionality with blivedm_rs.

## Prerequisites

The TTS functionality requires the [danmu-tts](https://github.com/jiahaoxiang2000/danmu-tts) server to be running. This server provides high-quality neural voices and supports multiple TTS backends.

## Setup Instructions

### 1. Install the TTS Server

```bash
# Clone the danmu-tts repository
git clone https://github.com/jiahaoxiang2000/danmu-tts.git
cd danmu-tts

# Follow the installation instructions in the repository
# This typically involves setting up Python dependencies and TTS backends
```

### 2. Start the TTS Server

```bash
# Start the server (usually on port 8000)
# See the danmu-tts README for specific startup commands
# Example: python app.py or similar
```

### 3. Configure blivedm_rs

Update your Rust application to use the TTS plugin:

```rust
use plugins::tts_handler_default;
use client::scheduler::Scheduler;

let mut scheduler = Scheduler::new();

// For local server
let tts_handler = tts_handler_default("http://localhost:8000".to_string());

// For remote server (replace with your server's IP)
let tts_handler = tts_handler_default("http://192.168.71.202:8000".to_string());

scheduler.add_sequential_handler(tts_handler);
```

## Supported Features

- **Multiple TTS Backends**: Edge TTS, XTTS, Piper
- **Neural Voices**: High-quality Chinese and other language voices
- **Sequential Processing**: Messages are spoken in order without overlap
- **Error Handling**: Graceful handling of network issues
- **Configurable Quality**: Adjustable audio quality and sample rates

## Testing

You can test the integration using the provided example:

```bash
cd blivedm_rs
cargo run --bin tts_example
```

This will send test messages to your TTS server and play the generated audio.

## Troubleshooting

1. **No audio output**: Check system audio settings and ensure speakers/headphones are connected
2. **Connection errors**: Verify the TTS server is running and accessible at the configured URL
3. **Audio quality issues**: Adjust the TTS quality settings in your configuration

For more information, see the [danmu-tts documentation](https://github.com/jiahaoxiang2000/danmu-tts).
