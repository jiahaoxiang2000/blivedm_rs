# API Reference

## Core Components

### Client

The `Client` struct is the main entry point for interacting with BiliBili live streams.

```rust
pub struct Client {
    room_id: u32,
    // ... private fields
}

impl Client {
    pub fn new(room_id: u32) -> Self;
    pub async fn connect(&self) -> Result<(), Error>;
    pub async fn next_message(&self) -> Option<Message>;
    // ... other methods
}
```

### Message

The `Message` enum represents different types of messages received from the live stream.

```rust
pub enum Message {
    Chat(ChatMessage),
    Gift(GiftMessage),
    SuperChat(SuperChatMessage),
    // ... other message types
}
```

## Error Handling

All errors are wrapped in the `Error` type:

```rust
pub enum Error {
    ConnectionError(String),
    ParsingError(String),
    // ... other error types
}
```

For more details, refer to the source code documentation.
