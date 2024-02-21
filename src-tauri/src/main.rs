// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::thread;
use std::time::Duration;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
mod utils;
use utils::{get_latest_version, open_link};

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
                    if lastest != current_version {
                        let _ = tauri::WindowBuilder::new(
                            app,
                            "local_1",
                            tauri::WindowUrl::App("confirm.html".into()),
                        )
                        .build();
                        let window = app.get_window("local_1").unwrap();
                        if !window.is_visible().expect("REASON") {
                            window.show().unwrap();
                        }
                        thread::spawn(move || {
                            // 等待5秒钟
                            thread::sleep(Duration::from_secs(5));
                            window.hide().unwrap();
                        });
                    } else {
                        let _ = tauri::WindowBuilder::new(
                            app,
                            "local_2",
                            tauri::WindowUrl::App("latest.html".into()),
                        )
                        .build();
                        let window = app.get_window("local_2").unwrap();
                        if !window.is_visible().expect("REASON") {
                            window.show().unwrap();
                        }
                        thread::spawn(move || {
                            // 等待5秒钟
                            thread::sleep(Duration::from_secs(5));
                            window.hide().unwrap();
                        });
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
        .invoke_handler(tauri::generate_handler![open_link])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
