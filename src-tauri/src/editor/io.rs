//! This module provides IO related functions for the app.
use std::fs; //Filesystem module
use std::path::PathBuf; //PathBuf datatype to store path strings
use uuid::Uuid; //Uuid module to generate unique ids
// use tauri_plugin_dialog::DialogExt; //DialogExt trait to show dialog boxes

use dirs; //dirs module to get the path of the documents directory
use sanitize_filename; //sanitize_filename module to sanitize filenames

use crate::editor::markdown_handler;

use crate::{DocumentData, RecentFileInfo, UserData}; //Importing the DocumentData, RecentFileInfo and UserData structs
use crate::{CURRENT_OPEN_TAB, RECENT_FILES, TABS}; //Importing the CURRENT_OPEN_TAB, RECENT_FILES and TABS mutexes

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
/// Then return the variable path, that holds the path to the Rhyolite directory.
pub fn get_documents_dir() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        // On Android, use the app's private storage directory
        let path = PathBuf::from("/data/user/0/com.rhyolite.dev/Rhyolite");
        // Create the directory if it doesn't exist
        fs::create_dir_all(&path).expect("Could not create Rhyolite directory");
        path
    }

    #[cfg(not(target_os = "android"))]
    {
        // Original desktop behavior
        let mut path = dirs::document_dir().expect("Could not find Documents directory");
        path.push("Rhyolite");
        // Create the directory if it doesn't exist
        fs::create_dir_all(&path).expect("Could not create Rhyolite directory");
        path
    }
}

/// Return the path to the default Rhyolite Trove directory.
/// The function takes in the name that the default trove directory will have
/// and then creates a directory at 'documents/Rhyolite/trove_name' where trove_name is the
/// name of the default trove.
///
/// A trove is a folder that stores Rhyolite notes.
pub fn get_trove_dir(trove_name: &str) -> PathBuf {
    //Get the path to documents/Rhyolite.
    let documents_dir = get_documents_dir();

    //Append the default trove name to the 'documents/Rhyolite path'.
    let trove_dir = documents_dir.join(trove_name);

    //Then create the path 'documents/Rhyolite/trove_name' if it does not
    fs::create_dir_all(&trove_dir).expect("Could not create Trove directory");

    //retrun the path of the default trove directory.
    trove_dir
}

/// This function is called when the app is closing.
/// It saves the complete tabs information.
/// It locks the TABS, CURRENT_OPEN_TAB and RECENT_FILES mutexes and then
/// converts the IndexMap values to Vec for storage.
/// Then it creates a UserData struct and stores the tabs, last open tab and recent files in it.
/// Then it creates a directory 'appdata' in the documents directory and stores the userdata in a file
/// 'userdata.json' in the appdata directory.
/// If there is an error in saving the userdata, it prints the error.
pub fn on_app_close() {
    // Save the complete tabs information
    let tabs = TABS
        .lock()
        .map_err(|e| format!("Failed to lock TABS: {}", e))
        .unwrap();
    let current_open_tab = CURRENT_OPEN_TAB
        .lock()
        .map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e))
        .unwrap();
    let recent_files = RECENT_FILES
        .lock()
        .map_err(|e| format!("Failed to lock RECENT_FILES: {}", e))
        .unwrap();

    // Convert HashMap values to Vec for storage
    let tabs_vec: Vec<_> = tabs.values().cloned().collect();
    let user_data = UserData {
        tabs: tabs_vec,
        last_open_tab: current_open_tab.clone(),
        recent_files: recent_files.clone(),
    };

    let appdata_dir = get_documents_dir().join("appdata");
    fs::create_dir_all(&appdata_dir).expect("Could not create appdata directory");
    let userdata_path = appdata_dir.join("userdata.json");

    match serde_json::to_string_pretty(&user_data) {
        Ok(json_content) => {
            if let Err(e) = fs::write(userdata_path, json_content) {
                eprintln!("Failed to save userdata: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to serialize userdata: {}", e),
    }
}

/// This function saves the user data.
/// It locks the TABS, CURRENT_OPEN_TAB and RECENT_FILES mutexes and then
/// converts the IndexMap values to Vec for storage.
/// Then it creates a UserData struct and stores the tabs, last open tab and recent files in it.
/// Then it creates a directory 'appdata' in the documents directory and stores the userdata in a file
/// 'userdata.json' in the appdata directory.
/// If there is an error in saving the userdata, it returns the error.
/// If the userdata is saved successfully, it returns Ok(()).
pub fn save_user_data() -> Result<(), String> {
    let tabs = TABS
        .lock()
        .map_err(|e| format!("Failed to lock TABS: {}", e))?;
    let current_open_tab = CURRENT_OPEN_TAB
        .lock()
        .map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e))?;
    let recent_files = RECENT_FILES
        .lock()
        .map_err(|e| format!("Failed to lock RECENT_FILES: {}", e))?;

    // Convert HashMap values to Vec for storage
    let tabs_vec: Vec<_> = tabs.values().cloned().collect();
    let user_data = UserData {
        tabs: tabs_vec,
        last_open_tab: current_open_tab.clone(),
        recent_files: recent_files.clone(),
    };

    let appdata_dir = get_documents_dir().join("appdata");
    fs::create_dir_all(&appdata_dir).expect("Could not create appdata directory");
    let userdata_path = appdata_dir.join("userdata.json");

    match serde_json::to_string_pretty(&user_data) {
        Ok(json_content) => fs::write(userdata_path, json_content)
            .map_err(|e| format!("Failed to save userdata: {}", e)),
        Err(e) => Err(format!("Failed to serialize userdata: {}", e)),
    }
}

/// This function saves the document.
#[tauri::command]
pub fn save_document(id: String, title: String, content: String) -> Result<String, String> {
    let mut recent_files = RECENT_FILES
        .lock()
        .map_err(|e| format!("Failed to lock RECENT_FILES: {}", e))?;
    if let Some(doc) = recent_files.iter_mut().find(|doc| doc.id == id) {
        doc.title = title.clone();
    } else {
        recent_files.push(RecentFileInfo {
            id: id.clone(),
            title: title.clone(),
        });
    }
    // Create a vault directory within documents_dir
    let trove_dir = get_trove_dir("Untitled_Trove");

    // Convert HTML to Markdown
    let markdown_content = markdown_handler::html_to_markdown(&content);

    // Add title as heading
    // let full_markdown = format!("# {}\n\n{}", title, markdown_content);

    // Use .md extension instead of .json
    let safe_filename = sanitize_filename::sanitize(format!("{}.md", title));
    let file_path = trove_dir.join(&safe_filename);

    let tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;

    let old_title = tabs.get(&id).map(|tab| tab.title.clone()).unwrap_or_else(|| String::from("Untitled"));
    let old_path = trove_dir.join(sanitize_filename::sanitize(format!("{}.md", old_title)));

   
    if old_path != file_path && old_path.exists(){
        fs::remove_file(old_path)
            .map_err(|e| format!("Failed to delete old file: {}", e))?;
    }

    // Write markdown content directly to file
    match fs::write(&file_path, markdown_content) {
        Ok(_) => Ok(file_path.to_string_lossy().to_string()),
        Err(e) => Err(format!("Failed to write file: {}", e)),
    }
}

#[tauri::command]
pub fn delete_document(id: String) -> Result<Option<DocumentData>, String> {
    let mut tabs = TABS
        .lock()
        .map_err(|e| format!("Failed to lock TABS: {}", e))?;
    let mut recent_files = RECENT_FILES
        .lock()
        .map_err(|e| format!("Failed to lock RECENT_FILES: {}", e))?;
    let tab_title = tabs.get(&id)
        .map(|tab| tab.title.clone())
        .unwrap();
    recent_files.retain(|doc| doc.id != id);
    let trove_dir = get_trove_dir("Untitled_Trove");
    let filename = sanitize_filename::sanitize(format!("{}.md", tab_title));
    let file_path = trove_dir.join(&filename);

    // Remove the tab and get its index
    if let Some((index, _, _)) = tabs.shift_remove_full(&id) {
        // Get the tab at the same index (the one that shifted up)
        // If no tab at that index, get the last tab
        let next_tab = if let Some((next_id, next_tab)) = tabs.get_index(index).or_else(|| tabs.last()) {
            // Update current open tab
            let mut current_open_tab = CURRENT_OPEN_TAB
                .lock()
                .map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e))?;
            *current_open_tab = next_id.clone();

            // Get the document content for the next tab
            get_document_content(next_id.clone(), next_tab.title.clone())?
        } else {
            None
        };

        // Delete the file if it exists
        if file_path.exists() {
            fs::remove_file(&file_path)
                .map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e))?;
        }

        std::mem::drop(recent_files);
        std::mem::drop(tabs);
        // Save changes to userdata.json
        save_user_data()?;

        Ok(next_tab)
    } else {
        Err("Tab not found".to_string())
    }
}

#[tauri::command]
pub fn get_document_content(id: String, title: String) -> Result<Option<DocumentData>, String> {
    let trove_dir = get_trove_dir("Untitled_Trove");
    let file_path = trove_dir.join(format!("{}.md", title));

    if !file_path.exists() {
        return Ok(None);
    }

    match fs::read_to_string(&file_path) {
        Ok(content) => {
            let html_output = markdown_handler::markdown_to_html(&content);

            Ok(Some(DocumentData {
                id,
                title,
                content: html_output,
            }))
        }
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}

#[tauri::command]
pub fn load_last_open_tabs() -> Result<Vec<DocumentData>, String> {
    let appdata_dir = get_documents_dir().join("appdata");
    let userdata_path = appdata_dir.join("userdata.json");

    // Check if userdata.json exists
    if userdata_path.exists() {
        // Read and deserialize the UserData
        match fs::read_to_string(&userdata_path) {
            Ok(content) => {
                match serde_json::from_str::<UserData>(&content) {
                    Ok(user_data) => {
                        let mut recent_files_lock = RECENT_FILES
                            .lock()
                            .map_err(|e| format!("Failed to lock RECENT_FILES: {}", e))?;
                        *recent_files_lock = user_data.recent_files.clone();
                        let mut last_open_files = Vec::new();
                        let mut current_open_tab = CURRENT_OPEN_TAB
                            .lock()
                            .map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e))?;
                        *current_open_tab = user_data.last_open_tab.clone();

                        let mut tabs = TABS
                            .lock()
                            .map_err(|e| format!("Failed to lock TABS: {}", e))?;

                        // Clear existing tabs and load from user_data
                        tabs.clear();
                        for tab in user_data.tabs {
                            // Try to load each document by ID
                            match get_document_content(tab.id.clone(), tab.title.clone()) {
                                Ok(Some(doc)) => {
                                    last_open_files.push(doc);
                                    tabs.insert(tab.id.clone(), tab.clone());
                                }
                                _ => continue,
                            }
                        }

                        return Ok(last_open_files);
                    }
                    Err(e) => return Err(format!("Failed to deserialize userdata: {}", e)),
                }
            }
            Err(e) => return Err(format!("Failed to read userdata file: {}", e)),
        }
    }

    // If userdata.json doesn't exist, load all markdown files from the trove directory
    let trove_dir = get_trove_dir("Untitled_Trove");

    let files = match fs::read_dir(&trove_dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
            .filter_map(|entry| {
                let path = entry.path();
                let title = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(String::from)
                    .unwrap_or_default();

                let id = Uuid::new_v4().to_string();
                get_document_content(id, title).ok().flatten()
            })
            .collect(),
        Err(e) => return Err(format!("Failed to read directory: {}", e)),
    };

    Ok(files)
}

/// This function returns the metadata of the recent files.
#[tauri::command]
pub fn get_recent_files_metadata() -> Result<Vec<RecentFileInfo>, String> {
    if let Err(e) = save_user_data() {
        eprintln!("Warning: Failed to save user data: {}", e);
    }
    let appdata_dir = get_documents_dir().join("appdata");
    let userdata_path = appdata_dir.join("userdata.json");

    // Check if userdata.json exists
    if userdata_path.exists() {
        // Read and deserialize the UserData
        match fs::read_to_string(&userdata_path) {
            Ok(content) => match serde_json::from_str::<UserData>(&content) {
                Ok(user_data) => Ok(user_data.recent_files),
                Err(e) => Err(format!("Failed to deserialize userdata: {}", e)),
            },
            Err(e) => Err(format!("Failed to read userdata file: {}", e)),
        }
    } else {
        // If userdata.json doesn't exist, return empty vector
        Ok(Vec::new())
    }
}
