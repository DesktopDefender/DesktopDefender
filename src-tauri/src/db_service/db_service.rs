use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::process::Command;
use tauri::AppHandle;

use crate::config::config::OUIS_DB_PATH;

pub fn get_connection() -> Result<Connection, Box<dyn Error>> {
    if let Some(path) = OUIS_DB_PATH.get() {
        Connection::open(path).map_err(|e| e.into())
    } else {
        println!("Database path is not set yet.");
        Err("Database path is not set yet.".into())
    }
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

pub fn get_manufacturer_by_oui(conn: &Connection, oui: &str) -> Result<String> {
    let oui_upper = oui.to_uppercase();
    
    let mut stmt = conn.prepare("SELECT manufacturer FROM manufacturers WHERE oui = ?1")?;
    let mut rows = stmt.query(params![oui_upper])?;

    if let Some(row) = rows.next()? {
        Ok(row.get(0)?)
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}
