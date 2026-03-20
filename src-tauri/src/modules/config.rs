//! Settings management
//!
//! Persists and retrieves user preferences via the `settings` SQLite table.
//! Keys match frontend (camelCase) for JSON round-trip.

use log::info;
use serde::{Deserialize, Serialize};
use sqlx::Row;

/// Metadata for a stored skin file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinInfo {
    pub name: String,
    pub filename: String,
    pub absolute_path: String,
}

const SETTINGS_KEY: &str = "app";
pub const DEFAULT_GLOBAL_SHORTCUT: &str = "CommandOrControl+Shift+Space";

/// Application settings with sensible defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub hotzone_size: u32,
    pub animation_speed: f64,
    pub blur_intensity: f64,
    pub tint_color: String,
    pub theme: Theme,
    pub autostart: bool,
    #[serde(default = "default_global_shortcut")]
    pub global_shortcut: String,
    pub multi_monitor: bool,
    #[serde(default = "default_monitor_strategy")]
    pub monitor_strategy: MonitorStrategy,
    /// Bar width in pixels; 0 = use fraction of screen (1/3).
    #[serde(default = "default_bar_width_px")]
    pub bar_width_px: u32,
    /// Bar height in pixels.
    #[serde(default = "default_bar_height_px")]
    pub bar_height_px: u32,
    /// Filename of the active skin in {app_data}/skins/, or None for default glassmorphism.
    #[serde(default)]
    pub active_skin: Option<String>,
    #[serde(default = "default_alignment")]
    pub alignment: String,
}

fn default_alignment() -> String {
    "centered".to_string()
}

fn default_global_shortcut() -> String {
    DEFAULT_GLOBAL_SHORTCUT.to_string()
}

fn default_bar_width_px() -> u32 {
    480
}

fn default_bar_height_px() -> u32 {
    72
}

fn default_monitor_strategy() -> MonitorStrategy {
    MonitorStrategy::Primary
}

/// Theme selection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum MonitorStrategy {
    Primary,
    Cursor,
    LastActive,
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
            global_shortcut: default_global_shortcut(),
            multi_monitor: false,
            monitor_strategy: default_monitor_strategy(),
            bar_width_px: 480,
            bar_height_px: 72,
            active_skin: None,
            alignment: default_alignment(),
        }
    }
}

impl AppSettings {
    pub fn normalize(mut self) -> Self {
        self.global_shortcut = self.global_shortcut.trim().to_string();
        if self.multi_monitor && self.monitor_strategy == MonitorStrategy::Primary {
            self.monitor_strategy = MonitorStrategy::Cursor;
        }
        self.multi_monitor = self.monitor_strategy != MonitorStrategy::Primary;
        self
    }
}

/// Load/save settings from the `settings` table (key = "app", value = JSON).
pub struct ConfigManager;

impl ConfigManager {
    /// Load settings from SQLite; returns defaults if no row or parse error.
    pub async fn load() -> Result<AppSettings, String> {
        let pool = crate::modules::shelf_store::ShelfStore::pool().await?;

        let row = sqlx::query("SELECT value FROM settings WHERE key = ?1")
            .bind(SETTINGS_KEY)
            .fetch_optional::<&sqlx::SqlitePool>(pool)
            .await
            .map_err(|e| e.to_string())?;

        match row {
            Some(r) => {
                let value: String = r.try_get("value").map_err(|e: sqlx::Error| e.to_string())?;
                serde_json::from_str::<AppSettings>(&value)
                    .map(|settings| settings.normalize())
                    .map_err(|e| format!("settings parse: {e}"))
            }
            None => {
                info!("ConfigManager: no settings row, using defaults");
                Ok(AppSettings::default())
            }
        }
    }

    /// Save settings to SQLite.
    pub async fn save(settings: &AppSettings) -> Result<(), String> {
        let pool = crate::modules::shelf_store::ShelfStore::pool().await?;
        let settings = settings.clone().normalize();

        let value = serde_json::to_string(&settings).map_err(|e| e.to_string())?;
        sqlx::query(
            "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?1, ?2, strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))",
        )
        .bind(SETTINGS_KEY)
        .bind(&value)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
        info!("ConfigManager: settings saved");
        Ok(())
    }

    /// Reset to defaults and return them.
    #[allow(dead_code)]
    pub async fn reset() -> Result<AppSettings, String> {
        let defaults = AppSettings::default();
        Self::save(&defaults).await?;
        Ok(defaults)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_settings_default_is_sane() {
        let s = AppSettings::default();
        assert_eq!(s.hotzone_size, 5);
        assert!((s.animation_speed - 1.0).abs() < f64::EPSILON);
        assert!(s.blur_intensity >= 0.0);
        assert_eq!(s.theme, Theme::System);
    }

    #[test]
    fn app_settings_roundtrip_json_matches_camel_case() {
        let original = AppSettings::default();
        let json = serde_json::to_string(&original).expect("serialize");
        // ensure camelCase keys are present
        assert!(json.contains("hotzoneSize"));
        assert!(json.contains("animationSpeed"));
        assert!(json.contains("blurIntensity"));
        assert!(json.contains("tintColor"));
        assert!(json.contains("globalShortcut"));
        assert!(json.contains("multiMonitor"));
        assert!(json.contains("monitorStrategy"));
        assert!(json.contains("barWidthPx"));
        assert!(json.contains("barHeightPx"));

        let decoded: AppSettings = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded.hotzone_size, original.hotzone_size);
        assert_eq!(decoded.animation_speed, original.animation_speed);
        assert_eq!(decoded.blur_intensity, original.blur_intensity);
        assert_eq!(decoded.tint_color, original.tint_color);
        assert_eq!(decoded.theme, original.theme);
        assert_eq!(decoded.autostart, original.autostart);
        assert_eq!(decoded.global_shortcut, original.global_shortcut);
        assert_eq!(decoded.multi_monitor, original.multi_monitor);
        assert_eq!(decoded.monitor_strategy, original.monitor_strategy);
        assert_eq!(decoded.bar_width_px, original.bar_width_px);
        assert_eq!(decoded.bar_height_px, original.bar_height_px);
        assert_eq!(decoded.active_skin, original.active_skin);
    }

    #[test]
    fn normalize_maps_legacy_multi_monitor_to_cursor_strategy() {
        let legacy_json = r#"{
            "hotzoneSize": 5,
            "animationSpeed": 1.0,
            "blurIntensity": 20.0,
            "tintColor": "rgba(255, 255, 255, 0.1)",
            "theme": "system",
            "autostart": false,
            "multiMonitor": true,
            "barWidthPx": 480,
            "barHeightPx": 72,
            "activeSkin": null,
            "alignment": "centered"
        }"#;

        let decoded = serde_json::from_str::<AppSettings>(legacy_json)
            .expect("legacy settings should deserialize")
            .normalize();

        assert_eq!(decoded.monitor_strategy, MonitorStrategy::Cursor);
        assert!(decoded.multi_monitor);
        assert_eq!(decoded.global_shortcut, DEFAULT_GLOBAL_SHORTCUT);
    }
}
