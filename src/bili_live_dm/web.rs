
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use serde_json::Value;
use std::collections::HashMap;
use tungstenite::Message;
use tungstenite::stream::*;
use brotlic::{CompressorWriter, DecompressorReader};
use serde::{Deserialize, Serialize};

pub const UID_INIT_URL:&str = "https://api.bilibili.com/x/web-interface/nav";

pub const BUVID_INIT_URL:&str = "https://data.bilibili.com/v/";

pub const ROOM_INIT_URL:&str = "https://api.live.bilibili.com/xlive/web-room/v1/index/getInfoByRoom";

pub const DANMAKU_SERVER_CONF_URL:&str = "https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo";

// pub const DEFAULT_DANMAKU_SERVER_HOST:&str = "broadcastlv.chat.bilibili.com";

// pub const DEFAULT_DANMAKU_SERVER_PORT:i32 = 2243;

// pub const DEFAULT_DANMAKU_SERVER_WSSPORT:i32 = 443;

// pub const DEFAULT_DANMAKU_SERVER_WSPORT:i32 = 2244;

pub const USER_AGENT:&str= "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36";

pub trait Handler{
    fn deal_message(&self);
}

 


#[derive(Debug)]
pub struct DanmuServer{
    pub host:String,
    pub port:i32,
    pub wss_port:i32,
    pub ws_port:i32,
}

impl DanmuServer{
    pub fn deafult()->DanmuServer{
        DanmuServer{
            host:String::from("broadcastlv.chat.bilibili.com"),
            port:2243,
            wss_port:443,
            ws_port:2244,

        }
    }
}


// pub fn get_danmu_info_from(body:&str)->Vec<DanmuServer>{
//     let json: Value = serde_json::from_str(body).unwrap();
//     let category = json["cmd"].as_array().unwrap();
    


// }




pub enum Operation {
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



pub fn make_packet(body: &str, ops: Operation) -> Vec<u8> {
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
    pub pack_len: u32,
    pub raw_header_size: u16,
    pub ver: u16,
    pub operation: u32,
    pub seq_id: u32,
}

pub fn get_msg_header(v_s: &[u8]) -> MSG_HEAD {
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


pub fn handle(json:Value)->String{
    let category = json["cmd"].as_str().unwrap();
    let res:String;
    match category {
        "DANMU_MSG" =>{
            // println!("{}:{}",json["info"][2][1].to_string(), json["info"][1].to_string());
            res = format!("{}发送弹幕:{}",json["info"][2][1].to_string(), json["info"][1].to_string());
            // debug!("{}发送了弹幕:{}",json["info"][2][1].to_string(), json["info"][1].to_string());
        },
        "SEND_GIFT" =>{
            // println!("{}送出礼物:{}",json["data"]["uname"].to_string(),json["data"]["giftName"].to_string());
            // debug!("{}送出了礼物:{}",json["data"]["uname"].to_string(),json["data"]["giftName"].to_string());
            res = format!("{}送出礼物:{}",json["info"][2][1].to_string(), json["info"][1].to_string());
        },
        _ =>{
            println!("未知消息:{:?}",json);
            res="未知消息".to_string();
        }
    }
    res

}

fn parse_business_message(head: MSG_HEAD, body: &[u8], temp:&mut Vec<String>) {
    if head.operation == 5{
        if head.ver == 3 {
            let res = decompress(body).unwrap();
            parse_ws_message(&res,temp);
        } else if head.ver == 0 {
            let s = String::from_utf8(body.to_vec()).unwrap();
            let res_json:Value = serde_json::from_str(s.as_str()).unwrap();
            let res = handle(res_json);
            if res!="未知消息".to_string(){
                temp.push(res);
            }
        } else {
            println!("未知压缩格式")
        }
    
    }
    
}


pub fn decompress(body: &[u8])->std::io::Result<Vec<u8>> {
    //brotli解压缩byte数组
    let mut decompressed_reader: DecompressorReader<&[u8]> = DecompressorReader::new(body);
    let mut decoded_input = Vec::new();

    let size = decompressed_reader.read_to_end(&mut decoded_input)?;
     Ok(decoded_input)

     
}

fn parse_ws_message(v: & Vec<u8> , temp:&mut Vec<String>) {
    // let total_len = v.len();
    let mut offset = 0;
    let header = &v[0..16];
    let mut head_1 = get_msg_header(header);

    if head_1.operation == 5 || head_1.operation == 8 {
        loop {
            // let pl = head_1.pack_len.clone();
            let body: &[u8] = &v[offset + 16..offset + (head_1.pack_len as usize)];
            // let s = String::from_utf8(body.to_vec()).unwrap();
            parse_business_message(head_1, body,temp);
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

pub fn analyze_msg(msg: Message)->Vec<String> {
    let mut a = Vec::new();
    match msg {
        Message::Text(s) => {
            let res: &str = &s[1..s.len() - 1];
            // print!("Text {:?}", res);
        }
        Message::Binary(v) => {
            // print!("Binary {:?}", v);
            
            parse_ws_message(&v,&mut a);
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
    a
}



#[derive(Debug, Serialize, Deserialize)]
pub struct AuthMessage{
    uid: u64,
    roomid: u64,
    protover:i32,
    platform:String,
    type_:i32,
    buvid:String,
    key:String,
}
 

impl AuthMessage{
    pub fn from(map:&HashMap<String,String>)->AuthMessage{
        AuthMessage{
            uid: map.get("uid").unwrap().parse::<u64>().unwrap(),
            roomid: map.get("room_id").unwrap().parse::<u64>().unwrap(),
            protover:3,
            platform:"web".to_string(),
            type_:2,
            buvid:map.get("buvid").unwrap().to_string(),
            key:map.get("token").unwrap().to_string(),
        }
    }


}
 