// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use md5::compute;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn drag_file(data: &str) -> String {
    let result = compute(&data);
    let md5_hex: String = format!("{:x}", result);
    md5_hex[..16].to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![drag_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}