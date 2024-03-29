// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate hickory_resolver;
extern crate pcap;
extern crate pnet;

mod network_monitor;
use crate::network_monitor::monitor;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use tauri::Manager;

static HOSTNAME_CACHE: Lazy<Mutex<HashMap<Ipv4Addr, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

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
