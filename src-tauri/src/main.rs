// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod devices;
use crate::devices::devices::{get_devices, get_hostname};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_devices, get_hostname])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
