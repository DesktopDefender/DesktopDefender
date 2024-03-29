// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod devices;
use crate::devices::devices::{init_arp_listener, get_hostname};
use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_hostname])
    .setup(|app| {
      init_arp_listener(
        app.get_window("main").expect("Failed to get main window"),
      );
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
