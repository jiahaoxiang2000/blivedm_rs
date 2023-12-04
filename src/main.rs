use imgui::*;
use std::collections::VecDeque;
mod support;
mod bili_live_dm;
use futures::channel::mpsc;
use futures::Sink;

fn main() {
    
    let mut danmu_queue: VecDeque<String> = VecDeque::with_capacity(20);

    
    let (mut tx,mut rx) = mpsc::channel(64);
    tx.try_send("开启弹幕机".to_string());

    let system = support::init(file!());

    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];

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
