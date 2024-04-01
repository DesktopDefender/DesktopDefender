use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::process::Command;
use tauri::AppHandle;

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
    let mut stmt = conn.prepare("SELECT manufacturer FROM manufacturers WHERE oui = ?1")?;
    let mut rows = stmt.query(params![oui])?;

    if let Some(row) = rows.next()? {
        Ok(row.get(0)?)
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}
