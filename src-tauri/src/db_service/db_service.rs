use chrono::prelude::*;
use csv::ReaderBuilder;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

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
    pub date_added: String,
}

pub fn setup_network_db() {
    /*
        Path needs to be set to src-tauri. Trying to resolve paths nested inside src-tauri for example src-tauri/db-stuff
        results in needing to include files in tauri.conf.json resources, code for resolving the paths and storing the paths
        in global variables were they can be accessed. Also the file would need to be created beforehand to reference it in
        tauri.conf.json resources.
        Storing it in src-tauri is the easiest solution for now.
    */
    let conn = Connection::open("network.db").expect("Failed to open database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS networks (
            router_mac TEXT PRIMARY KEY,
            ip_address TEXT,
            manufacturer TEXT DEFAULT 'Unknown',
            country TEXT DEFAULT 'Unknown'
        )",
        [],
    )
    .expect("Failed to create networks table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS devices (
            mac_address TEXT,
            router_mac TEXT,
            ip_address TEXT,
            hostname TEXT DEFAULT 'Unknown',
            manufacturer TEXT DEFAULT 'Unknown',
            country TEXT DEFAULT 'Unknown',
            date_added TEXT,
            PRIMARY KEY (mac_address, router_mac),
            FOREIGN KEY (router_mac) REFERENCES networks(router_mac)
        )",
        [],
    )
    .expect("Failed to create devices table");
}

pub fn setup_ouis_db() -> Result<(), Box<dyn Error>> {
    /*
        Path needs to be set to src-tauri. Trying to resolve paths nested inside src-tauri for example src-tauri/db-stuff
        results in needing to include files in tauri.conf.json resources, code for resolving the paths and storing the paths
        in global variables were they can be accessed. Also the file would need to be created beforehand to reference it in
        tauri.conf.json resources.
        Storing it in src-tauri is the easiest solution for now.
    */

    let mut conn = Connection::open("ouis.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS manufacturers (
            oui TEXT PRIMARY KEY,
            manufacturer TEXT,
            country TEXT
        )",
        [],
    )?;

    let file = File::open("ouis.csv")?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(BufReader::new(file));

    let tx = conn.transaction()?;
    for result in rdr.records() {
        let record = result?;

        let oui = record.get(0).ok_or("missing field 'oui'")?;
        let manufacturer = record.get(1).ok_or("missing field 'manufacturer'")?;
        let country = record.get(2).ok_or("missing field 'country'")?;

        tx.execute(
            "INSERT INTO manufacturers (oui, manufacturer, country) VALUES (?1, ?2, ?3)",
            params![oui, manufacturer, country],
        )?;
    }
    tx.commit()?;

    Ok(())
}

pub fn get_network_devices(conn: &Connection, router_mac: &str) -> Result<Vec<Device>> {
    let mut stmt = conn.prepare(
        "SELECT mac_address, ip_address, hostname, manufacturer, country, date_added FROM devices WHERE router_mac = ?",
    )?;
    let device_iter = stmt.query_map(params![router_mac], |row| {
        Ok(Device {
            mac_address: row.get(0)?,
            ip_address: row.get(1)?,
            hostname: row.get(2)?,
            manufacturer: row.get(3)?,
            country: row.get(4)?,
            date_added: row.get(5)?,
        })
    })?;

    let mut devices = Vec::new();
    for device in device_iter {
        devices.push(device?);
    }

    Ok(devices)
}

pub fn get_network(
    conn: &rusqlite::Connection,
    router_mac: &str,
) -> Result<Option<Network>, rusqlite::Error> {
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

pub fn add_to_networks_table(
    conn: &Connection,
    router_mac: &str,
    ip_address: &str,
    manufacturer: &str,
    country: &str,
) -> Result<()> {
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

pub fn add_to_device_table(
    conn: &Connection,
    mac_address: &str,
    ip_address: &str,
    manufacturer: &str,
    country: &str,
    router_mac: &str,
) -> Result<()> {
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
        let date_added = Local::now().format("%d/%m/%y").to_string();

        println!(
            "Adding device with MAC address {}, IP address {}, and manufacturer {} to router MAC {}",
            mac_address, ip_address, manufacturer, router_mac
        );
        conn.execute(
            "INSERT INTO devices (mac_address, router_mac, ip_address, manufacturer, country, hostname, date_added) VALUES (?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![mac_address, router_mac, ip_address, manufacturer, country, "Unknown", date_added],
        )?;
    }

    Ok(())
}

pub fn add_hostname(
    conn: &Connection,
    mac_address: &str,
    router_mac: &str,
    new_hostname: &str,
) -> Result<()> {
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

pub fn get_manufacturer_by_oui(conn: &Connection, mac_address: &str) -> Result<(String, String)> {
    let oui = mac_address.replace(":", "").to_lowercase()[..6].to_string();
    let oui_upper = oui.to_uppercase();

    let mut stmt =
        conn.prepare("SELECT manufacturer, country FROM manufacturers WHERE oui = ?1")?;
    let mut rows = stmt.query(params![oui_upper])?;

    if let Some(row) = rows.next()? {
        let manufacturer: String = row.get(0)?;
        let country: String = row.get(1)?;
        Ok((manufacturer, country))
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OuiResponse {
    pub manufacturer: String,
    pub country: String,
}

#[tauri::command]
pub fn client_get_manufacturer_by_oui(mac_address: &str) -> Result<String, String> {
    let ouis_conn = Connection::open("ouis.db").map_err(|e| e.to_string())?;

    let oui = mac_address.replace(":", "").to_lowercase()[..6].to_string();
    let oui_upper = oui.to_uppercase();

    let mut stmt = ouis_conn
        .prepare("SELECT manufacturer, country FROM manufacturers WHERE oui = ?1")
        .map_err(|e| e.to_string())?;
    let mut rows = stmt.query(params![oui_upper]).map_err(|e| e.to_string())?;

    let response = if let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let manufacturer: String = row.get(0).unwrap_or_else(|_| "Unknown".to_string());
        let country: String = row.get(1).unwrap_or_else(|_| "Unknown".to_string());

        OuiResponse {
            manufacturer,
            country,
        }
    } else {
        OuiResponse {
            manufacturer: "Unknown".to_string(),
            country: "Unknown".to_string(),
        }
    };

    serde_json::to_string(&response).map_err(|e| e.to_string())
}
