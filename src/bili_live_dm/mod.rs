
use serde_json::Value;
use tungstenite::{ Message,client,protocol::*};
use std::time::{Duration, Instant};
use native_tls::TlsStream;
use std::net::TcpStream;

use url::Url;
use native_tls::TlsConnector;

use reqwest::StatusCode;

use serde_json;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE,USER_AGENT};
use std::sync::{Arc, Mutex}; 
use http::Response;
use futures_channel::mpsc::Sender;
use std::thread;


pub mod web;
use web::*;




 
fn init_uid(headers:HeaderMap) -> (StatusCode,String) {
    // 设置请求的URL
    let client = reqwest::blocking::Client::builder().https_only(true).build().unwrap();
    let response = client.get(web::UID_INIT_URL).headers(headers).send();
    println!("init_uid:{:?}",response);
    let mut stat:StatusCode;
    let mut body:String;
    match response{
        Ok(resp)=>{
            stat = resp.status();
            body = resp.text().unwrap();
        },
        Err(e)=>{
            panic!("init uid failed");
        }
    
    }
    (stat, body)
}

fn init_buvid(headers:HeaderMap) ->  (StatusCode,String) {
    // 设置请求的URL
    let client = reqwest::blocking::Client::builder().https_only(true).build().unwrap();
    let response = client.get(web::BUVID_INIT_URL).headers(headers).send();
    let mut stat:StatusCode;
    let mut buvid:String = "".to_string();
    match response{
        Ok(resp)=>{
            stat = resp.status();
            let cookies =  resp.cookies();
            for i in cookies{
                if "buvid3".eq(i.name()){
                    buvid = i.value().to_string();
                }
            }
        },
        Err(e)=>{
            panic!("init buvid failed");
        }
    
    }
    (stat, buvid)
}

fn init_room(headers:HeaderMap,temp_room_id:&str)-> (StatusCode,String){
    let client = reqwest::blocking::Client::builder().https_only(true).build().unwrap();
    let url = web::ROOM_INIT_URL.to_string()+"?room_id="+temp_room_id;
    let response = client.get(url).headers(headers).send();
    let mut stat:StatusCode;
    let mut body:String;
    match response{
        Ok(resp)=>{
            stat = resp.status();
            body = resp.text().unwrap();
        },
        Err(e)=>{
            panic!("init buvid failed");
        }
    
    }
    (stat, body)
}

fn init_host_server(headers:HeaderMap,room_id:u64)-> (StatusCode,String){
    let client = reqwest::blocking::Client::builder().https_only(true).build().unwrap();
    let url = web::DANMAKU_SERVER_CONF_URL.to_string()+"?id="+room_id.to_string().as_str()+"&type=0";
    let response = client.get(url).headers(headers).send();
    let mut stat:StatusCode;
    let mut body:String;
    match response{
        Ok(resp)=>{
            stat = resp.status();
            body = resp.text().unwrap();
        },
        Err(e)=>{
            panic!("init buvid failed");
        }
    
    }
    (stat, body)
}

pub fn gen_damu_list(list:&Value)->Vec<DanmuServer>{
      let server_list = list.as_array().unwrap();
      let mut res :Vec<DanmuServer>=Vec::new();
      if server_list.len()==0{
         let d = DanmuServer::deafult();
         res.push(d);
      }
      for s in server_list{
        res.push(DanmuServer{
            host:s["host"].as_str().unwrap().to_string(),
            port:s["port"].as_u64().unwrap() as i32,
            wss_port:s["wss_port"].as_u64().unwrap() as i32,
            ws_port:s["ws_port"].as_u64().unwrap() as i32,
         });
      }
      res
}

fn find_server(vd:Vec<DanmuServer>)->(String,String,String){
    let  (host, wss_port) = (vd.get(0).unwrap().host.clone(), vd.get(0).unwrap().wss_port);
    (host.clone(),format!("{}:{}",host.clone(),wss_port),format!("wss://{}:{}/sub",host,wss_port))

}


pub fn init_server(sessdata:&str,room_id:&str)->(Value,AuthMessage){
    let mut cookies = HashMap::new();
    cookies.insert("SESSDATA".to_string(), sessdata.to_string());
    let mut auth_map = HashMap::new();
    // 构建一个HeaderMap，将Cookie添加到请求头中
    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE,
        HeaderValue::from_str(
            &cookies
                .iter()
                .map(|(name, value)| format!("{}={}", name, value))
                .collect::<Vec<_>>()
                .join("; "),
        ).unwrap(),
            
    );
    headers.insert(USER_AGENT, HeaderValue::from_str(web::USER_AGENT).unwrap());
        // let res = get(web::UID_INIT_URL);
    let (_, bod1y)= init_uid(headers.clone());
    let body1_v:Value = serde_json::from_str(bod1y.as_str()).unwrap();

    println!("uid{:?}",body1_v["data"]["mid"]);
    auth_map.insert("uid".to_string(), body1_v["data"]["mid"].as_i64().unwrap().to_string());
    let(res_stat2,buvid) = init_buvid(headers.clone());
    println!("2222222222222222{:?}",buvid.clone());
    auth_map.insert("buvid".to_string(), buvid.to_string());
    let(_,body3) = init_room(headers.clone(),room_id);
    // println!("body3{:?}",body3);
    let body3_v:Value = serde_json::from_str(body3.as_str()).unwrap();
    let room_info = &body3_v["data"]["room_info"];
    let room_id = room_info["room_id"].as_u64().unwrap();
    println!("roomid{:?},roomowner{:?}",room_info["room_id"],room_info["uid"]);
    auth_map.insert("room_id".to_string(), room_id.to_string());
    let(_,body4) = init_host_server(headers.clone(),room_id);
    let body4_res:Value = serde_json::from_str(body4.as_str()).unwrap();
    let server_info  = &body4_res["data"];
    let token =  &body4_res["data"]["token"].as_str().unwrap();
    println!("token == {:?}",token);
    auth_map.insert("token".to_string(), token.to_string());
    let auth_msg = AuthMessage::from(&auth_map);
    (server_info.clone(),auth_msg)
}


pub fn connect(v:Value)->(WebSocket<TlsStream<TcpStream>>,Response<Option<Vec<u8>>>){
    let danmu_server = gen_damu_list(&v);
        
    println!("danmu_server == {:?}",danmu_server);

    //初始化完成，开始监听danmu
    // let mut retry_count = 0;
    let (host,url,ws_url,)= find_server(danmu_server);
    println!("ws_url地址:{}",ws_url);
    println!("host:{}",host);
    println!("url地址:{}",url);

    let connector: TlsConnector = TlsConnector::new().unwrap();
    let stream: TcpStream = TcpStream::connect(url).unwrap() ;
    let mut stream: native_tls::TlsStream<TcpStream> = connector.connect(host.as_str(), stream).unwrap();       
    let (mut socket, resp) =client(Url::parse(ws_url.as_str()).unwrap(),stream).expect("Can't connect");
    (socket,resp)
}

pub struct BiliLiveClient{
    ws:WebSocket<TlsStream<TcpStream>>,
    auth_msg:String,
    ss:Sender<String>,
 
}

impl BiliLiveClient{
    pub fn new(sessdata:&str,room_id:&str,r:Sender<String>)->Self{
        let (v,auth) = init_server(sessdata,room_id);
        let (ws,res) = connect(v["host_list"].clone());
        BiliLiveClient{
            ws:ws,
            auth_msg:serde_json::to_string(&auth).unwrap(),
            ss:r,
 
        }
    }

    pub fn send_auth(&mut self){
        let _ = self.ws.send(Message::Binary(make_packet(self.auth_msg.as_str(), Operation::AUTH)));
    }

    pub fn send_heart_beat(&mut self){
        let _ = self.ws.send(Message::Binary(make_packet("{}", Operation::HEARTBEAT)));
        
    }


    pub fn parse_ws_message(&mut self ,resv: Vec<u8>){
        let mut offset = 0;
        let header = &resv[0..16];
        let mut head_1 = get_msg_header(header);

        if head_1.operation == 5 || head_1.operation == 8 {
            loop {
                // let pl = head_1.pack_len.clone();
                let body: &[u8] = &resv[offset + 16..offset + (head_1.pack_len as usize)];
                // let s = String::from_utf8(body.to_vec()).unwrap();
                self.parse_business_message(head_1, body);
                offset += head_1.pack_len as usize;
                if offset >= resv.len() {
                    break;
                }
                
                let temp_head = &resv[offset..(offset+16)];
                head_1 = get_msg_header(temp_head); 
            }
        } else if head_1.operation == 3 {
            let mut body: [u8;4]= [0,0,0,0] ;
            body[0]=resv[16];
            body[1]=resv[17];
            body[2]=resv[18];
            body[3]=resv[19];
            let popularity = i32::from_be_bytes(body);
            println!("popularity:{}",popularity);
        } else {
            println!(
                "未知消息, unknown message operation={:?}, header={:?}}}",
                head_1.operation, head_1
            )
        }

    }

    pub fn parse_business_message(&mut self,h:MSG_HEAD,b: &[u8]){
        if h.operation == 5{
            if h.ver == 3 {
                let res: Vec<u8> = decompress(b).unwrap();
                self.parse_ws_message(res);
            } else if h.ver == 0 {
                let s = String::from_utf8(b.to_vec()).unwrap();
                let res_json:Value = serde_json::from_str(s.as_str()).unwrap();
                let res = handle(res_json);
                if "未知消息".to_string()==res{
                    return;
                }
                let _ = self.ss.try_send(res);
            } else {
                println!("未知压缩格式");
            }
        
        }else if  h.operation == 8{
            self.send_heart_beat();
        }else {
            println!("未知消息格式{}",h.operation);
        }
    
    
    }

    pub fn recive(&mut self){
        if self.ws.can_read(){
            let msg =  self.ws.read();
            match msg {
                Ok(m)=>{
                    let res = m.into_data();
                    if res.len()>=16{
                        self.parse_ws_message(res);
                    }
                },
                Err(e)=>{
                    panic!("read msg error");
                }
            }
            
            
        }
    }


    

}




#[cfg(test)]
mod test {
    use crate::bili_live_dm::*;
   
    #[test]
    fn send_https(){
        let ss="3b2be85e%2C1716943731%2C5b579%2Ac2CjA1nhbZeS1AyhLoHnccXYPEfYZEShmZkQEvS0zl3h2ddHDngOmoDvhxVkibLOC9_1ESVmdreUJPR2FmQ0FoVVJETDhRVjdGUEZXU210TU5ya1FQLUNWNFE0eWlnbmVDUU5UNmJVeEpJZHZGWnZYVVIwZHByWHl0YjNDMFpkelhKOGJzQVhiOWJRIIEC";
        let room_id = "5050";
        init_server(ss,room_id);
    }




}
 
