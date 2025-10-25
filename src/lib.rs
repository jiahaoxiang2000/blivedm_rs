// src/lib.rs
//! Bilibili live room danmaku WebSocket client library with TTS and plugin support

pub mod client;
pub mod plugins;
pub mod tui;

// Re-export commonly used items from client
pub use client::{
    auth,
    browser_cookies,
    models,
    scheduler,
    websocket,
    get_cookies_or_browser,
};

// Re-export plugin modules and helpers
pub use plugins::{
    terminal_display,
    tts,
    auto_reply,
    terminal_display_handler,
    tts_handler,
    tts_handler_default,
    tts_handler_command,
    auto_reply_handler,
};
