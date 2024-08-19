use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::mpsc;
use std::time::SystemTime;

use blake2::{Blake2b512, Digest};
use chrono::{DateTime, Local};
use reqwest;
use serde::{Deserialize, Serialize};
use tauri::{self, AppHandle, Manager, WebviewWindow, Wry};
use tauri::menu::MenuItem;
use tauri_plugin_autostart::ManagerExt;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

#[tauri::command]
pub fn restart() {
    tauri::process::restart(&tauri::Env::default())
}

#[tauri::command]
pub fn show_page(app: AppHandle) {
    println!("show_page");
    hide_or_show(app.get_webview_window("main").unwrap());
}

#[tauri::command]
pub fn open_local_dir(target: &str) {
    println!("open_local_dir: {}", target);
    let path = Path::new(target);
    if path.exists() {
        if path.is_dir() {
            if cfg!(target_os = "windows") {
                let _ = std::process::Command::new("explorer").arg(path).spawn();
            } else if cfg!(target_os = "macos") {
                let _ = std::process::Command::new("open").arg(path).spawn();
            } else if cfg!(target_os = "linux") {
                let _ = std::process::Command::new("xdg-open").arg(path).spawn();
            }
        } else {
            let _ = std::process::Command::new("explorer")
                .arg(path.parent().unwrap())
                .spawn();
        }
    }
}

#[tauri::command]
pub fn open_with_wps(target: &str, name: &str) {
    let file_path = Path::new(target).join(Path::new(name));
    let _ = std::process::Command::new("wps")
        .arg(file_path)
        .spawn();
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

pub fn hide_or_show<'a>(window: WebviewWindow) -> &'a str {
    if window.is_visible().unwrap() {
        window.hide().unwrap();
        "显示(S)"
    } else {
        window
            .set_always_on_top(true)
            .expect("Failed to set window as topmost");
        window.show().unwrap();
        "隐藏(H)"
    }
}

#[derive(Serialize)]
pub struct FileTile {
    name: String,
    path: String,
    blake2b512: String,
    last_modified: String,
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
    let os_path = Path::new(&path);
    if os_path.is_dir() {
        return FileTile {
            name: String::from("--"),
            blake2b512: String::from("--"),
            last_modified: String::from("--"),
            path,
        };
    }
    let file_name = Path::new(&path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let file_parent = Path::new(&path)
        .parent()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    return match file {
        Ok(file) => {
            let last_modified = match file.metadata() {
                Ok(metadata) => match metadata.modified() {
                    Ok(time) => system_time_to_date_time(time),
                    Err(_) => blank,
                },
                Err(_) => blank,
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
                name: file_name,
                blake2b512: hex::encode(hasher.finalize()),
                last_modified,
                path: file_parent,
            }
        }
        Err(_) => FileTile {
            name: file_name,
            blake2b512: blank,
            last_modified: String::from("--"),
            path: file_parent,
        },
    };
}

pub fn handle_file(path: String, tx: mpsc::Sender<FileTile>) {
    let file_tile = calculate_blake2b512(path.to_string());
    tx.send(file_tile).unwrap();
}

pub fn handle_directory(path: String, tx: mpsc::Sender<FileTile>) {
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path().to_string_lossy().into_owned();
                    handle_file(file_path, tx.clone());
                }
            }
        }
        Err(e) => eprintln!("Failed to read directory: {}", e),
    }
}

pub fn handle_hide_or_show(window: WebviewWindow, hide: MenuItem<Wry>) {
    let title = hide_or_show(window);
    hide.set_text(title).expect("Failed to set tray text");
}

pub fn handle_auto_start(app: AppHandle<Wry>, auto_start: MenuItem<Wry>) {
    let autostart_manager = app.autolaunch();
    let is_enabled = autostart_manager.is_enabled().unwrap();
    if is_enabled {
        let _ = autostart_manager.disable();
    } else {
        let _ = autostart_manager.enable();
    }
    auto_start
        .set_text(if is_enabled {
            "开机自启动(❌)"
        } else {
            "开机自启动(✔️)"
        })
        .expect("Failed to set tray text");
}