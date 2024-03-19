// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::env;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn double(number: i64) -> i64 {
    (number * 2) as i64
}

#[tauri::command]
fn get_os() -> String {
    format!("{}", env::consts::OS)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, double, get_os])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
