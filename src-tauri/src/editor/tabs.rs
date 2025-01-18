//! This module provides document tabs related functions for the app.
use uuid::Uuid;

use crate::TABS; 

use crate::CURRENT_OPEN_TAB;
use crate::RECENT_FILES;
use crate:: Tab;
use crate::RecentFileInfo;
use crate::IO_OPS;
use std::path::PathBuf; 

use super::io::{save_user_data, get_trove_dir, save_document};

#[tauri::command]
pub fn send_current_open_tab(id: String) {
    let mut current_open_tab = CURRENT_OPEN_TAB.lock().map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e)).unwrap();
    *current_open_tab = id.clone();
}

#[tauri::command]
pub fn get_current_open_tab() -> Result<String, String> {
    let current_open_tab = CURRENT_OPEN_TAB.lock().map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e)).unwrap();
    Ok(current_open_tab.clone())
}

#[tauri::command]
pub fn get_tabs() -> Result<Vec<Tab>, String> {
    let tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    // Convert IndexMap values to Vec, maintaining order
    Ok(tabs.values().cloned().collect())
}


#[tauri::command]
pub fn new_tab() -> Result<Tab, String> {
    // Try to acquire IO_OPS lock first
    if let Ok(_io_ops_unlock) = IO_OPS.try_lock() {
        // If we get the lock, proceed with the rest of the function
        let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
        let mut recent_files = RECENT_FILES.lock().map_err(|e| format!("Failed to lock RECENT_FILES: {}", e))?;
		let mut current_open_tab = CURRENT_OPEN_TAB.lock().map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e))?;
        
        // Generate a new unique ID
        let new_id = Uuid::new_v4().to_string();
        
        let trove_dir = get_trove_dir("Untitled_Trove");
        let title = check_path_exists(&trove_dir, 0);
        
        // Create new tab
        let new_tab = Tab {
            id: new_id.clone(),
            title: title.clone(),
        };
        
        // Insert into IndexMap
        tabs.insert(new_id.clone(), new_tab.clone());
        
        // Add to recent files
        recent_files.push(RecentFileInfo {
            id: new_id.clone(),
            title: "Untitled".to_string(),
        });
        
        // Update current open tab

        *current_open_tab = new_id.clone();
        std::mem::drop(current_open_tab);
        std::mem::drop(tabs);
        std::mem::drop(recent_files);
        
        // Save changes to userdata.json
        save_user_data()?;
        let _ = save_document(new_id, title, String::new());
        
        Ok(new_tab)
    } else {
        // If we can't get the lock, return an error
        Err("IO operations in progress. Please try again.".to_string())
    }
}

fn check_path_exists(trove_dir: &PathBuf, iter: u32) -> String {
    let title = if iter == 0 {
        sanitize_filename::sanitize("Untitled.md")
    } else {
        sanitize_filename::sanitize(format!("Untitled {}.md", iter))
    };
    
    let file_path = trove_dir.join(&title);
    if !file_path.exists() {
        title.strip_suffix(".md").unwrap_or(&title).to_string()
    } else {
        check_path_exists(trove_dir, iter + 1)
    }
}

#[tauri::command]
pub fn update_tab_title(id: String, title: String) -> Result<Tab, String> {
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    
    // Get the tab, update its title, and insert it back
    if let Some(mut tab) = tabs.get(&id).cloned() {
        tab.title = title;
        tabs.insert(id, tab.clone());
        Ok(tab)
    } else {
        Err("Tab not found".to_string())
    }
}

#[tauri::command]
pub fn load_tab(id: String, title: String) -> Result<Tab, String> {
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    
    let new_tab = Tab {
        id: id.clone(),
        title
    };
    
    tabs.insert(id, new_tab.clone());
    
    Ok(new_tab)
}

#[tauri::command]
pub fn cycle_tabs() -> Result<String, String> {
    let tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    let mut current_open_tab = CURRENT_OPEN_TAB.lock()
        .map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e))?;
    
    if tabs.is_empty() {
        return Err("No tabs available".to_string());
    }
    
    // Find current index
    if let Some(current_index) = tabs.get_full(&*current_open_tab).map(|(i, _, _)| i) {
        // Calculate next index
        let next_index = (current_index + 1) % tabs.len();
        
        // Get the ID of the next tab
        if let Some((next_id, _)) = tabs.get_index(next_index) {
            *current_open_tab = next_id.clone();
            return Ok(next_id.clone());
        }
    }
    
    Err("Failed to cycle tabs".to_string())
}

#[tauri::command]
pub fn delete_tab(id: String) -> Result<(), String> {
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    tabs.shift_remove(&id);
    Ok(())
}

#[tauri::command]
pub fn close_tab(id: String) -> Result<Option<String>, String> {
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    
    if tabs.len() <= 1 {
        return Ok(None); // Don't close the last tab
    }
    
    if let Some((index, _, _)) = tabs.shift_remove_full(&id) {
        // Get the next tab ID (either at same index or last tab)
        let next_tab_id = tabs.get_index(index)
            .or_else(|| tabs.last())
            .map(|(id, _)| id.clone());
            
        // Update current open tab if needed
        if let Some(next_id) = &next_tab_id {
            let mut current_open_tab = CURRENT_OPEN_TAB.lock()
                .map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e))?;
            *current_open_tab = next_id.clone();
        }
        
        Ok(next_tab_id)
    } else {
        Err("Tab not found".to_string())
    }
}