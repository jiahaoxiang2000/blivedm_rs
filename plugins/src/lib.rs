pub mod terminal_display;
pub mod tts;

use client::scheduler::EventHandler;
use std::sync::Arc;

/// Helper to create the handler as Arc<dyn EventHandler>
pub fn terminal_display_handler() -> Arc<dyn EventHandler> {
    Arc::new(terminal_display::TerminalDisplayHandler)
}

/// Helper to create the TTS handler as Arc<dyn EventHandler>
pub fn tts_handler(tts_command: String, tts_args: Vec<String>) -> Arc<dyn EventHandler> {
    Arc::new(tts::TtsHandler::new(tts_command, tts_args))
}

#[cfg(test)]
mod tests {}
