//! Mouse tracking & hotzone detection
//!
//! Monitors cursor position and triggers the popup bar when the mouse
//! enters the configurable hotzone at the top edge of the screen.

use serde::{Deserialize, Serialize};

/// Defines the rectangular hotzone region at the top of the screen.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotzoneConfig {
    pub height: u32,
    pub enabled: bool,
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
    pub fn new(config: HotzoneConfig) -> Self {
        Self {
            config,
            is_active: false,
        }
    }

    /// Start listening for mouse events in the hotzone.
    pub fn start(&mut self) -> Result<(), String> {
        todo!()
    }

    /// Stop listening for mouse events.
    pub fn stop(&mut self) -> Result<(), String> {
        todo!()
    }

    /// Check if cursor is currently within the hotzone.
    pub fn is_cursor_in_hotzone(&self) -> bool {
        todo!()
    }

    /// Update hotzone configuration at runtime.
    pub fn update_config(&mut self, config: HotzoneConfig) {
        self.config = config;
    }
}
