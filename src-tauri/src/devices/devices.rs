use serde::Serialize;
use std::str;
use std::process::Command;


#[derive(Serialize)]
pub struct ArpEntry {
    pub ip_address: String,
    pub mac_address: String,
    pub hostname: String
}

impl Default for ArpEntry {
    fn default() -> Self {
        ArpEntry {
            ip_address: Default::default(),
            mac_address: Default::default(),
            hostname: "Unknown".to_string(),
        }
    }
}

#[tauri::command]
pub async fn get_hostname(ip_address: String) -> String {
    println!("CAllING get_hostname");

    let output = Command::new("dig")
        .args([
            "-x", &ip_address,
            "-p", "5353",
            "@224.0.0.251",
            "+short"
        ])
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
        },
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


#[tauri::command]
pub async fn get_devices() -> Vec<ArpEntry> {
    println!("CAllING get_devices");

    let output = Command::new("arp")
        .arg("-a")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
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
                Some(ArpEntry { ip_address, mac_address, ..Default::default() })
            } else {
                None
            }
        } else {
            None
        }
    }).collect()
}