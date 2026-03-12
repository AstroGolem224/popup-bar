//! Settings management
//!
//! Persists and retrieves user preferences including hotzone sensitivity,
//! animation parameters, glassmorphism settings, and startup behavior.
//! Phase 2: SQLite-backed persistence. Phase 0: In-memory defaults only.

use serde::{Deserialize, Serialize};
use log::info;

/// Application settings with sensible defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Hotzone trigger height in pixels.
    pub hotzone_size: u32,
    /// Animation speed multiplier (1.0 = normal).
    pub animation_speed: f64,
    /// CSS backdrop-filter blur radius.
    pub blur_intensity: f64,
    /// RGBA tint color for the glassmorphism overlay.
    pub tint_color: String,
    /// Theme selection.
    pub theme: Theme,
    /// Launch on system startup.
    pub autostart: bool,
    /// Show on all monitors or primary only.
    pub multi_monitor: bool,
}

/// Theme selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            hotzone_size: 5,
            animation_speed: 1.0,
            blur_intensity: 20.0,
            tint_color: String::from("rgba(255, 255, 255, 0.1)"),
            theme: Theme::System,
            autostart: false,
            multi_monitor: false,
        }
    }
}

/// Manages reading and writing application settings.
/// Phase 2: SQLite-backed. Phase 0: Returns defaults.
pub struct ConfigManager;

impl ConfigManager {
    /// Load settings from disk, returning defaults if none exist.
    pub fn load() -> Result<AppSettings, String> {
        info!("ConfigManager: load (returning defaults — Phase 2 for persistence)");
        Ok(AppSettings::default())
    }

    /// Save settings to disk.
    pub fn save(_settings: &AppSettings) -> Result<(), String> {
        info!("ConfigManager: save (stub — Phase 2)");
        Ok(())
    }

    /// Reset settings to defaults.
    pub fn reset() -> Result<AppSettings, String> {
        info!("ConfigManager: reset to defaults");
        Ok(AppSettings::default())
    }
}
