//! This module provides IO related functions for the app.
use std::fs;
use std::path::PathBuf;

use dirs;
use sanitize_filename;


use crate::TABS;
use crate:: UserData;
use crate:: DocumentData;
use crate::CURRENT_OPEN_TAB;


/// This function finds the path to the 'documents'
/// directory for different 'os' and returns the PathBuf(a mutable path string)
/// 
/// First we define a mutable variable path of datatype PathBuf,
/// then we store the path in the variable, returned by the document_dir function that
/// finds the path of the documents dir.
/// 
/// Then we append the dir 'Rhyolite' to the documents path.
/// If this newly created path directory does not exist then create it using create_dir_all
/// function.
/// 
/// Then return the variable path, that holds the path to the FextifyPlus directory.
pub fn get_documents_dir() -> PathBuf {
    let mut path = dirs::document_dir().expect("Could not find Documents directory");
    path.push("Rhyolite");
    
    // Create the directory if it doesn't exist
    fs::create_dir_all(&path).expect("Could not create FextifyPlus directory");
    
    path
}

pub fn on_app_close() {
    // Save the complete tabs information
    let tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e)).unwrap();
    let current_open_tab = CURRENT_OPEN_TAB.lock().map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e)).unwrap();
    
    // Convert HashMap values to Vec for storage
    let tabs_vec: Vec<_> = tabs.values().cloned().collect();
    let user_data = UserData { 
        tabs: tabs_vec, 
        last_open_tab: current_open_tab.clone() 
    };

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

#[tauri::command]
pub fn save_document(id: String, title: String, content: String) -> Result<String, String> {
    let documents_dir = get_documents_dir();
    
    // Use the id as the filename
    let safe_filename = sanitize_filename::sanitize(format!("{}.json", id));
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
pub fn delete_document(id: String) -> Result<(), String> {
    let documents_dir = get_documents_dir();
    let filename = sanitize_filename::sanitize(format!("{}.json", id));
    let file_path = documents_dir.join(&filename);
    
    // Remove the tab from TABS HashMap
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    tabs.remove(&id);
    
    // Delete the file if it exists
    if file_path.exists() {
        fs::remove_file(&file_path)
            .map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e))?;
    }
    
    Ok(())
}

#[tauri::command]
pub fn get_document_content(id: String) -> Result<Option<DocumentData>, String> {
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
pub fn load_recent_files() -> Result<Vec<DocumentData>, String> {
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
                        let mut current_open_tab = CURRENT_OPEN_TAB.lock()
                            .map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e))?;
                        *current_open_tab = user_data.last_open_tab.clone();
                        
                        let mut tabs = TABS.lock()
                            .map_err(|e| format!("Failed to lock TABS: {}", e))?;
                        
                        // Clear existing tabs and load from user_data
                        tabs.clear();
                        for tab in user_data.tabs {
                            // Try to load each document by ID
                            match get_document_content(tab.id.clone()) {
                                Ok(Some(doc)) => {
                                    recent_files.push(doc);
                                    tabs.insert(tab.id.clone(), tab.clone());
                                }
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