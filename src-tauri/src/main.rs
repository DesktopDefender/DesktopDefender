// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate pcap;
extern crate pnet;

mod network_monitor;
use crate::network_monitor::monitor;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            monitor::init_traffic_listener(
                app.get_window("main").expect("Failed to get main window"),
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
