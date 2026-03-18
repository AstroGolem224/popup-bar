//! Popup Bar — Application entry point
//!
//! Configures and launches the Tauri application with all plugins,
//! commands, and the platform-specific window vibrancy setup.

mod commands;
mod modules;

use commands::{shelf_commands, settings_commands, system_commands};
use log::{info, warn};
use modules::config::ConfigManager;
use modules::hotzone::{HotzoneConfig, HotzoneTracker};
use modules::shelf_store::ShelfStore;
use modules::window_manager::{BarRect, PopupWindowManager, WindowConfig};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::Manager;

pub struct ManagerState(pub Mutex<PopupWindowManager>);
pub struct BarRectState(pub Mutex<BarRect>);

/// Initialize and run the Tauri application.
pub fn run() {
    // Set up logging to both stderr AND a log file in the project directory
    let log_dir = {
        // Try well-known project dir first, then fall back to exe dir
        let project_dir = std::path::PathBuf::from(r"C:\Users\matth\OneDrive\Dokumente\GitHub\popup-bar\popup-bar");
        if project_dir.is_dir() {
            project_dir
        } else {
            std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(std::env::temp_dir)
        }
    };
    let date_str = chrono::Local::now().format("%Y-%m-%d").to_string();
    // Find next available sequence number
    let mut seq = 1u32;
    loop {
        let candidate = log_dir.join(format!("popup-bar_{}_{:03}.log", date_str, seq));
        if !candidate.exists() {
            break;
        }
        seq += 1;
    }
    let log_path = log_dir.join(format!("popup-bar_{}_{:03}.log", date_str, seq));
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_path)
        .ok();

    let log_path_display = log_path.display().to_string();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    let file_for_logger = log_file.map(|f| std::sync::Mutex::new(f));
    let file_arc: Option<std::sync::Arc<std::sync::Mutex<std::fs::File>>> =
        file_for_logger.map(|m| std::sync::Arc::new(m));
    let file_arc_clone = file_arc.clone();

    env_logger::Builder::from_default_env()
        .format(move |buf, record| {
            use std::io::Write;
            let line = format!(
                "[{} {} {}] {}\n",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                record.args()
            );
            let _ = buf.write_all(line.as_bytes());
            // Also write to file
            if let Some(ref file_mutex) = file_arc_clone {
                if let Ok(mut f) = file_mutex.lock() {
                    let _ = std::io::Write::write_all(&mut *f, line.as_bytes());
                }
            }
            Ok(())
        })
        .init();

    info!("Starting Popup Bar v{}", env!("CARGO_PKG_VERSION"));
    info!("Log file: {}", log_path_display);

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![
            settings_commands::set_launch_at_login,
            shelf_commands::get_shelf_items,
            shelf_commands::add_shelf_item,
            shelf_commands::remove_shelf_item,
            shelf_commands::update_shelf_item,
            shelf_commands::add_dropped_paths,
            shelf_commands::reorder_shelf_items,
            shelf_commands::get_item_groups,
            shelf_commands::create_item_group,
            shelf_commands::update_item_group,
            shelf_commands::delete_item_group,
            shelf_commands::get_icon_data,
            settings_commands::get_settings,
            settings_commands::update_settings,
            settings_commands::list_skins,
            settings_commands::import_skin,
            settings_commands::set_active_skin,
            settings_commands::delete_skin,
            settings_commands::get_skin_data,
            settings_commands::import_skin_bytes,
            system_commands::get_platform_info,
            system_commands::show_window,
            system_commands::complete_show_window,
            system_commands::hide_window,
            system_commands::complete_hide_window,
            system_commands::set_settings_expanded,
            system_commands::open_shelf_item,
            system_commands::exit_app,
        ])
        .setup(|app| {
            let main_window = app.get_webview_window("main")
                .expect("main window not found");

            // Apply platform-specific vibrancy/transparency
            configure_window_vibrancy(&main_window);

            // Register drag-drop handler for the main window (replaces JS-level listener for files)
            setup_drop_handler(app.handle(), &main_window);

            let bar_rect_mutex = Mutex::new(BarRect::default());
            app.manage(BarRectState(bar_rect_mutex));

            let manager_mutex = Mutex::new(PopupWindowManager::new(WindowConfig::default()));
            app.manage(ManagerState(manager_mutex));

            let settings = tauri::async_runtime::block_on(ConfigManager::load())
                .unwrap_or_default();
            
            let mut hotzone_tracker = HotzoneTracker::new(HotzoneConfig {
                size: settings.hotzone_size,
                top_enabled: true,
                delay_ms: 200,
            });
            let hotzone_start_ok = hotzone_tracker.start(app.handle().clone()).is_ok();
            if !hotzone_start_ok {
                warn!("Hotzone initialization failed");
            } else {
                info!("Hotzone tracker initialized");
            }

            // Main bar window is created via tauri.conf.json or already available

            match app.path().app_data_dir() {
                Ok(app_data_dir) => {
                    if let Err(e) = std::fs::create_dir_all(&app_data_dir) {
                        warn!("Could not create app data dir: {e}");
                    } else {
                        let db_path = app_data_dir.join("popup-bar.db");
                        ShelfStore::set_db_path(db_path.clone());
                        // info!("Database path: {}", db_path.display()); // Removed

                        let skins_dir = app_data_dir.join("skins");
                        if let Err(e) = std::fs::create_dir_all(&skins_dir) {
                            warn!("Could not create skins dir: {e}");
                        } else {
                            // info!("Skins directory: {}", skins_dir.display()); // Removed
                        }
                    }
                }
                Err(e) => warn!("App data dir not available: {e}"),
            }
            if let Err(err) = tauri::async_runtime::block_on(ShelfStore::init_db()) {
                warn!("ShelfStore init failed: {err}");
            } else {
                // info!("ShelfStore initialized"); // Removed
            }

            app.manage(Mutex::new(hotzone_tracker));

            // info!("Window configured"); // Removed
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Apply platform-specific window vibrancy effects.
///
/// - Windows: Acrylic blur via DWM
/// - macOS: NSVisualEffectView with HudWindow material
/// - Linux: CSS-only fallback (no native vibrancy)
fn configure_window_vibrancy(_window: &tauri::WebviewWindow) {
    #[cfg(target_os = "windows")]
    {
        use window_vibrancy::apply_acrylic;
        match apply_acrylic(_window, Some((18, 18, 18, 125))) {
            Ok(_) => {},
            Err(e) => log::warn!("Failed to apply Acrylic vibrancy: {e}"),
        }
    }

    #[cfg(target_os = "macos")]
    {
        use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
        match apply_vibrancy(_window, NSVisualEffectMaterial::HudWindow, None, None) {
            Ok(_) => {},
            Err(e) => log::warn!("Failed to apply macOS vibrancy: {e}"),
        }
    }

    #[cfg(target_os = "linux")]
    {
    }
}


/// Registers a Rust-level drag-and-drop handler on a window.
/// This ensures file drops are handled consistently and avoids duplicate event processing in JavaScript.
fn setup_drop_handler(app: &tauri::AppHandle, window: &tauri::WebviewWindow) {
    let win_label = window.label().to_string();
    let app_handle = app.clone();
    
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::DragDrop(drag_event) = event {
            match drag_event {
                tauri::DragDropEvent::Drop { paths, .. } => {
                    let label = win_label.clone();
                    let path_strings: Vec<String> = paths.iter()
                        .filter_map(|p| p.to_str().map(|s| s.to_string()))
                        .collect();
                    
                    if !path_strings.is_empty() {
                        let handle = app_handle.clone();
                        let container = "main";
                        tauri::async_runtime::spawn(async move {
                            match crate::commands::shelf_commands::add_dropped_paths(path_strings, Some(container.to_string())).await {
                                Ok(items) => {
                                    // Emit event so all windows refresh their shelf items
                                    use tauri::Emitter;
                                    let _ = handle.emit("shelf_items_changed", ());
                                }
                                Err(e) => warn!("[drop-handler] {} add_dropped_paths failed: {}", label, e),
                            }
                        });
                    }
                }
                _ => {}
            }
        }
    });
}
