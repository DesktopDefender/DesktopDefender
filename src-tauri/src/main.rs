// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate hickory_resolver;
extern crate pcap;
extern crate pnet;

mod network_monitor;
use crate::network_monitor::monitor;
use dotenvy::dotenv;
use network_monitor::info::Info;
use once_cell::sync::Lazy;
// use parking_lot::Mutex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::net::Ipv4Addr;
use tauri::Manager;
use tokio::sync::Mutex;

static IP_CACHE: Lazy<Mutex<HashMap<String, Info>>> = Lazy::new(|| Mutex::new(HashMap::new()));

static IP_SET: Lazy<Mutex<HashSet<Ipv4Addr>>> = Lazy::new(|| Mutex::new(HashSet::new()));

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            dotenv().ok();
            let api_key = env::var("API_TOKEN").expect("API_TOKEN must be set");

            monitor::init_traffic_emitter(
                app.get_window("main").expect("Failed to get main window"),
            );
            network_monitor::info::init_info_emitter(
                app.get_window("main").expect("Failed to get main window"),
                api_key,
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
