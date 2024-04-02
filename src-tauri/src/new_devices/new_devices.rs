use std::process::Command;
use std::result::Result;
use serde::{Deserialize, Serialize};

use crate::db_service::db_service::{get_connection_to_network, get_network_devices, add_to_device_table};



// TODO PORT SCAN, ONLY NEEDS ON LOAD
#[tauri::command]
pub fn get_network_info(router_mac: &str) -> Result<String, String> {
    let conn = get_connection_to_network().map_err(|e| e.to_string())?;

    let arp_entries = get_arp_table().map_err(|e| e.to_string())?;
    for entry in arp_entries {
        add_to_device_table(&conn, &entry.mac_address, router_mac)
            .map_err(|e| e.to_string())?;
    }

    match get_network_devices(&conn, router_mac) {
        Ok(devices_json) => Ok(devices_json),
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
                        mac_address
                    });
                }
            }   
        }     
    }
    Ok(entries)
}