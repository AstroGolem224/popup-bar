//! Tauri commands for shelf item CRUD operations.
//!
//! Exposes shelf data operations to the React frontend via Tauri's
//! invoke API.

use crate::modules::dnd_handler::DndHandler;
use crate::modules::icon_resolver::IconResolver;
use crate::modules::shelf_store::{ItemGroup, ItemType, Position, ShelfItem, ShelfStore};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use log::warn;
use std::path::Path;
use std::str::FromStr;
use uuid::Uuid;

#[tauri::command]
pub async fn get_shelf_items() -> Result<Vec<ShelfItem>, String> {
    ShelfStore::get_all_items().await
}

#[tauri::command]
pub async fn add_shelf_item(path: String, item_type: String) -> Result<ShelfItem, String> {
    let parsed_type = ItemType::from_str(&item_type)?;
    let mut item = ShelfStore::build_item_from_inputs(path.clone(), parsed_type);
    let resolver = IconResolver::new(
        std::env::temp_dir()
            .join("popup-bar-icon-cache")
            .to_string_lossy()
            .to_string(),
    );
    match resolver.resolve_icon(&path).await {
        Ok(icon) => item.icon_cache_key = icon.path,
        Err(err) => warn!("add_shelf_item: icon resolution failed: {err}"),
    }
    ShelfStore::add_item(item).await
}

#[tauri::command]
pub async fn remove_shelf_item(id: String) -> Result<(), String> {
    ShelfStore::remove_item(&id).await
}

#[tauri::command]
pub async fn update_shelf_item(item: ShelfItem) -> Result<ShelfItem, String> {
    ShelfStore::update_item(item).await
}

#[tauri::command]
pub async fn add_dropped_paths(paths: Vec<String>) -> Result<Vec<ShelfItem>, String> {
    let mut items = DndHandler::build_items_from_paths(paths)?;
    let resolver = IconResolver::new(
        std::env::temp_dir()
            .join("popup-bar-icon-cache")
            .to_string_lossy()
            .to_string(),
    );
    for item in &mut items {
        match resolver.resolve_icon(&item.path).await {
            Ok(icon) => item.icon_cache_key = icon.path,
            Err(err) => warn!("add_dropped_paths: icon resolution failed: {err}"),
        }
    }
    ShelfStore::add_items(items).await
}

#[tauri::command]
pub async fn reorder_shelf_items(ordered_ids: Vec<String>) -> Result<(), String> {
    ShelfStore::reorder_items(ordered_ids).await
}

/// Fetch all item groups from the store.
#[tauri::command]
pub async fn get_item_groups() -> Result<Vec<ItemGroup>, String> {
    ShelfStore::get_all_groups().await
}

/// Create a new item group with a generated id and default position.
#[tauri::command]
pub async fn create_item_group(name: String, color: Option<String>) -> Result<ItemGroup, String> {
    let group = ItemGroup {
        id: Uuid::new_v4().to_string(),
        name,
        color,
        position: Position { x: 0.0, y: 0.0 },
    };
    ShelfStore::create_group(group).await
}

/// Update an existing item group (name, color, position).
#[tauri::command]
pub async fn update_item_group(group: ItemGroup) -> Result<ItemGroup, String> {
    ShelfStore::update_group(&group).await?;
    Ok(group.clone())
}

/// Delete an item group by id.
#[tauri::command]
pub async fn delete_item_group(id: String) -> Result<(), String> {
    ShelfStore::delete_group(&id).await
}

/// Icon cache dir used for resolving and validating icon paths.
fn icon_cache_dir() -> std::path::PathBuf {
    std::env::temp_dir().join("popup-bar-icon-cache")
}

/// Return icon file as base64 + mime so the frontend can show it without asset protocol.
/// Only serves files under the icon cache directory.
#[tauri::command]
pub async fn get_icon_data(icon_path: String) -> Result<(String, String), String> {
    let path = Path::new(&icon_path);
    let cache_dir = icon_cache_dir();
    if !path.starts_with(&cache_dir) || !path.exists() {
        return Err("get_icon_data: path not in cache or missing".into());
    }
    let bytes = tokio::fs::read(&path).await.map_err(|e| e.to_string())?;
    let mime: String = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| match e.to_lowercase().as_str() {
            "png" => "image/png",
            "svg" => "image/svg+xml",
            _ => "application/octet-stream",
        })
        .unwrap_or("application/octet-stream")
        .to_string();
    Ok((BASE64.encode(&bytes), mime))
}
