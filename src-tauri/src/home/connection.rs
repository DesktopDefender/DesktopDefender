use reqwest;
use reqwest::blocking::Client;
use std::{thread, time::Duration};
use tauri::Window;

#[tauri::command]
pub fn init_connection_listener(window: Window) {
    std::thread::spawn(move || loop {
        let client = Client::new();
        let connected = check_connectivity(&client);
        window
            .emit("connection_status", &connected)
            .expect("Failed to emit event");
        thread::sleep(Duration::from_secs(3));
    });
}

fn check_connectivity(client: &Client) -> bool {
    let test_url = "https://www.google.com";
    client.get(test_url).send().is_ok()
}
