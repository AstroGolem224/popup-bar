//! File, application, and URL opening
//!
//! Launches shelf items using the appropriate system handler based on
//! the item type. Files open in their default application, apps are
//! executed, and URLs open in the default browser.

use super::shelf_store::ItemType;

/// Launches an item based on its type and path.
pub struct Launcher;

impl Launcher {
    /// Open a shelf item using the system default handler.
    pub fn open(item_type: &ItemType, path: &str) -> Result<(), String> {
        todo!()
    }

    /// Open a file with a specific application.
    pub fn open_with(path: &str, app_path: &str) -> Result<(), String> {
        todo!()
    }

    /// Reveal a file or folder in the system file manager.
    pub fn reveal_in_file_manager(path: &str) -> Result<(), String> {
        todo!()
    }

    /// Check if a path or URL is still valid/accessible.
    pub fn validate_target(path: &str) -> bool {
        todo!()
    }
}
