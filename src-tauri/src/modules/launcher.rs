//! File, application, and URL opening
//!
//! Launches shelf items using the appropriate system handler based on
//! the item type. Files open in their default application, apps are
//! executed, and URLs open in the default browser.
//! Full implementation in Phase 2 (basic) / Phase 3 (advanced).

use super::shelf_store::ItemType;
use log::info;

/// Launches an item based on its type and path.
pub struct Launcher;

impl Launcher {
    /// Open a shelf item using the system default handler.
    /// Phase 2: Uses `open` crate or tauri-plugin-shell.
    pub fn open(_item_type: &ItemType, _path: &str) -> Result<(), String> {
        info!("Launcher: open (stub — Phase 2)");
        Err("Launcher: open not implemented (Phase 2)".into())
    }

    /// Open a file with a specific application.
    pub fn open_with(_path: &str, _app_path: &str) -> Result<(), String> {
        info!("Launcher: open_with (stub — Phase 3)");
        Err("Launcher: open_with not implemented (Phase 3)".into())
    }

    /// Reveal a file or folder in the system file manager.
    pub fn reveal_in_file_manager(_path: &str) -> Result<(), String> {
        info!("Launcher: reveal_in_file_manager (stub — Phase 2)");
        Err("Launcher: reveal_in_file_manager not implemented (Phase 2)".into())
    }

    /// Check if a path or URL is still valid/accessible.
    /// Returns true by default until Phase 2.
    pub fn validate_target(_path: &str) -> bool {
        true
    }
}
