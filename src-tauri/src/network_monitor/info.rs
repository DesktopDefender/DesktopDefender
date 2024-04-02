use std::fmt;

use ipinfo::{BatchReqOpts, IpInfo, IpInfoConfig};
use serde::{de::Error, Deserialize, Serialize};
use tauri::Window;
use tokio::time::{sleep, Duration};

use crate::{IP_CACHE, IP_SET};

#[derive(Serialize, Deserialize)]
pub struct Info {
    country: String,
    flag: String,
    hostname: String,
    ip: String,
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Country: {}, Flag: {}, Hostname: {}, IP: {}",
            self.country, self.flag, self.hostname, self.ip
        )
    }
}

#[tauri::command]
pub fn init_info_emitter(window: Window) {
    tauri::async_runtime::spawn(async move {
        loop {
            match ip_lookup().await {
                Ok(info_json) => window
    
                    .emit("info", &info_json)
                    .expect("Failed to emit info"),
                Err(e) => eprintln!("Error getting ip info {}", e),
            }
            sleep(Duration::from_secs(10)).await;
        }
    });
}

fn setup_config() -> IpInfo {
    let config = IpInfoConfig {
        token: Some("ed3f54e75bfc39".to_string()),
        ..Default::default()
    };

    let ipinfo = IpInfo::new(config).expect("should construct");

    return ipinfo;
}

async fn ip_lookup() -> Result<String, serde_json::Error> {
    let mut ip_set = IP_SET.lock().await;
    let mut ip_cache = IP_CACHE.lock().await;

    let ip_strings: Vec<String> = ip_set.clone().iter().map(|ip| ip.to_string()).collect();
    let ip_str_slices: Vec<&str> = ip_strings.iter().map(AsRef::as_ref).collect();

    let mut ip_info = setup_config();

    let res = ip_info
        .lookup_batch(&ip_str_slices, BatchReqOpts::default())
        .await;
    match res {
        Ok(r) => {
            for (key, val) in r.iter() {
                let info = Info {
                    ip: key.to_string(),
                    hostname: val
                        .hostname
                        .as_ref()
                        .map(|s| s.clone())
                        .unwrap_or_else(|| "Unknown".to_string()),
                    country: val
                        .country_name
                        .as_ref()
                        .map(|s| s.clone())
                        .unwrap_or_else(|| "Unknown".to_string()),
                    flag: val
                        .country_flag
                        .as_ref()
                        .map(|cf| cf.emoji.clone())
                        .unwrap_or("⛳".to_string()),
                };
                ip_cache.insert(key.to_string(), info);
            }
            ip_set.clear();
            return serde_json::to_string(&*ip_cache);
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(serde_json::Error::custom("IP lookup failed"))
        }
    }
}
