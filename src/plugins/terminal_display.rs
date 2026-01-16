use crate::client::models::BiliMessage;
use crate::client::scheduler::{EventContext, EventHandler};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// A plugin that adds BiliMessages to a shared message buffer for TUI display.
pub struct TerminalDisplayHandler {
    /// Shared message buffer for TUI
    message_buffer: Arc<Mutex<VecDeque<String>>>,
}

impl TerminalDisplayHandler {
    /// Create a new TerminalDisplayHandler with a shared message buffer
    pub fn new(message_buffer: Arc<Mutex<VecDeque<String>>>) -> Self {
        Self { message_buffer }
    }
}

impl EventHandler for TerminalDisplayHandler {
    fn handle(&self, msg: &BiliMessage, _context: &EventContext) {
        let formatted_msg = match msg {
            BiliMessage::Danmu { user, text } => {
                format!("[Danmu] {}: {}", user, text)
            }
            BiliMessage::Gift { user, gift } => {
                format!("[Gift] {} sent a gift: {}", user, gift)
            }
            BiliMessage::Unsupported => "[Unsupported message type]".to_string(),
        };

        // Add message to buffer using the TuiApp helper method
        crate::tui::app::TuiApp::add_message(&self.message_buffer, formatted_msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::models::BiliMessage;
    use crate::client::scheduler::EventHandler;
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_terminal_display_handler_adds_danmu() {
        let buffer = Arc::new(Mutex::new(VecDeque::new()));
        let handler = TerminalDisplayHandler::new(Arc::clone(&buffer));
        let msg = BiliMessage::Danmu {
            user: "test_user".to_string(),
            text: "hello world".to_string(),
        };
        let context = EventContext {
            cookies: None,
            room_id: 12345,
        };
        handler.handle(&msg, &context);

        let messages = buffer.lock().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[Danmu] test_user: hello world");
    }

    #[test]
    fn test_terminal_display_handler_adds_gift() {
        let buffer = Arc::new(Mutex::new(VecDeque::new()));
        let handler = TerminalDisplayHandler::new(Arc::clone(&buffer));
        let msg = BiliMessage::Gift {
            user: "gift_user".to_string(),
            gift: "rocket".to_string(),
        };
        let context = EventContext {
            cookies: None,
            room_id: 12345,
        };
        handler.handle(&msg, &context);

        let messages = buffer.lock().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[Gift] gift_user sent a gift: rocket");
    }

    #[test]
    fn test_terminal_display_handler_adds_unsupported() {
        let buffer = Arc::new(Mutex::new(VecDeque::new()));
        let handler = TerminalDisplayHandler::new(Arc::clone(&buffer));
        let msg = BiliMessage::Unsupported;
        let context = EventContext {
            cookies: None,
            room_id: 12345,
        };
        handler.handle(&msg, &context);

        let messages = buffer.lock().unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[Unsupported message type]");
    }
}
