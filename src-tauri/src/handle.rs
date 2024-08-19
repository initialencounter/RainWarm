use std::fs;
use std::sync::mpsc;

use tauri::{AppHandle, WebviewWindow, Wry};
use tauri::menu::MenuItem;
use tauri_plugin_autostart::ManagerExt;

use crate::utils::{calculate_blake2b512, FileTile, hide_or_show};

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