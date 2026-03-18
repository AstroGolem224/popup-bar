//! Linux platform implementation
//!
//! Provides PlatformProvider for X11 and Wayland.
//! Phase 1: Hotzone via XInput2 (X11) / layer-shell trigger strip (Wayland).
//! Phase 4: Icon extraction via Freedesktop Icon Theme Spec.

use super::{MousePosition, PlatformProvider, MonitorInfo};
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

    /// Parse Icon= from a Freedesktop .desktop file. Returns the path only if it is absolute.
    fn parse_desktop_icon_path(desktop_path: &str) -> Result<String, String> {
        let content = std::fs::read_to_string(desktop_path)
            .map_err(|e| format!("read .desktop failed: {e}"))?;
        for line in content.lines() {
            let line = line.trim();
            if let Some(value) = line.strip_prefix("Icon=") {
                let value = value.trim();
                if value.starts_with('/') {
                    return Ok(value.to_string());
                }
                return Err("Icon= is not an absolute path (theme icons not resolved)".into());
            }
        }
        Err("no Icon= key in .desktop file".into())
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
        Err("Hotzone mouse position not implemented on Linux yet (Phase 1)".into())
    }

    fn set_window_vibrancy(&self, _blur_radius: f64, _tint_color: &str) -> Result<(), String> {
        info!("LinuxProvider: no native vibrancy — CSS fallback active");
        Ok(())
    }

    fn extract_icon(&self, path: &str, _size: u32) -> Result<Vec<u8>, String> {
        if path.starts_with("http://") || path.starts_with("https://") {
            return Err("URL icons not resolved via Linux file icon extraction".into());
        }

        let path_buf = std::path::Path::new(path);
        if !path_buf.exists() {
            return Err("path does not exist".into());
        }

        let icon_path = if path_buf.is_file() {
            let ext = path_buf
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase());
            if ext.as_deref() == Some("desktop") {
                Self::parse_desktop_icon_path(path)?
            } else {
                return Err("Linux icon extraction supports .desktop files only".into());
            }
        } else {
            return Err("Linux icon extraction supports .desktop files only".into());
        };

        let icon_path = std::path::Path::new(&icon_path);
        if !icon_path.exists() {
            return Err("Icon= path from .desktop does not exist".into());
        }

        let bytes = std::fs::read(icon_path).map_err(|e| format!("read icon file failed: {e}"))?;
        let ext = icon_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        if ext.as_deref() == Some("svg") {
            return Err("SVG icons are cached as-is; use fallback path for PNG".into());
        }
        if ext.as_deref() == Some("png") || ext.as_deref() == Some("xpm") {
            return Ok(bytes);
        }
        Ok(bytes)
    }

    fn launch_item(&self, _path: &str) -> Result<(), String> {
        // Phase 2: xdg-open
        Err("Launcher not implemented on Linux (Phase 2)".into())
    }

    fn get_primary_monitor(&self) -> Option<MonitorInfo> {
        Some(MonitorInfo {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        })
    }
}
