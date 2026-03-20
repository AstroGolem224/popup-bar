//! Windows platform implementation
//!
//! Provides PlatformProvider for Windows 10/11.
//! Phase 1: Hotzone via SetWindowsHookEx (WH_MOUSE_LL).
//! Phase 4: Icon extraction via SHGetFileInfo.

use super::{MousePosition, PlatformProvider};
use base64::Engine;
use log::info;
use std::process::Command;
use windows_sys::Win32::Foundation::POINT;
use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;

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
        let mut point = POINT { x: 0, y: 0 };
        // SAFETY: GetCursorPos writes to the provided POINT pointer.
        let ok = unsafe { GetCursorPos(&mut point) };
        if ok == 0 {
            return Err("GetCursorPos failed".to_string());
        }

        Ok(MousePosition {
            x: point.x as f64,
            y: point.y as f64,
        })
    }

    fn set_window_vibrancy(&self, _blur_radius: f64, _tint_color: &str) -> Result<(), String> {
        // Handled via window-vibrancy crate in lib.rs setup
        info!("WindowsProvider: vibrancy handled in lib.rs setup");
        Ok(())
    }

    fn extract_icon(&self, path: &str, _size: u32) -> Result<Vec<u8>, String> {
        if path.starts_with("http://") || path.starts_with("https://") {
            return Err("URL icons are not resolved via Windows file icon extraction".into());
        }

        let escaped = path.replace('\'', "''");
        let script = format!(
            r#"$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$target = '{escaped}'
if (-not (Test-Path $target)) {{ throw 'Path not found' }}
$icon = [System.Drawing.Icon]::ExtractAssociatedIcon($target)
if ($null -eq $icon) {{ throw 'No associated icon' }}
$bitmap = $icon.ToBitmap()
$ms = New-Object System.IO.MemoryStream
$bitmap.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
$bytes = $ms.ToArray()
$bitmap.Dispose()
$icon.Dispose()
[System.Convert]::ToBase64String($bytes)"#
        );

        let output = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .output()
            .map_err(|e| format!("failed to invoke powershell for icon extraction: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("icon extraction failed: {}", stderr.trim()));
        }

        let base64_png = String::from_utf8(output.stdout)
            .map_err(|e| format!("icon extraction output decode failed: {e}"))?;
        let trimmed = base64_png.trim();
        if trimmed.is_empty() {
            return Err("icon extraction returned empty output".into());
        }

        base64::engine::general_purpose::STANDARD
            .decode(trimmed)
            .map_err(|e| format!("failed to decode extracted icon png: {e}"))
    }

    fn launch_item(&self, _path: &str) -> Result<(), String> {
        // Phase 2: ShellExecuteW
        Err("Launcher not implemented on Windows (Phase 2)".into())
    }
}
