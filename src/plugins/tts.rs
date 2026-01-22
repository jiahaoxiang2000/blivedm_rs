use crate::client::models::BiliMessage;
use crate::client::scheduler::{EventContext, EventHandler};
use base64::{Engine as _, engine::general_purpose};
use log::{debug, error, info, warn};
use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::process::Command;
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

#[derive(Deserialize, Debug)]
struct TtsResponse {
    audio_data: String,
    metadata: TtsMetadata,
    #[allow(dead_code)]
    cached: bool,
}

#[derive(Deserialize, Debug)]
struct TtsMetadata {
    #[allow(dead_code)]
    backend: String,
    #[allow(dead_code)]
    #[serde(skip_serializing_if = "Option::is_none")]
    voice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f64>,
    #[allow(dead_code)]
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<u32>,
    #[allow(dead_code)]
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[allow(dead_code)]
    #[serde(skip_serializing_if = "Option::is_none")]
    size_bytes: Option<u64>,
}

/// Alibaba DashScope TTS request structure
#[derive(Serialize, Debug)]
struct AliTtsRequest {
    model: String,
    input: AliTtsInput,
}

#[derive(Serialize, Debug)]
struct AliTtsInput {
    text: String,
    voice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_type: Option<String>,
}

/// Alibaba DashScope TTS SSE response structure
#[derive(Deserialize, Debug)]
struct AliTtsResponse {
    output: Option<AliTtsOutput>,
    #[allow(dead_code)]
    request_id: Option<String>,
}

#[derive(Deserialize, Debug)]
struct AliTtsOutput {
    #[serde(default)]
    audio: Option<AliTtsAudio>,
    /// Finish reason: "null" for intermediate, "stop" for final
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct AliTtsAudio {
    /// Base64 encoded audio data chunk (may be empty)
    #[serde(default)]
    data: Option<String>,
    /// Audio URL (only in the final response when finish_reason is "stop")
    #[serde(default)]
    url: Option<String>,
    #[allow(dead_code)]
    #[serde(default)]
    id: Option<String>,
    #[allow(dead_code)]
    #[serde(default)]
    expires_at: Option<u64>,
}

/// TTS backend configuration
#[derive(Debug, Clone)]
pub enum TtsMode {
    /// Use REST API for TTS with advanced neural voices
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
        /// Audio volume (0.0 to 1.0, default is 1.0)
        volume: Option<f32>,
    },
    /// Use Alibaba DashScope TTS API (qwen3-tts)
    AliTts {
        /// DashScope API key (from DASHSCOPE_API_KEY env or config)
        api_key: String,
        /// Model to use (e.g., "qwen3-tts-flash")
        model: String,
        /// Voice ID to use (e.g., "Cherry", "Chelsie", etc.)
        voice: String,
        /// Language type (e.g., "Chinese", "English")
        language_type: Option<String>,
        /// Audio volume (0.0 to 1.0, default is 1.0)
        volume: Option<f32>,
    },
    /// Use local command-line TTS programs
    Command {
        /// The TTS command to use (e.g., "say" on macOS, "espeak-ng" on Linux)
        tts_command: String,
        /// Optional extra arguments for the TTS command (e.g., ["-v", "SinJi"])
        tts_args: Vec<String>,
    },
}

/// A plugin that sends Danmaku text to a TTS service and plays the audio sequentially.
///
/// This handler supports two modes:
/// 1. REST API mode: Sends text to a TTS REST API server, receives base64-encoded audio data,
///    decodes it and plays through the system's audio output
/// 2. Command mode: Uses local command-line TTS programs (like `say` on macOS or `espeak-ng` on Linux)
///
/// Messages are processed sequentially to avoid overlapping audio.
pub struct TtsHandler {
    /// TTS configuration (either REST API or command-based)
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
        let worker_handle = thread::spawn(move || match &mode_clone {
            TtsMode::RestApi { .. } => {
                Self::run_rest_api_worker(receiver, mode_clone);
            }
            TtsMode::AliTts { .. } => {
                Self::run_ali_tts_worker(receiver, mode_clone);
            }
            TtsMode::Command { .. } => {
                Self::run_command_worker(receiver, mode_clone);
            }
        });

        TtsHandler {
            mode,
            sender,
            _worker_handle: worker_handle,
        }
    }

    /// Create a new TTS handler with REST API using default Chinese voice settings
    pub fn new_rest_api_default(server_url: String) -> Self {
        Self::new_rest_api_default_with_volume(server_url, 1.0)
    }

    /// Create a new TTS handler with REST API using default Chinese voice settings and custom volume
    pub fn new_rest_api_default_with_volume(server_url: String, volume: f32) -> Self {
        let mode = TtsMode::RestApi {
            server_url,
            voice: Some("zh-CN-XiaoxiaoNeural".to_string()),
            backend: Some("edge".to_string()),
            quality: Some("medium".to_string()),
            format: Some("wav".to_string()),
            sample_rate: Some(22050),
            volume: Some(volume),
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
        Self::new_rest_api_with_volume(
            server_url,
            voice,
            backend,
            quality,
            format,
            sample_rate,
            None,
        )
    }

    /// Create a new TTS handler with REST API and custom configuration including volume
    pub fn new_rest_api_with_volume(
        server_url: String,
        voice: Option<String>,
        backend: Option<String>,
        quality: Option<String>,
        format: Option<String>,
        sample_rate: Option<u32>,
        volume: Option<f32>,
    ) -> Self {
        let mode = TtsMode::RestApi {
            server_url,
            voice,
            backend,
            quality,
            format,
            sample_rate,
            volume,
        };
        Self::new(mode)
    }

    /// Create a new TTS handler with command-line TTS
    pub fn new_command(tts_command: String, tts_args: Vec<String>) -> Self {
        let mode = TtsMode::Command {
            tts_command,
            tts_args,
        };
        Self::new(mode)
    }

    /// Create a new TTS handler with Alibaba DashScope TTS using default settings
    pub fn new_ali_tts_default(api_key: String) -> Self {
        Self::new_ali_tts(
            api_key,
            "qwen3-tts-flash".to_string(),
            "Cherry".to_string(),
            Some("Chinese".to_string()),
            None,
        )
    }

    /// Create a new TTS handler with Alibaba DashScope TTS and custom configuration
    pub fn new_ali_tts(
        api_key: String,
        model: String,
        voice: String,
        language_type: Option<String>,
        volume: Option<f32>,
    ) -> Self {
        let mode = TtsMode::AliTts {
            api_key,
            model,
            voice,
            language_type,
            volume,
        };
        Self::new(mode)
    }

    /// Worker thread for REST API TTS processing
    fn run_rest_api_worker(receiver: std::sync::mpsc::Receiver<String>, mode: TtsMode) {
        if let TtsMode::RestApi {
            server_url,
            voice,
            backend,
            quality,
            format,
            sample_rate,
            volume,
        } = mode
        {
            // Create a tokio runtime for HTTP requests
            let rt = tokio::runtime::Runtime::new().unwrap();
            let client = reqwest::Client::new();

            // Initialize audio output stream (this will be reused for all audio playback)
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();

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
                                match response.json::<TtsResponse>().await {
                                    Ok(tts_response) => {
                                        info!("TTS generated successfully");

                                        // Decode base64 audio data and play it
                                        match general_purpose::STANDARD
                                            .decode(&tts_response.audio_data)
                                        {
                                            Ok(audio_bytes) => {
                                                // Create a cursor from the audio bytes
                                                let cursor = Cursor::new(audio_bytes);

                                                // Create a decoder for the audio format
                                                match Decoder::new(cursor) {
                                                    Ok(source) => {
                                                        // Create a new sink for this audio
                                                        let sink =
                                                            Sink::try_new(&stream_handle).unwrap();

                                                        // Set volume if specified (default to 1.0 if not set)
                                                        let audio_volume = volume.unwrap_or(1.0);
                                                        sink.set_volume(audio_volume);

                                                        // Append the audio source to the sink
                                                        sink.append(source);

                                                        // Wait for the audio to finish playing
                                                        sink.sleep_until_end();

                                                        debug!("Audio playback completed");
                                                    }
                                                    Err(e) => error!(
                                                        "Failed to decode audio format: {}",
                                                        e
                                                    ),
                                                }
                                            }
                                            Err(e) => {
                                                error!("Failed to decode base64 audio data: {}", e)
                                            }
                                        }
                                    }
                                    Err(e) => error!("Failed to parse TTS response: {}", e),
                                }
                            } else {
                                warn!("TTS request failed with status: {}", response.status());
                            }
                        }
                        Err(e) => error!("Failed to send TTS request: {}", e),
                    }
                });
            }
        }
    }

    /// Worker thread for Alibaba DashScope TTS processing with SSE streaming
    fn run_ali_tts_worker(receiver: std::sync::mpsc::Receiver<String>, mode: TtsMode) {
        use futures::StreamExt;

        if let TtsMode::AliTts {
            api_key,
            model,
            voice,
            language_type,
            volume,
        } = mode
        {
            // Create a tokio runtime for HTTP requests
            let rt = tokio::runtime::Runtime::new().unwrap();
            let client = reqwest::Client::new();

            // Initialize audio output stream (this will be reused for all audio playback)
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();

            while let Ok(message) = receiver.recv() {
                let request = AliTtsRequest {
                    model: model.clone(),
                    input: AliTtsInput {
                        text: message,
                        voice: voice.clone(),
                        language_type: language_type.clone(),
                    },
                };

                // Make HTTP request to DashScope TTS service with SSE
                rt.block_on(async {
                    match client
                        .post("https://dashscope.aliyuncs.com/api/v1/services/aigc/multimodal-generation/generation")
                        .header("Authorization", format!("Bearer {}", api_key))
                        .header("Content-Type", "application/json")
                        .header("X-DashScope-SSE", "enable")
                        .json(&request)
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.status().is_success() {
                                // Collect all audio chunks from SSE stream
                                let mut audio_chunks: Vec<Vec<u8>> = Vec::new();
                                let mut audio_url: Option<String> = None;
                                let mut stream = response.bytes_stream();

                                let mut buffer = String::new();

                                while let Some(chunk_result) = stream.next().await {
                                    match chunk_result {
                                        Ok(chunk) => {
                                            // Append chunk to buffer
                                            if let Ok(text) = std::str::from_utf8(&chunk) {
                                                buffer.push_str(text);

                                                // Process complete SSE events in buffer
                                                while let Some(event_end) = buffer.find("\n\n") {
                                                    let event = buffer[..event_end].to_string();
                                                    buffer = buffer[event_end + 2..].to_string();

                                                    // Parse SSE event - look for data: lines
                                                    for line in event.lines() {
                                                        if let Some(data) = line.strip_prefix("data:") {
                                                            let data = data.trim();
                                                            if data.is_empty() || data == "[DONE]" {
                                                                continue;
                                                            }

                                                            match serde_json::from_str::<AliTtsResponse>(data) {
                                                                Ok(ali_response) => {
                                                                    if let Some(output) = ali_response.output {
                                                                        if let Some(audio) = output.audio {
                                                                            // Check for base64 audio data (non-empty)
                                                                            if let Some(ref audio_data) = audio.data {
                                                                                if !audio_data.is_empty() {
                                                                                    match general_purpose::STANDARD.decode(audio_data) {
                                                                                        Ok(decoded) => {
                                                                                            if !decoded.is_empty() {
                                                                                                audio_chunks.push(decoded);
                                                                                            }
                                                                                        }
                                                                                        Err(e) => {
                                                                                            debug!("Failed to decode audio chunk: {}", e);
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            // Check for audio URL (final response)
                                                                            if let Some(url) = audio.url {
                                                                                debug!("Audio URL received: {}", url);
                                                                                audio_url = Some(url);
                                                                            }
                                                                        }
                                                                        // Check if this is the final response
                                                                        if let Some(ref reason) = output.finish_reason {
                                                                            if reason == "stop" {
                                                                                debug!("Received final response with finish_reason: stop");
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(e) => {
                                                                    debug!("Failed to parse SSE data: {} - data: {}", e, data);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            error!("Error reading SSE stream: {}", e);
                                            break;
                                        }
                                    }
                                }

                                // Try to play audio - prefer URL download over streamed chunks
                                // Streamed MP3 chunks cannot be simply concatenated due to headers/frames
                                let audio_data = if let Some(url) = audio_url {
                                    // Download complete audio from URL (preferred method)
                                    info!("AliTTS: downloading audio from URL");
                                    match client.get(&url).send().await {
                                        Ok(audio_response) => {
                                            if audio_response.status().is_success() {
                                                match audio_response.bytes().await {
                                                    Ok(bytes) => {
                                                        info!("AliTTS: downloaded {} bytes", bytes.len());
                                                        Some(bytes.to_vec())
                                                    }
                                                    Err(e) => {
                                                        error!("Failed to read audio bytes: {}", e);
                                                        None
                                                    }
                                                }
                                            } else {
                                                error!("Failed to download audio: {}", audio_response.status());
                                                None
                                            }
                                        }
                                        Err(e) => {
                                            error!("Failed to fetch audio URL: {}", e);
                                            None
                                        }
                                    }
                                } else if !audio_chunks.is_empty() {
                                    // Fallback: try to use collected base64 chunks
                                    // Note: This may not work correctly for MP3 format due to concatenation issues
                                    warn!("AliTTS: No URL provided, attempting to use streamed chunks (may have decoding issues)");
                                    let combined: Vec<u8> = audio_chunks.into_iter().flatten().collect();
                                    info!("AliTTS: using {} bytes from streamed chunks", combined.len());
                                    Some(combined)
                                } else {
                                    warn!("No audio data or URL received from AliTTS");
                                    None
                                };

                                // Play the audio
                                if let Some(audio_bytes) = audio_data {
                                    let cursor = Cursor::new(audio_bytes);
                                    match Decoder::new(cursor) {
                                        Ok(source) => {
                                            let sink = Sink::try_new(&stream_handle).unwrap();
                                            let audio_volume = volume.unwrap_or(1.0);
                                            sink.set_volume(audio_volume);
                                            sink.append(source);
                                            sink.sleep_until_end();
                                            debug!("Audio playback completed");
                                        }
                                        Err(e) => error!("Failed to decode audio format: {}", e),
                                    }
                                }
                            } else {
                                let status = response.status();
                                let body = response.text().await.unwrap_or_default();
                                warn!("AliTTS request failed with status: {} - {}", status, body);
                            }
                        }
                        Err(e) => error!("Failed to send AliTTS request: {}", e),
                    }
                });
            }
        }
    }

    /// Worker thread for command-line TTS processing
    fn run_command_worker(receiver: std::sync::mpsc::Receiver<String>, mode: TtsMode) {
        if let TtsMode::Command {
            tts_command,
            tts_args,
        } = mode
        {
            while let Ok(message) = receiver.recv() {
                let mut command = Command::new(&tts_command);
                for arg in &tts_args {
                    command.arg(arg);
                }

                // Execute TTS command and wait for it to complete
                match command.arg(&message).status() {
                    Ok(status) => {
                        if status.success() {
                            debug!("TTS command completed successfully");
                        } else {
                            warn!("TTS command failed with status: {}", status);
                        }
                    }
                    Err(e) => error!("Failed to execute TTS command: {}", e),
                }
            }
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
    use crate::client::models::BiliMessage;
    use crate::client::scheduler::EventHandler;

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
    fn test_tts_handler_command_mode() {
        // Test command-based TTS (cross-platform using echo)
        let handler = TtsHandler::new_command("echo".to_string(), vec![]);

        let msg = BiliMessage::Danmu {
            user: "test_user".to_string(),
            text: "test message".to_string(),
        };
        let context = EventContext {
            cookies: None,
            room_id: 12345,
        };
        handler.handle(&msg, &context);

        // Give the worker thread some time to process the message
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_tts_handler_macos_voice() {
        let handler = TtsHandler::new_command(
            "say".to_string(),
            vec!["-v".to_string(), "Mei-Jia".to_string()],
        );

        let msg = BiliMessage::Danmu {
            user: "用户".to_string(),
            text: "你好".to_string(),
        };
        let context = EventContext {
            cookies: None,
            room_id: 12345,
        };
        handler.handle(&msg, &context);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_tts_handler_linux_voice() {
        let handler = TtsHandler::new_command(
            "espeak-ng".to_string(),
            vec!["-v".to_string(), "cmn".to_string()],
        );

        let msg = BiliMessage::Danmu {
            user: "用户".to_string(),
            text: "你好".to_string(),
        };
        let context = EventContext {
            cookies: None,
            room_id: 12345,
        };
        handler.handle(&msg, &context);
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

    #[test]
    fn test_tts_handler_with_volume() {
        // Test with custom volume setting
        let handler =
            TtsHandler::new_rest_api_default_with_volume("http://localhost:8000".to_string(), 0.5);

        let msg = BiliMessage::Danmu {
            user: "test_user".to_string(),
            text: "volume test".to_string(),
        };
        let context = EventContext {
            cookies: None,
            room_id: 12345,
        };
        handler.handle(&msg, &context);

        // Test with custom configuration including volume
        let handler_custom = TtsHandler::new_rest_api_with_volume(
            "http://localhost:8000".to_string(),
            Some("zh-CN-XiaoxiaoNeural".to_string()),
            Some("edge".to_string()),
            Some("high".to_string()),
            Some("wav".to_string()),
            Some(44100),
            Some(0.8),
        );
        let context = EventContext {
            cookies: None,
            room_id: 12345,
        };
        handler_custom.handle(&msg, &context);
    }

    #[test]
    fn test_ali_tts_handler_default() {
        // Test with a mock API key (won't actually make requests in this test)
        let handler = TtsHandler::new_ali_tts_default("test_api_key".to_string());

        let msg = BiliMessage::Danmu {
            user: "测试用户".to_string(),
            text: "你好".to_string(),
        };
        let context = EventContext {
            cookies: None,
            room_id: 12345,
        };
        handler.handle(&msg, &context);
    }

    #[test]
    fn test_ali_tts_handler_custom_config() {
        let handler = TtsHandler::new_ali_tts(
            "test_api_key".to_string(),
            "qwen3-tts-flash".to_string(),
            "Chelsie".to_string(),
            Some("English".to_string()),
            Some(0.8),
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
    fn test_ali_tts_request_serialization() {
        let request = AliTtsRequest {
            model: "qwen3-tts-flash".to_string(),
            input: AliTtsInput {
                text: "你好世界".to_string(),
                voice: "Cherry".to_string(),
                language_type: Some("Chinese".to_string()),
            },
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("qwen3-tts-flash"));
        assert!(json.contains("你好世界"));
        assert!(json.contains("Cherry"));
        assert!(json.contains("Chinese"));
    }
}
