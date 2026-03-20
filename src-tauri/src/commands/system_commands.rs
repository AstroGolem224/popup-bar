//! Tauri commands for system-level operations.
//!
//! Window show/hide, tray integration, and monitor placement exposed to the frontend.

use crate::modules::config::MonitorStrategy;
use crate::modules::launcher::Launcher;
use crate::modules::platform::create_provider;
use crate::modules::shelf_store::ItemType;
use crate::modules::window_manager::BarRect;
use crate::{BarRectState, LastMonitorState, ManagerState, SettingsState};
use log::warn;
use serde::Serialize;
use std::str::FromStr;
use tauri::{AppHandle, Emitter, Manager, State, WebviewWindow};

pub const EVENT_TOGGLE_VISIBILITY: &str = "system:toggle-visibility";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MonitorSnapshot {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
struct MonitorBounds {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

#[derive(Debug, Clone)]
struct PositionedBar {
    rect: BarRect,
    monitor: MonitorSnapshot,
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

fn position_on_monitor(
    window: &WebviewWindow,
    monitor_strategy: &MonitorStrategy,
    last_monitor: Option<MonitorSnapshot>,
    bar_width_px: u32,
    bar_height_px: u32,
) -> Result<PositionedBar, String> {
    let label = window.label();
    let monitor = resolve_target_monitor(window, monitor_strategy, last_monitor)?;
    let bar_width = clamp_bar_width(bar_width_px, monitor.width);
    let bar_height = clamp_bar_height(bar_height_px);

    match label {
        "main" => {
            let center_x = monitor.x + (monitor.width as i32 - bar_width as i32) / 2;
            window
                .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                    x: center_x,
                    y: monitor.y,
                }))
                .map_err(|e| e.to_string())?;
            window
                .set_size(tauri::Size::Physical(tauri::PhysicalSize {
                    width: bar_width,
                    height: bar_height,
                }))
                .map_err(|e| e.to_string())?;

            Ok(PositionedBar {
                rect: BarRect {
                    x: center_x,
                    y: monitor.y,
                    width: bar_width,
                    height: bar_height,
                },
                monitor: monitor_to_snapshot(monitor),
            })
        }
        _ => Err(format!("unsupported window label for positioning: {label}")),
    }
}

fn resolve_target_monitor(
    window: &WebviewWindow,
    monitor_strategy: &MonitorStrategy,
    last_monitor: Option<MonitorSnapshot>,
) -> Result<MonitorBounds, String> {
    match monitor_strategy {
        MonitorStrategy::Primary => resolve_primary_monitor(window),
        MonitorStrategy::Cursor => resolve_cursor_monitor(window)
            .or_else(|_| resolve_primary_monitor(window)),
        MonitorStrategy::LastActive => resolve_last_active_monitor(window, last_monitor)
            .or_else(|_| resolve_cursor_monitor(window))
            .or_else(|_| resolve_primary_monitor(window)),
    }
}

fn resolve_cursor_monitor(window: &WebviewWindow) -> Result<MonitorBounds, String> {
    let cursor = create_provider()
        .get_mouse_position()
        .map_err(|e| e.to_string())?;

    let monitors = window.available_monitors().map_err(|e| e.to_string())?;
    monitors
        .into_iter()
        .map(|monitor| monitor_to_bounds(&monitor))
        .find(|bounds| point_in_monitor(cursor.x, cursor.y, *bounds))
        .ok_or_else(|| "no monitor under cursor".to_string())
}

fn resolve_last_active_monitor(
    window: &WebviewWindow,
    last_monitor: Option<MonitorSnapshot>,
) -> Result<MonitorBounds, String> {
    let Some(last_monitor) = last_monitor else {
        return Err("no last active monitor".to_string());
    };

    let monitors = window.available_monitors().map_err(|e| e.to_string())?;
    monitors
        .into_iter()
        .map(|monitor| monitor_to_bounds(&monitor))
        .find(|bounds| monitor_matches_snapshot(*bounds, last_monitor))
        .ok_or_else(|| "last active monitor unavailable".to_string())
}

fn resolve_primary_monitor(window: &WebviewWindow) -> Result<MonitorBounds, String> {
    window
        .primary_monitor()
        .map_err(|e| e.to_string())?
        .map(|monitor| monitor_to_bounds(&monitor))
        .ok_or_else(|| "no primary monitor".to_string())
}

fn monitor_to_bounds(monitor: &tauri::Monitor) -> MonitorBounds {
    let position = monitor.position();
    let size = monitor.size();

    MonitorBounds {
        x: position.x,
        y: position.y,
        width: size.width,
        height: size.height,
    }
}

fn monitor_to_snapshot(bounds: MonitorBounds) -> MonitorSnapshot {
    MonitorSnapshot {
        x: bounds.x,
        y: bounds.y,
        width: bounds.width,
        height: bounds.height,
    }
}

fn monitor_matches_snapshot(bounds: MonitorBounds, snapshot: MonitorSnapshot) -> bool {
    bounds.x == snapshot.x
        && bounds.y == snapshot.y
        && bounds.width == snapshot.width
        && bounds.height == snapshot.height
}

fn point_in_monitor(x: f64, y: f64, bounds: MonitorBounds) -> bool {
    x >= bounds.x as f64
        && x < (bounds.x + bounds.width as i32) as f64
        && y >= bounds.y as f64
        && y < (bounds.y + bounds.height as i32) as f64
}

pub fn emit_toggle_visibility(app: &AppHandle) -> Result<(), String> {
    app.emit(EVENT_TOGGLE_VISIBILITY, ())
        .map_err(|e| e.to_string())
}

pub fn open_settings_window(app: &AppHandle, expanded: bool) -> Result<(), String> {
    if !expanded {
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
    let height = 560.0;

    tauri::WebviewWindowBuilder::new(
        app,
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

/// Show the main popup bar window.
#[tauri::command]
pub async fn show_window(
    window: WebviewWindow,
    window_manager: State<'_, ManagerState>,
    bar_rect: State<'_, BarRectState>,
    last_monitor_state: State<'_, LastMonitorState>,
    settings_state: State<'_, SettingsState>,
) -> Result<Option<u64>, String> {
    let settings = settings_state
        .0
        .read()
        .map(|state| state.clone())
        .map_err(|_| "failed to read settings state".to_string())?;
    let label = window.label();

    let last_monitor = last_monitor_state
        .0
        .lock()
        .map_err(|_| "failed to lock last monitor state".to_string())
        .map(|state| *state)?;

    let mut manager = window_manager.0.lock().map_err(|_| {
        warn!("[show_window] failed to lock window manager");
        "failed to lock window manager".to_string()
    })?;

    let token = manager.request_show().map_err(|e| {
        warn!("[show_window] request_show failed: {}", e);
        e.to_string()
    })?;

    if token.is_some() {
        let positioned = position_on_monitor(
            &window,
            &settings.monitor_strategy,
            last_monitor,
            settings.bar_width_px,
            settings.bar_height_px,
        )?;

        if let Ok(mut rect) = bar_rect.0.lock() {
            *rect = positioned.rect.clone();
        }

        if let Ok(mut monitor_state) = last_monitor_state.0.lock() {
            *monitor_state = Some(positioned.monitor);
        }

        window.show().map_err(|e| {
            warn!("[show_window] window.show() failed for {}: {}", label, e);
            e.to_string()
        })?;
    }

    let wants_visible = manager.wants_visible();
    drop(manager);
    let _ = crate::update_tray_toggle_label(window.app_handle(), wants_visible);
    Ok(token)
}

/// Mark show animation lifecycle as completed.
#[tauri::command]
pub async fn complete_show_window(
    app: AppHandle,
    token: u64,
    window_manager: State<'_, ManagerState>,
) -> Result<bool, String> {
    let mut manager = window_manager
        .0
        .lock()
        .map_err(|_| "failed to lock window manager".to_string())?;
    let applied = manager.confirm_shown(token).map_err(|e| e.to_string())?;
    let wants_visible = manager.wants_visible();
    drop(manager);
    let _ = crate::update_tray_toggle_label(&app, wants_visible);
    Ok(applied)
}

/// Start hide lifecycle. The actual OS hide happens in `complete_hide_window`.
#[tauri::command]
pub async fn hide_window(
    app: AppHandle,
    window_manager: State<'_, ManagerState>,
) -> Result<Option<u64>, String> {
    let mut manager = window_manager
        .0
        .lock()
        .map_err(|_| "failed to lock window manager".to_string())?;
    let token = manager.request_hide().map_err(|e| e.to_string())?;
    let wants_visible = manager.wants_visible();
    drop(manager);
    let _ = crate::update_tray_toggle_label(&app, wants_visible);
    Ok(token)
}

/// Finalize hide lifecycle after animation completes.
#[tauri::command]
pub async fn complete_hide_window(
    window: WebviewWindow,
    token: u64,
    window_manager: State<'_, ManagerState>,
    bar_rect: State<'_, BarRectState>,
) -> Result<bool, String> {
    let mut manager = window_manager
        .0
        .lock()
        .map_err(|_| "failed to lock window manager".to_string())?;

    let applied = manager.confirm_hidden(token).map_err(|e| e.to_string())?;
    if applied {
        if let Ok(mut rect) = bar_rect.0.lock() {
            *rect = BarRect::default();
        }
        window.hide().map_err(|e| e.to_string())?;
    }

    let wants_visible = manager.wants_visible();
    drop(manager);
    let _ = crate::update_tray_toggle_label(window.app_handle(), wants_visible);
    Ok(applied)
}

/// Open the settings panel as a separate window.
#[tauri::command]
pub async fn set_settings_expanded(
    app: AppHandle,
    expanded: bool,
) -> Result<(), String> {
    open_settings_window(&app, expanded)
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
