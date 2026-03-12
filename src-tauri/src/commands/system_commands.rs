//! Tauri commands for system-level operations.
//!
//! Window show/hide and platform information exposed to the frontend.

use serde::Serialize;
use tauri::WebviewWindow;

/// Basic platform information.
#[derive(Serialize)]
pub struct PlatformInfo {
    /// Operating system name (e.g., "linux", "windows", "macos").
    pub os: String,
    /// CPU architecture (e.g., "x86_64", "aarch64").
    pub arch: String,
    /// Application version from Cargo.toml.
    pub version: String,
}

/// Get platform and version information.
#[tauri::command]
pub fn get_platform_info() -> PlatformInfo {
    PlatformInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

/// Show the main popup bar window.
#[tauri::command]
pub async fn show_window(window: WebviewWindow) -> Result<(), String> {
    window.show().map_err(|e| e.to_string())
}

/// Hide the main popup bar window.
#[tauri::command]
pub async fn hide_window(window: WebviewWindow) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}
