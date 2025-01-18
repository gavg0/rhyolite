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

/// This function returns the path to the documents directory.
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

/// This function returns the path to the default trove directory.
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

/// Runs when the app is closing and saves the user data.
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

/// This function saves the user data to the userdata.json file.
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

	// Append .md to the title and sanitize it
	let safe_filename = sanitize_filename::sanitize(format!("{}.md", title));
	let file_path = trove_dir.join(&safe_filename);

	// Get the current open tab
	let tabs = TABS
		.lock()
		.map_err(|e| format!("Failed to lock TABS: {}", e))?;

	// Get the old title and path(Old title is the title of the document before saving, if changed)
	let old_title = tabs
		.get(&id)
		.map(|tab| tab.title.clone())
		.unwrap_or_else(|| String::from("Untitled"));
	let old_path = trove_dir.join(sanitize_filename::sanitize(format!("{}.md", old_title)));

	// if the title has changed, delete the old file
	if old_path != file_path && old_path.exists() {
		fs::remove_file(old_path).map_err(|e| format!("Failed to delete old file: {}", e))?;
	}

	// Write markdown content directly to file
	match fs::write(&file_path, markdown_content) {
		Ok(_) => Ok(file_path.to_string_lossy().to_string()),
		Err(e) => Err(format!("Failed to write file: {}", e)),
	}
}

#[tauri::command]
pub fn delete_document(id: String) -> Result<Option<DocumentData>, String> {

	//Get a lock on all the mutexes
	let mut tabs = TABS
		.lock()
		.map_err(|e| format!("Failed to lock TABS: {}", e))?;

	let mut recent_files = RECENT_FILES
		.lock()
		.map_err(|e| format!("Failed to lock RECENT_FILES: {}", e))?;

	// Get the title of the document to be deleted from the tabs indexmap
	let tab_title = tabs.get(&id).map(|tab| tab.title.clone()).unwrap();
   
	// Get the path of the document to be deleted
	let trove_dir = get_trove_dir("Untitled_Trove");

	// Clean up stale entries that don't exist on disk
	{
		tabs.retain(|_, tab| {
			let file_path = trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &tab.title)));
			file_path.exists()
		});
	
		recent_files.retain(|file| {
			let file_path = trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &file.title)));
			file_path.exists()
		});
	}
	let filename = sanitize_filename::sanitize(format!("{}.md", tab_title));
	let file_path = trove_dir.join(&filename);
	
	// Remove the tab and get its index(shift_remove_full returns the index of the removed tab)
	if let Some((index, _, _)) = tabs.shift_remove_full(&id) {

		// Get the tab at the same index (the one that shifted up)
		// If no tab at that index, get the last tab
		let next_tab =
			if let Some((next_id, next_tab)) = tabs.get_index(index).or_else(|| tabs.last()) {
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

		
		// Check if the file exists
		if file_path.exists() {
			// Delete the file if it exists
			fs::remove_file(&file_path)
				.map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e))?;
		}

		// Remove the document from the recent files
		recent_files.retain(|doc| doc.id != id);

		// Drop the locks on the mutexes
		std::mem::drop(recent_files);
		std::mem::drop(tabs);

		// Save changes to userdata.json
		save_user_data()?;

		// Return the next tab as Some(DocumentData) if it exists else None
		Ok(next_tab)
	} else {
		// If the tab is not found, return an error
		Err("Tab not found".to_string())
	}
}

/// This function gets the content of the document by its id and title.
#[tauri::command]
pub fn get_document_content(id: String, title: String) -> Result<Option<DocumentData>, String> {

	// Get the path of the document using title
	let trove_dir = get_trove_dir("Untitled_Trove");
	let file_path = trove_dir.join(format!("{}.md", title));

	// Check if the file exists
	if !file_path.exists() {
		// If the file does not exist, return None
		return Ok(None);
	}

	// Read the file content using the file path
	match fs::read_to_string(&file_path) {
		// If the file is read successfully, convert the markdown content to HTML
		Ok(content) => {
			let html_output = markdown_handler::markdown_to_html(&content);

			// Return the document data as Some(DocumentData)
			Ok(Some(DocumentData {
				id,
				title,
				content: html_output,
			}))
		}
		// If there is an error in reading the file, return the error
		Err(e) => Err(format!("Failed to read file: {}", e)),
	}
}

/// This function loads the tabs active/opened in the last app section.
#[tauri::command]
pub fn load_last_open_tabs() -> Result<Vec<DocumentData>, String> {

	// Get the path of the userdata.json file
	let appdata_dir = get_documents_dir().join("appdata");
	let userdata_path = appdata_dir.join("userdata.json");

	// Check if userdata.json exists
	if userdata_path.exists() {

		// Read and deserialize the UserData
		match fs::read_to_string(&userdata_path) {
			Ok(content) => {

				// Deserialize the UserData using serde_json
				match serde_json::from_str::<UserData>(&content) {
					Ok(user_data) => {

						// Lock the mutexes and update the values
						let mut recent_files_lock = RECENT_FILES
							.lock()
							.map_err(|e| format!("Failed to lock RECENT_FILES: {}", e))?;
						*recent_files_lock = user_data.recent_files.clone();

						// Create a vector to store the last open files
						let mut last_open_files = Vec::new();

						// Update the current open tab
						let mut current_open_tab = CURRENT_OPEN_TAB
							.lock()
							.map_err(|e| format!("Failed to lock CURRENT_OPEN_TAB: {}", e))?;
						*current_open_tab = user_data.last_open_tab.clone();

						// Lock the tabs mutex and update the values
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
