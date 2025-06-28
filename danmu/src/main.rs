// src/main.rs
// Standalone binary to test integration of the terminal display plugin with the BiliLiveClient

use clap::Parser;
use client::scheduler::Scheduler;
use client::websocket::BiliLiveClient;
use futures::channel::mpsc;
use futures::stream::StreamExt;
use plugins::terminal_display::TerminalDisplayHandler;
use plugins::tts::TtsHandler;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Cookies for Bilibili authentication (optional - will auto-detect from browser if not provided)
    #[arg(long, value_name = "COOKIES")]
    cookies: Option<String>,

    /// Room ID to connect to
    #[arg(long, value_name = "ROOM_ID")]
    room_id: Option<String>,

    /// TTS REST API server URL
    #[arg(long, value_name = "URL")]
    tts_server: Option<String>,

    /// TTS voice ID (e.g., "zh-CN-XiaoxiaoNeural")
    #[arg(long, value_name = "VOICE")]
    tts_voice: Option<String>,

    /// TTS backend ("edge", "xtts", "piper")
    #[arg(long, value_name = "BACKEND")]
    tts_backend: Option<String>,

    /// TTS audio quality ("low", "medium", "high")
    #[arg(long, value_name = "QUALITY")]
    tts_quality: Option<String>,

    /// TTS audio format (e.g., "wav")
    #[arg(long, value_name = "FORMAT")]
    tts_format: Option<String>,

    /// TTS sample rate (e.g., 22050, 44100)
    #[arg(long, value_name = "RATE")]
    tts_sample_rate: Option<u32>,

    /// TTS audio volume (0.0 to 1.0)
    #[arg(long, value_name = "VOLUME")]
    tts_volume: Option<f32>,

    /// Local TTS command (e.g., "say", "espeak-ng")
    #[arg(long, value_name = "COMMAND")]
    tts_command: Option<String>,

    /// Comma-separated arguments for TTS command
    #[arg(long, value_name = "ARGS")]
    tts_args: Option<String>,

    /// Enable debug logging
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    // Load cookies and room_id from CLI args or environment variables
    let cookies = args.cookies.or_else(|| {
        env::var("Cookie")
            .ok()
            .filter(|s| !s.is_empty() && s != "SESSDATA=dummy_sessdata")
    });

    let room_id = args
        .room_id
        .or_else(|| env::var("ROOM_ID").ok())
        .unwrap_or_else(|| "24779526".to_string());

    // Initialize logging
    if args.debug || env::var("DEBUG").unwrap_or_default() == "1" {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init();
    }

    // Create client with automatic browser cookie detection
    let (tx, mut rx) = mpsc::channel(64);
    let mut client = match BiliLiveClient::new_auto(cookies.as_deref(), &room_id, tx) {
        Ok(client) => {
            log::info!("Successfully created client with automatic cookie detection");
            client
        }
        Err(e) => {
            eprintln!("Failed to create client: {}", e);
            eprintln!(
                "Please ensure you are logged into bilibili.com in your browser, or provide cookies manually."
            );
            std::process::exit(1);
        }
    };
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

    // Configure TTS based on command-line arguments
    if let Some(server_url) = args.tts_server {
        // REST API TTS configuration
        let tts_handler = Arc::new(TtsHandler::new_rest_api_with_volume(
            server_url,
            args.tts_voice,
            args.tts_backend,
            args.tts_quality,
            args.tts_format,
            args.tts_sample_rate,
            args.tts_volume,
        ));
        scheduler.add_sequential_handler(tts_handler);
        println!("TTS configured with REST API server");
    } else if let Some(tts_command) = args.tts_command {
        // Command-line TTS configuration
        let tts_args = args
            .tts_args
            .map(|s| s.split(',').map(|s| s.to_string()).collect())
            .unwrap_or_default();
        let tts_handler = Arc::new(TtsHandler::new_command(tts_command, tts_args));
        scheduler.add_sequential_handler(tts_handler);
        println!("TTS configured with local command");
    } else {
        println!("No TTS configuration provided. Use --tts-server or --tts-command to enable TTS.");
    }

    // Print configuration information
    println!("Bilibili Danmu Client");
    println!("Connected to room: {}", room_id);
    if let Some(cookies_val) = &cookies {
        println!(
            "Using provided cookies: {}...",
            &cookies_val.chars().take(30).collect::<String>()
        );
    } else {
        println!("Using auto-detected cookies from browser");
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
