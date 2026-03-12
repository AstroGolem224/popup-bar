//! Windows platform implementation
//!
//! Provides PlatformProvider for Windows 10/11.
//! Phase 1: Hotzone via SetWindowsHookEx (WH_MOUSE_LL).
//! Phase 4: Icon extraction via SHGetFileInfo.

use super::{MousePosition, PlatformProvider};
use log::info;

/// Windows platform provider.
pub struct WindowsProvider;

impl WindowsProvider {
    /// Create a new Windows platform provider.
    pub fn new() -> Self {
        info!("WindowsProvider: initialized");
        Self
    }
}

impl PlatformProvider for WindowsProvider {
    fn register_hotzone(&self, _height: u32) -> Result<(), String> {
        info!("WindowsProvider: register_hotzone (stub — Phase 1)");
        Ok(())
    }

    fn unregister_hotzone(&self) -> Result<(), String> {
        info!("WindowsProvider: unregister_hotzone (stub — Phase 1)");
        Ok(())
    }

    fn get_mouse_position(&self) -> Result<MousePosition, String> {
        // Phase 1: GetCursorPos via windows crate
        Ok(MousePosition { x: 0.0, y: 0.0 })
    }

    fn set_window_vibrancy(&self, _blur_radius: f64, _tint_color: &str) -> Result<(), String> {
        // Handled via window-vibrancy crate in lib.rs setup
        info!("WindowsProvider: vibrancy handled in lib.rs setup");
        Ok(())
    }

    fn extract_icon(&self, _path: &str, _size: u32) -> Result<Vec<u8>, String> {
        // Phase 4: SHGetFileInfo / IExtractIcon
        Err("Icon extraction not implemented on Windows (Phase 4)".into())
    }

    fn launch_item(&self, _path: &str) -> Result<(), String> {
        // Phase 2: ShellExecuteW
        Err("Launcher not implemented on Windows (Phase 2)".into())
    }
}
