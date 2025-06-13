use client::models::BiliMessage;
use client::scheduler::EventHandler;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};
use rodio::{Decoder, OutputStream, Sink};

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

#[derive(Deserialize, Debug)]
struct TtsResponse {
    audio_data: String,
    metadata: TtsMetadata,
    cached: bool,
}

#[derive(Deserialize, Debug)]
struct TtsMetadata {
    #[serde(default)]
    backend: String,
    #[serde(default)]
    voice: String,
    duration: Option<f64>,
    #[serde(default)]
    sample_rate: u32,
    #[serde(default)]
    format: String,
    #[serde(default)]
    size_bytes: u64,
}

/// A plugin that sends Danmaku text to a TTS REST API service and plays the audio sequentially.
/// 
/// This handler:
/// 1. Receives Danmaku messages through the EventHandler interface
/// 2. Sends the text to a TTS REST API server  
/// 3. Receives base64-encoded audio data in the response
/// 4. Decodes the audio data and plays it through the system's audio output
/// 5. Processes messages sequentially to avoid overlapping audio
/// 
/// The TTS server should accept POST requests to `/tts` with JSON payload containing
/// the text and optional configuration parameters, and return audio data in base64 format.
pub struct TtsHandler {
    /// The base URL of the TTS server (e.g., "http://localhost:8000")
    pub server_url: String,
    /// Voice ID to use for TTS (e.g., "zh-CN-XiaoxiaoNeural")
    pub voice: Option<String>,
    /// TTS backend to use (e.g., "edge", "xtts", "piper")
    pub backend: Option<String>,
    /// Audio quality ("low", "medium", "high")
    pub quality: Option<String>,
    /// Audio format (e.g., "wav")
    pub format: Option<String>,
    /// Sample rate for audio
    pub sample_rate: Option<u32>,
    /// Channel sender for queuing TTS messages
    sender: Sender<String>,
    /// Background thread handle for TTS processing
    _worker_handle: JoinHandle<()>,
}

impl TtsHandler {
    pub fn new(
        server_url: String,
        voice: Option<String>,
        backend: Option<String>,
        quality: Option<String>,
        format: Option<String>,
        sample_rate: Option<u32>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel::<String>();
        
        // Clone configuration for the worker thread
        let url = server_url.clone();
        let voice_config = voice.clone();
        let backend_config = backend.clone();
        let quality_config = quality.clone();
        let format_config = format.clone();
        let sample_rate_config = sample_rate;
        
        // Spawn worker thread to process TTS queue sequentially
        let worker_handle = thread::spawn(move || {
            // Create a tokio runtime for HTTP requests
            let rt = tokio::runtime::Runtime::new().unwrap();
            let client = reqwest::Client::new();
            
            // Initialize audio output stream (this will be reused for all audio playback)
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            
            while let Ok(message) = receiver.recv() {
                let request = TtsRequest {
                    text: message,
                    voice: voice_config.clone(),
                    backend: backend_config.clone(),
                    quality: quality_config.clone(),
                    format: format_config.clone(),
                    sample_rate: sample_rate_config,
                };
                
                // Make HTTP request to TTS service
                rt.block_on(async {
                    match client
                        .post(&format!("{}/tts", url))
                        .header("Content-Type", "application/json")
                        .json(&request)
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.status().is_success() {
                                match response.json::<TtsResponse>().await {
                                    Ok(tts_response) => {
                                        println!("TTS generated successfully: {} bytes, duration: {:.2}s", 
                                               tts_response.metadata.size_bytes, 
                                               tts_response.metadata.duration.unwrap_or(0.0));
                                        
                                        // Decode base64 audio data and play it
                                        match general_purpose::STANDARD.decode(&tts_response.audio_data) {
                                            Ok(audio_bytes) => {
                                                // Create a cursor from the audio bytes
                                                let cursor = Cursor::new(audio_bytes);
                                                
                                                // Create a decoder for the audio format
                                                match Decoder::new(cursor) {
                                                    Ok(source) => {
                                                        // Create a new sink for this audio
                                                        let sink = Sink::try_new(&stream_handle).unwrap();
                                                        
                                                        // Append the audio source to the sink
                                                        sink.append(source);
                                                        
                                                        // Wait for the audio to finish playing
                                                        sink.sleep_until_end();
                                                        
                                                        println!("Audio playback completed");
                                                    }
                                                    Err(e) => eprintln!("Failed to decode audio format: {}", e),
                                                }
                                            }
                                            Err(e) => eprintln!("Failed to decode base64 audio data: {}", e),
                                        }
                                    }
                                    Err(e) => eprintln!("Failed to parse TTS response: {}", e),
                                }
                            } else {
                                eprintln!("TTS request failed with status: {}", response.status());
                            }
                        }
                        Err(e) => eprintln!("Failed to send TTS request: {}", e),
                    }
                });
            }
        });
        
        TtsHandler {
            server_url,
            voice,
            backend,
            quality,
            format,
            sample_rate,
            sender,
            _worker_handle: worker_handle,
        }
    }
    
    /// Create a new TtsHandler with default Chinese voice settings
    pub fn new_default(server_url: String) -> Self {
        Self::new(
            server_url,
            Some("zh-CN-XiaoxiaoNeural".to_string()),
            Some("edge".to_string()),
            Some("medium".to_string()),
            Some("wav".to_string()),
            Some(22050),
        )
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

    #[test]
    fn test_tts_handler_danmu() {
        // Test with a mock server URL (won't actually make requests in this test)
        let handler = TtsHandler::new_default("http://localhost:8000".to_string());
        
        let text = "您好，欢迎来到直播间。".to_string();
        let msg = BiliMessage::Danmu {
            user: "测试用户".to_string(),
            text: text.clone(),
        };
        handler.handle(&msg);
    }

    #[test]
    fn test_tts_handler_custom_config() {
        let handler = TtsHandler::new(
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
        handler.handle(&msg);
    }

    #[test]
    fn test_tts_handler_sequential_processing() {
        use std::time::Duration;
        
        // Use default configuration for testing
        let handler = TtsHandler::new_default("http://localhost:8000".to_string());
        
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
