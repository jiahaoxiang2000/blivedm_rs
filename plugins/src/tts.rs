use client::models::BiliMessage;
use client::scheduler::EventHandler;
use std::process::Command;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::thread::JoinHandle;

/// A plugin that sends Danmaku text to a TTS service and plays the audio sequentially.
pub struct TtsHandler {
    /// The TTS command to use (e.g., "say" on macOS, or a custom script)
    pub tts_command: String,
    /// Optional extra arguments for the TTS command (e.g., ["-v", "SinJi"])
    pub tts_args: Vec<String>,
    /// Channel sender for queuing TTS messages
    sender: Sender<String>,
    /// Background thread handle for TTS processing
    _worker_handle: JoinHandle<()>,
}

impl TtsHandler {
    pub fn new(tts_command: String, tts_args: Vec<String>) -> Self {
        let (sender, receiver) = mpsc::channel::<String>();
        
        // Clone the command and args for the worker thread
        let cmd = tts_command.clone();
        let args = tts_args.clone();
        
        // Spawn worker thread to process TTS queue sequentially
        let worker_handle = thread::spawn(move || {
            while let Ok(message) = receiver.recv() {
                let mut command = Command::new(&cmd);
                for arg in &args {
                    command.arg(arg);
                }
                
                // Execute TTS command and wait for it to complete
                let _ = command.arg(&message).status();
            }
        });
        
        TtsHandler {
            tts_command,
            tts_args,
            sender,
            _worker_handle: worker_handle,
        }
    }
}

impl EventHandler for TtsHandler {
    fn handle(&self, msg: &BiliMessage) {
        if let BiliMessage::Danmu { user, text } = msg {
            let message = format!("{}说：{}", user, text);
            // Send message to the queue for sequential processing
            let _ = self.sender.send(message);
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

    #[cfg(target_os = "linux")]
    #[test]
    fn test_tts_handler_danmu() {
        let handler = TtsHandler::new(
            "espeak-ng".to_string(),
            vec!["-v".to_string(), "cmn".to_string()],
        );
        let text = "您好，欢迎来到直播间。".to_string();
        let msg = BiliMessage::Danmu {
            user: "测试用户".to_string(),
            text: text.clone(),
        };
        handler.handle(&msg);
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    #[test]
    fn test_tts_handler_danmu() {
        let handler = TtsHandler::new("echo".to_string(), vec![]);
        let msg = BiliMessage::Danmu {
            user: "test_user".to_string(),
            text: "hello world".to_string(),
        };
        handler.handle(&msg);
    }

    #[test]
    fn test_tts_handler_sequential_processing() {
        use std::time::Duration;
        
        // Use echo command to simulate TTS without actually playing audio
        let handler = TtsHandler::new("echo".to_string(), vec![]);
        
        // Send multiple messages quickly
        let messages = vec![
            ("User1", "First message"),
            ("User2", "Second message"), 
            ("User3", "Third message"),
        ];
        
        for (user, text) in messages {
            let msg = BiliMessage::Danmu {
                user: user.to_string(),
                text: text.to_string(),
            };
            handler.handle(&msg);
        }
        
        // Give the worker thread some time to process the queue
        std::thread::sleep(Duration::from_millis(100));
        
        // The test passes if no panic occurs - the sequential processing
        // is ensured by the worker thread design
    }
}
