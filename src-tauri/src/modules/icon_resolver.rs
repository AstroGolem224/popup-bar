//! Icon extraction & caching
//!
//! Extracts icons from files, applications, and URLs using platform-native
//! APIs. Caches extracted icons to disk for fast subsequent lookups.

use serde::{Deserialize, Serialize};

/// Cached icon metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedIcon {
    pub cache_key: String,
    pub path: String,
    pub format: IconFormat,
    pub size: u32,
}

/// Supported icon output formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IconFormat {
    Png,
    Svg,
}

/// Resolves and caches icons for shelf items.
pub struct IconResolver {
    cache_dir: String,
}

impl IconResolver {
    pub fn new(cache_dir: String) -> Self {
        Self { cache_dir }
    }

    /// Extract an icon for the given file path and cache it.
    pub async fn resolve_icon(&self, path: &str) -> Result<CachedIcon, String> {
        todo!()
    }

    /// Get a cached icon by its cache key.
    pub fn get_cached(&self, cache_key: &str) -> Option<CachedIcon> {
        todo!()
    }

    /// Clear the entire icon cache.
    pub fn clear_cache(&self) -> Result<(), String> {
        todo!()
    }

    /// Remove a single cached icon.
    pub fn evict(&self, cache_key: &str) -> Result<(), String> {
        todo!()
    }
}
