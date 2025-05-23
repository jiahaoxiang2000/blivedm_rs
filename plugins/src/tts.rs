use client::models::BiliMessage;
use client::scheduler::EventHandler;
use std::process::Command;

/// A plugin that sends Danmaku text to a TTS service and plays the audio.
pub struct TtsHandler {
    /// The TTS command to use (e.g., "say" on macOS, or a custom script)
    pub tts_command: String,
    /// Optional extra arguments for the TTS command (e.g., ["-v", "SinJi"])
    pub tts_args: Vec<String>,
}

impl TtsHandler {
    pub fn new(tts_command: String, tts_args: Vec<String>) -> Self {
        TtsHandler {
            tts_command,
            tts_args,
        }
    }
}

impl EventHandler for TtsHandler {
    fn handle(&self, msg: &BiliMessage) {
        if let BiliMessage::Danmu { user, text } = msg {
            let message = format!("{}说：{}", user, text);
            let mut cmd = Command::new(&self.tts_command);
            for arg in &self.tts_args {
                cmd.arg(arg);
            }
            let _ = cmd.arg(&message).spawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::models::BiliMessage;
    use client::scheduler::EventHandler;

    #[cfg(target_os = "macos")]
    #[test]
    fn test_tts_handler_danmu() {
        let handler = TtsHandler::new(
            "say".to_string(),
            vec!["-v".to_string(), "SinJi".to_string()],
        );
        let text = "您好，我叫SinJi。我讲普通话。".to_string();
        let msg = BiliMessage::Danmu {
            user: "test user".to_string(),
            text: text.clone(),
        };
        handler.handle(&msg);
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_tts_handler_danmu() {
        let handler = TtsHandler::new("echo".to_string(), vec![]);
        let msg = BiliMessage::Danmu {
            user: "test_user".to_string(),
            text: "hello world".to_string(),
        };
        handler.handle(&msg);
    }
}
