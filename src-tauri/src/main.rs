// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod db_service;
mod devices;
mod home;

use crate::devices::devices::{get_network_info, get_router_info, initalize_devices};

use crate::db_service::db_service::{setup_network_db, setup_ouis_db};
use crate::home::connection::init_connection_listener;

use devices::devices::handle_hostname_request;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_router_info,
            initalize_devices,
            get_network_info
        ])
        .setup(|app| {
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

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
