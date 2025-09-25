use client::models::BiliMessage;
use client::scheduler::{EventContext, EventHandler};
use log::{error, info, warn};
use serde::Serialize;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::thread::JoinHandle;

#[derive(Serialize, Debug)]
struct TtsRequest {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backend: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<u32>,
}


/// TTS backend configuration
#[derive(Debug, Clone)]
pub enum TtsMode {
    /// Use REST API for TTS server communication only
    RestApi {
        /// The base URL of the TTS server (e.g., "http://localhost:8000")
        server_url: String,
        /// Voice ID to use for TTS (e.g., "zh-CN-XiaoxiaoNeural")
        voice: Option<String>,
        /// TTS backend to use (e.g., "edge", "xtts", "piper")
        backend: Option<String>,
        /// Audio quality ("low", "medium", "high")
        quality: Option<String>,
        /// Audio format (e.g., "wav")
        format: Option<String>,
        /// Sample rate for audio
        sample_rate: Option<u32>,
    },
}

/// A plugin that sends Danmaku text to a TTS server.
///
/// This handler sends text to a TTS REST API server. The server handles audio generation
/// and playback. Messages are processed sequentially.
pub struct TtsHandler {
    /// TTS configuration
    #[allow(dead_code)]
    mode: TtsMode,
    /// Channel sender for queuing TTS messages
    sender: Sender<String>,
    /// Background thread handle for TTS processing
    _worker_handle: JoinHandle<()>,
}

impl TtsHandler {
    /// Create a new TTS handler with the specified mode
    pub fn new(mode: TtsMode) -> Self {
        let (sender, receiver) = mpsc::channel::<String>();

        // Clone the mode for the worker thread
        let mode_clone = mode.clone();

        // Spawn worker thread to process TTS queue sequentially
        let worker_handle = thread::spawn(move || {
            Self::run_rest_api_worker(receiver, mode_clone);
        });

        TtsHandler {
            mode,
            sender,
            _worker_handle: worker_handle,
        }
    }

    /// Create a new TTS handler with REST API using default Chinese voice settings
    pub fn new_rest_api_default(server_url: String) -> Self {
        let mode = TtsMode::RestApi {
            server_url,
            voice: Some("zh-CN-XiaoxiaoNeural".to_string()),
            backend: Some("edge".to_string()),
            quality: Some("medium".to_string()),
            format: Some("wav".to_string()),
            sample_rate: Some(22050),
        };
        Self::new(mode)
    }

    /// Create a new TTS handler with REST API and custom configuration
    pub fn new_rest_api(
        server_url: String,
        voice: Option<String>,
        backend: Option<String>,
        quality: Option<String>,
        format: Option<String>,
        sample_rate: Option<u32>,
    ) -> Self {
        let mode = TtsMode::RestApi {
            server_url,
            voice,
            backend,
            quality,
            format,
            sample_rate,
        };
        Self::new(mode)
    }

    /// Worker thread for REST API TTS processing
    fn run_rest_api_worker(receiver: std::sync::mpsc::Receiver<String>, mode: TtsMode) {
        let TtsMode::RestApi {
            server_url,
            voice,
            backend,
            quality,
            format,
            sample_rate,
        } = mode;
        // Create a tokio runtime for HTTP requests
        let rt = tokio::runtime::Runtime::new().unwrap();
        let client = reqwest::Client::new();

        while let Ok(message) = receiver.recv() {
            let request = TtsRequest {
                text: message,
                voice: voice.clone(),
                backend: backend.clone(),
                quality: quality.clone(),
                format: format.clone(),
                sample_rate,
            };

            // Make HTTP request to TTS service
            rt.block_on(async {
                match client
                    .post(&format!("{}/tts", server_url))
                    .header("Content-Type", "application/json")
                    .json(&request)
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            info!("TTS request sent successfully to server");
                        } else {
                            warn!("TTS request failed with status: {}", response.status());
                        }
                    }
                    Err(e) => error!("Failed to send TTS request: {}", e),
                }
            });
        }
    }


    /// Legacy method - kept for backward compatibility
    #[deprecated(note = "Use new_rest_api_default instead")]
    pub fn new_default(server_url: String) -> Self {
        Self::new_rest_api_default(server_url)
    }
}

impl EventHandler for TtsHandler {
    fn handle(&self, msg: &BiliMessage, _context: &EventContext) {
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

    #[test]
    fn test_tts_handler_danmu() {
        // Test with a mock server URL (won't actually make requests in this test)
        let handler = TtsHandler::new_rest_api_default("http://localhost:8000".to_string());

        let text = "您好，欢迎来到直播间。".to_string();
        let msg = BiliMessage::Danmu {
            user: "测试用户".to_string(),
            text: text.clone(),
        };
        let context = EventContext {
            cookies: None,
            room_id: 12345,
        };
        handler.handle(&msg, &context);
    }

    #[test]
    fn test_tts_handler_custom_config() {
        let handler = TtsHandler::new_rest_api(
            "http://localhost:8000".to_string(),
            Some("zh-CN-XiaoxiaoNeural".to_string()),
            Some("edge".to_string()),
            Some("high".to_string()),
            Some("wav".to_string()),
            Some(44100),
        );

        let msg = BiliMessage::Danmu {
            user: "test_user".to_string(),
            text: "hello world".to_string(),
        };
        let context = EventContext {
            cookies: None,
            room_id: 12345,
        };
        handler.handle(&msg, &context);
    }

    #[test]
    fn test_tts_handler_sequential_processing() {
        use std::time::Duration;

        // Use default configuration for testing
        let handler = TtsHandler::new_rest_api_default("http://localhost:8000".to_string());

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
            let context = EventContext {
                cookies: None,
                room_id: 12345,
            };
            handler.handle(&msg, &context);
        }

        // Give the worker thread some time to process the queue
        std::thread::sleep(Duration::from_millis(100));

        // The test passes if no panic occurs - the sequential processing
        // is ensured by the worker thread design
    }




    #[test]
    fn test_tts_request_serialization() {
        let request = TtsRequest {
            text: "Hello world".to_string(),
            voice: Some("zh-CN-XiaoxiaoNeural".to_string()),
            backend: Some("edge".to_string()),
            quality: Some("medium".to_string()),
            format: Some("wav".to_string()),
            sample_rate: Some(22050),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Hello world"));
        assert!(json.contains("zh-CN-XiaoxiaoNeural"));
        assert!(json.contains("edge"));
    }

}
