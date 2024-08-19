use serde::Serialize;
use tauri::{DragDropEvent, WindowEvent};
use tauri_plugin_autostart::{MacosLauncher};

use utils::{restart, show_page};

use crate::handle::{handle_drag_drop_event, handle_setup};
use crate::utils::{open_local_dir, open_with_wps};

mod utils;
mod handle;
mod menu;

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
            handle_setup(app);
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
