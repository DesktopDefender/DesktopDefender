// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate hickory_resolver;
extern crate pcap;
extern crate pnet;

mod db_service;
mod devices;
mod helpers;
mod home;
mod network_monitor;
mod router;

use crate::db_service::db_service::{setup_network_db, setup_ouis_db};
use crate::devices::devices::{get_network_info, get_router_info, initalize_devices};
use crate::helpers::port_scanner::find_open_ports;
use crate::home::connection::init_connection_listener;
use crate::network_monitor::monitor;
use crate::router::find_ip::find_ip;
use crate::router::find_mac::find_mac_address;

use devices::devices::handle_hostname_request;
use dotenvy::dotenv;
use network_monitor::info::Info;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::env;
use std::net::Ipv4Addr;
use tauri::Manager;
use tokio::sync::Mutex;

static IP_CACHE: Lazy<Mutex<HashMap<String, Info>>> = Lazy::new(|| Mutex::new(HashMap::new()));

static IP_SET: Lazy<Mutex<HashSet<Ipv4Addr>>> = Lazy::new(|| Mutex::new(HashSet::new()));

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            find_mac_address,
            find_ip,
            find_open_ports,
            get_router_info,
            initalize_devices,
            get_network_info
        ])
        .setup(|app| {
            dotenv().ok();
            let api_key = env::var("IPINFO_TOKEN").expect("IPINFO_TOKEN must be set");

            setup_network_db();
            let _ = setup_ouis_db();

            init_connection_listener(app.get_window("main").expect("Failed to get main window"));

            let app_handle = app.app_handle().clone();
            let _event_id = app.listen_global("hostname_request", move |event| {
                if let Err(e) =
                    handle_hostname_request(app_handle.clone(), event.payload().map(String::from))
                {
                    eprintln!("Error processing hostname_request: {}", e);
                }
            });

            monitor::init_traffic_emitter(
                app.get_window("main").expect("Failed to get main window"),
            );
            network_monitor::info::init_info_emitter(
                app.get_window("main").expect("Failed to get main window"),
                api_key,
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_router_info,
            initalize_devices,
            get_network_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
