use std::{fs, thread};
use std::path::PathBuf;
use std::sync::mpsc;

use tauri::{AppHandle, Emitter, Manager, Window, Wry};
use tauri::menu::MenuItem;
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconEvent};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use crate::Link;
use crate::utils::{calculate_blake2b512, check_update, FileTile, hide_or_show};

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

pub fn handle_hide_or_show(app: &AppHandle<Wry>, hide: &MenuItem<Wry>) {
    let window = app.get_webview_window("main").unwrap();
    let title = hide_or_show(window);
    hide.set_text(title).expect("Failed to set tray text");
}

pub fn handle_auto_start(app: &AppHandle<Wry>, auto_start: &MenuItem<Wry>) {
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

pub fn handle_tray_icon_event(tray: &TrayIcon, event: &TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        let app = tray.app_handle();
        if let Some(window) = app.get_webview_window("main") {
            hide_or_show(window);
        }
    }
}

pub fn handle_drag_drop_event(window: &Window, paths: &Vec<PathBuf>) {
    let app = window.app_handle();
    let (tx, rx) = mpsc::channel();

    // 启动一个线程处理拖拽的文件或目录
    let paths = paths.clone();
    thread::spawn(move || {
        for path in paths {
            let tx = tx.clone();
            let path_str = path.to_string_lossy().into_owned();
            if path.is_file() {
                handle_file(path_str, tx);
            } else {
                handle_directory(path_str, tx);
            }
        }
    });

    // 启动另一个线程接收和发送文件信息
    let app_clone = app.clone();
    thread::spawn(move || {
        for file_tile in rx {
            app_clone.emit("file_tile", Some(&file_tile)).unwrap();
        }
    });
}

pub fn handle_menu_event_update(app: &AppHandle<Wry>) {
    let current_version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let latest = check_update(String::from("000"));
    if latest == "000" {
        app.dialog().message("检查更新失败!").kind(MessageDialogKind::Error).show(|_| {});
    } else if latest != current_version {
        app.dialog().message(format!("发现新版本{}，是否前往", latest)).kind(MessageDialogKind::Info).show(|_| {});
        app.emit("open_link", Some(Link { link: "https://github.com/initialencounter/RainWarm/releases/latest".to_string() })).unwrap();
    } else {
        app.dialog().message("当前版本是最新版").kind(MessageDialogKind::Info).show(|_| {});
    }
}