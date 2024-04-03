use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process::Command;
use tauri::AppHandle;

use crate::config::config::{NETWORK_DB_PATH, OUIS_DB_PATH};


#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub mac_address: String,
    pub ip_address: String,
    pub manufacturer: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub mac_address: String,
    pub ip_address: String,
    pub hostname: String,
    pub manufacturer: String,
    pub country: String,
}

pub fn get_connection_to_ouis() -> Result<Connection, Box<dyn Error>> {
    if let Some(path) = OUIS_DB_PATH.get() {
        Connection::open(path).map_err(|e| e.into())
    } else {
        println!("Database path is not set yet.");
        Err("Database path is not set yet.".into())
    }
}

pub fn get_connection_to_network() -> Result<Connection, Box<dyn Error>> {
    if let Some(path) = NETWORK_DB_PATH.get() {
        Connection::open(path).map_err(|e| e.into())
    } else {
        println!("Database path is not set yet.");
        Err("Database path is not set yet.".into())
    }
}

pub fn get_network_devices(conn: &Connection, router_mac: &str) -> Result<Vec<Device>> {
    let mut stmt = conn.prepare(
        "SELECT mac_address, ip_address, hostname, manufacturer, country FROM devices WHERE router_mac = ?",
    )?;
    let device_iter = stmt.query_map(params![router_mac], |row| {
        Ok(Device {
            mac_address: row.get(0)?,
            ip_address: row.get(1)?,
            hostname: row.get(2)?,
            manufacturer: row.get(3)?,
            country: row.get(4)?,
        })
    })?;

    let mut devices = Vec::new();
    for device in device_iter {
        devices.push(device?);
    }

    Ok(devices)
}


pub fn get_network(conn: &rusqlite::Connection, router_mac: &str) -> Result<Option<Network>, rusqlite::Error> {
    rusqlite::OptionalExtension::optional(conn.query_row(
        "SELECT router_mac, ip_address, manufacturer, country FROM networks WHERE router_mac = ?1",
        rusqlite::params![router_mac],
        |row| {
            Ok(Network {
                mac_address: row.get(0)?,
                ip_address: row.get(1)?,
                manufacturer: row.get(2)?,
                country: row.get(3)?,
            })
        },
    ))
}



pub fn add_to_networks_table(conn: &Connection, router_mac: &str, ip_address: &str, manufacturer: &str, country: &str) -> Result<()> {

    println!("add_to_networks_table");

    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM networks WHERE router_mac = ?1)",
        rusqlite::params![router_mac],
        |row| row.get(0),
    )?;

    if !exists {
        println!(
            "Adding network with MAC address {}, IP address {}, and manufacturer {}",
            router_mac, ip_address, manufacturer
        );
        conn.execute(
            "INSERT INTO networks (router_mac, ip_address, manufacturer, country) VALUES (?, ?, ?, ?)",
            rusqlite::params![router_mac, ip_address, manufacturer, country],
        )?;
    }

    Ok(())
}



pub fn add_to_device_table(conn: &Connection, mac_address: &str, ip_address: &str, manufacturer: &str, country: &str, router_mac: &str) -> Result<()> {
    if mac_address == router_mac {
        println!("Skipped adding device as it matches router MAC");
        return Ok(());
    }

    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM devices WHERE mac_address = ?1 AND router_mac = ?2)",
        rusqlite::params![mac_address, router_mac],
        |row| row.get(0),
    )?;

    if !exists {
        println!(
            "Adding device with MAC address {}, IP address {}, and manufacturer {} to router MAC {}",
            mac_address, ip_address, manufacturer, router_mac
        );
        conn.execute(
            "INSERT INTO devices (mac_address, router_mac, ip_address, manufacturer, country, hostname) VALUES (?, ?, ?, ?, ?, ?)",
            rusqlite::params![mac_address, router_mac, ip_address, manufacturer, country, "Unknown"],
        )?;
    }
    
    Ok(())
}


pub fn add_hostname(conn: &Connection, mac_address: &str, router_mac: &str, new_hostname: &str) -> Result<()> {
    println!("add_hostname");

    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM devices WHERE mac_address = ?1 AND router_mac = ?2)",
        rusqlite::params![mac_address, router_mac],
        |row| row.get(0),
    )?;
    if exists {
        println!(
            "Updating device with MAC address {} for router MAC {} with new hostname '{}'",
            mac_address, router_mac, new_hostname
        );
        conn.execute(
            "UPDATE devices SET hostname = ?3 WHERE mac_address = ?1 AND router_mac = ?2",
            params![mac_address, router_mac, new_hostname],
        )?;
    } else {
        println!(
            "Device with MAC address {} and router MAC {} not found, cannot update hostname.",
            mac_address, router_mac
        );
    }

    Ok(())
}


pub fn setup_db(app: &AppHandle) -> Result<Connection, Box<dyn Error>> {
    let script_path = app
        .path_resolver()
        .resolve_resource("db/setup_db.sh")
        .expect("failed to resolve resource");

    let script_status = Command::new("sh").arg(script_path).status()?;

    if !script_status.success() {
        eprintln!(
            "Failed to execute setup script. Exit code: {:?}",
            script_status.code()
        );
        return Err("Failed to setup database".into());
    }

    let db_path = app
        .path_resolver()
        .resolve_resource("db/OUIS.db")
        .expect("failed to resolve resource");

    Connection::open(db_path).map_err(|e| e.into())
}

pub fn get_manufacturer_by_oui(conn: &Connection, mac_address: &str) -> Result<(String, String)> {

    let oui = mac_address.replace(":", "").to_lowercase()[..6].to_string();
    let oui_upper = oui.to_uppercase();

    let mut stmt = conn.prepare("SELECT manufacturer, country FROM manufacturers WHERE oui = ?1")?;
    let mut rows = stmt.query(params![oui_upper])?;

    if let Some(row) = rows.next()? {
        let manufacturer: String = row.get(0)?;
        let country: String = row.get(1)?;
        Ok((manufacturer, country))
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}