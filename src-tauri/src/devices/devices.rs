use serde::Serialize;
use std::str;
use std::process::Command;

#[derive(Serialize)]
pub struct ArpEntry {
    pub ip_address: String,
    pub mac_address: String
}


#[tauri::command]
pub async fn get_devices() -> Vec<ArpEntry> {
    let output = Command::new("arp")
        .arg("-a")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        parse_arp_output(&stdout)
    } else {
        Vec::new()
    }
}

fn parse_arp_output(output: &str) -> Vec<ArpEntry> {
    output.lines().filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > 3 && parts[1].starts_with('(') && parts[1].ends_with(')') {
            let ip_address = parts[1].trim_matches('(').trim_matches(')').to_string();
            let mac_address = parts[3].to_string();
            if mac_address != "(incomplete)" {
                Some(ArpEntry { ip_address, mac_address })
            } else {
                None
            }
        } else {
            None
        }
    }).collect()
}