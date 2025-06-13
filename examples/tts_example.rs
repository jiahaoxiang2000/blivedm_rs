// Example demonstrating the TTS plugin with REST API
// This example shows how to use the TTS handler which will:
// 1. Send Danmaku text to a TTS REST API server
// 2. Receive base64-encoded audio data
// 3. Decode and play the audio through your system's audio output
//
// Prerequisites:
// - Set up and run the danmu-tts server from: https://github.com/jiahaoxiang2000/danmu-tts
// - The server should be running at http://192.168.71.202:8000 (or update the URL below)
//
// For local testing, you can also run the server on localhost:
// let tts_handler = tts_handler_default("http://localhost:8000".to_string());
use client::models::BiliMessage;
use client::scheduler::Scheduler;
use plugins::tts_handler_default;

fn main() {
    // Create scheduler
    let mut scheduler = Scheduler::new();

    // Add TTS handler with default Chinese voice
    // Make sure the danmu-tts server is running at http://192.168.71.202:8000
    // The handler will automatically decode base64 audio data and play it
    let tts_handler = tts_handler_default("http://192.168.71.202:8000".to_string());
    scheduler.add_sequential_handler(tts_handler);

    // Simulate some danmu messages
    let messages = vec![
        BiliMessage::Danmu {
            user: "观众1".to_string(),
            text: "大家好！".to_string(),
        },
        BiliMessage::Danmu {
            user: "观众2".to_string(),
            text: "主播加油！".to_string(),
        },
        BiliMessage::Danmu {
            user: "观众3".to_string(),
            text: "感谢分享！".to_string(),
        },
    ];

    // Trigger the messages - each will be converted to speech and played sequentially
    for msg in messages {
        scheduler.trigger(msg);
    }

    // Give time for TTS processing and audio playback
    // The handler processes messages sequentially, so each audio will play fully before the next
    std::thread::sleep(std::time::Duration::from_secs(10));

    println!(
        "TTS example completed. You should have heard audio playback if the TTS server is running."
    );
}
