// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
// use serde_json::{json, Value};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use dirs;
use sanitize_filename;
use uuid::Uuid;
//use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

#[derive(Serialize, Deserialize, Clone)]
struct DocumentData {
    id: String,  // Add this line
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Tab {
    order: u64,
    id: String,
    title: String
}

//static RECENT_FILES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
static TABS: Lazy<Mutex<Vec<Tab>>> = Lazy::new(|| Mutex::new(Vec::new()));
static TOTAL_TABS: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));


fn get_documents_dir() -> PathBuf {
    let mut path = dirs::document_dir().expect("Could not find Documents directory");
    path.push("FextifyPlus");
    
    // Create the directory if it doesn't exist
    fs::create_dir_all(&path).expect("Could not create FextifyPlus directory");
    
    path
}

#[tauri::command]
fn new_tab() -> Result<Tab, String> {
    // Lock TOTAL_TABS to update the total count
    let mut total_tabs = TOTAL_TABS.lock().map_err(|e| format!("Failed to lock TOTAL_TABS: {}", e))?;
    *total_tabs += 1;

    // Lock TABS to add a new tab
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;

    // Generate a new unique ID using UUID
    let new_id = Uuid::new_v4().to_string();

    // Create a new tab
    let new_tab = Tab {
        order: *total_tabs,
        id: new_id.clone(),
        title: String::new(),
    };

    // Add the tab to TABS
    tabs.push(new_tab.clone());

    Ok(new_tab)
}

#[tauri::command]
fn save_document(id: String, title: String, content: String) -> Result<String, String> {
    let documents_dir = get_documents_dir();
    
    // Use title as filename, fallback to 'untitled' if empty
    let filename = if title.trim().is_empty() { 
        "untitled".to_string() 
    } else { 
        title.trim().to_string() 
    };
    
    // Ensure filename is valid and append .json
    let safe_filename = sanitize_filename::sanitize(&format!("{}.json", filename));
    let file_path = documents_dir.join(&safe_filename);
    
    let document_data = DocumentData {
        id,  // Keep the original ID
        title: title.clone(),
        content: content.clone(),
    };
    
    // Save file
    match serde_json::to_string_pretty(&document_data) {
        Ok(json_content) => {
            // First, check if a file with this ID already exists
            let existing_files: Vec<PathBuf> = fs::read_dir(&documents_dir)
                .map_err(|e| format!("Failed to read directory: {}", e))?
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| {
                    let path = entry.path();
                    // Try to read the file content to extract the ID
                    fs::read_to_string(&path)
                        .ok()
                        .and_then(|content| 
                            serde_json::from_str::<DocumentData>(&content)
                                .ok()
                                .filter(|doc| doc.id == document_data.id)
                                .map(|_| path)
                        )
                })
                .collect();
            
            // Remove any existing files with the same ID
            for old_file in existing_files {
                fs::remove_file(&old_file).map_err(|e| format!("Failed to remove old file: {}", e))?;
            }
            
            // Write the new file
            match fs::write(&file_path, json_content) {
                Ok(_) => Ok(file_path.to_string_lossy().to_string()),
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
fn delete_document(id: String) -> Result<(), String> {
    let documents_dir = get_documents_dir();
    
    // Find and delete all files with the matching ID
    let files_to_delete: Vec<PathBuf> = fs::read_dir(&documents_dir)
        .map_err(|e| format!("Failed to read directory: {}", e))?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            // Try to read the file content to extract the ID
            fs::read_to_string(&path)
                .ok()
                .and_then(|content| 
                    serde_json::from_str::<DocumentData>(&content)
                        .ok()
                        .filter(|doc| doc.id == id)
                        .map(|_| path)
                )
        })
        .collect();
    
    // Delete all matching files
    for file_path in files_to_delete {
        fs::remove_file(&file_path)
            .map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e))?;
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_document,
            load_recent_files,
            delete_document,
            new_tab
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}