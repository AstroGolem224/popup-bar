//! Tauri commands for application settings.
//!
//! Exposes settings read/write to the React frontend.
//! Phase 0: Returns defaults. Phase 2: SQLite-backed.

use crate::modules::config::AppSettings;

/// Get current application settings.
#[tauri::command]
pub async fn get_settings() -> Result<AppSettings, String> {
    Ok(AppSettings::default())
}

/// Update application settings.
#[tauri::command]
pub async fn update_settings(_settings: AppSettings) -> Result<AppSettings, String> {
    // Phase 2: Persist to SQLite via ConfigManager::save()
    Ok(AppSettings::default())
}
