//! Tauri commands for shelf item CRUD operations.
//!
//! Exposes shelf data operations to the React frontend via Tauri's
//! invoke API. Full implementation in Phase 2.

use crate::modules::shelf_store::ShelfItem;

/// Get all shelf items. Returns empty list until Phase 2.
#[tauri::command]
pub async fn get_shelf_items() -> Result<Vec<ShelfItem>, String> {
    Ok(vec![])
}

/// Add a new item to the shelf.
#[tauri::command]
pub async fn add_shelf_item(_path: String, _item_type: String) -> Result<ShelfItem, String> {
    Err("add_shelf_item not implemented (Phase 2)".into())
}

/// Remove an item from the shelf by ID.
#[tauri::command]
pub async fn remove_shelf_item(_id: String) -> Result<(), String> {
    Err("remove_shelf_item not implemented (Phase 2)".into())
}

/// Update an existing shelf item.
#[tauri::command]
pub async fn update_shelf_item(_item: ShelfItem) -> Result<ShelfItem, String> {
    Err("update_shelf_item not implemented (Phase 2)".into())
}
