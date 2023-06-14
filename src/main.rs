use tungstenite::{connect, Message};
use url::Url;
use serde_json::*;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
enum Operation {
    HANDSHAKE,
    HANDSHAKE_REPLY ,
    HEARTBEAT ,
    HEARTBEAT_REPLY,
    SEND_MSG ,
    SEND_MSG_REPLY,
    DISCONNECT_REPLY ,
    AUTH ,
    AUTH_REPLY ,
    RAW ,
    PROTO_READY ,
    PROTO_FINISH ,
    CHANGE_ROOM ,
    CHANGE_ROOM_REPLY,
    REGISTER ,
    REGISTER_REPLY ,
    UNREGISTER ,
    UNREGISTER_REPLY ,
    // # B站业务自定义OP,
    // # MinBusinessOp : 1000,
    // # MaxBusinessOp : 10000
}

struct Operation_str {
    opr: Operation,
    value: i32
}

fn make_packet(body:&str,ops:Operation_str)->Vec<u8>{
    let json:Value =  serde_json::from_str(body).unwrap();
    // print!("{:?}{:?}",json["roomid"],json["key"] );
    let temp =json.to_string();
    let body_content:&[u8]= temp.as_bytes();

    let pack_len: [u8; 4] = ((16+body.len() )as u32).to_be_bytes();
    let raw_header_size:[u8; 2]  = (16 as u16).to_be_bytes();
    let ver:[u8; 2]=(1 as u16).to_be_bytes();
    let operation:[u8; 4]=ops.value.to_be_bytes();
    let seq_id:[u8; 4] = (1 as u32).to_be_bytes();
    
    let mut res= pack_len.to_vec() ;
    res.append(&mut raw_header_size.to_vec());
    res.append(&mut ver.to_vec());
    res.append(&mut operation.to_vec());
    res.append(&mut seq_id.to_vec());
    res.append(&mut body_content.to_vec());
    res
}


fn main() {
    env_logger::init();

    let (mut socket, response) =
        connect("ws://tx-gz-live-comet-02.chat.bilibili.com:2244/sub").expect("Can't connect");


    let auth: &str = "{\"uid\":0,\"roomid\":5050,\"protover\":3,\"plateform\":\"web\",\"type\":2,\"key\":\"WjtU0W1CkkXVqJKALYPrjK7EA4sojMY9E9KNSCg8-WEXSt5jL8YivpCXdOmNoCyXe3YeSUq7fMpUBVITV-4wJOtGuSHvQXKL5goc0PY3C6IKBB0rySe_KEyAnd4HYuA842wg2pNEJ6g9oQ==\"}";

    let h_o = Operation_str{
        opr:Operation::HEARTBEAT,
        value:2
    };

    let a_o = Operation_str{
        opr:Operation::AUTH,
        value:7
    };
    let res = make_packet(auth,a_o);

    print!("|/-\\|{:?}",res);



    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    let tnct = tokio::runtime::Builder::new_current_thread().build().unwrap();
    //发送授权验证
    socket.write_message(Message::Binary(res)).unwrap();
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        loop{
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            //实现心跳
            todo!();
        } 
    });

    loop {
        if socket.can_read(){
            let msg = socket.read_message().expect("Error reading message");
            println!("Received: {}", msg);
            //实现格式化返回数据
            todo!();
        }
    }
    // socket.close(None);
}
