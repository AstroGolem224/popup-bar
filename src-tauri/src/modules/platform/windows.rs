//! Windows platform implementation
//!
//! Provides PlatformProvider for Windows 10/11.
//! Phase 1: Hotzone via SetWindowsHookEx (WH_MOUSE_LL).
//! Phase 4: Icon extraction via SHGetFileInfo.

use super::{MousePosition, PlatformProvider};
use base64::Engine;
use std::process::Command;
use windows_sys::Win32::Foundation::POINT;
use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;
use super::MonitorInfo;

use std::sync::Mutex;
use std::time::{Duration, Instant};
 
/// Windows platform provider.
pub struct WindowsProvider {
    primary_monitor_cache: Mutex<Option<(MonitorInfo, Instant)>>,
}

impl WindowsProvider {
    /// Create a new Windows platform provider.
    pub fn new() -> Self {
        Self {
            primary_monitor_cache: Mutex::new(None),
        }
    }
}

impl PlatformProvider for WindowsProvider {
    fn register_hotzone(&self, _height: u32) -> Result<(), String> {
        Ok(())
    }

    fn unregister_hotzone(&self) -> Result<(), String> {
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

    fn get_primary_monitor(&self) -> Option<MonitorInfo> {
        // Check cache first (refresh every 2 seconds)
        if let Ok(cache) = self.primary_monitor_cache.lock() {
            if let Some((info, last_update)) = &*cache {
                if last_update.elapsed() < Duration::from_secs(2) {
                    return Some(*info);
                }
            }
        }

        use windows_sys::Win32::Graphics::Gdi::{
            EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO,
        };
        use windows_sys::Win32::Foundation::{BOOL, LPARAM, RECT};

        // Internal helper to find the primary monitor
        unsafe extern "system" fn monitor_enum_proc(
            hmonitor: HMONITOR,
            _: HDC,
            _: *mut RECT,
            lparam: LPARAM,
        ) -> BOOL {
            let mut info: MONITORINFO = std::mem::zeroed();
            info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
            if GetMonitorInfoW(hmonitor, &mut info) != 0 {
                // MONITORINFOF_PRIMARY is 0x1
                if (info.dwFlags & 0x1) != 0 {
                    let ptr = lparam as *mut Option<MonitorInfo>;
                    *ptr = Some(MonitorInfo {
                        x: info.rcMonitor.left,
                        y: info.rcMonitor.top,
                        width: (info.rcMonitor.right - info.rcMonitor.left) as u32,
                        height: (info.rcMonitor.bottom - info.rcMonitor.top) as u32,
                    });
                    return 0; // Stop enumeration
                }
            }
            1 // Continue enumeration
        }

        let mut primary_monitor: Option<MonitorInfo> = None;
        unsafe {
            EnumDisplayMonitors(
                std::ptr::null_mut(),
                std::ptr::null(),
                Some(monitor_enum_proc),
                &mut primary_monitor as *mut _ as LPARAM,
            );
        }

        if let Some(m) = primary_monitor {
            if let Ok(mut cache) = self.primary_monitor_cache.lock() {
                *cache = Some((m, Instant::now()));
            }
        }

        primary_monitor
    }
}
