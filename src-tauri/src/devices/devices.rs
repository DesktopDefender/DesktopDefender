use rusqlite::Connection;
use serde::Deserialize;
use serde_json::to_string;
use std::error::Error;
use std::process::Command;
use std::result::Result;
use std::str;
use tauri::{AppHandle, Manager};

use crate::db_service::db_service::{
    add_hostname, add_to_device_table, add_to_networks_table, get_manufacturer_by_oui, get_network,
    get_network_devices,
};

#[tauri::command]
pub fn get_router_info() -> Result<String, String> {
    let network_conn = Connection::open("network.db").expect("Failed to open database");
    let ouis_conn = Connection::open("ouis.db").expect("Failed to open database");

    let ip_output = Command::new("sh")
        .arg("-c")
        .arg("netstat -rn | grep default | head -n 1 | tr -s ' ' | cut -d ' ' -f 2")
        .output()
        .map_err(|e| e.to_string())?;

    let router_ip = str::from_utf8(&ip_output.stdout)
        .unwrap()
        .trim()
        .to_string();

    let arp_output = Command::new("arp")
        .arg("-an")
        .output()
        .map_err(|e| e.to_string())?;

    let arp_entries = str::from_utf8(&arp_output.stdout).unwrap();

    let mut mac_address_opt = None;

    for line in arp_entries.lines() {
        if line.contains(&router_ip) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            mac_address_opt = parts.get(3).map(|&mac| mac.to_string());
            break;
        }
    }

    let mac_address = mac_address_opt.ok_or("MAC address not found")?;

    let network = match get_network(&network_conn, &mac_address).map_err(|e| e.to_string())? {
        Some(network) => network,
        None => {
            let manufacturer_country = get_manufacturer_by_oui(&ouis_conn, &mac_address)
                .unwrap_or_else(|_| ("Unknown".to_string(), "Unknown".to_string()));
            let (manufacturer, country) = manufacturer_country;
            add_to_networks_table(
                &network_conn,
                &mac_address,
                &router_ip,
                &manufacturer,
                &country,
            )
            .map_err(|e| e.to_string())?;
            get_network(&network_conn, &mac_address)
                .map_err(|e| e.to_string())?
                .ok_or("Failed to add or retrieve network")?
        }
    };

    to_string(&network).map_err(|e| e.to_string())
}

fn is_valid_device_ip(device_ip: &str, router_ip: &str) -> bool {
    let router_prefix = router_ip
        .split('.')
        .take(2)
        .collect::<Vec<&str>>()
        .join(".");
    let device_prefix = device_ip
        .split('.')
        .take(2)
        .collect::<Vec<&str>>()
        .join(".");

    if router_prefix != device_prefix {
        return false;
    }

    // check if broadcast
    if device_ip.ends_with(".255") {
        return false;
    }

    true
}

// TODO PORT SCAN, ONLY NEEDS ON LOAD
#[tauri::command]
pub fn initalize_devices(router_mac: &str, router_ip: &str) -> Result<String, String> {
    let network_conn = Connection::open("network.db").expect("Failed to open database");
    let ouis_conn = Connection::open("ouis.db").expect("Failed to open database");

    let arp_entries = get_arp_table().map_err(|e| e.to_string())?;

    for entry in arp_entries {
        if is_valid_device_ip(&entry.ip_address, router_ip) {
            let manufacturer_country = get_manufacturer_by_oui(&ouis_conn, &entry.mac_address)
                .unwrap_or_else(|_| ("Unknown".to_string(), "Unknown".to_string()));
            let (manufacturer, country) = manufacturer_country;

            add_to_device_table(
                &network_conn,
                &entry.mac_address,
                &entry.ip_address,
                &manufacturer,
                &country,
                router_mac,
            )
            .map_err(|e| e.to_string())?;
        }
    }

    match get_network_devices(&network_conn, router_mac) {
        Ok(devices) => serde_json::to_string(&devices).map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_network_info(router_mac: &str) -> Result<String, String> {
    let network_conn = Connection::open("network.db").expect("Failed to open database");

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
    let req: HostnameRequest = serde_json::from_str(&event_payload.unwrap())?;
    resolve_network_hostnames(&req.router_mac, &app_handle); //?
    Ok(())
}

pub fn resolve_network_hostnames(router_mac: &str, app_handle: &AppHandle) {
    let network_conn = Connection::open("network.db").expect("Failed to open database");

    match get_network_devices(&network_conn, router_mac) {
        Ok(devices) => {
            for device in devices {
                if device.hostname == "Unknown" {
                    let new_hostname = resolve_hostname(&device.ip_address);
                    if new_hostname != "Unknown" {
                        let _ = add_hostname(
                            &network_conn,
                            &device.mac_address,
                            router_mac,
                            &new_hostname,
                        );

                        if let Err(e) = app_handle.emit_all("hostname_found", ()) {
                            eprintln!("Error emitting 'hostname_found': {:?}", e);
                        }
                    }
                }
            }
        }
        Err(e) => println!("Error fetching devices: {}", e.to_string()),
    }
}

fn resolve_hostname(ip_address: &str) -> String {
    println!("Resolving for {}", ip_address);
    let output = Command::new("dig")
        .args(["-x", ip_address, "-p", "5353", "@224.0.0.251", "+short"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !stdout.is_empty() {
                stdout
            } else {
                "Unknown".to_string()
            }
        }
        Ok(_) => "Unknown".to_string(),
        Err(_) => "Unknown".to_string(),
    }
}
