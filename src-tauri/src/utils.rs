use reqwest;
use serde::{Deserialize, Serialize};
use tauri::{self, WebviewWindow};
use webbrowser;
use blake2::{Blake2b512, Digest};
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::{SystemTime};
use chrono::{DateTime, Local};

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

#[tauri::command]
pub fn open_link(url: &str) {
    if let Err(e) = webbrowser::open(&url) {
        eprintln!("Failed to open link: {}", e);
    }
}

#[tauri::command]
pub fn restart() {
    tauri::process::restart(&tauri::Env::default())
}

pub fn check_update(flag: String) -> String {
    let url = "https://api.github.com/repos/initialencounter/rainwarm/releases/latest";
    let client = reqwest::blocking::Client::new();

    let resp = match client
        .get(url)
        .header(reqwest::header::USER_AGENT, "rust-app")
        .send()
    {
        Ok(response) => response,
        Err(_) => return flag,
    };

    if !resp.status().is_success() {
        return flag;
    }

    let release = match resp.json::<Release>() {
        Ok(release) => release,
        Err(_) => return flag,
    };
    release.tag_name
}

pub fn hide_or_show(window: WebviewWindow) {
    if window.is_visible().unwrap() {
        window.hide().unwrap();
    } else {
        window.set_always_on_top(true).expect("Failed to set window as topmost");
        window.show().unwrap();
    }
}

#[derive(Serialize)]
pub struct FileTile {
    blake2b512: String,
    last_modified: String,
    path: String,
}

fn system_time_to_date_time(time: SystemTime) -> String {
    let datetime: DateTime<Local> = DateTime::from(time);
    // 格式化日期和时间
    return datetime.format("%Y-%m-%d %H:%M:%S").to_string();
}

// 计算单文件Blake2b512
pub fn calculate_blake2b512(path: String) -> FileTile {
    let file = File::open(&path);
    let blank = String::from("--");
    return match file {
        Ok(file) => {
            let last_modified = match file.metadata() {
                Ok(metadata) => match metadata.modified() {
                    Ok(time) => system_time_to_date_time(time),
                    Err(_) => blank
                }
                Err(_) => blank
            };
            let mut reader = BufReader::new(file);
            let mut hasher = Blake2b512::new();
            let mut buffer = [0u8; 1024];
            loop {
                let n = reader.read(&mut buffer).unwrap();
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            FileTile {
                blake2b512: hex::encode(hasher.finalize()),
                last_modified,
                path,
            }
        }
        Err(_) => {
            FileTile {
                blake2b512: blank,
                last_modified: String::from("--"),
                path,
            }
        }
    };
}