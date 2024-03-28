// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate pcap;
extern crate pnet;

mod network_monitor;
use crate::network_monitor::monitor::listen_to_traffic;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![listen_to_traffic])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
