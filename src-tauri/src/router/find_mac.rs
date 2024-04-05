use std::process::Command;

fn get_nth_value(input: &str, n: usize) -> Option<&str> {
    input.split_whitespace().nth(n)
}

#[tauri::command]
pub fn find_mac_address(ip: &str) -> String {
    match find_mac_from_arp(ip) {
        Ok(o) => return o,
        Err(e) => return e.to_string(),
    }
}

fn find_mac_from_arp(ip: &str) -> Result<String, String> {
    let cmd = Command::new("arp").arg("-n").arg(ip).output();

    match cmd {
        Ok(o) if o.status.success() => {
            let output = String::from_utf8_lossy(&o.stdout).to_string();
            let mac = get_nth_value(output.as_str(), 3);

            match mac {
                Some(value) => Ok(value.to_string()),
                None => Err("Error, unexpected arp output".to_string()),
            }
        }
        Ok(o) => Err(String::from_utf8_lossy(&o.stderr).trim().to_string()),
        Err(e) => Err(e.to_string()),
    }
}
