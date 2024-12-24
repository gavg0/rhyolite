// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
// use std::fs;
// use std::path::PathBuf;
// use serde_json::{json, Value};
use std::sync::Mutex;
use once_cell::sync::Lazy;
// use dirs;
// use sanitize_filename;
// use uuid::Uuid;
use tauri::WindowEvent;
mod tabs;
mod io;
//use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

//A struct for DocumentData datatype that stores id, title and content of the document
#[derive(Serialize, Deserialize, Clone)]
pub struct DocumentData {
    id: String,  
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tab {
    order: u64,
    id: String,
    title: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
    tabs: Vec<Tab>,  // Store complete Tab structs instead of just IDs
    last_open_tab: String //store the id of the last open tab before the app closed.
}


pub static TABS: Lazy<Mutex<Vec<Tab>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static TOTAL_TABS: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));
pub static CURRENT_OPEN_TAB: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(("").to_string()));

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .on_window_event(|window, event| {
            match event {
                WindowEvent::CloseRequested { .. } => {
                    // Call the function to save UserData when the app is closing
                    io::on_app_close();

                    // Prevent the window from closing immediately
                    window.close().unwrap();
                }
                _ => {}
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            io::save_document,
            io::load_recent_files,
            io::delete_document,
            io::get_document_content,
            tabs::new_tab,
            tabs::load_tab,
            tabs::reset_tab_order_count,
            tabs::reorder_tabs,
            tabs::get_tabs,
            tabs::send_current_open_tab,
            tabs::get_current_open_tab
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}