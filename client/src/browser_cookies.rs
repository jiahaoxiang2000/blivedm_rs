// src/client/browser_cookies.rs
//! Browser cookie reading functionality for automatic SESSDATA detection

use chrono::{DateTime, TimeZone, Utc};
use directories::UserDirs;
use log::{debug, info, warn};
use sqlite::Connection;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: Option<DateTime<Utc>>,
    pub secure: bool,
    pub http_only: bool,
}

#[derive(Debug)]
pub enum Browser {
    Chrome,
    Firefox,
    Edge,
    Chromium,
    Opera,
}

impl Browser {
    pub fn get_cookie_db_path(&self) -> Option<PathBuf> {
        let user_dirs = UserDirs::new()?;
        let home_dir = user_dirs.home_dir();

        match self {
            Browser::Chrome => {
                #[cfg(target_os = "linux")]
                {
                    Some(home_dir.join(".config/google-chrome/Default/Cookies"))
                }
                #[cfg(target_os = "macos")]
                {
                    Some(home_dir.join("Library/Application Support/Google/Chrome/Default/Cookies"))
                }
                #[cfg(target_os = "windows")]
                {
                    Some(
                        home_dir
                            .join("AppData/Local/Google/Chrome/User Data/Default/Network/Cookies"),
                    )
                }
            }
            Browser::Firefox => {
                #[cfg(target_os = "linux")]
                {
                    let firefox_dir = home_dir.join(".mozilla/firefox");
                    Self::find_firefox_profile_cookies(&firefox_dir)
                }
                #[cfg(target_os = "macos")]
                {
                    let firefox_dir = home_dir.join("Library/Application Support/Firefox/Profiles");
                    Self::find_firefox_profile_cookies(&firefox_dir)
                }
                #[cfg(target_os = "windows")]
                {
                    let firefox_dir = home_dir.join("AppData/Roaming/Mozilla/Firefox/Profiles");
                    Self::find_firefox_profile_cookies(&firefox_dir)
                }
            }
            Browser::Edge => {
                #[cfg(target_os = "linux")]
                {
                    Some(home_dir.join(".config/microsoft-edge/Default/Cookies"))
                }
                #[cfg(target_os = "macos")]
                {
                    Some(
                        home_dir.join("Library/Application Support/Microsoft Edge/Default/Cookies"),
                    )
                }
                #[cfg(target_os = "windows")]
                {
                    Some(
                        home_dir
                            .join("AppData/Local/Microsoft/Edge/User Data/Default/Network/Cookies"),
                    )
                }
            }
            Browser::Chromium => {
                #[cfg(target_os = "linux")]
                {
                    Some(home_dir.join(".config/chromium/Default/Cookies"))
                }
                #[cfg(target_os = "macos")]
                {
                    Some(home_dir.join("Library/Application Support/Chromium/Default/Cookies"))
                }
                #[cfg(target_os = "windows")]
                {
                    Some(home_dir.join("AppData/Local/Chromium/User Data/Default/Network/Cookies"))
                }
            }
            Browser::Opera => {
                #[cfg(target_os = "linux")]
                {
                    Some(home_dir.join(".config/opera/Default/Cookies"))
                }
                #[cfg(target_os = "macos")]
                {
                    Some(home_dir.join(
                        "Library/Application Support/com.operasoftware.Opera/Default/Cookies",
                    ))
                }
                #[cfg(target_os = "windows")]
                {
                    Some(
                        home_dir
                            .join("AppData/Roaming/Opera Software/Opera Stable/Network/Cookies"),
                    )
                }
            }
        }
    }

    fn find_firefox_profile_cookies(firefox_dir: &Path) -> Option<PathBuf> {
        if !firefox_dir.exists() {
            return None;
        }

        // Look for the default profile directory
        let entries = fs::read_dir(firefox_dir).ok()?;
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let dir_name = path.file_name()?.to_str()?;
                    if dir_name.contains(".default") || dir_name.contains(".default-release") {
                        let cookies_path = path.join("cookies.sqlite");
                        if cookies_path.exists() {
                            return Some(cookies_path);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_all_supported() -> Vec<Browser> {
        vec![
            Browser::Chrome,
            Browser::Firefox,
            Browser::Edge,
            Browser::Chromium,
            Browser::Opera,
        ]
    }
}

/// Read cookies from a browser's cookie database
pub fn read_cookies_from_browser(
    browser: &Browser,
    domain_filter: Option<&str>,
) -> Result<Vec<Cookie>, String> {
    let db_path = browser
        .get_cookie_db_path()
        .ok_or_else(|| "Could not determine cookie database path".to_string())?;

    if !db_path.exists() {
        return Err(format!("Cookie database not found at: {:?}", db_path));
    }

    debug!("Reading cookies from: {:?}", db_path);

    // Create a temporary copy of the database since browsers might have it locked
    let temp_path = std::env::temp_dir().join(format!("temp_cookies_{}.db", std::process::id()));
    if let Err(e) = fs::copy(&db_path, &temp_path) {
        return Err(format!("Failed to copy cookie database: {}", e));
    }

    let result = match browser {
        Browser::Firefox => read_firefox_cookies(&temp_path, domain_filter),
        _ => read_chromium_cookies(&temp_path, domain_filter),
    };

    // Clean up temporary file
    let _ = fs::remove_file(&temp_path);

    result
}

fn read_chromium_cookies(
    db_path: &Path,
    domain_filter: Option<&str>,
) -> Result<Vec<Cookie>, String> {
    let connection =
        Connection::open(db_path).map_err(|e| format!("Failed to open cookie database: {}", e))?;

    let mut query =
        "SELECT name, value, host_key, path, expires_utc, is_secure, is_httponly FROM cookies"
            .to_string();

    if let Some(domain) = domain_filter {
        query.push_str(&format!(" WHERE host_key LIKE '%{}'", domain));
    }

    let mut cookies = Vec::new();

    connection
        .iterate(query, |pairs| {
            let mut cookie_data = HashMap::new();
            for &(column, value) in pairs.iter() {
                cookie_data.insert(column, value.unwrap_or(""));
            }

            let expires = if let Some(expires_str) = cookie_data.get("expires_utc") {
                if let Ok(expires_microseconds) = expires_str.parse::<i64>() {
                    // Chrome stores time as microseconds since Windows epoch (1601-01-01)
                    // Convert to Unix timestamp (seconds since 1970-01-01)
                    let windows_epoch_offset = 11644473600_i64; // seconds between 1601 and 1970
                    let unix_timestamp = (expires_microseconds / 1_000_000) - windows_epoch_offset;
                    Utc.timestamp_opt(unix_timestamp, 0).single()
                } else {
                    None
                }
            } else {
                None
            };

            let cookie = Cookie {
                name: cookie_data.get("name").unwrap_or(&"").to_string(),
                value: cookie_data.get("value").unwrap_or(&"").to_string(),
                domain: cookie_data.get("host_key").unwrap_or(&"").to_string(),
                path: cookie_data.get("path").unwrap_or(&"").to_string(),
                expires,
                secure: cookie_data.get("is_secure").unwrap_or(&"0") == &"1",
                http_only: cookie_data.get("is_httponly").unwrap_or(&"0") == &"1",
            };

            cookies.push(cookie);
            true
        })
        .map_err(|e| format!("Failed to query cookies: {}", e))?;

    Ok(cookies)
}

fn read_firefox_cookies(
    db_path: &Path,
    domain_filter: Option<&str>,
) -> Result<Vec<Cookie>, String> {
    let connection =
        Connection::open(db_path).map_err(|e| format!("Failed to open cookie database: {}", e))?;

    let mut query =
        "SELECT name, value, host, path, expiry, isSecure, isHttpOnly FROM moz_cookies".to_string();

    if let Some(domain) = domain_filter {
        query.push_str(&format!(" WHERE host LIKE '%{}'", domain));
    }

    let mut cookies = Vec::new();

    connection
        .iterate(query, |pairs| {
            let mut cookie_data = HashMap::new();
            for &(column, value) in pairs.iter() {
                cookie_data.insert(column, value.unwrap_or(""));
            }

            let expires = if let Some(expires_str) = cookie_data.get("expiry") {
                if let Ok(expires_timestamp) = expires_str.parse::<i64>() {
                    Utc.timestamp_opt(expires_timestamp, 0).single()
                } else {
                    None
                }
            } else {
                None
            };

            let cookie = Cookie {
                name: cookie_data.get("name").unwrap_or(&"").to_string(),
                value: cookie_data.get("value").unwrap_or(&"").to_string(),
                domain: cookie_data.get("host").unwrap_or(&"").to_string(),
                path: cookie_data.get("path").unwrap_or(&"").to_string(),
                expires,
                secure: cookie_data.get("isSecure").unwrap_or(&"0") == &"1",
                http_only: cookie_data.get("isHttpOnly").unwrap_or(&"0") == &"1",
            };

            cookies.push(cookie);
            true
        })
        .map_err(|e| format!("Failed to query cookies: {}", e))?;

    Ok(cookies)
}

/// Find SESSDATA cookie from all supported browsers
pub fn find_bilibili_cookies_as_string() -> Option<String> {
    let browsers = Browser::get_all_supported();
    let mut all_cookies = vec![];

    for browser in browsers {
        info!("Checking browser: {:?}", browser);

        if let Ok(cookies) = read_cookies_from_browser(&browser, Some("bilibili.com")) {
            all_cookies.extend(cookies);
        }
    }

    let mut valid_cookies = all_cookies
        .into_iter()
        .filter(|cookie| {
            if let Some(expires) = cookie.expires {
                if Utc::now() > expires {
                    warn!(
                        "Found expired {} cookie, expires: {:?}",
                        cookie.name, expires
                    );
                    return false;
                }
            }
            true
        })
        .collect::<Vec<_>>();

    // Deduplicate cookies, keeping the one with the latest expiry
    valid_cookies.sort_by(|a, b| {
        if a.name != b.name {
            a.name.cmp(&b.name)
        } else {
            b.expires.cmp(&a.expires) // None is smaller
        }
    });
    valid_cookies.dedup_by(|a, b| a.name == b.name);

    if valid_cookies.is_empty() {
        warn!("No valid bilibili cookies found in any browser");
        return None;
    }

    info!("Found {} valid bilibili cookies", valid_cookies.len());

    let cookie_string = valid_cookies
        .iter()
        .map(|c| format!("{}={}", c.name, c.value))
        .collect::<Vec<String>>()
        .join("; ");

    if cookie_string.contains("SESSDATA") {
        Some(cookie_string)
    } else {
        warn!("No SESSDATA cookie found among the valid cookies");
        None
    }
}

/// Get all bilibili cookies from browsers for debugging
pub fn get_all_bilibili_cookies() -> HashMap<String, String> {
    let mut all_cookies = HashMap::new();
    let browsers = Browser::get_all_supported();

    for browser in browsers {
        if let Ok(cookies) = read_cookies_from_browser(&browser, Some("bilibili.com")) {
            for cookie in cookies {
                // Only include non-expired cookies
                if let Some(expires) = cookie.expires {
                    if Utc::now() > expires {
                        continue;
                    }
                }

                // Use the most recent cookie if duplicates exist
                let key = format!("{}_{}", cookie.name, cookie.domain);
                all_cookies.insert(key, cookie.value);
            }
        }
    }

    all_cookies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_path_detection() {
        for browser in Browser::get_all_supported() {
            let path = browser.get_cookie_db_path();
            println!("{:?} cookie path: {:?}", browser, path);
        }
    }

    #[test]
    fn test_find_sessdata() {
        // This test will only work if you have bilibili cookies in your browser
        if let Some(sessdata) = find_bilibili_cookies_as_string() {
            println!("Found SESSDATA: {}", &sessdata[..20.min(sessdata.len())]);
            assert!(!sessdata.is_empty());
        } else {
            println!("No SESSDATA found - this is normal if you're not logged into bilibili");
        }
    }
}
