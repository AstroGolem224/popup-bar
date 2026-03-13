//! Tauri commands for application settings.
//!
//! Exposes settings read/write to the React frontend; persists via ConfigManager.

use crate::modules::config::{AppSettings, ConfigManager};
use tauri::{Emitter, WebviewWindow};

/// Get current application settings from SQLite.
#[tauri::command]
pub async fn get_settings() -> Result<AppSettings, String> {
    ConfigManager::load().await
}

/// Update application settings and emit settings_changed.
#[tauri::command]
pub async fn update_settings(
    window: WebviewWindow,
    settings: AppSettings,
) -> Result<AppSettings, String> {
    ConfigManager::save(&settings).await?;
    window
        .emit("settings_changed", &settings)
        .map_err(|e: tauri::Error| e.to_string())?;
    Ok(settings)
}

/// Enable or disable launch at login (autostart).
#[tauri::command]
pub async fn set_launch_at_login(
    enabled: bool,
    autostart: tauri::State<'_, tauri_plugin_autostart::AutoLaunchManager>,
) -> Result<(), String> {
    if enabled {
        autostart.enable().map_err(|e| e.to_string())?;
    } else {
        autostart.disable().map_err(|e| e.to_string())?;
    }
    Ok(())
}
