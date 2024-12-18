// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use serde_json::{json, Value};
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[derive(Serialize, Deserialize)]
struct JsonData {
    children: Vec<Child>,
}

#[derive(Serialize, Deserialize)]
struct Child {
    id: String,
    path: String,
}

static PATH: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new("C:/Users/User/Documents/GitHub/text-editor/Folder".to_string())
});

#[tauri::command]
fn save_file(path: String, content: String) -> Result<(), String> {
    // Lock the Mutex and clone the string
    let base_path = PATH.lock().map_err(|_| "Failed to lock PATH Mutex")?.clone();
    
    // Create the full path
    let full_path = PathBuf::from(base_path).join(path);
    
    // Write the content to the file
    fs::write(&full_path, &content)
        .map_err(|e| format!("Failed to write to file: {}", e))?;
    
    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            save_file
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
