// src/bin/integration_bili_live_client.rs
// Standalone binary for integration_bili_live_client logic

use blivedm::client::websocket::BiliLiveClient;
use futures::channel::mpsc;
use futures::stream::StreamExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

fn main() {
    if std::env::var("DEBUG").unwrap_or_default() == "1" {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init();
    }
    // Get SESSDATA from environment variable for real test
    let sessdata = std::env::var("SESSDATA").unwrap_or_else(|_| "dummy_sessdata".to_string());
    let (tx, mut rx) = mpsc::channel(64);
    let mut client = BiliLiveClient::new(&sessdata, "24779526", tx);
    client.send_auth();
    client.send_heart_beat();
    let shared_client: Arc<Mutex<BiliLiveClient>> = Arc::new(Mutex::new(client));
    let heart_beats: Arc<Mutex<BiliLiveClient>> = Arc::clone(&shared_client);

    thread::spawn(move || loop {
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
    });

    let rec_msg: Arc<Mutex<BiliLiveClient>> = Arc::clone(&shared_client);
    thread::spawn(move || loop {
        match rec_msg.lock() {
            Ok(mut rec_c) => {
                rec_c.recive();
            }
            Err(e) => {
                eprintln!("Error acquiring lock on stream: {}", e);
                break;
            }
        }
        thread::sleep(Duration::new(0, 10));
    });

    // create a thread to print the rx channel messages using tokio runtime
    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
        while let Some(msg) = rx.next().await {
            println!("Received message: {:?}", msg);
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
    // close the client
}
