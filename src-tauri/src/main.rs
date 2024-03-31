// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod router;
use crate::router::find_ip;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_ip::greet])
        .invoke_handler(tauri::generate_handler![find_ip::find_ip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
