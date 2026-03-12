//! Linux platform implementation
//!
//! Provides PlatformProvider for X11 and Wayland.
//! Phase 1: Hotzone via XInput2 (X11) / layer-shell trigger strip (Wayland).
//! Phase 4: Icon extraction via Freedesktop Icon Theme Spec.

use super::{MousePosition, PlatformProvider};
use log::info;

/// Linux platform provider with X11/Wayland detection.
pub struct LinuxProvider;

impl LinuxProvider {
    /// Create a new Linux platform provider.
    pub fn new() -> Self {
        let display_server = if std::env::var("WAYLAND_DISPLAY").is_ok() {
            "Wayland"
        } else {
            "X11"
        };
        info!("LinuxProvider: detected display server: {display_server}");
        Self
    }
}

impl PlatformProvider for LinuxProvider {
    fn register_hotzone(&self, _height: u32) -> Result<(), String> {
        info!("LinuxProvider: register_hotzone (stub — Phase 1)");
        Ok(())
    }

    fn unregister_hotzone(&self) -> Result<(), String> {
        info!("LinuxProvider: unregister_hotzone (stub — Phase 1)");
        Ok(())
    }

    fn get_mouse_position(&self) -> Result<MousePosition, String> {
        // Phase 1: XInput2 on X11, pointer protocol on Wayland
        Ok(MousePosition { x: 0.0, y: 0.0 })
    }

    fn set_window_vibrancy(&self, _blur_radius: f64, _tint_color: &str) -> Result<(), String> {
        info!("LinuxProvider: no native vibrancy — CSS fallback active");
        Ok(())
    }

    fn extract_icon(&self, _path: &str, _size: u32) -> Result<Vec<u8>, String> {
        // Phase 4: Freedesktop Icon Theme Spec + gtk_icon_theme_lookup_icon
        Err("Icon extraction not implemented on Linux (Phase 4)".into())
    }

    fn launch_item(&self, _path: &str) -> Result<(), String> {
        // Phase 2: xdg-open
        Err("Launcher not implemented on Linux (Phase 2)".into())
    }
}
