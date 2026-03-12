//! macOS platform implementation
//!
//! Provides PlatformProvider for macOS.
//! Phase 1: Hotzone via CGEventTap (kCGEventMouseMoved).
//! Phase 4: Icon extraction via NSWorkspace.icon(forFile:).

use super::{MousePosition, PlatformProvider};
use log::info;

/// macOS platform provider.
pub struct MacOSProvider;

impl MacOSProvider {
    /// Create a new macOS platform provider.
    pub fn new() -> Self {
        info!("MacOSProvider: initialized");
        Self
    }
}

impl PlatformProvider for MacOSProvider {
    fn register_hotzone(&self, _height: u32) -> Result<(), String> {
        info!("MacOSProvider: register_hotzone (stub — Phase 1)");
        Ok(())
    }

    fn unregister_hotzone(&self) -> Result<(), String> {
        info!("MacOSProvider: unregister_hotzone (stub — Phase 1)");
        Ok(())
    }

    fn get_mouse_position(&self) -> Result<MousePosition, String> {
        // Phase 1: CGEventGetLocation via core-graphics crate
        Ok(MousePosition { x: 0.0, y: 0.0 })
    }

    fn set_window_vibrancy(&self, _blur_radius: f64, _tint_color: &str) -> Result<(), String> {
        // Handled via window-vibrancy crate in lib.rs setup
        info!("MacOSProvider: vibrancy handled in lib.rs setup");
        Ok(())
    }

    fn extract_icon(&self, _path: &str, _size: u32) -> Result<Vec<u8>, String> {
        // Phase 4: NSWorkspace.icon(forFile:)
        Err("Icon extraction not implemented on macOS (Phase 4)".into())
    }

    fn launch_item(&self, _path: &str) -> Result<(), String> {
        // Phase 2: NSWorkspace.open
        Err("Launcher not implemented on macOS (Phase 2)".into())
    }
}
