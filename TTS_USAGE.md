# TTS Usage Quick Reference

This document provides quick examples for using both TTS modes in blivedm_rs.

## REST API Mode (Advanced Neural Voices)

### Setup
1. Start the danmu-tts server: https://github.com/jiahaoxiang2000/danmu-tts
2. Use the REST API handlers in your code

### Basic Usage
```rust
use plugins::tts_handler_default;

// Default Chinese voice
let tts = tts_handler_default("http://localhost:8000".to_string());
scheduler.add_sequential_handler(tts);
```

### Custom Configuration
```rust
use plugins::tts_handler;

let tts = tts_handler(
    "http://localhost:8000".to_string(),    
    Some("zh-CN-XiaoxiaoNeural".to_string()), // Voice
    Some("edge".to_string()),                  // Backend
    Some("high".to_string()),                  // Quality
    Some("wav".to_string()),                   // Format
    Some(44100),                               // Sample rate
);
scheduler.add_sequential_handler(tts);
```

## Command Mode (Local TTS Programs)

### macOS
```rust
use plugins::tts_handler_command;

// English voice
let tts = tts_handler_command("say".to_string(), vec![]);

// Chinese voice
let tts = tts_handler_command(
    "say".to_string(),
    vec!["-v".to_string(), "Mei-Jia".to_string()]
);
scheduler.add_sequential_handler(tts);
```

### Linux
```rust
use plugins::tts_handler_command;

// Install: sudo apt-get install espeak-ng
let tts = tts_handler_command(
    "espeak-ng".to_string(),
    vec!["-v".to_string(), "cmn".to_string()]  // Chinese
);
scheduler.add_sequential_handler(tts);
```

### Cross-Platform Testing
```rust
use plugins::tts_handler_command;

// Just echoes the text (for testing)
let tts = tts_handler_command("echo".to_string(), vec![]);
scheduler.add_sequential_handler(tts);
```

## Complete Example

See `examples/tts_example.rs` for a comprehensive example that demonstrates both modes.

Run with: `cargo run --bin tts_example`

## Choosing the Right Mode

- **REST API Mode**: High-quality neural voices, requires external server
- **Command Mode**: Simple setup, works offline, uses system TTS programs

Both modes process messages sequentially to prevent audio overlap.
