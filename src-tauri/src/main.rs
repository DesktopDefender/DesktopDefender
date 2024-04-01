// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod home;
mod devices;
mod db_service;
use crate::home::connection::init_connection_listener;
use crate::devices::devices::{init_arp_listener, get_hostname, handle_hostname_request};
use crate::db_service::db_service::{setup_db, get_manufacturer_by_oui};

use rusqlite::Connection;
use tauri::{AppHandle, Manager};


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_hostname])
    .setup(|app| {
      let app_handle = app.app_handle().clone();
      let connection = setup_db(&app_handle).expect("Failed to setup the database");


      init_arp_listener(
        app.get_window("main").expect("Failed to get main window"),
      );

      init_connection_listener(
        app.get_window("main").expect("Failed to get main window"),
      );

      
      let _event_id = app.listen_global("hostname_request", move |event| {
        if let Err(e) = handle_hostname_request(app_handle.clone(), event.payload().map(String::from)) {
          eprintln!("Error processing hostname_request: {}", e);
        }
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}