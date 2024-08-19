use tauri::{App, Wry};
use tauri::menu::{MenuItem, MenuItemBuilder};
use tauri_plugin_autostart::ManagerExt;

pub fn create_menu_item(app: &mut App) -> [MenuItem<Wry>; 7] {
    let help_ = MenuItemBuilder::new("帮助(H)").id("help").build(app).unwrap();
    let quit = MenuItemBuilder::new("退出(X)").id("quit").build(app).unwrap();
    let hide = MenuItemBuilder::new("隐藏(H)").id("hide").build(app).unwrap();
    let about = MenuItemBuilder::new("关于(A)").id("about").build(app).unwrap();
    let update = MenuItemBuilder::new("检查更新(U)").id("update").build(app).unwrap();
    let restart_ = MenuItemBuilder::new("重启(R)").id("restart").build(app).unwrap();
    let autostart_manager = app.autolaunch();
    let auto_start_title = if autostart_manager.is_enabled().unwrap() { "开机自启动(✔️)" } else { "开机自启动(❌)" };
    let auto_start = MenuItemBuilder::new(auto_start_title).id("auto_start").build(app).unwrap();
    [help_, quit, hide, about, update, restart_, auto_start]
}