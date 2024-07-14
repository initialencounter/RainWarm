use std::sync::{mpsc};
use std::{fs, thread};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri::{
    Manager,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
};
use tauri::tray::MouseButtonState;
use tauri::{WindowEvent, DragDropEvent, Emitter};

mod utils;
use utils::{check_update, open_link, restart, set_window_topmost, calculate_blake2b512};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
// use window_vibrancy::apply_blur;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let quit = MenuItemBuilder::new("退出(X)").id("quit").build(app).unwrap();
            let hide = MenuItemBuilder::new("隐藏(H)").id("hide").build(app).unwrap();
            let about = MenuItemBuilder::new("关于(A)").id("about").build(app).unwrap();
            let update = MenuItemBuilder::new("检查更新(U)").id("update").build(app).unwrap();
            let restart_ = MenuItemBuilder::new("重启(R)").id("restart").build(app).unwrap();
            let tray_menu = MenuBuilder::new(app)
                .items(&[&update, &restart_, &about, &hide, &quit]) // insert the menu items here
                .build()
                .unwrap();
            let _ = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => app.exit(0),
                    "hide" => {
                        let window = app.get_webview_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                        } else {
                            window.show().unwrap();
                        }
                    }
                    "restart" => restart(),
                    "about" => open_link("https://github.com/initialencounter/rainwarm"),
                    "update" => {
                        let current_version = format!("v{}", env!("CARGO_PKG_VERSION"));
                        let latest = check_update(String::from("000"));
                        if latest == "000" {
                            app.dialog().message("检查更新失败!").kind(MessageDialogKind::Error).show(|_| {});
                        } else if latest != current_version {
                            app.dialog().message(format!("发现新版本{}，是否前往", latest)).kind(MessageDialogKind::Info).show(|_| {});
                            open_link(
                                "https://github.com/initialencounter/RainWarm/releases/latest",
                            )
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
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app).unwrap();
            let main_window = app.get_webview_window("main").unwrap();
            set_window_topmost(main_window);
            // #[cfg(target_os = "macos")]
            // apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
            //     .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            //
            // #[cfg(target_os = "windows")]
            // apply_blur(&main_window, Some((18, 18, 18, 125)))
            //     .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::DragDrop(DragDropEvent::Drop { paths, .. }) => {
                let app = window.app_handle();
                let (tx, rx) = mpsc::channel();
                for path in paths.iter() {
                    if path.is_file() {
                        let tx = tx.clone();
                        let file_path = path.to_string_lossy().into_owned();
                        thread::spawn(move || {
                            let file_tile = calculate_blake2b512(file_path);
                            tx.send(file_tile).unwrap();
                        });
                    }else {
                        match fs::read_dir(&path) {
                            Ok(entries) => {
                                entries.for_each(|entry| {
                                    let file_path = entry.unwrap().path().to_string_lossy().into_owned();
                                    let tx = tx.clone();
                                    thread::spawn(move || {
                                        let file_tile = calculate_blake2b512(file_path);
                                        tx.send(file_tile).unwrap();
                                    });
                                });
                            }
                            _ => {}
                        }
                    }
                }
                // 接收计算结果并发射事件
                let app = app.clone();
                thread::spawn(move || {
                    for file_tile in rx {
                        app.emit("file_tile", Some(&file_tile)).unwrap();
                    }
                });
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![open_link, restart])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
