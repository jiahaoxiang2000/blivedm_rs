// src/tui/mod.rs
//! TUI module for displaying messages and handling user input

pub mod app;
pub mod ui;
pub mod event;

pub use app::TuiApp;
pub use event::run_tui;
