// src/client/auth.rs
//! Authentication helpers for Bilibili live danmaku WebSocket client

use reqwest::header::{HeaderMap, HeaderValue, COOKIE, USER_AGENT};
use reqwest::StatusCode;
use serde_json::Value;
use std::collections::HashMap;

pub fn init_uid(headers: HeaderMap) -> (StatusCode, String) {
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .build()
        .unwrap();
    let response = client
        .get(crate::client::auth::UID_INIT_URL)
        .headers(headers)
        .send();
    let mut stat: StatusCode;
    let mut body: String;
    match response {
        Ok(resp) => {
            stat = resp.status();
            body = resp.text().unwrap();
        }
        Err(_) => {
            panic!("init uid failed");
        }
    }
    (stat, body)
}

pub fn init_buvid(headers: HeaderMap) -> (StatusCode, String) {
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .build()
        .unwrap();
    let response = client
        .get(crate::client::auth::BUVID_INIT_URL)
        .headers(headers)
        .send();
    let mut stat: StatusCode;
    let mut buvid: String = "".to_string();
    match response {
        Ok(resp) => {
            stat = resp.status();
            let cookies = resp.cookies();
            for i in cookies {
                if "buvid3".eq(i.name()) {
                    buvid = i.value().to_string();
                }
            }
        }
        Err(_) => {
            panic!("init buvid failed");
        }
    }
    (stat, buvid)
}

pub fn init_room(headers: HeaderMap, temp_room_id: &str) -> (StatusCode, String) {
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .build()
        .unwrap();
    let url = format!(
        "{}?room_id={}",
        crate::client::auth::ROOM_INIT_URL,
        temp_room_id
    );
    let response = client.get(url).headers(headers).send();
    let mut stat: StatusCode;
    let mut body: String;
    match response {
        Ok(resp) => {
            stat = resp.status();
            body = resp.text().unwrap();
        }
        Err(_) => {
            panic!("init buvid failed");
        }
    }
    (stat, body)
}

pub fn init_host_server(headers: HeaderMap, room_id: u64) -> (StatusCode, String) {
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .build()
        .unwrap();
    let url = format!(
        "{}?id={}&type=0",
        crate::client::auth::DANMAKU_SERVER_CONF_URL,
        room_id
    );
    let response = client.get(url).headers(headers).send();
    let mut stat: StatusCode;
    let mut body: String;
    match response {
        Ok(resp) => {
            stat = resp.status();
            body = resp.text().unwrap();
        }
        Err(_) => {
            panic!("init buvid failed");
        }
    }
    (stat, body)
}

pub const UID_INIT_URL: &str = "https://api.bilibili.com/x/web-interface/nav";
pub const BUVID_INIT_URL: &str = "https://data.bilibili.com/v/";
pub const ROOM_INIT_URL: &str =
    "https://api.live.bilibili.com/xlive/web-room/v1/index/getInfoByRoom";
pub const DANMAKU_SERVER_CONF_URL: &str =
    "https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo";
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.0.0 Safari/537.36";

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderMap;

    #[test]
    fn test_uid_url_constant() {
        assert!(UID_INIT_URL.contains("bilibili.com"));
    }

    #[test]
    fn test_init_uid_returns_status_and_body() {
        // This is a dummy test; in real use, mock HTTP or use a test server
        let headers = HeaderMap::new();
        let result = std::panic::catch_unwind(|| super::init_uid(headers));
        assert!(result.is_ok());
    }
}
