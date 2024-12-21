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
use tauri::WindowEvent;
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

#[derive(Serialize, Deserialize, Clone)]
struct UserData {
    tabs: Vec<Tab>,  // Store complete Tab structs instead of just IDs
}

// static RECENT_FILES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
static TABS: Lazy<Mutex<Vec<Tab>>> = Lazy::new(|| Mutex::new(Vec::new()));
static TOTAL_TABS: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

fn on_app_close() {
    // Save the complete tabs information
    let tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e)).unwrap();
    let user_data = UserData { tabs: tabs.clone() };

    let appdata_dir = get_documents_dir().join("appdata");
    fs::create_dir_all(&appdata_dir).expect("Could not create appdata directory");
    let userdata_path = appdata_dir.join("userdata.json");

    match serde_json::to_string_pretty(&user_data) {
        Ok(json_content) => {
            if let Err(e) = fs::write(userdata_path, json_content) {
                eprintln!("Failed to save userdata: {}", e);
            }
        },
        Err(e) => eprintln!("Failed to serialize userdata: {}", e),
    }
}

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
        title: ("Untitled").to_string(),
    };

    // Add the tab to TABS
    tabs.push(new_tab.clone());

    Ok(new_tab)
}

#[tauri::command]
fn load_tab(id_in: String, title: String) -> Result<Tab, String> {
    // Lock TOTAL_TABS to update the total count
    let mut total_tabs = TOTAL_TABS.lock().map_err(|e| format!("Failed to lock TOTAL_TABS: {}", e))?;
    *total_tabs += 1;

    // Lock TABS to add a new tab
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;

    // Create a new tab
    let new_tab = Tab {
        order: *total_tabs,
        id: id_in,
        title: title,
    };

    // Add the tab to TABS
    tabs.push(new_tab.clone());

    Ok(new_tab)
}

#[tauri::command]
fn reset_tab_order_count() -> Result<(), String> {
    let mut total_tabs = TOTAL_TABS.lock().map_err(|e| format!("Failed to lock TOTAL_TABS: {}", e))?;
    *total_tabs = 0;
    Ok(())
}

#[tauri::command]
fn reorder_tabs() -> Result<Vec<Tab>, String> {
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    let mut ordered_tabs = Vec::new();
    
    // Create new tabs with updated order
    for (index, tab) in tabs.iter().enumerate() {
        ordered_tabs.push(Tab {
            order: (index + 1) as u64,
            id: tab.id.clone(),
            title: tab.title.clone()
        });
    }
    
    // Replace old tabs with reordered tabs
    *tabs = ordered_tabs.clone();
    Ok(ordered_tabs)
}

#[tauri::command]
fn get_document_content(id: String) -> Result<Option<DocumentData>, String> {
    let documents_dir = get_documents_dir();
    
    // Try to find the file with the name matching the ID
    let file_path = documents_dir.join(format!("{}.json", id));
    
    // Check if the file exists
    if !file_path.exists() {
        return Ok(None);  // Return None if no file is found
    }

    // Read the content of the file
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            match serde_json::from_str::<DocumentData>(&content) {
                Ok(doc) => Ok(Some(doc)),
                Err(e) => Err(format!("Failed to parse JSON from file: {}", e)),
            }
        },
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}

#[tauri::command]
fn save_document(id: String, title: String, content: String) -> Result<String, String> {
    let documents_dir = get_documents_dir();
    
    // Use the id as the filename
    let safe_filename = sanitize_filename::sanitize(&format!("{}.json", id));
    let file_path = documents_dir.join(&safe_filename);
    
    let document_data = DocumentData {
        id: id.clone(),  // Keep the original ID
        title: title.clone(),
        content: content.clone(),
    };
    
    // Serialize the document data
    match serde_json::to_string_pretty(&document_data) {
        Ok(json_content) => {
            // If a file with the same ID already exists, we can simply overwrite it
            // Write the new file
            match fs::write(&file_path, json_content) {
                Ok(_) => Ok(file_path.to_string_lossy().to_string()),
                Err(e) => Err(format!("Failed to write file: {}", e)),
            }
        },
        Err(e) => Err(format!("Failed to serialize document: {}", e)),
    }
}


#[tauri::command]
fn load_recent_files() -> Result<Vec<DocumentData>, String> {
    let appdata_dir = get_documents_dir().join("appdata");
    let userdata_path = appdata_dir.join("userdata.json");

    // Check if userdata.json exists
    if userdata_path.exists() {
        // Read and deserialize the UserData
        match fs::read_to_string(&userdata_path) {
            Ok(content) => {
                match serde_json::from_str::<UserData>(&content) {
                    Ok(user_data) => {
                        let mut recent_files = Vec::new();
                        
                        // Sort tabs by order
                        let mut tabs = user_data.tabs;
                        tabs.sort_by_key(|tab| tab.order);
                        
                        // Load tabs in the correct order
                        for tab in tabs {
                            // Try to load each document by ID
                            match get_document_content(tab.id.clone()) {
                                Ok(Some(doc)) => recent_files.push(doc),
                                _ => continue,
                            }
                        }
                        
                        return Ok(recent_files);
                    },
                    Err(e) => return Err(format!("Failed to deserialize userdata: {}", e)),
                }
            },
            Err(e) => return Err(format!("Failed to read userdata file: {}", e)),
        }
    }

    // If userdata.json doesn't exist, load all available documents
    let documents_dir = get_documents_dir();
    
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
        Err(e) => return Err(format!("Failed to read directory: {}", e)),
    };

    Ok(files)
}

#[tauri::command]
fn delete_document(id: String) -> Result<(), String> {
    let documents_dir = get_documents_dir();
    let filename = sanitize_filename::sanitize(&format!("{}.json", id));
    let file_path = documents_dir.join(&filename);
    
    // Remove the tab from TABS
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    tabs.retain(|tab| tab.id != id);
    
    // Check if the file exists and delete it
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e))?;
        Ok(())
    } else {
        Err(format!("File with ID {} not found", id))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_window_event(|window, event| {
            match event {
                WindowEvent::CloseRequested { .. } => {
                    // Call the function to save UserData when the app is closing
                    on_app_close();

                    // Prevent the window from closing immediately
                    window.close().unwrap();
                }
                _ => {}
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_document,
            load_recent_files,
            delete_document,
            new_tab,
            load_tab,
            get_document_content,
            reset_tab_order_count,
            reorder_tabs
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}