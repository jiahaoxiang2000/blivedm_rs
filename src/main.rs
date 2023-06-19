use std::io::{Error,Result};
use serde_json::Value;

use std::io;

use std::time::{Duration, Instant};
use brotlic::{CompressorWriter, DecompressorReader};
 
 
use std::io::{BufReader, Read,Write};
use tokio::{task, time};
use tungstenite::stream::*;
use tungstenite::{connect, Message, WebSocket,client};
use url::Url;
use native_tls::TlsConnector;
use std::collections::HashMap;
use std::net::TcpStream;
use std::thread;
use log::{debug, error, log_enabled, info, Level};
 

enum Operation {
    HANDSHAKE,
    HANDSHAKE_REPLY,
    HEARTBEAT,
    HEARTBEAT_REPLY,
    SEND_MSG,
    SEND_MSG_REPLY,
    DISCONNECT_REPLY,
    AUTH,
    AUTH_REPLY,
    RAW,
    PROTO_READY,
    PROTO_FINISH,
    CHANGE_ROOM,
    CHANGE_ROOM_REPLY,
    REGISTER,
    REGISTER_REPLY,
    UNREGISTER,
    UNREGISTER_REPLY,
    // # B站业务自定义OP,
    // # MinBusinessOp : 1000,
    // # MaxBusinessOp : 10000
}

// struct Operation_str {
//     opr: Operation,
//     value: i32
// }

 


fn make_packet(body: &str, ops: Operation) -> Vec<u8> {
    let json: Value = serde_json::from_str(body).unwrap();
    // print!("{:?}{:?}",json["roomid"],json["key"] );
    let temp = json.to_string();
    let body_content: &[u8] = temp.as_bytes();

    let pack_len: [u8; 4] = ((16 + body.len()) as u32).to_be_bytes();
    let raw_header_size: [u8; 2] = (16 as u16).to_be_bytes();
    let ver: [u8; 2] = (1 as u16).to_be_bytes();
    let mut operation: [u8; 4] = [0; 4];
    match ops {
        Operation::AUTH => {
            operation = (7 as u32).to_be_bytes();
        }
        Operation::HEARTBEAT => {
            operation = (2 as u32).to_be_bytes();
        }
        _ => {
            operation = (2 as u32).to_be_bytes();
        }
    }
    let seq_id: [u8; 4] = (1 as u32).to_be_bytes();

    let mut res = pack_len.to_vec();
    res.append(&mut raw_header_size.to_vec());
    res.append(&mut ver.to_vec());
    res.append(&mut operation.to_vec());
    res.append(&mut seq_id.to_vec());
    res.append(&mut body_content.to_vec());
    res
}

#[derive(Copy, Clone, Debug)]
pub struct MSG_HEAD {
    pack_len: u32,
    raw_header_size: u16,
    ver: u16,
    operation: u32,
    seq_id: u32,
}

fn get_msg_header(v_s: &[u8]) -> MSG_HEAD {
    let mut pack_len: [u8; 4] = [0; 4];
    let mut raw_header_size: [u8; 2] = [0; 2];
    let mut ver: [u8; 2] = [0; 2];
    let mut operation: [u8; 4] = [0; 4];
    let mut seq_id: [u8; 4] = [0; 4];
    for (i, v) in v_s.iter().enumerate() {
        if i < 4 {
            pack_len[i] = *v;
            continue;
        }
        if i < 6 {
            raw_header_size[i - 4] = *v;
            continue;
        }
        if i < 8 {
            ver[i - 6] = *v;
            continue;
        }
        if i < 12 {
            operation[i - 8] = *v;
            continue;
        }
        if i < 16 {
            seq_id[i - 12] = *v;
            continue;
        }
    }

    MSG_HEAD {
        pack_len: u32::from_be_bytes(pack_len),
        raw_header_size: u16::from_be_bytes(raw_header_size),
        ver: u16::from_be_bytes(ver),
        operation: u32::from_be_bytes(operation),
        seq_id: u32::from_be_bytes(seq_id),
    }
}

fn handle(json:Value){
    let category = json["cmd"].as_str().unwrap();
    match category {
        "DANMU_MSG" =>{
            println!("{}:{}",json["info"][2][1].to_string(), json["info"][1].to_string());

            // debug!("{}发送了弹幕:{}",json["info"][2][1].to_string(), json["info"][1].to_string());
        },
        "SEND_GIFT" =>{
            println!("{}送出了礼物:{}",json["data"]["uname"].to_string(),json["data"]["giftName"].to_string());
            // debug!("{}送出了礼物:{}",json["data"]["uname"].to_string(),json["data"]["giftName"].to_string());
        },
        _ =>{
        }

        
    }


}




fn parse_business_message(head: MSG_HEAD, body: &[u8]) {
    if head.operation == 5{
        if head.ver == 3 {
            let res = decompress(body).unwrap();
            parse_ws_message(res);
        } else if head.ver == 0 {
            let s = String::from_utf8(body.to_vec()).unwrap();
            let res_json:Value = serde_json::from_str(s.as_str()).unwrap();
            handle(res_json);
            
        } else {
            println!("未知压缩格式")
        }
    
    }
    
}

fn decompress(body: &[u8])->Result<Vec<u8>> {
    //brotli解压缩byte数组
    let mut decompressed_reader = DecompressorReader::new(body);
    let mut decoded_input = Vec::new();

    let size = decompressed_reader.read_to_end(&mut decoded_input)?;
     Ok(decoded_input)

     
}

fn parse_ws_message(v: Vec<u8> )  {
    // let total_len = v.len();
    let mut offset = 0;
    let header = &v[0..16];
    let mut head_1 = get_msg_header(header);

    if head_1.operation == 5 || head_1.operation == 8 {
        loop {
            // let pl = head_1.pack_len.clone();
            let body: &[u8] = &v[offset + 16..offset + (head_1.pack_len as usize)];
            // let s = String::from_utf8(body.to_vec()).unwrap();
            parse_business_message(head_1, body);
            offset += head_1.pack_len as usize;
            if offset >= v.len() {
                break;
            }
             
            let temp_head = &v[offset..(offset+16)];
            head_1 = get_msg_header(temp_head); 
        }
    } else if head_1.operation == 3 {
        println!(
            "心跳重发请求, unknown message operation={:?}, header={:?}}}",
            head_1.operation, head_1
        )
    } else {
        println!(
            "未知消息, unknown message operation={:?}, header={:?}}}",
            head_1.operation, head_1
        )
    }
}

fn analyze_msg(msg: Message) {
    match msg {
        Message::Text(s) => {
            let res: &str = &s[1..s.len() - 1];
            // print!("Text {:?}", res);
        }
        Message::Binary(v) => {
            // print!("Binary {:?}", v);
            
            parse_ws_message(v);
        }
        Message::Ping(v) => {
            println!("Ping {:?}", v);
        }
        Message::Pong(v) => {
            println!("Pong {:?}", v);
        }
        Message::Close(v) => {
            println!("Close {:?}", v);
        }
        Message::Frame(v) => {
            println!("Frame {:?}", v);
        }
        _ => {
            println!("类型出错");
        }
    }
}
 
 
fn get(url:&str)->core::result::Result<String,Box<dyn std::error::Error>>{
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body);
    Ok(body)
}

 fn main()  {
    env_logger::init();
    println!("开始连接");
    // let (mut socket, response) =
    //     connect("ws://hw-bj-live-comet-06.chat.bilibili.com:2244/sub").expect("Can't connect");


    let room_init_url = format!("https://api.live.bilibili.com/room/v1/Room/room_init?id={}","813364");
    // let ini_res =get(room_init_url.as_str());
    println!("{}",room_init_url);
    
    let resp = get(room_init_url.as_str());

    let room_info:Value  = serde_json::from_str(resp.unwrap().as_str()).unwrap();
    println!("直播间真实id {:#?}", room_info["data"]["room_id"]);
    let live_info_url: String = format!("https://api.live.bilibili.com/room/v1/Danmu/getConf?room_id={}&platform=pc&player=web",room_info["data"]["room_id"]);


    let resp_1 = get(live_info_url.as_str());

    let live_info:Value=  serde_json::from_str(resp_1.unwrap().as_str()).unwrap();
    
    let host = live_info["data"]["host_server_list"][0]["host"].as_str();
    let wss_port = live_info["data"]["host_server_list"][0]["wss_port"].to_string();
    let token = live_info["data"]["token"].as_str();

    // let   host:core::result::Result<&str, Box<dyn std::error::Error>> = Ok("broadcastlv.chat.bilibili.com");
    let mut url1 = host.unwrap().to_string();
    url1.push_str(":");
    url1.push_str(wss_port.as_str());
   
    let mut url2 = String::from("wss://");
    url2.push_str(host.unwrap() );
    url2.push_str(":");
    url2.push_str(wss_port.as_str());
    url2.push_str("/sub");

    println!("地址 {}",url1);

    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect(url1).unwrap() ;
    let mut stream = connector.connect(host.unwrap(), stream).unwrap();       
    let (mut socket, response) =client(Url::parse(&url2).unwrap(),stream).expect("Can't connect");
    
    
    let mut  temp = String::from("{\"uid\":0,\"roomid\":"); 
    temp.push_str(room_info["data"]["room_id"].to_string().as_str());
    temp.push_str(",\"protover\":3,\"plateform\":\"web\",\"type\":2,\"key\":\"");
    temp.push_str(token.unwrap());
    temp.push_str("\"}");
    let auth: &str = temp.as_str();
    
    // let h_o = Operation_str{
    //     opr:Operation::HEARTBEAT,
    //     value:2
    // };

    // let a_o = Operation_str{
    //     opr:Operation::AUTH,
    //     value:7
    // };
    // let h_o = Operation::HEARTBEAT(2);
    // let a_o = Operation::HEARTBEAT(7);
    let res = make_packet(auth, Operation::AUTH);

    println!("|/-\\|{:?}", res);

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");

    // let tnct = tokio::runtime::Builder::new_current_thread().build().unwrap();
    //发送授权验证
    socket.write_message(Message::Binary(res)).unwrap();
    // let rt = Runtime::new().unwrap();
    // rt.block_on(async {
    //     loop{
    //         tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    //         //实现心跳
    //         // todo!();

    //         let heart_msg = make_packet("{}",Operation::HEARTBEAT);
    //         socket.write_message(Message::Binary(heart_msg)).unwrap();
    //     }
    // });

    // let rt = Runtime::new().unwrap();
    // let _guard = rt.enter();
    // let arc_socket = Arc::new(socket );
    // let copy_socket = socket.clone();

    println!("开始接收");
    let mut now = Instant::now();
    loop {
         
        if socket.can_read() {
            let msg = socket.read_message().expect("Error reading message");
            // println!("Received: {}", msg);
            //实现格式化返回数据
            // todo!();
            analyze_msg(msg);
        }
        if Instant::now() > now + Duration::from_secs(20) {
            now = Instant::now();
            if socket.can_write() {
                println!("发送心跳");
                socket
                    .write_message(Message::Binary(make_packet("{}", Operation::HEARTBEAT)))
                    .unwrap();
            }
        }
        // thread::sleep(Duration::from_secs(1));
    }

    // socket.close(None);
}
