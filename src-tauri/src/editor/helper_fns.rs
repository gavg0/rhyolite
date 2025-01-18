// New function to periodically clean up stale entries
pub fn cleanup_stale_entries() -> Result<(), String> {
    let mut tabs = TABS.lock().map_err(|e| format!("Failed to lock TABS: {}", e))?;
    let mut recent_files = RECENT_FILES.lock().map_err(|e| format!("Failed to lock RECENT_FILES: {}", e))?;
    let trove_dir = get_trove_dir("Untitled_Trove");

    tabs.retain(|_, tab| {
        let file_path = trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &tab.title)));
        file_path.exists()
    });

    recent_files.retain(|file| {
        let file_path = trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &file.title)));
        file_path.exists()
    });

    save_user_data()
}