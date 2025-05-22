pub mod terminal_display;
use client::scheduler::EventHandler;
use std::sync::Arc;

/// Helper to create the handler as Arc<dyn EventHandler>
pub fn terminal_display_handler() -> Arc<dyn EventHandler> {
    Arc::new(terminal_display::TerminalDisplayHandler)
}

#[cfg(test)]
mod tests {}
