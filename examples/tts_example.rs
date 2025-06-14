// Example demonstrating both TTS plugin modes
// This example shows how to use both the REST API TTS handler and command-line TTS handler:
//
// 1. REST API Mode (recommended for production):
//    - Send Danmaku text to a TTS REST API server
//    - Receive base64-encoded audio data
//    - Decode and play the audio through your system's audio output
//    - Prerequisites: Set up and run the danmu-tts server from: https://github.com/jiahaoxiang2000/danmu-tts
//
// 2. Command Mode (simple setup):
//    - Use local command-line TTS programs
//    - Works with `say` on macOS, `espeak-ng` on Linux, or any custom TTS command
//    - No external server required
//
// For local testing, you can also run the server on localhost:
// let tts_handler = tts_handler_default("http://localhost:8000".to_string());
use client::models::BiliMessage;
use client::scheduler::Scheduler;
use plugins::{tts_handler_command, tts_handler_default};

fn main() {
    println!("TTS Example - Testing both REST API and Command modes");

    // Example 1: REST API Mode
    println!("\n=== Testing REST API Mode ===");
    test_rest_api_mode();

    // Example 2: Command Mode
    println!("\n=== Testing Command Mode ===");
    test_command_mode();

    println!("\nTTS example completed!");
}

fn test_rest_api_mode() {
    // Create scheduler for REST API TTS
    let mut scheduler = Scheduler::new();

    // Add TTS handler with default Chinese voice
    // Make sure the danmu-tts server is running at http://192.168.71.202:8000
    // The handler will automatically decode base64 audio data and play it
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

    // Trigger the messages - each will be converted to speech and played sequentially
    for msg in messages {
        scheduler.trigger(msg);
    }

    // Give time for TTS processing and audio playback
    std::thread::sleep(std::time::Duration::from_secs(6));
}

fn test_command_mode() {
    // Create scheduler for command-line TTS
    let mut scheduler = Scheduler::new();

    // Choose TTS command based on platform
    #[cfg(target_os = "macos")]
    let tts_handler = tts_handler_command(
        "say".to_string(),
        vec!["-v".to_string(), "Mei-Jia".to_string()],
    );

    #[cfg(target_os = "linux")]
    let tts_handler = tts_handler_command(
        "espeak-ng".to_string(),
        vec!["-v".to_string(), "cmn".to_string()],
    );

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let tts_handler = tts_handler_command("echo".to_string(), vec![]);

    scheduler.add_sequential_handler(tts_handler);

    // Simulate some danmu messages for command-line TTS
    let messages = vec![
        BiliMessage::Danmu {
            user: "观众3".to_string(),
            text: "命令行模式测试".to_string(),
        },
        BiliMessage::Danmu {
            user: "观众4".to_string(),
            text: "本地语音合成".to_string(),
        },
    ];

    // Trigger the messages
    for msg in messages {
        scheduler.trigger(msg);
    }

    // Give time for command execution
    std::thread::sleep(std::time::Duration::from_secs(4));
}
