#![allow(dead_code)]
#![allow(clippy::all)]
//! Icon extraction & caching
//!
//! Extracts icons from files, applications, and URLs using platform-native
//! APIs. Caches extracted icons to disk for fast subsequent lookups.

use crate::modules::platform::create_provider;
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

/// Cached icon metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedIcon {
    /// Unique cache key (hash of source path + size).
    pub cache_key: String,
    /// Path to the cached icon file on disk.
    pub path: String,
    /// Output format of the cached icon.
    pub format: IconFormat,
    /// Icon size in pixels (square).
    pub size: u32,
}

/// Supported icon output formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IconFormat {
    Png,
    Svg,
}

/// Resolves and caches icons for shelf items.
/// Phase 4: Platform-specific icon extraction + LRU disk cache.
pub struct IconResolver {
    cache_dir: String,
}

impl IconResolver {
    /// Create a new resolver with the given cache directory.
    pub fn new(cache_dir: String) -> Self {
        Self { cache_dir }
    }

    /// Extract an icon for the given file path and cache it.
    pub async fn resolve_icon(&self, path: &str) -> Result<CachedIcon, String> {
        let cache_key = Self::hash_path(path);
        if let Some(cached) = self.get_cached(&cache_key) {
            return Ok(cached);
        }

        let db_path = crate::modules::shelf_store::ShelfStore::get_db_path();
        let url = format!("sqlite:{}", db_path.display());
        let pool = sqlx::SqlitePool::connect(&url)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(existing_row) = sqlx::query(
            "SELECT icon_path, source_path FROM icon_cache WHERE hash = ?1",
        )
        .bind(&cache_key)
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?
        {
            let existing_path: String =
                existing_row.try_get("icon_path").map_err(|e| e.to_string())?;
            let source_path: String = existing_row
                .try_get("source_path")
                .unwrap_or_else(|_| String::new());
            if Path::new(&existing_path).exists() {
                if source_path.is_empty() || Path::new(&source_path).exists() {
                    return Ok(CachedIcon {
                        cache_key,
                        path: existing_path,
                        format: IconFormat::Png,
                        size: 256,
                    });
                }
            }
            let _ = self.evict(&cache_key);
            let _ = sqlx::query("DELETE FROM icon_cache WHERE hash = ?1")
                .bind(&cache_key)
                .execute(&pool)
                .await;
        }

        let provider = create_provider();
        let icon_bytes = provider.extract_icon(path, 256);

        let cache_dir = PathBuf::from(&self.cache_dir);
        tokio::fs::create_dir_all(&cache_dir)
            .await
            .map_err(|e| e.to_string())?;

        let (file_path, format) = match icon_bytes {
            Ok(bytes) => {
                let icon_path = cache_dir.join(format!("{cache_key}.png"));
                tokio::fs::write(&icon_path, bytes)
                    .await
                    .map_err(|e| e.to_string())?;
                (icon_path, IconFormat::Png)
            }
            Err(_) => {
                let icon_path = cache_dir.join(format!("{cache_key}.svg"));
                let svg = Self::fallback_svg(path);
                tokio::fs::write(&icon_path, svg)
                    .await
                    .map_err(|e| e.to_string())?;
                (icon_path, IconFormat::Svg)
            }
        };

        sqlx::query(
            "INSERT OR REPLACE INTO icon_cache (hash, icon_path, source_path, created_at) VALUES (?1, ?2, ?3, strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))",
        )
        .bind(&cache_key)
        .bind(file_path.to_string_lossy().to_string())
        .bind(path)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        info!("IconResolver: resolved icon for path");
        Ok(CachedIcon {
            cache_key,
            path: file_path.to_string_lossy().to_string(),
            format,
            size: 256,
        })
    }

    /// Get a cached icon by its cache key.
    pub fn get_cached(&self, cache_key: &str) -> Option<CachedIcon> {
        let png = PathBuf::from(&self.cache_dir).join(format!("{cache_key}.png"));
        if png.exists() {
            return Some(CachedIcon {
                cache_key: cache_key.to_string(),
                path: png.to_string_lossy().to_string(),
                format: IconFormat::Png,
                size: 256,
            });
        }

        let svg = PathBuf::from(&self.cache_dir).join(format!("{cache_key}.svg"));
        if svg.exists() {
            return Some(CachedIcon {
                cache_key: cache_key.to_string(),
                path: svg.to_string_lossy().to_string(),
                format: IconFormat::Svg,
                size: 256,
            });
        }

        None
    }

    /// Clear the entire icon cache.
    pub fn clear_cache(&self) -> Result<(), String> {
        let cache_dir = PathBuf::from(&self.cache_dir);
        if cache_dir.exists() {
            std::fs::remove_dir_all(&cache_dir).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    /// Remove a single cached icon.
    pub fn evict(&self, cache_key: &str) -> Result<(), String> {
        for ext in ["png", "svg"] {
            let icon_path = PathBuf::from(&self.cache_dir).join(format!("{cache_key}.{ext}"));
            if icon_path.exists() {
                std::fs::remove_file(icon_path).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    /// Get the cache directory path.
    pub fn cache_dir(&self) -> &str {
        &self.cache_dir
    }

    fn hash_path(path: &str) -> String {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    }

    fn fallback_svg(path: &str) -> String {
        let icon = if path.starts_with("http://") || path.starts_with("https://") {
            "URL".to_string()
        } else {
            let extension = Path::new(path)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_ascii_uppercase())
                .unwrap_or_else(|| "FILE".to_string());
            if extension.len() <= 4 {
                extension
            } else {
                "APP".to_string()
            }
        };

        format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256" viewBox="0 0 256 256">
  <rect x="16" y="16" width="224" height="224" rx="42" fill="#1f2430"/>
  <rect x="22" y="22" width="212" height="212" rx="36" fill="#2b3245"/>
  <text x="128" y="144" text-anchor="middle" font-family="Segoe UI, Arial, sans-serif" font-size="54" font-weight="700" fill="#f0f3ff">{}</text>
</svg>"##,
            icon
        )
    }
}
