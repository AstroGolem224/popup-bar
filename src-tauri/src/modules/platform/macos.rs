//! macOS platform implementation
//!
//! Provides PlatformProvider for macOS.
//! Phase 1: Hotzone via CGEventTap (kCGEventMouseMoved).
//! Phase 4: Icon extraction via NSWorkspace.icon(forFile:).

use super::{MousePosition, PlatformProvider, MonitorInfo};
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
        Err("Hotzone mouse position not implemented on macOS yet (Phase 1)".into())
    }

    fn set_window_vibrancy(&self, _blur_radius: f64, _tint_color: &str) -> Result<(), String> {
        // Handled via window-vibrancy crate in lib.rs setup
        info!("MacOSProvider: vibrancy handled in lib.rs setup");
        Ok(())
    }

    fn extract_icon(&self, path: &str, size: u32) -> Result<Vec<u8>, String> {
        if path.starts_with("http://") || path.starts_with("https://") {
            return Err("URL icons not resolved via macOS file icon extraction".into());
        }

        let path_buf = std::path::Path::new(path);
        if !path_buf.exists() {
            return Err("path does not exist".into());
        }

        let icns_path = if path_buf.is_dir() {
            let ext = path_buf.extension().and_then(|e| e.to_str());
            if ext == Some("app") {
                let resources = path_buf.join("Contents/Resources");
                let mut icns = None;
                if let Ok(entries) = std::fs::read_dir(resources) {
                    for entry in entries.flatten() {
                        let p = entry.path();
                        if p.extension().and_then(|e| e.to_str()) == Some("icns") {
                            icns = Some(p);
                            break;
                        }
                    }
                }
                icns.ok_or("no .icns found in app bundle")?
            } else {
                return Err("macOS icon extraction only supports .app bundles".into());
            }
        } else {
            return Err("macOS icon extraction only supports .app bundles".into());
        };

        let out_path = std::env::temp_dir().join(format!("popup-bar-icon-{}.png", std::process::id()));
        let status = std::process::Command::new("sips")
            .args([
                "-s",
                "format",
                "png",
                "-z",
                &format!("{}", size),
                &format!("{}", size),
                icns_path.to_string_lossy().as_ref(),
                "--out",
                out_path.to_string_lossy().as_ref(),
            ])
            .status()
            .map_err(|e| format!("sips failed: {e}"))?;

        if !status.success() {
            let _ = std::fs::remove_file(&out_path);
            return Err("sips conversion failed".into());
        }

        let bytes = std::fs::read(&out_path).map_err(|e| format!("read png failed: {e}"))?;
        let _ = std::fs::remove_file(&out_path);
        Ok(bytes)
    }

    fn launch_item(&self, _path: &str) -> Result<(), String> {
        // Phase 2: NSWorkspace.open
        Err("Launcher not implemented on macOS (Phase 2)".into())
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
