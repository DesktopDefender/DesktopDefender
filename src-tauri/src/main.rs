// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod db_service;
mod home;
mod devices;

use crate::devices::devices::{get_router_info, initalize_devices, get_network_info};

use crate::db_service::db_service::setup_db;
use crate::home::connection::init_connection_listener;

use crate::config::config::{OUIS_DB_PATH, NETWORK_DB_PATH};

use devices::devices::handle_hostname_request;
use tauri::Manager;



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_router_info, initalize_devices, get_network_info])
        .setup(|app| {
            let app_handle = app.app_handle().clone();
            setup_db(&app_handle).expect("Failed to setup the database");

            let ouis_path = app_handle
                .path_resolver()
                .resolve_resource("db/OUIS.db")
                .expect("Failed to resolve database path");

            OUIS_DB_PATH.set(ouis_path).expect("Failed to set DB path");


            let network_path = app_handle
                .path_resolver()
                .resolve_resource("db/network.db")
                .expect("Failed to resolve database path");

            NETWORK_DB_PATH.set(network_path).expect("Failed to set DB path");


            init_connection_listener(app.get_window("main").expect("Failed to get main window"));

            let _event_id = app.listen_global("hostname_request", move |event| {
                if let Err(e) =
                    handle_hostname_request(app_handle.clone(), event.payload().map(String::from))
                {
                    eprintln!("Error processing hostname_request: {}", e);
                }
            });


            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
