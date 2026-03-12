//! Settings management
//!
//! Persists and retrieves user preferences including hotzone sensitivity,
//! animation parameters, glassmorphism settings, and startup behavior.

use serde::{Deserialize, Serialize};

/// Application settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub hotzone_size: u32,
    pub animation_speed: f64,
    pub blur_intensity: f64,
    pub tint_color: String,
    pub theme: Theme,
    pub autostart: bool,
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
pub struct ConfigManager;

impl ConfigManager {
    /// Load settings from disk, returning defaults if none exist.
    pub fn load() -> Result<AppSettings, String> {
        todo!()
    }

    /// Save settings to disk.
    pub fn save(settings: &AppSettings) -> Result<(), String> {
        todo!()
    }

    /// Reset settings to defaults.
    pub fn reset() -> Result<AppSettings, String> {
        todo!()
    }
}
