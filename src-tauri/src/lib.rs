use serde::Serialize;
use std::sync::mpsc;
use std::thread;
use tauri::tray::MouseButtonState;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri::{DragDropEvent, Emitter, WindowEvent};

mod utils;
use crate::utils::{handle_directory, handle_file, hide_or_show, open_local_dir, open_with_wps};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use utils::{check_update, restart, show_page};
#[derive(Serialize, Clone)]
struct Link {
    link: String,
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let help_ = MenuItemBuilder::new("帮助(H)").id("help").build(app).unwrap();
            let quit = MenuItemBuilder::new("退出(X)").id("quit").build(app).unwrap();
            let hide = MenuItemBuilder::new("隐藏(H)").id("hide").build(app).unwrap();
            let about = MenuItemBuilder::new("关于(A)").id("about").build(app).unwrap();
            let update = MenuItemBuilder::new("检查更新(U)").id("update").build(app).unwrap();
            let restart_ = MenuItemBuilder::new("重启(R)").id("restart").build(app).unwrap();
            let tray_menu = MenuBuilder::new(app)
                .items(&[&help_, &update, &restart_, &about, &hide, &quit]) // insert the menu items here
                .build()
                .unwrap();
            let _ = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "help" => app.emit("open_link", Some(Link{link: "https://github.com/initialencounter/RainWarm?tab=readme-ov-file#使用帮助".to_string() })).unwrap(),
                    "quit" => app.exit(0),
                    "hide" => {
                        let window = app.get_webview_window("main").unwrap();
                        hide_or_show(window);
                    }
                    "restart" => restart(),
                    "about" => app.emit("open_link", Some(Link{link: "https://github.com/initialencounter/rainwarm".to_string() })).unwrap(),
                    "update" => {
                        let current_version = format!("v{}", env!("CARGO_PKG_VERSION"));
                        let latest = check_update(String::from("000"));
                        if latest == "000" {
                            app.dialog().message("检查更新失败!").kind(MessageDialogKind::Error).show(|_| {});
                        } else if latest != current_version {
                            app.dialog().message(format!("发现新版本{}，是否前往", latest)).kind(MessageDialogKind::Info).show(|_| {});
                            app.emit("open_link", Some(Link{link: "https://github.com/initialencounter/RainWarm/releases/latest".to_string() })).unwrap();
                        } else {
                            app.dialog().message("当前版本是最新版").kind(MessageDialogKind::Info).show(|_| {});
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
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
                })
                .build(app).unwrap();
            app.get_webview_window("main").unwrap().set_always_on_top(true).expect("Failed to set window as topmost");
            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::DragDrop(DragDropEvent::Drop { paths, .. }) => {
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
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![restart,open_local_dir,open_with_wps,show_page])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
