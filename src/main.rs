use imgui::*;
use std::collections::VecDeque;
mod support;
mod bili_live_dm;
use tungstenite::Message;
use futures::channel::mpsc;
use std::sync::{Arc, Mutex}; 
use std::thread;
use bili_live_dm::web::*;
use std::time::Duration;

fn main() {
    let sessdata = "3b2be85e%2C1716943731%2C5b579%2Ac2CjA1nhbZeS1AyhLoHnccXYPEfYZEShmZkQEvS0zl3h2ddHDngOmoDvhxVkibLOC9_1ESVmdreUJPR2FmQ0FoVVJETDhRVjdGUEZXU210TU5ya1FQLUNWNFE0eWlnbmVDUU5UNmJVeEpJZHZGWnZYVVIwZHByWHl0YjNDMFpkelhKOGJzQVhiOWJRIIEC";
    let (server_info,auth_msg) = bili_live_dm::init_server(sessdata, "813364");
    // let danmu_server = bili_live_dm::gen_damu_list(&server_info["host_list"]);
    let (mut socket, _resp) = bili_live_dm::connect(server_info["host_list"].clone());
     
        
    //发送授权报文 
    let auth_msg_str = serde_json::to_string(&auth_msg).unwrap();
    println!("授权消息:{}",auth_msg_str);   
    socket.send(Message::Binary(make_packet(auth_msg_str.as_str(), Operation::AUTH))).unwrap();
         
    // socket.send(Message::Binary(make_packet("{}", Operation::HEARTBEAT))).unwrap();
         
    let shared_stream = Arc::new(Mutex::new(socket));
    let heart_beats = Arc::clone(&shared_stream);

    thread::spawn(move || {
        loop {
            match heart_beats.lock() {
                Ok(mut locked_stream) => {
                    if locked_stream.can_write(){
                        locked_stream.send(Message::Binary(make_packet("{}", Operation::HEARTBEAT)))
                        .unwrap();
                    }
                            
                },
                Err(e) => {
                    eprintln!("Error acquiring lock on stream: {}", e);
                    break;
                }
            }
            thread::sleep(Duration::new(30, 0));
        }
    });


    let (tx,mut rx) = mpsc::channel(64);
    
    let rec_msg = Arc::clone(&shared_stream);
        // let tx = tx.clone();
    thread::spawn(move || {
        loop{
            match rec_msg.lock() {
                Ok(mut locked_stream) => {
                    if locked_stream.can_read(){
                         
                        let msg = locked_stream.read().expect("Error reading message");
                        let res_msg_arr = analyze_msg(msg);
                        for i in res_msg_arr{
                            let mut tx = tx.clone();
                            let _ = tx.try_send(i);
                        }
                        
                    }
                }
                Err(e) =>{
                    eprintln!("Error acquiring lock on stream: {}", e);
                    break;
                }
            }
            
        
        }
    });

    
    let mut danmu_queue: VecDeque<String> = VecDeque::with_capacity(20);
     
    danmu_queue.push_front("开启弹幕机".to_string());

    let system = support::init(file!());

 

    system.main_loop(move |_, ui| {
        let rec = rx.try_next();
        match rec{
            Ok(msg)=>{
                if danmu_queue.len()>=20{
                    danmu_queue.pop_front();
                }
                let recive = msg.unwrap();
                danmu_queue.push_back(recive);
            },
            _=>{  
            }
        }

        ui.window("Hello world")
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(|| {
                let v = danmu_queue.clone();
                for i in v{
                    ui.text_wrapped(i);

                } 
            });
    });
}
