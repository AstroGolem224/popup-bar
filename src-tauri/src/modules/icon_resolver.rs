//! Icon extraction & caching
//!
//! Extracts icons from files, applications, and URLs using platform-native
//! APIs. Caches extracted icons to disk for fast subsequent lookups.
//! Full implementation in Phase 4.

use serde::{Deserialize, Serialize};
use log::info;

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
    pub async fn resolve_icon(&self, _path: &str) -> Result<CachedIcon, String> {
        info!("IconResolver: resolve_icon (stub — Phase 4)");
        Err("Icon resolution not implemented (Phase 4)".into())
    }

    /// Get a cached icon by its cache key.
    pub fn get_cached(&self, _cache_key: &str) -> Option<CachedIcon> {
        info!("IconResolver: get_cached (stub — Phase 4)");
        None
    }

    /// Clear the entire icon cache.
    pub fn clear_cache(&self) -> Result<(), String> {
        info!("IconResolver: clear_cache (stub — Phase 4)");
        Ok(())
    }

    /// Remove a single cached icon.
    pub fn evict(&self, _cache_key: &str) -> Result<(), String> {
        info!("IconResolver: evict (stub — Phase 4)");
        Ok(())
    }

    /// Get the cache directory path.
    pub fn cache_dir(&self) -> &str {
        &self.cache_dir
    }
}
