use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use std::process::Command;
use std::result::Result;
use std::error::Error;

use crate::db_service::db_service::{
    add_to_device_table, get_connection_to_network, get_connection_to_ouis, get_manufacturer_by_oui, get_network_devices, add_hostname
};

// TODO PORT SCAN, ONLY NEEDS ON LOAD
#[tauri::command]
pub fn get_network_info(router_mac: &str) -> Result<String, String> {
    let network_conn = get_connection_to_network().map_err(|e| e.to_string())?;
    let ouis_conn = get_connection_to_ouis().map_err(|e| e.to_string())?;

    let arp_entries = get_arp_table().map_err(|e| e.to_string())?;
    for entry in arp_entries {

        let manufacturer_result = get_manufacturer_by_oui(&ouis_conn, &entry.mac_address);
        let manufacturer = match manufacturer_result {
            Ok(manu) => manu,
            Err(_) => "Unknown".to_string(),
        };

        add_to_device_table(&network_conn, &entry.mac_address, &entry.ip_address, &manufacturer, router_mac)
            .map_err(|e| e.to_string())?;
    }

    
    match get_network_devices(&network_conn, router_mac) {
        Ok(devices) => serde_json::to_string(&devices).map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub struct ArpEntry {
    pub ip_address: String,
    pub mac_address: String,
}


fn get_arp_table() -> Result<Vec<ArpEntry>, String> {
    let output = Command::new("arp")
        .arg("-an")
        .output()
        .map_err(|e| e.to_string())?;

    let mut entries = Vec::new();

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 3 && parts[1].starts_with('(') && parts[1].ends_with(')') {
                let ip_address = parts[1].trim_matches('(').trim_matches(')').to_string();
                let mac_address = parts[3].to_string();

                if mac_address != "(incomplete)" {
                    entries.push(ArpEntry {
                        ip_address,
                        mac_address,
                    });
                }
            }
        }
    }
    Ok(entries)
}



#[derive(Deserialize)]
pub struct HostnameRequest {
    router_mac: String,
}


pub fn handle_hostname_request(
    app_handle: AppHandle,
    event_payload: Option<String>,
) -> Result<(), Box<dyn Error>> {
    println!("handle_hostname_request");
    let req: HostnameRequest = serde_json::from_str(&event_payload.unwrap())?;
    resolve_network_hostnames(&req.router_mac, &app_handle); //?
    Ok(())
}


pub fn resolve_network_hostnames(router_mac: &str, app_handle: &AppHandle) {
    println!("resolve_hostname, router_mac: {}", router_mac);

    let found_hostname = true;

    match get_connection_to_network() {
        Ok(network_conn) => {
     
            match get_network_devices(&network_conn, router_mac) {
                Ok(devices) => {
                    for device in devices {

                        if device.hostname == "Unknown" {
                            let new_hostname = resolve_hostname(&device.ipAddress);
                            if new_hostname != "Unknown" {
                                println!("{:?} --- Hostname found {}", device, new_hostname);
                                let _ = add_hostname(&network_conn, &device.macAddress, router_mac, &new_hostname);
                            }
                        }
                    }
                },
                Err(e) => println!("Error fetching devices: {}", e.to_string()),
            }
        },
        Err(e) => println!("Error establishing network connection: {}", e.to_string()),
    }
}

fn resolve_hostname(ip_address: &str) -> String {
    println!("resolving for {}", ip_address);

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
