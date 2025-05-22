// src/client/auth.rs
//! Authentication helpers for Bilibili live danmaku WebSocket client

use reqwest::header::HeaderMap;
use reqwest::StatusCode;

pub fn init_uid(headers: HeaderMap) -> (StatusCode, String) {
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .build()
        .unwrap();
    let response = client.get(UID_INIT_URL).headers(headers).send();
    log::debug!("init uid response: {:?}", response);
    let stat: StatusCode;
    let body: String;
    match response {
        Ok(resp) => {
            stat = resp.status();
            body = resp.text().unwrap();
            log::info!("init uid response: {:?}", body);
        }
        Err(_) => {
            panic!("init uid failed");
        }
    }
    (stat, body)
}

/// Initializes the buvid by sending a request and extracting the 'buvid3' cookie.
///
/// Note: This function is not used for document creation.
///
/// # Panics
///
/// Panics if the request fails.
pub fn init_buvid(headers: HeaderMap) -> (StatusCode, String) {
    // Not used for document creation.
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .build()
        .unwrap();
    let response = client.get(BUVID_INIT_URL).headers(headers).send();
    let stat: StatusCode;
    let mut buvid: String = "".to_string();
    match response {
        Ok(resp) => {
            stat = resp.status();
            let cookies = resp.cookies();
            for i in cookies {
                log::debug!("init buvid response cookie : {:?}", i);
                if "buvid3".eq(i.name()) {
                    buvid = i.value().to_string();
                    log::info!("init buvid response: {:?}", buvid);
                }
            }
        }
        Err(_) => {
            panic!("init buvid failed");
        }
    }
    (stat, buvid)
}

/// Initializes the room by sending a request with the given room ID.
///
/// Note: This function should NOT be used for document creation.
///
/// # Panics
///
/// Panics if the request fails.
pub fn init_room(headers: HeaderMap, temp_room_id: &str) -> (StatusCode, String) {
    let client = reqwest::blocking::Client::builder()
        .https_only(true)
        .build()
        .unwrap();
    let url = format!("{}?room_id={}", ROOM_INIT_URL, temp_room_id);
    let response = client.get(url).headers(headers).send();
    let stat: StatusCode;
    let body: String;
    match response {
        Ok(resp) => {
            stat = resp.status();
            body = resp.text().unwrap();
            log::info!("init room response: {:?}", body);
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
    let url = format!("{}?id={}&type=0", DANMAKU_SERVER_CONF_URL, room_id);
    let response = client.get(url).headers(headers).send();
    log::debug!("init host server response: {:?}", response);
    let stat: StatusCode;
    let body: String;
    match response {
        Ok(resp) => {
            stat = resp.status();
            body = resp.text().unwrap();
            log::info!("init host server response body: {:?}", body);
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

    #[test]
    fn test_uid_url_constant() {
        assert!(UID_INIT_URL.contains("bilibili.com"));
    }
}
