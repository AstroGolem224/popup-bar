//! Popup Bar — Application entry point
//!
//! Configures and launches the Tauri application with all plugins,
//! commands, and the platform-specific window vibrancy setup.

mod commands;
mod modules;

use commands::{shelf_commands, settings_commands, system_commands};
use log::info;
use tauri::Manager;

/// Initialize and run the Tauri application.
pub fn run() {
    env_logger::init();
    info!("Starting Popup Bar v{}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            shelf_commands::get_shelf_items,
            shelf_commands::add_shelf_item,
            shelf_commands::remove_shelf_item,
            shelf_commands::update_shelf_item,
            settings_commands::get_settings,
            settings_commands::update_settings,
            system_commands::get_platform_info,
            system_commands::show_window,
            system_commands::hide_window,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main")
                .expect("main window not found");

            // Apply platform-specific vibrancy/transparency
            configure_window_vibrancy(&window);

            info!("Window configured — hotzone will be initialized in Phase 1");
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
