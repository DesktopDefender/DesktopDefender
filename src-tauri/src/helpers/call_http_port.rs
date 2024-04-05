#[tauri::command]
pub async fn call_http_port(host: &str, port: i32) -> Result<String, String> {
    let address = format!("http://{}:{}", host, port);
    let response = reqwest::get(&address)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    return Ok(response);
}
