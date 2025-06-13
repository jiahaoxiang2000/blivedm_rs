# Usage Guide

## Installation

To use blivedm_rs in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
blivedm_rs = "0.1.0"
```

## Basic Example

```rust
use blivedm_rs::bili_live_dm::web::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new BiliBili live client
    let client = Client::new(12345); // Replace with actual room ID
    
    // Connect to the live room
    client.connect().await?;
    
    // Process messages
    while let Some(msg) = client.next_message().await {
        println!("Received message: {:?}", msg);
    }
    
    Ok(())
}
```

For more detailed examples, see the [API Reference](api.md).

## TTS Integration

For text-to-speech functionality, set up the [danmu-tts server](https://github.com/jiahaoxiang2000/danmu-tts):

```rust
use plugins::tts_handler_default;
use client::scheduler::Scheduler;

let mut scheduler = Scheduler::new();

// Add TTS handler (requires danmu-tts server running)
let tts_handler = tts_handler_default("http://localhost:8000".to_string());
scheduler.add_sequential_handler(tts_handler);

// Process messages with TTS enabled
for message in messages {
    scheduler.trigger(message);
}
```

See the [Plugins documentation](plugins.md) for detailed TTS setup instructions.
