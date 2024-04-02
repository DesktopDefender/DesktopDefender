use serde::{Deserialize, Serialize};
use std::process::Command;
use std::result::Result;

use crate::db_service::db_service::{
    add_to_device_table, get_connection_to_network, get_connection_to_ouis, get_network_devices, update_device_manufacturer, get_manufacturer_by_oui
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

    // Re-fetch updated devices list to return
    match get_network_devices(&network_conn, router_mac) {
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
                        mac_address,
                    });
                }
            }
        }
    }
    Ok(entries)
}
