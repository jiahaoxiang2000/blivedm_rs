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
