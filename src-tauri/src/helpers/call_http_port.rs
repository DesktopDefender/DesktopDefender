use regex::Regex;
use reqwest::{Client, Response};

#[tauri::command]
pub async fn call_http_port(host: &str, port: i32) -> Result<String, String> {
    let address = create_address_url(host, port);

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(&address)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // If we've arrived here without errors,
    // we have a http server, presumably the admin portal
    let mut text = response.text().await.map_err(|e| e.to_string())?;

    if got_redirect_response(text.as_str()) {
        let redirected = get_redirected_response(text.as_str(), client.clone(), address.as_str())
            .await
            .unwrap();
        text = redirected.text().await.map_err(|e| e.to_string())?;
    }

    let endpoints: Vec<String> = find_endpoints(host, port, text.as_str(), client.clone()).await;

    println!("Endpoints: (found {}) ", endpoints.len());
    for e in endpoints {
        println!(" - {}", e);
    }
    println!("");

    return Ok(text);
}

fn create_address_url(host: &str, port: i32) -> String {
    if port == 443 {
        return format!("https://{}:{}", host, port);
    } else {
        return format!("http://{}:{}", host, port);
    }
}

fn got_redirect_response(text: &str) -> bool {
    // Currently, to check whether a client-side redirect was returned
    // we simply check whether only a script was returned
    return text.starts_with("<script>") && text.ends_with("</script>");
}

async fn get_redirected_response(
    text: &str,
    client: Client,
    address: &str,
) -> Result<Response, String> {
    let url = capture_url_from_redirect_script(text);

    match url {
        None => {
            println!("No match found for {}", text);
            return Err("No redirect found".to_string());
        }
        Some(t) => {
            // redirect url found, lets go there
            let new_address = format!("{}{}", address, t);
            let response = client
                .get(&new_address)
                .send()
                .await
                .map_err(|e| e.to_string());
            return response;
        }
    }
}

fn capture_url_from_redirect_script(redirect_script: &str) -> Option<String> {
    // regex which finds the url content wrapped in single quotes
    let rx = Regex::new(r"url\s*=\s*'([^']+)'").unwrap();
    return rx
        .captures(redirect_script)
        .and_then(|caps| caps.get(1).map(|match_| match_.as_str().to_string()));
}

// Finds and returns a vector of the contents of script tags like these:
//  <script defer="defer" src="/main.1f13cbe8ee4a4f1a0848.js"></script>
// a returned string should follow this format:
// 'defer="defer" src="/main.1f13cbe8ee4a4f1a0848.js"'
fn find_script_tags(http: &str) -> Vec<String> {
    let rx = Regex::new(r"<script (.*?)></script>").unwrap();
    let matches = rx
        .captures_iter(http)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect();

    return matches;
}

fn find_script_src(script: &str) -> Option<String> {
    let rx = Regex::new("src\\s*=\\s*\"([^']+)\"").unwrap();
    return rx
        .captures(script)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));
}

fn find_endpoints_from_code(code: &str) -> Vec<String> {
    // find all possible endpoints within the javascript code
    let regex_pattern = r#""(/[a-zA-Z0-9\/]+)""#;
    let rx = Regex::new(regex_pattern).unwrap();
    let matches = rx
        .captures_iter(code)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect();
    return matches;
}

async fn find_endpoints(host: &str, port: i32, text: &str, client: Client) -> Vec<String> {
    let mut endpoints: Vec<String> = vec![];
    let scripts = find_script_tags(text);

    for script in scripts {
        println!("script: {script}");
        let src = find_script_src(script.as_str());

        // if a src was found
        if let Some(s) = src {
            let js_path = format!("{}{}", create_address_url(host, port), s);
            let js_response_result = client.get(&js_path).send().await;

            if let Ok(js_response) = js_response_result {
                if let Ok(js_code) = js_response.text().await {
                    let mut endpoints_found = find_endpoints_from_code(&js_code);
                    endpoints.append(&mut endpoints_found);
                }
            }
        }
    }
    return endpoints;
}
