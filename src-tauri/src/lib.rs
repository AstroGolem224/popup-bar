mod commands;
mod modules;

use commands::{shelf_commands, settings_commands, system_commands};

pub fn run() {
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
        .setup(|_app| {
            // TODO: Initialize hotzone listener
            // TODO: Initialize database
            // TODO: Set up system tray
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
