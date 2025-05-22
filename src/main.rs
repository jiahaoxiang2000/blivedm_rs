use std::collections::VecDeque;
mod bili_live_dm;
use bili_live_dm::{web::*, BiliLiveClient};
use futures::channel::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let sessdata = "";
    // let (server_info,auth_msg) = bili_live_dm::init_server(sessdata, "813364");
    // // let danmu_server = bili_live_dm::gen_damu_list(&server_info["host_list"]);
    // let (mut socket, _resp) = bili_live_dm::connect(server_info["host_list"].clone());

    // //发送授权报文
    // let auth_msg_str = serde_json::to_string(&auth_msg).unwrap();
    // println!("授权消息:{}",auth_msg_str);
    // socket.send(Message::Binary(make_packet(auth_msg_str.as_str(), Operation::AUTH))).unwrap();

    // socket.send(Message::Binary(make_packet("{}", Operation::HEARTBEAT))).unwrap();

    let (tx, mut rx) = mpsc::channel(64);
    let mut client = BiliLiveClient::new(sessdata, "5050", tx);
    client.send_auth();
    client.send_heart_beat();
    let shared_client = Arc::new(Mutex::new(client));
    let heart_beats = Arc::clone(&shared_client);

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

    let rec_msg = Arc::clone(&shared_client);
    // let tx = tx.clone();
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
