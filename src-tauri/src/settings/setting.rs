use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct SettingOption {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub setting: Vec<Setting>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Setting {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub select: Option<Vec<String>>,
    #[serde(default)]
    pub check: Option<bool>,
    #[serde(default)]
    pub selected: Option<String>,
}

#[tauri::command]
pub fn save_setting(title: String, value: String) -> Result<(), String> {
    let settings_str = fs::read_to_string("../settings.json")
        .map_err(|e| format!("Failed to read settings file: {}", e))?;
    let mut settings: Vec<SettingOption> = serde_json::from_str(&settings_str)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    for section in &mut settings {
        for setting in &mut section.setting {
            if setting.title == title {
                setting.selected = Some(value);
                let json = serde_json::to_string_pretty(&settings)
                    .map_err(|e| format!("Failed to serialize settings: {}", e))?;
                fs::write("../settings.json", json)
                    .map_err(|e| format!("Failed to write settings file: {}", e))?;
                return Ok(());
            }
        }
    }

    Err(format!("No setting found with title: {}", title))
}

#[tauri::command]
pub fn get_setting_value(title: String) -> Result<Setting, String> {
    let settings_str = fs::read_to_string("../settings.json")
        .map_err(|e| format!("Failed to read settings file: {}", e))?;
    let settings: Vec<SettingOption> = serde_json::from_str(&settings_str)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    for section in &settings {
        for setting in &section.setting {
            if setting.title == title {
                return Ok(setting.clone());
            }
        }
    }

    Err(format!("No setting found with title: {}", title))
}


#[tauri::command]
pub fn get_all_settings() -> Result<Vec<SettingOption>, String> {
    let settings_str = fs::read_to_string("../settings.json")
        .map_err(|e| format!("Failed to read settings file: {}", e))?;
    let settings: Vec<SettingOption> = serde_json::from_str(&settings_str)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;
    Ok(settings)
}

#[tauri::command]
pub fn toggle_check(title: String) -> Result<(), String> {
    let settings_str = fs::read_to_string("../settings.json")
        .map_err(|e| format!("Failed to read settings file: {}", e))?;
    let mut settings: Vec<SettingOption> = serde_json::from_str(&settings_str)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    for section in &mut settings {
        for setting in &mut section.setting {
            if setting.title == title {
                setting.check = Some(!setting.check.unwrap_or(false));
                let json = serde_json::to_string_pretty(&settings)
                    .map_err(|e| format!("Failed to serialize settings: {}", e))?;
                fs::write("../settings.json", json)
                    .map_err(|e| format!("Failed to write settings file: {}", e))?;
                return Ok(());
            }
        }
    }

    Err(format!("No setting found with title: {}", title))
}

