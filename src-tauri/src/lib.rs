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
use std::sync::{Arc, Mutex};
use tauri::Manager;

/// Initialize and run the Tauri application.
pub fn run() {
    env_logger::init();
    info!("Starting Popup Bar v{}", env!("CARGO_PKG_VERSION"));

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
            let window = app.get_webview_window("main")
                .expect("main window not found");

            // Apply platform-specific vibrancy/transparency
            configure_window_vibrancy(&window);

            let bar_rect: Arc<Mutex<BarRect>> = Arc::new(Mutex::new(BarRect::default()));
            app.manage(bar_rect.clone());

            let hotzone_height = tauri::async_runtime::block_on(ConfigManager::load())
                .map(|s| s.hotzone_size)
                .unwrap_or(5);
            let mut hotzone_tracker = HotzoneTracker::new(HotzoneConfig {
                height: hotzone_height,
                enabled: true,
                delay_ms: 200,
            });
            let hotzone_start_ok = hotzone_tracker.start(app.handle().clone(), bar_rect).is_ok();
            if !hotzone_start_ok {
                warn!("Hotzone initialization failed");
            } else {
                info!("Hotzone tracker initialized");
            }

            match app.path().app_data_dir() {
                Ok(app_data_dir) => {
                    if let Err(e) = std::fs::create_dir_all(&app_data_dir) {
                        warn!("Could not create app data dir: {e}");
                    } else {
                        let db_path = app_data_dir.join("popup-bar.db");
                        ShelfStore::set_db_path(db_path.clone());
                        info!("Database path: {}", db_path.display());
                    }
                }
                Err(e) => warn!("App data dir not available: {e}"),
            }
            if let Err(err) = tauri::async_runtime::block_on(ShelfStore::init_db()) {
                warn!("ShelfStore init failed: {err}");
            } else {
                info!("ShelfStore initialized");
            }

            app.manage(Mutex::new(hotzone_tracker));
            app.manage(Mutex::new(PopupWindowManager::new(WindowConfig::default())));

            info!("Window configured");
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
            Ok(_) => info!("Windows Acrylic vibrancy applied"),
            Err(e) => log::warn!("Failed to apply Acrylic vibrancy: {e}"),
        }
    }

    #[cfg(target_os = "macos")]
    {
        use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
        match apply_vibrancy(_window, NSVisualEffectMaterial::HudWindow, None, None) {
            Ok(_) => info!("macOS vibrancy applied"),
            Err(e) => log::warn!("Failed to apply macOS vibrancy: {e}"),
        }
    }

    #[cfg(target_os = "linux")]
    {
        info!("Linux: no native vibrancy — using CSS backdrop-filter fallback");
    }
}
