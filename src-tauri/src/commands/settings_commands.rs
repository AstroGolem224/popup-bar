//! Tauri commands for application settings.
//!
//! Exposes settings read/write to the React frontend; persists via ConfigManager.

use crate::modules::config::{AppSettings, ConfigManager, SkinInfo};
use base64::Engine;
use std::path::PathBuf;
use tauri::{Emitter, Manager, WebviewWindow};
use uuid::Uuid;

/// Get current application settings from SQLite.
#[tauri::command]
pub async fn get_settings() -> Result<AppSettings, String> {
    ConfigManager::load().await
}

#[tauri::command]
pub async fn update_settings(
    app: tauri::AppHandle,
    settings: AppSettings,
    hotzone_tracker: tauri::State<'_, std::sync::Mutex<crate::modules::hotzone::HotzoneTracker>>,
) -> Result<AppSettings, String> {
    ConfigManager::save(&settings).await?;
    
    log::info!("[settings] update_settings called: hotzone_size={}", settings.hotzone_size);
 
    if let Ok(mut tracker) = hotzone_tracker.lock() {
        log::info!("[settings] Updating hotzone tracker config...");
        tracker.update_config(crate::modules::hotzone::HotzoneConfig {
            size: settings.hotzone_size,
            top_enabled: true,
            delay_ms: 200,
        });
    } else {
        log::warn!("[settings] FAILED to lock hotzone_tracker!");
    }

    app.emit("settings_changed", &settings)
        .map_err(|e: tauri::Error| e.to_string())?;
    Ok(settings)
}

fn skins_dir(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    app_handle
        .path()
        .app_data_dir()
        .map(|d| d.join("skins"))
        .map_err(|e| e.to_string())
}

/// List all PNG/JPG skin files in the app data skins directory.
#[tauri::command]
pub async fn list_skins(app: tauri::AppHandle) -> Result<Vec<SkinInfo>, String> {
    let dir = skins_dir(&app)?;
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut skins = Vec::new();
    let entries = std::fs::read_dir(&dir).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        if ext == "png" || ext == "jpg" || ext == "jpeg" {
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            let name = path
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            skins.push(SkinInfo {
                name,
                filename,
                absolute_path: path.to_string_lossy().to_string(),
            });
        }
    }
    skins.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(skins)
}

/// Import a skin file from the given path into the app data skins directory.
#[tauri::command]
pub async fn import_skin(app: tauri::AppHandle, source_path: String) -> Result<SkinInfo, String> {
    let src = PathBuf::from(&source_path);
    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    if ext != "png" && ext != "jpg" && ext != "jpeg" {
        return Err("Nur PNG und JPG werden unterstützt".to_string());
    }

    let dir = skins_dir(&app)?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let stem = src
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("skin");
    let filename = format!("{}-{}.{}", stem, &Uuid::new_v4().to_string()[..8], ext);
    let dest = dir.join(&filename);

    std::fs::copy(&src, &dest).map_err(|e| {
        log::error!("Skin-Import fehlgeschlagen (copy): {e}");
        format!("Kopieren fehlgeschlagen: {e}")
    })?;

    log::info!("Skin erfolgreich importiert: {}", filename);
    Ok(SkinInfo {
        name: stem.to_string(),
        filename,
        absolute_path: dest.to_string_lossy().to_string(),
    })
}

/// Set the active skin by filename, or None for default glassmorphism.
#[tauri::command]
pub async fn set_active_skin(
    app: tauri::AppHandle,
    filename: Option<String>,
) -> Result<AppSettings, String> {
    let mut settings = ConfigManager::load().await?;
    settings.active_skin = filename;
    ConfigManager::save(&settings).await?;
    app.emit("settings_changed", &settings)
        .map_err(|e: tauri::Error| e.to_string())?;
    Ok(settings)
}

/// Delete a skin file and clear active skin if it was selected.
#[tauri::command]
pub async fn delete_skin(
    app: tauri::AppHandle,
    _window: WebviewWindow,
    filename: String,
) -> Result<AppSettings, String> {
    let dir = skins_dir(&app)?;
    let path = dir.join(&filename);
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| e.to_string())?;
    }

    let mut settings = ConfigManager::load().await?;
    if settings.active_skin.as_deref() == Some(&filename) {
        settings.active_skin = None;
        ConfigManager::save(&settings).await?;
        app.emit("settings_changed", &settings)
            .map_err(|e: tauri::Error| e.to_string())?;
    }
    Ok(settings)
}

/// Return a skin image as a data URL (base64-encoded).
/// Fallback for the asset protocol — works even without asset scope.
#[tauri::command]
pub async fn get_skin_data(app: tauri::AppHandle, filename: String) -> Result<String, String> {
    let dir = skins_dir(&app)?;
    let path = dir.join(&filename);
    if !path.exists() {
        log::error!("Get-Skin-Data fehlgeschlagen: Datei nicht gefunden ({:?})", path);
        return Err(format!("Skin not found: {}", filename));
    }
    let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png")
        .to_lowercase();
    let mime = if ext == "jpg" || ext == "jpeg" {
        "image/jpeg"
    } else {
        "image/png"
    };
    Ok(format!("data:{};base64,{}", mime, b64))
}

/// Import a skin from raw bytes (used when no file-dialog plugin is available).
#[tauri::command]
pub async fn import_skin_bytes(
    app: tauri::AppHandle,
    filename_stem: String,
    ext: String,
    bytes: Vec<u8>,
) -> Result<SkinInfo, String> {
    let ext = ext.to_lowercase();
    if ext != "png" && ext != "jpg" && ext != "jpeg" {
        return Err("Nur PNG und JPG werden unterstuetzt".to_string());
    }

    let dir = skins_dir(&app)?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let filename = format!("{}-{}.{}", filename_stem, &Uuid::new_v4().to_string()[..8], ext);
    let dest = dir.join(&filename);

    std::fs::write(&dest, &bytes).map_err(|e| {
        log::error!("Skin-Import (bytes) fehlgeschlagen: {e}");
        format!("Schreiben fehlgeschlagen: {e}")
    })?;

    log::info!("Skin (bytes) importiert: {}", filename);
    Ok(SkinInfo {
        name: filename_stem,
        filename,
        absolute_path: dest.to_string_lossy().to_string(),
    })
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
