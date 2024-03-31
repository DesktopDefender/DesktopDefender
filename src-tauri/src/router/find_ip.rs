use std::net::Ipv4Addr;
use std::process::Command;

#[tauri::command]
pub fn find_ip() -> String {
    match get_netstat_ip() {
        Ok(output) => return output,
        Err(error) => return error.to_string(),
    }
}

fn get_netstat_ip() -> Result<String, String> {
    let cmd = Command::new("sh")
        .arg("-c")
        .arg("netstat -rn | grep default | head -n 1 | tr -s ' ' | cut -d ' ' -f 2")
        .output();

    match cmd {
        Ok(o) if o.status.success() => {
            let stdout = String::from_utf8_lossy(&o.stdout).trim().to_string();

            if is_valid_ipv4(stdout.as_str()) {
                Ok(stdout)
            } else {
                Err("No valid ipv4 found. Are you connected to the internet?".to_string())
            }
        }
        Ok(o) => Err(String::from_utf8_lossy(&o.stderr).trim().to_string()),
        Err(e) => Err(e.to_string()),
    }
}

fn is_valid_ipv4(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>().is_ok()
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    println!("called in rust");
    return format!("{}!", name);
}
