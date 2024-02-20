// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use reqwest;
use serde::Deserialize;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use webbrowser;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}
fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出(X)");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏(H)");
    let about = CustomMenuItem::new("about".to_string(), "关于(A)");
    let update = CustomMenuItem::new("update".to_string(), "检查更新(U)");
    let tray_menu = SystemTrayMenu::new()
        .add_item(update)
        .add_item(about)
        .add_item(hide)
        .add_item(quit); // insert the menu items here
    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().expect("REASON") {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "about" => open_link("https://github.com/initialencounter/rainwarm"),
                "update" => {
                    let current_version = format!("v{}", env!("CARGO_PKG_VERSION"));
                    let lastest = get_latest_version(current_version.as_str());
                    println!("{}",lastest);
                    if lastest != current_version {
                        open_link(format!("https://github.com/initialencounter/RainWarm/releases/tag/{}",lastest).as_str())
                    }
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn open_link(url: &str) {
    if let Err(e) = webbrowser::open(&url) {
        eprintln!("Failed to open link: {}", e);
    }
}

fn get_latest_version(current_version: &str) -> String {
    let url = "https://api.github.com/repos/initialencounter/rainwarm/releases/latest";
    let client = reqwest::blocking::Client::new();
    
    let resp = match client.get(url).header(reqwest::header::USER_AGENT, "rust-app").send() {
        Ok(response) => response,
        Err(_) => return current_version.to_string(),
    };
    
    if !resp.status().is_success() {
        return current_version.to_string();
    }
    
    let release = match resp.json::<Release>() {
        Ok(release) => release,
        Err(_) => return current_version.to_string(),
    };
    
    release.tag_name
}
