use futures_util::Stream;
use std::io::{Error,Result};
use serde_json::Value;

use std::io;

use std::time::{Duration, Instant};
 

use alloc_no_stdlib::define_allocator_memory_pool;
use alloc_stdlib::{heap_alloc::HeapAlloc, std_alloc::StandardAlloc, HeapPrealloc};

use brotlic::{CompressorWriter, DecompressorReader};
use huffman::Tree;
use std::fs::File;
use std::io::{BufReader, Read,Write};
use tokio::{task, time};
use tungstenite::stream::*;
use tungstenite::{connect, Message, WebSocket,client};
use url::Url;
use native_tls::TlsConnector;
 
use std::net::TcpStream;




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
            print!("{}发送了弹幕:{}",json["info"][2][1].to_string(), json["info"][1].to_string());
        },
        "SEND_GIFT" =>{
            print!("{}送出了礼物:{}",json["data"]["uname"].to_string(),json["data"]["giftName"].to_string())
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
            print!("未知压缩格式")
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
        print!(
            "心跳重发请求, unknown message operation={:?}, header={:?}}}",
            head_1.operation, head_1
        )
    } else {
        print!(
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
            print!("Ping {:?}", v);
        }
        Message::Pong(v) => {
            print!("Pong {:?}", v);
        }
        Message::Close(v) => {
            print!("Close {:?}", v);
        }
        Message::Frame(v) => {
            print!("Frame {:?}", v);
        }
        _ => {
            print!("类型出错");
        }
    }
}
fn main() {
    env_logger::init();
    print!("开始连接");
    // let (mut socket, response) =
    //     connect("ws://hw-bj-live-comet-06.chat.bilibili.com:2244/sub").expect("Can't connect");

    let connector = TlsConnector::new().unwrap();
    let stream = TcpStream::connect("hw-gz-live-comet-05.chat.bilibili.com:443").unwrap() ;
    let mut stream = connector.connect("hw-gz-live-comet-05.chat.bilibili.com", stream).unwrap();       
    let (mut socket, response) =client(Url::parse("wss://hw-gz-live-comet-05.chat.bilibili.com:443/sub").unwrap(),stream).expect("Can't connect");
    
    
    
    let auth: &str = "{\"uid\":0,\"roomid\":5050,\"protover\":3,\"plateform\":\"web\",\"type\":2,\"key\":\"\"}";

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

    print!("|/-\\|{:?}", res);

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

    print!("开始接收");
    let mut now = Instant::now();
    loop {
        print!("循环中 {:?} \r\n", now);
        if socket.can_read() {
            let msg = socket.read_message().expect("Error reading message");
            // println!("Received: {}", msg);
            //实现格式化返回数据
            // todo!();
            analyze_msg(msg);
        }
        if Instant::now() > now + Duration::from_secs(30) {
            now = Instant::now();
            if socket.can_write() {
                print!("发送心跳");
                socket
                    .write_message(Message::Binary(make_packet("{}", Operation::HEARTBEAT)))
                    .unwrap();
            }
        }
    }

    // socket.close(None);
}
