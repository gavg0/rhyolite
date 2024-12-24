use uuid::Uuid;
use super::TABS; 
use super::TOTAL_TABS;
use super::CURRENT_OPEN_TAB;
use super:: Tab;

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
    Ok(tabs.clone())
}

#[tauri::command]
pub fn new_tab() -> Result<Tab, String> {
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
pub fn load_tab(id_in: String, title: String) -> Result<Tab, String> {
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
pub fn reset_tab_order_count() -> Result<(), String> {
    let mut total_tabs = TOTAL_TABS.lock().map_err(|e| format!("Failed to lock TOTAL_TABS: {}", e))?;
    *total_tabs = 0;
    Ok(())
}

#[tauri::command]
pub fn reorder_tabs() -> Result<Vec<Tab>, String> {
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    
    // Create new tabs with sequential ordering starting from 1
    let ordered_tabs: Vec<Tab> = tabs.iter()
        .enumerate()
        .map(|(index, tab)| Tab {
            order: (index + 1) as u64,
            id: tab.id.clone(),
            title: tab.title.clone()
        })
        .collect();
    
    // Update the stored tabs
    *tabs = ordered_tabs.clone();
    
    // Update TOTAL_TABS to match the actual number of tabs
    let mut total_tabs = TOTAL_TABS.lock().map_err(|e| format!("Failed to lock TOTAL_TABS: {}", e))?;
    *total_tabs = tabs.len() as u64;
    
    Ok(ordered_tabs)
}