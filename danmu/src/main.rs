// src/main.rs
// Standalone binary to test integration of the terminal display plugin with the BiliLiveClient

use client::scheduler::Scheduler;
use client::websocket::BiliLiveClient;
use futures::channel::mpsc;
use futures::stream::StreamExt;
use plugins::terminal_display::TerminalDisplayHandler;
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

    // Add the TTS handler for macOS Chinese voice
    #[cfg(target_os = "macos")]
    {
        use plugins::tts_handler;
        let tts = tts_handler(
            "say".to_string(),
            vec!["-v".to_string(), "Mei-Jia".to_string()],
        );
        scheduler.add_sequential_handler(tts);
    }
    #[cfg(target_os = "linux")]
    {
        use plugins::tts_handler;
        let tts = tts_handler(
            "espeak-ng".to_string(),
            vec!["-v".to_string(), "cmn".to_string()],
        );
        scheduler.add_sequential_handler(tts);
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        use plugins::tts_handler;
        let tts = tts_handler("echo".to_string(), vec![]);
        scheduler.add_sequential_handler(tts);
    }

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
