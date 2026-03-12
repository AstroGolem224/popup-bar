//! Tauri commands for shelf item CRUD operations.

use crate::modules::shelf_store::ShelfItem;

#[tauri::command]
pub async fn get_shelf_items() -> Result<Vec<ShelfItem>, String> {
    todo!()
}

#[tauri::command]
pub async fn add_shelf_item(path: String, item_type: String) -> Result<ShelfItem, String> {
    let _ = (path, item_type);
    todo!()
}

#[tauri::command]
pub async fn remove_shelf_item(id: String) -> Result<(), String> {
    let _ = id;
    todo!()
}

#[tauri::command]
pub async fn update_shelf_item(item: ShelfItem) -> Result<ShelfItem, String> {
    let _ = item;
    todo!()
}
