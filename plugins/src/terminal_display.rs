use client::models::BiliMessage;
use client::scheduler::EventHandler;

/// A plugin that prints BiliMessages to the terminal.
pub struct TerminalDisplayHandler;

impl EventHandler for TerminalDisplayHandler {
    fn handle(&self, msg: &BiliMessage) {
        match msg {
            BiliMessage::Danmu { user, text } => {
                println!("[Danmu] {}: {}", user, text);
            }
            BiliMessage::Gift { user, gift } => {
                println!("[Gift] {} sent a gift: {}", user, gift);
            }
            BiliMessage::Unsupported => {
                println!("[Unsupported message type]");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::models::BiliMessage;
    use client::scheduler::EventHandler;

    #[test]
    fn test_terminal_display_handler_prints_danmu() {
        let handler = TerminalDisplayHandler;
        let msg = BiliMessage::Danmu {
            user: "test_user".to_string(),
            text: "hello world".to_string(),
        };
        // This will print to stdout, but we just want to ensure it doesn't panic
        handler.handle(&msg);
    }

    #[test]
    fn test_terminal_display_handler_prints_gift() {
        let handler = TerminalDisplayHandler;
        let msg = BiliMessage::Gift {
            user: "gift_user".to_string(),
            gift: "rocket".to_string(),
        };
        handler.handle(&msg);
    }

    #[test]
    fn test_terminal_display_handler_prints_unsupported() {
        let handler = TerminalDisplayHandler;
        let msg = BiliMessage::Unsupported;
        handler.handle(&msg);
    }
}
