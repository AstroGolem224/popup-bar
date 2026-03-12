//! Mouse tracking & hotzone detection
//!
//! Monitors cursor position and triggers the popup bar when the mouse
//! enters the configurable hotzone at the top edge of the screen.
//! Full implementation in Phase 1.

use serde::{Deserialize, Serialize};
use log::info;

/// Defines the rectangular hotzone region at the top of the screen.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotzoneConfig {
    /// Height of the hotzone trigger area in pixels.
    pub height: u32,
    /// Whether the hotzone is currently enabled.
    pub enabled: bool,
    /// Delay before triggering (debounce) in milliseconds.
    pub delay_ms: u64,
}

impl Default for HotzoneConfig {
    fn default() -> Self {
        Self {
            height: 5,
            enabled: true,
            delay_ms: 200,
        }
    }
}

/// Tracks mouse position and manages hotzone activation state.
pub struct HotzoneTracker {
    config: HotzoneConfig,
    is_active: bool,
}

impl HotzoneTracker {
    /// Create a new tracker with the given config.
    pub fn new(config: HotzoneConfig) -> Self {
        Self {
            config,
            is_active: false,
        }
    }

    /// Start listening for mouse events in the hotzone.
    /// Phase 1: Platform-specific implementation via PlatformProvider.
    pub fn start(&mut self) -> Result<(), String> {
        info!("HotzoneTracker: start requested (stub — Phase 1)");
        self.is_active = true;
        Ok(())
    }

    /// Stop listening for mouse events.
    pub fn stop(&mut self) -> Result<(), String> {
        info!("HotzoneTracker: stop requested (stub — Phase 1)");
        self.is_active = false;
        Ok(())
    }

    /// Check if cursor is currently within the hotzone.
    /// Always returns false until Phase 1 implementation.
    pub fn is_cursor_in_hotzone(&self) -> bool {
        false
    }

    /// Update hotzone configuration at runtime.
    pub fn update_config(&mut self, config: HotzoneConfig) {
        self.config = config;
    }

    /// Whether the tracker is currently running.
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
