//! Data model, CRUD operations, and SQLite persistence
//!
//! Manages the shelf items and groups stored in a local SQLite database.
//! Provides the core data layer for creating, reading, updating, and
//! deleting shelf entries.

use serde::{Deserialize, Serialize};

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
pub struct ShelfStore;

impl ShelfStore {
    /// Initialize the database schema.
    pub async fn init_db() -> Result<(), String> {
        todo!()
    }

    /// Get all shelf items.
    pub async fn get_all_items() -> Result<Vec<ShelfItem>, String> {
        todo!()
    }

    /// Add a new item to the shelf.
    pub async fn add_item(item: ShelfItem) -> Result<ShelfItem, String> {
        todo!()
    }

    /// Remove an item by ID.
    pub async fn remove_item(id: &str) -> Result<(), String> {
        todo!()
    }

    /// Update an existing item.
    pub async fn update_item(item: ShelfItem) -> Result<ShelfItem, String> {
        todo!()
    }

    /// Get all item groups.
    pub async fn get_all_groups() -> Result<Vec<ItemGroup>, String> {
        todo!()
    }

    /// Create a new item group.
    pub async fn create_group(group: ItemGroup) -> Result<ItemGroup, String> {
        todo!()
    }

    /// Delete a group and ungroup its items.
    pub async fn delete_group(id: &str) -> Result<(), String> {
        todo!()
    }
}
