//! File, application, and URL opening
//!
//! Launches shelf items using the appropriate system handler based on
//! the item type. Files open in their default application, apps are
//! executed, and URLs open in the default browser.
//! Full implementation in Phase 2 (basic) / Phase 3 (advanced).

use super::shelf_store::ItemType;
use std::process::Command;
use tauri::AppHandle;

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
    pub fn open(_app: &AppHandle, _item_type: &ItemType, path: &str) -> Result<(), String> {
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

        #[cfg(target_os = "macos")]
        {
            let status = Command::new("open")
                .arg(path)
                .status()
                .map_err(|e| format!("Launcher: open failed: {e}"))?;
            if status.success() {
                Ok(())
            } else {
                Err(format!("Launcher: open exited with code {:?}", status.code()))
            }
        }

        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        {
            let status = Command::new("xdg-open")
                .arg(path)
                .status()
                .map_err(|e| format!("Launcher: open failed: {e}"))?;
            if status.success() {
                Ok(())
            } else {
                Err(format!("Launcher: open exited with code {:?}", status.code()))
            }
        }
    }

    /// Open a file with a specific application.
    ///
    /// Currently not implemented; reserved für eine spätere phase mit
    /// expliziter programmauswahl.
    #[allow(dead_code)]
    pub fn open_with(_app: &AppHandle, _path: &str, _app_path: &str) -> Result<(), String> {
        Err("Launcher: open_with not implemented (Phase 3)".into())
    }

    /// Reveal a file or folder in the system file manager.
    ///
    /// Delegiert aktuell an `open`, sodass das OS entscheidet, ob es
    /// ein reveal oder open ist. Eine spezialisierte implementation
    /// kann später ergänzt werden.
    #[allow(dead_code)]
    pub fn reveal_in_file_manager(_app: &AppHandle, path: &str) -> Result<(), String> {
        if !Self::validate_target(path) {
            return Err("Launcher: invalid target path".into());
        }

        #[cfg(target_os = "windows")]
        {
            let status = Command::new("explorer")
                .args(["/select,", path])
                .status()
                .map_err(|e| format!("Launcher: reveal_in_file_manager failed: {e}"))?;
            if status.success() {
                Ok(())
            } else {
                Err(format!(
                    "Launcher: reveal_in_file_manager exited with code {:?}",
                    status.code()
                ))
            }
        }

        #[cfg(target_os = "macos")]
        {
            let status = Command::new("open")
                .args(["-R", path])
                .status()
                .map_err(|e| format!("Launcher: reveal_in_file_manager failed: {e}"))?;
            if status.success() {
                Ok(())
            } else {
                Err(format!(
                    "Launcher: reveal_in_file_manager exited with code {:?}",
                    status.code()
                ))
            }
        }

        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        {
            let target = std::path::Path::new(path);
            let open_path = if target.is_dir() {
                target
            } else {
                target.parent().unwrap_or(target)
            };
            let status = Command::new("xdg-open")
                .arg(open_path)
                .status()
                .map_err(|e| format!("Launcher: reveal_in_file_manager failed: {e}"))?;
            if status.success() {
                Ok(())
            } else {
                Err(format!(
                    "Launcher: reveal_in_file_manager exited with code {:?}",
                    status.code()
                ))
            }
        }
    }

    /// Check if a path or URL is still superficially valid/accessible.
    ///
    /// Die shell-plugin-ebene wendet eigene regex-basierte validierung
    /// an; hier filtern wir nur offensichtliche ungültige werte.
    pub fn validate_target(path: &str) -> bool {
        !path.trim().is_empty()
    }
}
