// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tauri::WindowEvent;
mod editor;

///DocumentData struct, datatype that stores id, title and content of the document.
#[derive(Serialize, Deserialize, Clone)]
pub struct DocumentData {
    id: String,  
    title: String,
    content: String,
}

///Tab struct, used to store order(index of the tab), id of the document and title of the document.
#[derive(Serialize, Deserialize, Clone)]
pub struct Tab {
    id: String,
    title: String
}

///Userdata Struct, used to store the userdata, like last open tab and all the open tabs.
#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
    tabs: Vec<Tab>,  
    last_open_tab: String,
    recent_files: Vec<String>
}

//Mutex Variable declarations:-
///A Vector data type to store all the tabs in an assending order(depending upon the order value of the Tab):
pub static TABS: Lazy<Mutex<IndexMap<String, Tab>>> = Lazy::new(|| Mutex::new(IndexMap::new()));
///A String that stores the id of the current open tab in the editor:
pub static CURRENT_OPEN_TAB: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(("").to_string()));
pub static RECENT_FILES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

//Main tauri function.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_window_event(|window, event| {

            if let WindowEvent::CloseRequested { .. } = event {
                // Call the function to save UserData when the app is closing
                editor::io::on_app_close();

                // Prevent the window from closing immediately
                window.close().unwrap();
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            editor::io::save_document,
            editor::io::load_recent_files,
            editor::io::delete_document,
            editor::io::get_document_content,
            editor::tabs::new_tab,
            editor::tabs::load_tab,
            editor::tabs::delete_tab,
            editor::tabs::get_tabs,
            editor::tabs::send_current_open_tab,
            editor::tabs::get_current_open_tab,
            editor::tabs::update_tab_title,
            editor::tabs::cycle_tabs,
            editor::tabs::close_tab
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}