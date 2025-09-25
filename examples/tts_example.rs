// Example demonstrating TTS plugin REST API mode
// This example shows how to use the TTS handler:
//
// REST API Mode:
//    - Send Danmaku text to a TTS REST API server
//    - The server handles audio generation and playback
//    - Prerequisites: Set up and run the danmu-tts server from: https://github.com/jiahaoxiang2000/danmu-tts
//
// For local testing, you can also run the server on localhost:
// let tts_handler = tts_handler_default("http://localhost:8000".to_string());
use client::models::BiliMessage;
use client::scheduler::{Scheduler, EventContext};
use plugins::tts_handler_default;

fn main() {
    println!("TTS Example - Testing REST API mode");

    // REST API Mode
    println!("\n=== Testing REST API Mode ===");
    test_rest_api_mode();

    println!("\nTTS example completed!");
}

fn test_rest_api_mode() {
    // Create scheduler for REST API TTS
    let context = EventContext { cookies: None, room_id: 12345 };
    let mut scheduler = Scheduler::new(context);

    // Add TTS handler with default Chinese voice
    // Make sure the danmu-tts server is running at http://192.168.71.202:8000
    // The handler will send HTTP requests to the TTS server
    let tts_handler = tts_handler_default("http://192.168.71.202:8000".to_string());
    scheduler.add_sequential_handler(tts_handler);

    // Simulate some danmu messages for REST API
    let messages = vec![
        BiliMessage::Danmu {
            user: "观众1".to_string(),
            text: "REST API 模式测试".to_string(),
        },
        BiliMessage::Danmu {
            user: "观众2".to_string(),
            text: "神经网络语音合成".to_string(),
        },
    ];

    // Trigger the messages - each will be sent to the TTS server sequentially
    for msg in messages {
        scheduler.trigger(msg);
    }

    // Give time for TTS processing
    std::thread::sleep(std::time::Duration::from_secs(3));
}
