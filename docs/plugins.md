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
