// src/client/mod.rs
//! WebSocket client for Bilibili live danmaku messages
//! This module implements the WebSocket client component of the architecture

pub mod auth;
pub mod models;
pub mod websocket;

// Re-export the refactored WebSocket client
pub use websocket::BiliLiveClient;
