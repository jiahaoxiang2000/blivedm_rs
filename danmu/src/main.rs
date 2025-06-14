// src/main.rs
// Standalone binary to test integration of the terminal display plugin with the BiliLiveClient

use client::scheduler::Scheduler;
use client::websocket::BiliLiveClient;
use futures::channel::mpsc;
use futures::stream::StreamExt;
use plugins::terminal_display::TerminalDisplayHandler;
use plugins::tts_handler_default;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

fn main() {
    // Load SESSDATA and room_id from CLI args or environment variables
    let args: Vec<String> = env::args().collect();
    let sessdata = if args.len() > 1 {
        args[1].clone()
    } else {
        env::var("SESSDATA").unwrap_or_else(|_| "dummy_sessdata".to_string())
    };
    let room_id = if args.len() > 2 {
        args[2].clone()
    } else {
        env::var("ROOM_ID").unwrap_or_else(|_| "24779526".to_string())
    };

    if env::var("DEBUG").unwrap_or_default() == "1" {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init();
    }
    // Get SESSDATA from environment variable for real test
    let (tx, mut rx) = mpsc::channel(64);
    let mut client = BiliLiveClient::new(&sessdata, &room_id, tx);
    client.send_auth();
    client.send_heart_beat();
    let shared_client: Arc<Mutex<BiliLiveClient>> = Arc::new(Mutex::new(client));
    let heart_beats: Arc<Mutex<BiliLiveClient>> = Arc::clone(&shared_client);

    thread::spawn(move || {
        loop {
            match heart_beats.lock() {
                Ok(mut heart_beats_c) => {
                    heart_beats_c.send_heart_beat();
                }
                Err(e) => {
                    eprintln!("Error acquiring lock on stream: {}", e);
                    break;
                }
            }
            thread::sleep(Duration::new(20, 0));
        }
    });

    let rec_msg: Arc<Mutex<BiliLiveClient>> = Arc::clone(&shared_client);
    thread::spawn(move || {
        loop {
            match rec_msg.lock() {
                Ok(mut rec_c) => {
                    rec_c.recive();
                }
                Err(e) => {
                    eprintln!("Error acquiring lock on stream: {}", e);
                    break;
                }
            }
            thread::sleep(Duration::from_millis(10)); // instead of 10 microseconds
        }
    });

    // Set up the scheduler and add the terminal display handler
    let mut scheduler = Scheduler::new();
    let terminal_handler = Arc::new(TerminalDisplayHandler);
    scheduler.add_sequential_handler(terminal_handler);

    // Add the TTS handler - supports both REST API and command-based approaches

    // Option 1: Use REST API for advanced neural voices (recommended for production)
    // Make sure the danmu-tts server is running at http://192.168.71.202:8000
    // The handler will automatically decode base64 audio data and play it
    let tts_handler = tts_handler_default("http://192.168.71.202:8000".to_string());
    scheduler.add_sequential_handler(tts_handler);

    // Option 2: Use local command-line TTS (fallback or simple setup)
    // Uncomment one of the following based on your platform:

    // For macOS with Chinese voice:
    // use plugins::tts_handler_command;
    // let tts_handler = tts_handler_command("say".to_string(), vec!["-v".to_string(), "Mei-Jia".to_string()]);
    // scheduler.add_sequential_handler(tts_handler);

    // For Linux with espeak-ng:
    // use plugins::tts_handler_command;
    // let tts_handler = tts_handler_command("espeak-ng".to_string(), vec!["-v".to_string(), "cmn".to_string()]);
    // scheduler.add_sequential_handler(tts_handler);

    // For other platforms (testing):
    // use plugins::tts_handler_command;
    // let tts_handler = tts_handler_command("echo".to_string(), vec![]);
    // scheduler.add_sequential_handler(tts_handler);

    // create a thread to process the rx channel messages using tokio runtime and pass to scheduler
    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        while let Some(msg) = rx.next().await {
            scheduler.trigger(msg);
        }
    });

    // wait the user keyboard input to exit
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("Exiting...");
    // close the client
    match shared_client.lock() {
        Ok(mut _client) => {}
        Err(e) => {
            eprintln!("Error acquiring lock on stream: {}", e);
        }
    }
    // wait for the threads to finish
    thread::sleep(Duration::new(1, 0));
    println!("Threads finished.");
}
