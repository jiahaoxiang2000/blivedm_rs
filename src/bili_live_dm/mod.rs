
use serde_json::Value;
use tungstenite::{ Message,client};
use std::time::{Duration, Instant};

use reqwest::header::HeaderMap;
 
use url::Url;
use native_tls::TlsConnector;
use std::net::TcpStream;

use reqwest::StatusCode;

use serde_json;




mod web;
use web::*;



 
fn init_uid(headers:HeaderMap) -> (StatusCode,String) {
    // 设置请求的URL
    let client = reqwest::blocking::Client::builder().https_only(true).build().unwrap();
    let response = client.get(web::UID_INIT_URL).headers(headers).send();
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

fn gen_damu_list(list:&Value)->Vec<DanmuServer>{
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


 
#[cfg(test)]
mod test {
    use crate::bili_live_dm::*;
    use std::collections::HashMap;
    use futures::Sink;
    use reqwest::header::{HeaderMap, HeaderValue, COOKIE,USER_AGENT};
    use serde_json::Value;
    use std::thread;
    use std::time::Duration;
    // use websockets::{WebSocket, WebSocketWriteHalf};
    use tungstenite::{ Message,protocol::*};
    use std::net::{Ipv4Addr, SocketAddrV4,SocketAddr};
    use std::sync::{Arc, Mutex}; 
    use futures::sync::mpsc;



    #[test]
    fn it_works() {
        // 创建一个HashMap来存储Cookie
        let sessdata = "3b2be85e%2C1716943731%2C5b579%2Ac2CjA1nhbZeS1AyhLoHnccXYPEfYZEShmZkQEvS0zl3h2ddHDngOmoDvhxVkibLOC9_1ESVmdreUJPR2FmQ0FoVVJETDhRVjdGUEZXU210TU5ya1FQLUNWNFE0eWlnbmVDUU5UNmJVeEpJZHZGWnZYVVIwZHByWHl0YjNDMFpkelhKOGJzQVhiOWJRIIEC";
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
        let(_,body3) = init_room(headers.clone(),"5050");
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
        auth_map.insert("token".to_string(), token.to_string());
        let danmu_server = gen_damu_list(&server_info["host_list"]);
        println!("token == {:?}",token);
        println!("danmu_server == {:?}",danmu_server);

        //初始化完成，开始监听danmu
        let mut retry_count = 0;
        let (host,url,ws_url,)= find_server(danmu_server);
        println!("ws_url地址:{}",ws_url);
        println!("host:{}",host);
        println!("url地址:{}",url);

        let connector = TlsConnector::new().unwrap();
        let stream = TcpStream::connect(url).unwrap() ;
        let mut stream = connector.connect(host.as_str(), stream).unwrap();       
        let (mut socket, resp) =client(Url::parse(ws_url.as_str()).unwrap(),stream).expect("Can't connect");
        println!("resp:{:?}",resp);
         
         
        
        //发送授权报文
        let auth_msg = AuthMessage::from(&auth_map);
        
        let auth_msg_str = serde_json::to_string(&auth_msg).unwrap();
        println!("授权消息:{}",auth_msg_str);
        let la: Message = Message::Binary(web::make_packet(auth_msg_str.as_str(), web::Operation::AUTH));
        // print!("auth:{:?}",la);

        

        socket.send(la).unwrap();
         
        // socket.send(Message::Binary(make_packet("{}", Operation::HEARTBEAT))).unwrap();
         
        let shared_stream = Arc::new(Mutex::new(socket));
        let mut heart_beats = Arc::clone(&shared_stream);

        thread::spawn(move || {
            loop {
                match heart_beats.lock() {
                    Ok(mut locked_stream) => {
                        if locked_stream.can_write(){
                            locked_stream.send(Message::Binary(make_packet("{}", Operation::HEARTBEAT)))
                            .unwrap();
                        }
                            
                    }
                    Err(e) => {
                        eprintln!("Error acquiring lock on stream: {}", e);
                        break;
                    }
                }

                thread::sleep(Duration::new(30, 0));
            }
        });


        let (tx,mut rx) = mpsc::channel(64);
        
        let mut rec_msg = Arc::clone(&shared_stream);
        // let tx = tx.clone();
        loop{
            match rec_msg.lock() {
                Ok(mut locked_stream) => {
                    if locked_stream.can_read(){
                         
                        let msg = locked_stream.read().expect("Error reading message");
                        let res_msg_arr = web::analyze_msg(msg);
                        for i in res_msg_arr{
                            let tx = tx.clone();
                            let _= tx.send(i);
                        }
                        
                    }
                }
                Err(e) =>{
                    eprintln!("Error acquiring lock on stream: {}", e);
                    break;
                }
            }
            
        
        }
    }
}
 
