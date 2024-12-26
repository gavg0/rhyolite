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
    // Convert HashMap values to Vec for frontend
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
    
    // Insert into HashMap
    tabs.insert(new_id.clone(), new_tab.clone());
    
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
pub fn load_tab(id_in: String, title: String) -> Result<Tab, String> {
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    
    let new_tab = Tab {
        id: id_in.clone(),
        title: title,
    };
    
    tabs.insert(id_in, new_tab.clone());
    
    Ok(new_tab)
}



#[tauri::command]
pub fn delete_tab(id: String) -> Result<(), String> {
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    tabs.remove(&id);
    Ok(())
}