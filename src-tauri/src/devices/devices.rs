use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use std::error::Error;
use std::process::Command;
use std::str;
use std::{thread, time::Duration};
use tauri::Window;
use tauri::{AppHandle, Manager};

use crate::db_service::db_service::{get_connection_to_ouis, get_manufacturer_by_oui};

#[derive(Serialize, Deserialize)]
pub struct ArpEntry {
    pub ip_address: String,
    pub mac_address: String,
    pub hostname: String,
    pub manufacturer: String,
}

impl Default for ArpEntry {
    fn default() -> Self {
        ArpEntry {
            ip_address: Default::default(),
            mac_address: Default::default(),
            hostname: "Unknown".to_string(),
            manufacturer: Default::default(),
        }
    }
}

#[derive(Deserialize)]
pub struct HostnameRequest {
    ip_address: String,
}

#[derive(Serialize)]
struct HostnameResponse {
    ip_address: String,
    hostname: String,
}

pub fn resolve_hostname(ip_address: &str, app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
    println!("resolve_hostname");

    let output = Command::new("dig")
        .args(["-x", ip_address, "-p", "5353", "@224.0.0.251", "+short"])
        .output()?;

    let hostname = if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !stdout.is_empty() {
            stdout
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    };

    let response = HostnameResponse {
        ip_address: ip_address.to_string(),
        hostname,
    };

    app_handle
        .emit_all("hostname_response", &json!(response))
        .map_err(Into::into)
}

pub fn handle_hostname_request(
    app_handle: AppHandle,
    event_payload: Option<String>,
) -> Result<(), Box<dyn Error>> {
    println!("handle_hostname_request");
    let req: HostnameRequest = serde_json::from_str(&event_payload.unwrap())?;
    resolve_hostname(&req.ip_address, &app_handle)?;
    Ok(())
}

#[tauri::command]
pub fn init_arp_listener(window: Window) {
    std::thread::spawn(move || loop {
        match get_devices() {
            Ok(arp_entries) => {
                window
                    .emit("arp_table", &arp_entries)
                    .expect("Failed to emit event");
            }
            Err(e) => eprintln!("Error listening to traffic: {}", e),
        }
        thread::sleep(Duration::new(10, 0)); // 5 minutes interval
    });
}

pub fn get_devices() -> Result<String, String> {
    let output = Command::new("arp")
        .arg("-a")
        .output()
        .map_err(|e| e.to_string())?;

    let mut entries = Vec::new();

    let conn = get_connection_to_ouis().map_err(|e| e.to_string())?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 3 && parts[1].starts_with('(') && parts[1].ends_with(')') {
                let ip_address = parts[1].trim_matches('(').trim_matches(')').to_string();
                let mac_address = parts[3].to_string();

                if mac_address != "(incomplete)" {
                    let oui = mac_address.replace(":", "").to_lowercase()[..6].to_string();

                    let manufacturer = get_manufacturer_by_oui(&conn, &oui)
                        .unwrap_or_else(|_| "Unknown".to_string());

                    entries.push(ArpEntry {
                        ip_address,
                        mac_address,
                        manufacturer,
                        ..Default::default()
                    });
                }
            }
        }
    }
    serde_json::to_string(&entries).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_hostname(ip_address: String) -> String {
    println!("CALLING get_hostname");

    let output = Command::new("dig")
        .args(["-x", &ip_address, "-p", "5353", "@224.0.0.251", "+short"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("Command output: {}", stdout);
            if !stdout.is_empty() {
                stdout
            } else {
                println!("No hostname found for IP: {}", ip_address);
                "Unknown".to_string()
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
            println!("dig command failed: {}", stderr);
            "Unknown".to_string()
        }
        Err(e) => {
            println!("Failed to execute dig command: {}", e);
            "Unknown".to_string()
        }
    }
}
