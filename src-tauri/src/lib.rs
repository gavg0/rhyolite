// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
// use serde_json::{json, Value};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use dirs;
use sanitize_filename;

#[derive(Serialize, Deserialize, Clone)]
struct DocumentData {
    title: String,
    content: String,
}

static RECENT_FILES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn get_documents_dir() -> PathBuf {
    let mut path = dirs::document_dir().expect("Could not find Documents directory");
    path.push("FextifyPlus");
    
    // Create the directory if it doesn't exist
    fs::create_dir_all(&path).expect("Could not create FextifyPlus directory");
    
    path
}

#[tauri::command]
fn save_document(title: String, content: String) -> Result<String, String> {
    let documents_dir = get_documents_dir();
    
    // Use title as filename, or 'untitled' if empty
    let filename = if title.trim().is_empty() { 
        "untitled".to_string() 
    } else { 
        title.trim().to_string() 
    };
    
    // Ensure filename is valid
    let safe_filename = sanitize_filename::sanitize(&format!("{}.json", filename));
    let file_path = documents_dir.join(safe_filename);
    
    let document_data = DocumentData {
        title: title.clone(),
        content: content.clone(),
    };
    
    // Save file
    match serde_json::to_string_pretty(&document_data) {
        Ok(json_content) => {
            match fs::write(&file_path, json_content) {
                Ok(_) => {
                    // Update recent files
                    let mut recent_files = RECENT_FILES.lock().unwrap();
                    if !recent_files.contains(&file_path.to_string_lossy().to_string()) {
                        recent_files.push(file_path.to_string_lossy().to_string());
                    }
                    Ok(file_path.to_string_lossy().to_string())
                },
                Err(e) => Err(format!("Failed to write file: {}", e))
            }
        },
        Err(e) => Err(format!("Failed to serialize document: {}", e))
    }
}

#[tauri::command]
fn load_recent_files() -> Result<Vec<DocumentData>, String> {
    let documents_dir = get_documents_dir();
    
    // Read all JSON files in the directory
    let files = match fs::read_dir(&documents_dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension()
                    .map_or(false, |ext| ext == "json")
            })
            .filter_map(|entry| {
                let path = entry.path();
                fs::read_to_string(&path)
                    .ok()
                    .and_then(|content| serde_json::from_str::<DocumentData>(&content).ok())
            })
            .collect(),
        Err(e) => return Err(format!("Failed to read directory: {}", e))
    };
    
    Ok(files)
}

#[tauri::command]
fn delete_document(filename: String) -> Result<(), String> {
    let documents_dir = get_documents_dir();
    let file_path = documents_dir.join(filename);
    
    match fs::remove_file(file_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to delete file: {}", e))
    }
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_document,
            load_recent_files,
            delete_document
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
