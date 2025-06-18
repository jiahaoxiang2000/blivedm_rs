// examples/browser_cookie_test.rs
//! Example showing browser cookie detection functionality

use client::browser_cookies::{find_bilibili_sessdata, get_all_bilibili_cookies};

fn main() {
    // Initialize logging
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    println!("=== Bilibili Browser Cookie Detection Test ===\n");

    // Try to find SESSDATA specifically
    match find_bilibili_sessdata() {
        Some(sessdata) => {
            println!("✅ Found SESSDATA cookie!");
            println!("   Value preview: {}...", &sessdata[..20.min(sessdata.len())]);
            println!("   Length: {} characters", sessdata.len());
        }
        None => {
            println!("❌ No valid SESSDATA cookie found");
            println!("   Make sure you're logged into bilibili.com in your browser");
        }
    }

    println!("\n=== All Bilibili Cookies Found ===");
    let all_cookies = get_all_bilibili_cookies();
    
    if all_cookies.is_empty() {
        println!("No bilibili cookies found in any browser");
    } else {
        for (key, value) in all_cookies {
            let preview = if value.len() > 30 {
                format!("{}...", &value[..30])
            } else {
                value.clone()
            };
            println!("🍪 {}: {}", key, preview);
        }
    }

    println!("\n=== Browser Support Status ===");
    use client::browser_cookies::{Browser, read_cookies_from_browser};
    
    for browser in Browser::get_all_supported() {
        match read_cookies_from_browser(&browser, Some("bilibili.com")) {
            Ok(cookies) => {
                println!("✅ {:?}: {} bilibili cookies found", browser, cookies.len());
            }
            Err(e) => {
                println!("❌ {:?}: {}", browser, e);
            }
        }
    }
}
