use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

use crate::config::config::{NETWORK_DB_PATH, OUIS_DB_PATH};
use csv::ReaderBuilder;
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
}

pub fn setup_network_db() {
    println!("creating network.db");
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
            PRIMARY KEY (mac_address, router_mac),
            FOREIGN KEY (router_mac) REFERENCES networks(router_mac)
        )",
        [],
    )
    .expect("Failed to create devices table");
}

pub fn setup_ouis_db() -> Result<(), Box<dyn Error>> {
    println!("creating ouis.db");

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
        .has_headers(false) // Set to true if your CSV file has headers
        .from_reader(BufReader::new(file));

    let tx = conn.transaction()?;
    for result in rdr.records() {
        let record = result?;
        // The CSV crate already handles the removal of quotes if the fields are quoted.
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
