// client/test/main.rs
// Integration test entry for the client library

use client::websocket::BiliLiveClient;
use futures::channel::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn integration_bili_live_client() {
    let sessdata = "";
    let (tx, mut rx) = mpsc::channel(64);
    let mut client = BiliLiveClient::new(sessdata, "5050", tx);
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
}
