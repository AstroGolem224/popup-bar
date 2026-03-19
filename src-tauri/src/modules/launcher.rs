//! File, application, and URL opening
//!
//! Launches shelf items using the appropriate system handler based on
//! the item type. Files open in their default application, apps are
//! executed, and URLs open in the default browser.
//! Full implementation in Phase 2 (basic) / Phase 3 (advanced).

use super::shelf_store::ItemType;
use log::info;
use std::process::Command;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

/// Launches an item based on its type and path.
pub struct Launcher;

impl Launcher {
    /// Open a shelf item using the system default handler.
    ///
    /// Depending on the `ItemType`, this typically means:
    /// - `File` / `Folder` / `App`: open in the OS default handler
    /// - `Url`: open in the default browser
    ///
    /// On Windows we use `cmd /c start "" "path"` so that .lnk and paths
    /// with spaces work reliably (shell.open is deprecated and can misbehave).
    #[cfg_attr(target_os = "windows", allow(unused_variables))]
    pub fn open(app: &AppHandle, item_type: &ItemType, path: &str) -> Result<(), String> {
        if !Self::validate_target(path) {
            return Err("Launcher: invalid target path".into());
        }

        #[cfg(target_os = "windows")]
        {
            // cmd /c start "" "path" — empty "" is window title; path in quotes for spaces/special chars
            let status = Command::new("cmd")
                .args(["/C", "start", "", path])
                .status()
                .map_err(|e| format!("Launcher: open failed: {e}"))?;
            if status.success() {
                Ok(())
            } else {
                Err(format!(
                    "Launcher: start exited with code {:?}",
                    status.code()
                ))
            }
        }

        #[cfg(not(target_os = "windows"))]
        app.shell()
            .open(path.to_string(), None)
            .map_err(|e| format!("Launcher: open failed: {e}"))
    }

    /// Open a file with a specific application.
    ///
    /// Currently not implemented; reserved für eine spätere phase mit
    /// expliziter programmauswahl.
    pub fn open_with(_app: &AppHandle, _path: &str, _app_path: &str) -> Result<(), String> {
        Err("Launcher: open_with not implemented (Phase 3)".into())
    }

    /// Reveal a file or folder in the system file manager.
    ///
    /// Delegiert aktuell an `open`, sodass das OS entscheidet, ob es
    /// ein reveal oder open ist. Eine spezialisierte implementation
    /// kann später ergänzt werden.
    pub fn reveal_in_file_manager(app: &AppHandle, path: &str) -> Result<(), String> {
        if !Self::validate_target(path) {
            return Err("Launcher: invalid target path".into());
        }

        app.shell()
            .open(path.to_string(), None)
            .map_err(|e| format!("Launcher: reveal_in_file_manager failed: {e}"))
    }

    /// Check if a path or URL is still superficially valid/accessible.
    ///
    /// Die shell-plugin-ebene wendet eigene regex-basierte validierung
    /// an; hier filtern wir nur offensichtliche ungültige werte.
    pub fn validate_target(path: &str) -> bool {
        !path.trim().is_empty()
    }
}
