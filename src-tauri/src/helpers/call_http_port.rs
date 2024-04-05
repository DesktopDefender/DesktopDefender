use regex::Regex;
use reqwest::Client;

#[tauri::command]
pub async fn call_http_port(host: &str, port: i32) -> Result<String, String> {
    let address;
    if port == 443 {
        address = format!("https://{}:{}", host, port);
    } else {
        address = format!("http://{}:{}", host, port);
    }

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())?;

    let mut response = client
        .get(&address)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // If we've arrived here without errors,
    // we have a http server, presumably the admin portal
    let mut text = response.text().await.map_err(|e| e.to_string())?;

    if text.starts_with("<script>") && text.ends_with("</script>") {
        // likely, a client-side redirect was passed
        let url = capture_url_from_redirect_script(text.as_str());

        match url {
            None => {
                println!("No match found for {}", text);
                panic!("Don't know what to do");
            }
            Some(t) => {
                // redirect url found, lets go there
                println!("found url: {}", t);
                let new_address = format!("{}{}", address, t);
                response = client
                    .get(&new_address)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;
                text = response.text().await.map_err(|e| e.to_string())?;
            }
        }
    }

    println!("{}", text);

    return Ok(text);
}

fn capture_url_from_redirect_script(redirect_script: &str) -> Option<String> {
    // regex which finds the url content wrapped in single quotes
    let rx = Regex::new(r"url\s*=\s*'([^']+)'").unwrap();
    return rx
        .captures(redirect_script)
        .and_then(|caps| caps.get(1).map(|match_| match_.as_str().to_string()));
}
