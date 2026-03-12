//! Tauri commands for application settings.

use crate::modules::config::AppSettings;

#[tauri::command]
pub async fn get_settings() -> Result<AppSettings, String> {
    todo!()
}

#[tauri::command]
pub async fn update_settings(settings: AppSettings) -> Result<AppSettings, String> {
    let _ = settings;
    todo!()
}
