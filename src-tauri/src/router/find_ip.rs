#[tauri::command]
pub fn find_ip() -> String {
    "10.0.0.1".to_string()
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    println!("called in rust");
    return format!("{}!", name);
}
