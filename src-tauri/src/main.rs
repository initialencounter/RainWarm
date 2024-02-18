// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, Manager};
use webbrowser;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}
fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出(Exit)");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏(Hide)");
    let about = CustomMenuItem::new("about".to_string(), "关于(About)");
    let update = CustomMenuItem::new("update".to_string(), "检查更新(Update)");
    let tray_menu = SystemTrayMenu::new()
        .add_item(update)
        .add_item(about)
        .add_item(hide)
        .add_item(quit);// insert the menu items here
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
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    "about" => {
                        open_link("https://github.com/initialencounter/rainwarm")
                    }
                    "update" => {
                        let current_version = env!("CARGO_PKG_VERSION");
                        if let Err(err) = check_latest_version(current_version) {
                            eprintln!("Error: {}", err);
                        }else {
                            println!("Latest version");
                        }
                    }
                    _ => {}
                }
            }
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

fn check_latest_version(current_version: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/initialencounter/rainwarm/releases/latest";
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).header(reqwest::header::USER_AGENT, "rust-app").send()?;
    if !resp.status().is_success() {
        return Err("Failed to fetch latest release".into());
    }
    let release: Release = resp.json()?;
    if release.tag_name == current_version {
        println!("You are using the latest version: {}", current_version);
    } else {
        println!("There is a newer version available: {}", release.tag_name);
    }
    Ok(())

}