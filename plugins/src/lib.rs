pub mod terminal_display;
pub mod tts;

use client::scheduler::EventHandler;
use std::sync::Arc;

/// Helper to create the handler as Arc<dyn EventHandler>
pub fn terminal_display_handler() -> Arc<dyn EventHandler> {
    Arc::new(terminal_display::TerminalDisplayHandler)
}

/// Helper to create the TTS handler as Arc<dyn EventHandler>
/// Uses default Chinese voice settings
pub fn tts_handler_default(server_url: String) -> Arc<dyn EventHandler> {
    Arc::new(tts::TtsHandler::new_default(server_url))
}

/// Helper to create the TTS handler with custom configuration as Arc<dyn EventHandler>
pub fn tts_handler(
    server_url: String,
    voice: Option<String>,
    backend: Option<String>,
    quality: Option<String>,
    format: Option<String>,
    sample_rate: Option<u32>,
) -> Arc<dyn EventHandler> {
    Arc::new(tts::TtsHandler::new(
        server_url,
        voice,
        backend,
        quality,
        format,
        sample_rate,
    ))
}

#[cfg(test)]
mod tests {}
