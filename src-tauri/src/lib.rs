use serde::Serialize;
use tauri::{
    Manager,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
};
use tauri::{DragDropEvent, Emitter, WindowEvent};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

use utils::{restart, show_page};

use crate::handle::{handle_auto_start, handle_drag_drop_event, handle_hide_or_show, handle_menu_event_update, handle_tray_icon_event};
use crate::utils::{open_local_dir, open_with_wps};

mod utils;
mod handle;

#[derive(Serialize, Clone)]
struct Link {
    link: String,
}
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let help_ = MenuItemBuilder::new("帮助(H)").id("help").build(app).unwrap();
            let quit = MenuItemBuilder::new("退出(X)").id("quit").build(app).unwrap();
            let hide = MenuItemBuilder::new("隐藏(H)").id("hide").build(app).unwrap();
            let about = MenuItemBuilder::new("关于(A)").id("about").build(app).unwrap();
            let update = MenuItemBuilder::new("检查更新(U)").id("update").build(app).unwrap();
            let restart_ = MenuItemBuilder::new("重启(R)").id("restart").build(app).unwrap();
            let autostart_manager = app.autolaunch();
            let auto_start_title = if autostart_manager.is_enabled().unwrap() { "开机自启动(✔️)" } else { "开机自启动(❌)" };
            let auto_start = MenuItemBuilder::new(auto_start_title).id("auto_start").build(app).unwrap();
            let tray_menu = MenuBuilder::new(app)
                .items(&[&help_, &update, &restart_, &auto_start, &about, &hide, &quit]) // insert the menu items here
                .build()
                .unwrap();
            let _ = TrayIconBuilder::with_id("system-tray-1")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "help" => app.emit("open_link", Some(Link { link: "https://github.com/initialencounter/RainWarm?tab=readme-ov-file#使用帮助".to_string() })).unwrap(),
                    "quit" => app.exit(0),
                    "hide" => handle_hide_or_show(&app, &hide),
                    "restart" => restart(),
                    "about" => app.emit("open_link", Some(Link { link: "https://github.com/initialencounter/rainwarm".to_string() })).unwrap(),
                    "update" => handle_menu_event_update(&app),
                    "auto_start" => handle_auto_start(&app, &auto_start),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    handle_tray_icon_event(tray, &event);
                })
                .build(app).unwrap();
            app.get_webview_window("main").unwrap().set_always_on_top(true).expect("Failed to set window as topmost");
            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::DragDrop(DragDropEvent::Drop { paths, .. }) => {
                handle_drag_drop_event(window, &paths);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![restart,open_local_dir,open_with_wps,show_page])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
