//! Data model, CRUD operations, and SQLite persistence
//!
//! Manages the shelf items and groups stored in a local SQLite database.
//! Provides the core data layer for creating, reading, updating, and
//! deleting shelf entries. Full SQLite integration in Phase 2.

use serde::{Deserialize, Serialize};
use log::info;

/// The type of item stored on the shelf.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    File,
    Folder,
    App,
    Url,
}

/// A single item on the shelf.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShelfItem {
    pub id: String,
    pub item_type: ItemType,
    pub path: String,
    pub display_name: String,
    pub icon_cache_key: String,
    pub position_x: f64,
    pub position_y: f64,
    pub group_id: Option<String>,
    pub created_at: String,
    pub last_used: String,
}

/// A group of shelf items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemGroup {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub position_x: f64,
    pub position_y: f64,
}

/// Handles all shelf data persistence.
/// Phase 2: Backed by SQLite via tauri-plugin-sql.
pub struct ShelfStore;

impl ShelfStore {
    /// Initialize the database schema.
    /// Phase 2: Runs CREATE TABLE migrations.
    pub async fn init_db() -> Result<(), String> {
        info!("ShelfStore: init_db (stub — Phase 2)");
        Ok(())
    }

    /// Get all shelf items. Returns empty vec until Phase 2.
    pub async fn get_all_items() -> Result<Vec<ShelfItem>, String> {
        info!("ShelfStore: get_all_items (stub — Phase 2)");
        Ok(vec![])
    }

    /// Add a new item to the shelf.
    pub async fn add_item(_item: ShelfItem) -> Result<ShelfItem, String> {
        Err("ShelfStore: add_item not implemented (Phase 2)".into())
    }

    /// Remove an item by ID.
    pub async fn remove_item(_id: &str) -> Result<(), String> {
        Err("ShelfStore: remove_item not implemented (Phase 2)".into())
    }

    /// Update an existing item.
    pub async fn update_item(_item: ShelfItem) -> Result<ShelfItem, String> {
        Err("ShelfStore: update_item not implemented (Phase 2)".into())
    }

    /// Get all item groups. Returns empty vec until Phase 2.
    pub async fn get_all_groups() -> Result<Vec<ItemGroup>, String> {
        info!("ShelfStore: get_all_groups (stub — Phase 2)");
        Ok(vec![])
    }

    /// Create a new item group.
    pub async fn create_group(_group: ItemGroup) -> Result<ItemGroup, String> {
        Err("ShelfStore: create_group not implemented (Phase 2)".into())
    }

    /// Delete a group and ungroup its items.
    pub async fn delete_group(_id: &str) -> Result<(), String> {
        Err("ShelfStore: delete_group not implemented (Phase 2)".into())
    }
}
