//! Data model, CRUD operations, and SQLite persistence
//!
//! Manages the shelf items and groups stored in a local SQLite database.
//! Provides the core data layer for creating, reading, updating, and
//! deleting shelf entries.

use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::sync::OnceCell;
use uuid::Uuid;

/// The type of item stored on the shelf.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    File,
    Folder,
    App,
    Url,
}

impl FromStr for ItemType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "file" => Ok(Self::File),
            "folder" => Ok(Self::Folder),
            "app" => Ok(Self::App),
            "url" => Ok(Self::Url),
            _ => Err(format!("unsupported item type: {value}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// A single item on the shelf.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShelfItem {
    pub id: String,
    pub item_type: ItemType,
    pub path: String,
    pub display_name: String,
    pub icon_cache_key: String,
    pub position: Position,
    pub group_id: Option<String>,
    /// Which bar this item belongs to: "main", "left", or "right"
    #[serde(default = "default_container")]
    pub container: String,
    pub created_at: String,
    pub last_used: String,
}

fn default_container() -> String {
    "main".to_string()
}

/// A group of shelf items.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemGroup {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub position: Position,
}

pub struct ShelfStore;

static DB_PATH: OnceCell<PathBuf> = OnceCell::const_new();
static DB_POOL: OnceCell<sqlx::SqlitePool> = OnceCell::const_new();

impl ShelfStore {
    /// Set the database file path (must be called before any DB access, e.g. in setup).
    pub fn set_db_path(path: PathBuf) {
        let _ = DB_PATH.set(path);
    }

    /// Get the database path, or a default relative path if not set.
    pub fn get_db_path() -> PathBuf {
        DB_PATH.get().cloned().unwrap_or_else(|| PathBuf::from("popup-bar.db"))
    }

    pub async fn pool() -> Result<&'static sqlx::SqlitePool, String> {
        DB_POOL
            .get_or_try_init(|| async {
                let path = Self::get_db_path();
                let url = format!("sqlite:{}", path.display());
                let pool = sqlx::SqlitePool::connect(&url)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok::<sqlx::SqlitePool, String>(pool)
            })
            .await
    }

    /// Get the next available horizontal position for an item in a container.
    pub async fn get_next_position_x(container: &str) -> Result<f64, String> {
        let pool = Self::pool().await?;
        let row = sqlx::query("SELECT MAX(position_x) as max_x FROM shelf_items WHERE container = ?1")
            .bind(container)
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;
        
        let max_x: Option<f64> = row.try_get("max_x").unwrap_or(None);
        Ok(max_x.map(|x| x + 1.0).unwrap_or(0.0))
    }

    fn row_to_item(row: &sqlx::sqlite::SqliteRow) -> Result<ShelfItem, String> {
        let item_type_raw: String = row.try_get("item_type").map_err(|e| e.to_string())?;
        Ok(ShelfItem {
            id: row.try_get("id").map_err(|e| e.to_string())?,
            item_type: ItemType::from_str(&item_type_raw)?,
            path: row.try_get("path").map_err(|e| e.to_string())?,
            display_name: row.try_get("display_name").map_err(|e| e.to_string())?,
            icon_cache_key: row.try_get("icon_cache_key").map_err(|e| e.to_string())?,
            position: Position {
                x: row.try_get("position_x").map_err(|e| e.to_string())?,
                y: row.try_get("position_y").map_err(|e| e.to_string())?,
            },
            group_id: row.try_get("group_id").map_err(|e| e.to_string())?,
            container: row.try_get::<String, _>("container").unwrap_or_else(|_| "main".to_string()),
            created_at: row.try_get("created_at").map_err(|e| e.to_string())?,
            last_used: row.try_get("last_used").map_err(|e| e.to_string())?,
        })
    }

    fn row_to_group(row: &sqlx::sqlite::SqliteRow) -> Result<ItemGroup, String> {
        Ok(ItemGroup {
            id: row.try_get("id").map_err(|e| e.to_string())?,
            name: row.try_get("name").map_err(|e| e.to_string())?,
            color: row.try_get("color").map_err(|e| e.to_string())?,
            position: Position {
                x: row.try_get("position_x").map_err(|e| e.to_string())?,
                y: row.try_get("position_y").map_err(|e| e.to_string())?,
            },
        })
    }

    pub async fn init_db() -> Result<(), String> {
        let pool = Self::pool().await?;
        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        sqlx::query("PRAGMA journal_mode = WAL;")
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        sqlx::query("PRAGMA synchronous = NORMAL;")
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn get_all_items() -> Result<Vec<ShelfItem>, String> {
        let pool = Self::pool().await?;
        let rows = sqlx::query(
            "SELECT id, item_type, path, display_name, icon_cache_key, position_x, position_y, group_id, container, created_at, last_used
             FROM shelf_items
             ORDER BY position_x ASC, position_y ASC",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.iter().map(Self::row_to_item).collect()
    }

    /// Get items for a specific container ("main", "left", "right").
    pub async fn get_items_by_container(container: &str) -> Result<Vec<ShelfItem>, String> {
        let pool = Self::pool().await?;
        let rows = sqlx::query(
            "SELECT id, item_type, path, display_name, icon_cache_key, position_x, position_y, group_id, container, created_at, last_used
             FROM shelf_items
             WHERE container = ?1
             ORDER BY position_x ASC, position_y ASC",
        )
        .bind(container)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        rows.iter().map(Self::row_to_item).collect()
    }

    pub async fn add_item(mut item: ShelfItem) -> Result<ShelfItem, String> {
        Self::prepare_items_for_insert(std::slice::from_mut(&mut item)).await?;
        let pool = Self::pool().await?;

        sqlx::query(
            "INSERT INTO shelf_items (id, item_type, path, display_name, icon_cache_key, position_x, position_y, group_id, container, created_at, last_used)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        )
        .bind(&item.id)
        .bind(Self::item_type_as_str(&item.item_type))
        .bind(&item.path)
        .bind(&item.display_name)
        .bind(&item.icon_cache_key)
        .bind(item.position.x)
        .bind(item.position.y)
        .bind(&item.group_id)
        .bind(&item.container)
        .bind(&item.created_at)
        .bind(&item.last_used)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(item)
    }

    pub async fn add_items(mut items: Vec<ShelfItem>) -> Result<Vec<ShelfItem>, String> {
        if items.is_empty() {
            return Ok(items);
        }

        Self::prepare_items_for_insert(&mut items).await?;
        let pool = Self::pool().await?;
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

        for item in &items {
            sqlx::query(
                "INSERT INTO shelf_items (id, item_type, path, display_name, icon_cache_key, position_x, position_y, group_id, container, created_at, last_used)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            )
            .bind(&item.id)
            .bind(Self::item_type_as_str(&item.item_type))
            .bind(&item.path)
            .bind(&item.display_name)
            .bind(&item.icon_cache_key)
            .bind(item.position.x)
            .bind(item.position.y)
            .bind(&item.group_id)
            .bind(&item.container)
            .bind(&item.created_at)
            .bind(&item.last_used)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(items)
    }

    pub async fn remove_item(id: &str) -> Result<(), String> {
        let pool = Self::pool().await?;
        sqlx::query("DELETE FROM shelf_items WHERE id = ?1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn update_item(item: ShelfItem) -> Result<ShelfItem, String> {
        let pool = Self::pool().await?;
        sqlx::query(
            "UPDATE shelf_items
             SET item_type = ?1, path = ?2, display_name = ?3, icon_cache_key = ?4, position_x = ?5, position_y = ?6, group_id = ?7, container = ?8, last_used = ?9
             WHERE id = ?10",
        )
        .bind(match item.item_type {
            ItemType::File => "file",
            ItemType::Folder => "folder",
            ItemType::App => "app",
            ItemType::Url => "url",
        })
        .bind(&item.path)
        .bind(&item.display_name)
        .bind(&item.icon_cache_key)
        .bind(item.position.x)
        .bind(item.position.y)
        .bind(&item.group_id)
        .bind(&item.container)
        .bind(&item.last_used)
        .bind(&item.id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        let row = sqlx::query(
            "SELECT id, item_type, path, display_name, icon_cache_key, position_x, position_y, group_id, container, created_at, last_used
             FROM shelf_items WHERE id = ?1",
        )
        .bind(&item.id)
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())?;

        Self::row_to_item(&row)
    }

    pub async fn reorder_items(ordered_ids: Vec<String>) -> Result<(), String> {
        let pool = Self::pool().await?;
        let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
        for (index, id) in ordered_ids.iter().enumerate() {
            sqlx::query("UPDATE shelf_items SET position_x = ?1, position_y = 0.0 WHERE id = ?2")
                .bind(index as f64)
                .bind(id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
        }
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_all_groups() -> Result<Vec<ItemGroup>, String> {
        let pool = Self::pool().await?;
        let rows = sqlx::query(
            "SELECT id, name, color, position_x, position_y
             FROM item_groups
             ORDER BY position_x ASC, position_y ASC",
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
        rows.iter().map(Self::row_to_group).collect()
    }

    pub async fn create_group(group: ItemGroup) -> Result<ItemGroup, String> {
        let pool = Self::pool().await?;
        sqlx::query(
            "INSERT INTO item_groups (id, name, color, position_x, position_y)
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )
        .bind(&group.id)
        .bind(&group.name)
        .bind(&group.color)
        .bind(group.position.x)
        .bind(group.position.y)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(group)
    }

    pub async fn update_group(group: &ItemGroup) -> Result<(), String> {
        let pool = Self::pool().await?;
        sqlx::query(
            "UPDATE item_groups SET name = ?1, color = ?2, position_x = ?3, position_y = ?4 WHERE id = ?5",
        )
        .bind(&group.name)
        .bind(&group.color)
        .bind(group.position.x)
        .bind(group.position.y)
        .bind(&group.id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn delete_group(id: &str) -> Result<(), String> {
        let pool = Self::pool().await?;
        sqlx::query("DELETE FROM item_groups WHERE id = ?1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn build_item_from_inputs(path: String, item_type: ItemType, container: &str) -> ShelfItem {
        let display_name = std::path::Path::new(&path)
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| path.clone());

        ShelfItem {
            id: Uuid::new_v4().to_string(),
            item_type,
            path,
            display_name,
            icon_cache_key: String::new(),
            position: Position { x: 0.0, y: 0.0 },
            group_id: None,
            container: container.to_string(),
            created_at: String::new(),
            last_used: String::new(),
        }
    }

    async fn prepare_items_for_insert(items: &mut [ShelfItem]) -> Result<(), String> {
        if items.is_empty() {
            return Ok(());
        }

        let mut next_positions = HashMap::new();

        for item in items.iter() {
            if item.position.x != 0.0 || next_positions.contains_key(&item.container) {
                continue;
            }

            let next_x = Self::get_next_position_x(&item.container).await?;
            next_positions.insert(item.container.clone(), next_x);
        }

        for item in items.iter_mut() {
            if item.position.x == 0.0 {
                let next_x = next_positions.entry(item.container.clone()).or_insert(0.0);
                item.position.x = *next_x;
                *next_x += 1.0;
            }

            let now = Self::current_timestamp();
            if item.created_at.is_empty() {
                item.created_at = now.clone();
            }
            if item.last_used.is_empty() {
                item.last_used = now;
            }
        }

        Ok(())
    }

    fn current_timestamp() -> String {
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }

    fn item_type_as_str(item_type: &ItemType) -> &'static str {
        match item_type {
            ItemType::File => "file",
            ItemType::Folder => "folder",
            ItemType::App => "app",
            ItemType::Url => "url",
        }
    }
}
