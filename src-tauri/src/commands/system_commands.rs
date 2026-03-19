//! Tauri commands for system-level operations.
//!
//! Window show/hide and platform information exposed to the frontend.

use crate::modules::platform::create_provider;
use crate::modules::config::ConfigManager;
use crate::modules::launcher::Launcher;
use crate::modules::shelf_store::ItemType;
use crate::modules::window_manager::BarRect;
use crate::BarRectState;
use crate::ManagerState;
use serde::Serialize;
use log::warn;
use std::str::FromStr;
use tauri::{AppHandle, Manager, State, WebviewWindow};

/// Basic platform information.
#[derive(Serialize)]
pub struct PlatformInfo {
    /// Operating system name (e.g., "linux", "windows", "macos").
    pub os: String,
    /// CPU architecture (e.g., "x86_64", "aarch64").
    pub arch: String,
    /// Application version from Cargo.toml.
    pub version: String,
}

/// Get platform and version information.
#[tauri::command]
pub fn get_platform_info() -> PlatformInfo {
    PlatformInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

/// Height when settings panel is open so content is visible.
const BAR_HEIGHT_SETTINGS: u32 = 520;

/// Bar width as fraction of monitor width when bar_width_px is 0.
const BAR_WIDTH_FRACTION: u32 = 3;

fn clamp_bar_width(bar_width_px: u32, monitor_width: u32) -> u32 {
    let min_w = 200u32;
    let max_w = monitor_width.min(1200);
    if bar_width_px == 0 {
        (monitor_width / BAR_WIDTH_FRACTION).clamp(min_w, max_w)
    } else {
        bar_width_px.clamp(min_w, max_w)
    }
}

fn clamp_bar_height(bar_height_px: u32) -> u32 {
    bar_height_px.clamp(56, 180)
}

/// Position the main window on the primary monitor. Width and height from settings.
fn position_on_monitor(
    window: &WebviewWindow,
    primary_only: bool,
    bar_width_px: u32,
    bar_height_px: u32,
) -> Result<BarRect, String> {
    let label = window.label();
    let monitor = if primary_only {
        window
            .primary_monitor()
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "no primary monitor".to_string())?
    } else {
        window
            .primary_monitor()
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "no primary monitor".to_string())?
    };
    let pos = monitor.position();
    let size = monitor.size();

    let bar_width = clamp_bar_width(bar_width_px, size.width);
    let bar_height = clamp_bar_height(bar_height_px);

    match label {
        "main" => {
            let center_x = pos.x + (size.width as i32 - bar_width as i32) / 2;
            window
                .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                    x: center_x,
                    y: pos.y,
                }))
                .map_err(|e| e.to_string())?;
            window
                .set_size(tauri::Size::Physical(tauri::PhysicalSize {
                    width: bar_width,
                    height: bar_height,
                }))
                .map_err(|e| e.to_string())?;
            Ok(BarRect {
                x: center_x,
                y: pos.y,
                width: bar_width,
                height: bar_height,
            })
        }
        _ => Err(format!("unsupported window label for positioning: {label}")),
    }
}

/// Show the main popup bar window.
#[tauri::command]
pub async fn show_window(
    window: WebviewWindow,
    window_manager: State<'_, ManagerState>,
    bar_rect: State<'_, BarRectState>,
) -> Result<Option<u64>, String> {
    let settings = ConfigManager::load().await.unwrap_or_default();
    let primary_only = !settings.multi_monitor;
    let _label = window.label();

    let mut manager = window_manager.0
        .lock()
        .map_err(|_| {
            warn!("[show_window] failed to lock window manager");
            "failed to lock window manager".to_string()
        })?;

    let token = manager.request_show().map_err(|e| {
        warn!("[show_window] request_show failed: {}", e);
        e.to_string()
    })?;
    if token.is_some() {
        let rect = position_on_monitor(
            &window,
            primary_only,
            settings.bar_width_px,
            settings.bar_height_px,
        )?;
        if let Ok(mut r) = bar_rect.0.lock() {
            *r = rect;
        }
        window.show().map_err(|e| {
            warn!("[show_window] window.show() failed for {}: {}", _label, e);
            e.to_string()
        })?;
    }
    Ok(token)
}

/// Mark show animation lifecycle as completed.
#[tauri::command]
pub async fn complete_show_window(
    token: u64,
    window_manager: State<'_, ManagerState>,
) -> Result<bool, String> {
    let mut manager = window_manager.0
        .lock()
        .map_err(|_| "failed to lock window manager".to_string())?;
    manager.confirm_shown(token).map_err(|e| e.to_string())
}

/// Start hide lifecycle. The actual OS hide happens in `complete_hide_window`.
#[tauri::command]
pub async fn hide_window(
    window_manager: State<'_, ManagerState>,
) -> Result<Option<u64>, String> {
    let mut manager = window_manager.0
        .lock()
        .map_err(|_| "failed to lock window manager".to_string())?;
    manager.request_hide().map_err(|e| e.to_string())
}

/// Finalize hide lifecycle after animation completes.
#[tauri::command]
pub async fn complete_hide_window(
    window: WebviewWindow,
    token: u64,
    window_manager: State<'_, ManagerState>,
    bar_rect: State<'_, BarRectState>,
) -> Result<bool, String> {
    let mut manager = window_manager.0
        .lock()
        .map_err(|_| "failed to lock window manager".to_string())?;

    let applied = manager.confirm_hidden(token).map_err(|e| e.to_string())?;
    if applied {
        if let Ok(mut r) = bar_rect.0.lock() {
            *r = BarRect::default();
        }
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(applied)
}

/// Open the settings panel as a separate window.
#[tauri::command]
pub async fn set_settings_expanded(
    app: AppHandle,
    expanded: bool,
) -> Result<(), String> {
    if !expanded {
        // If we want to support closing via this command
        if let Some(settings_window) = app.get_webview_window("settings") {
            let _ = settings_window.close();
        }
        return Ok(());
    }

    if let Some(settings_window) = app.get_webview_window("settings") {
        let _ = settings_window.set_focus();
        return Ok(());
    }

    let mouse_pos = create_provider()
        .get_mouse_position()
        .unwrap_or(crate::modules::platform::MousePosition { x: 0.0, y: 0.0 });

    let width = 360.0;
    let height = 520.0;

    let _window = tauri::WebviewWindowBuilder::new(
        &app,
        "settings",
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("Popup Bar - Einstellungen")
    .inner_size(width, height)
    .position(mouse_pos.x - (width / 2.0), mouse_pos.y - 20.0)
    .resizable(true)
    .decorations(true)
    .always_on_top(true)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Open a shelf item using the platform's default handler.
#[tauri::command]
pub async fn open_shelf_item(
    app: AppHandle,
    item_type: String,
    path: String,
) -> Result<(), String> {
    let parsed_type = ItemType::from_str(&item_type)?;
    Launcher::open(&app, &parsed_type, &path)
}

/// Exit the application (e.g. from Exit button in the bar).
#[tauri::command]
pub fn exit_app(app: AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}
