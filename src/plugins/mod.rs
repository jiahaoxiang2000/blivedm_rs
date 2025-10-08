pub mod terminal_display;
pub mod tts;
pub mod auto_reply;

use crate::client::scheduler::EventHandler;
use std::sync::Arc;

/// Helper to create the handler as Arc<dyn EventHandler>
pub fn terminal_display_handler() -> Arc<dyn EventHandler> {
    Arc::new(terminal_display::TerminalDisplayHandler)
}

/// Helper to create the TTS handler as Arc<dyn EventHandler>
/// Uses default Chinese voice settings with REST API
pub fn tts_handler_default(server_url: String) -> Arc<dyn EventHandler> {
    Arc::new(tts::TtsHandler::new_rest_api_default(server_url))
}

/// Helper to create the TTS handler with REST API and custom configuration as Arc<dyn EventHandler>
pub fn tts_handler(
    server_url: String,
    voice: Option<String>,
    backend: Option<String>,
    quality: Option<String>,
    format: Option<String>,
    sample_rate: Option<u32>,
) -> Arc<dyn EventHandler> {
    Arc::new(tts::TtsHandler::new_rest_api(
        server_url,
        voice,
        backend,
        quality,
        format,
        sample_rate,
    ))
}

/// Helper to create the command-based TTS handler as Arc<dyn EventHandler>
/// For local TTS commands like `say` on macOS or `espeak-ng` on Linux
pub fn tts_handler_command(tts_command: String, tts_args: Vec<String>) -> Arc<dyn EventHandler> {
    Arc::new(tts::TtsHandler::new_command(tts_command, tts_args))
}

/// Helper to create the auto reply handler as Arc<dyn EventHandler>
pub fn auto_reply_handler(config: auto_reply::AutoReplyConfig) -> Arc<dyn EventHandler> {
    Arc::new(auto_reply::AutoReplyHandler::new(config))
}

#[cfg(test)]
mod tests {}
