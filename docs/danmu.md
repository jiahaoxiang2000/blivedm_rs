# Danmu Crate

This crate provides a standalone binary for integrating and testing plugins with the BiliLiveClient system. It is intended for end-to-end and integration testing of plugin functionality in a real Bilibili live room environment.

## Features
- Connects to a Bilibili live room using WebSocket and SESSDATA authentication.
- Receives danmaku (chat), gift, and other live messages.
- Passes received messages to a plugin event handler (e.g., terminal display) via the scheduler system.
- Demonstrates real-time event-driven plugin integration.

## Usage

You can provide `SESSDATA` and `room_id` either as command-line arguments or environment variables.

### Option 1: Command-line arguments

```sh
cargo run -p danmu -- <SESSDATA> <room_id>
```

### Option 2: Environment variables

```sh
export SESSDATA=your_sessdata
export ROOM_ID=your_room_id
cargo run -p danmu
```

If not provided, the binary will use default values for both.

3. Incoming messages will be processed by the registered plugin (e.g., printed to the terminal).

## Extending
- To test other plugins, register them in `main.rs` using the scheduler.
- See the `plugins` crate for available plugins and how to implement your own.

---
