//! This module provides document tabs related functions for the app.
use uuid::Uuid;

use crate::TABS; 

use crate::CURRENT_OPEN_TAB;
use crate:: Tab;

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
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    
    // Generate a new unique ID
    let new_id = Uuid::new_v4().to_string();
    
    // Create new tab
    let new_tab = Tab {
        id: new_id.clone(),
        title: "Untitled".to_string(),
    };
    
    // Insert into IndexMap
    tabs.insert(new_id.clone(), new_tab.clone());
    
    // Update current open tab
    let mut current_open_tab = CURRENT_OPEN_TAB.lock()
        .map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e))?;
    *current_open_tab = new_id;
    
    Ok(new_tab)
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

// #[tauri::command]
// pub fn close_tab()

print!("Hello World");