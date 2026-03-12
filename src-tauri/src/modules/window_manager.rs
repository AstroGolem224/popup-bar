//! Window position, size, and transparency management
//!
//! Controls the popup bar window lifecycle including slide-down animations,
//! positioning across monitors, and transparency/vibrancy effects.
//! Full animation logic in Phase 1.

use serde::{Deserialize, Serialize};
use log::info;

/// Window display state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WindowState {
    /// Window is not visible and not rendering.
    Hidden,
    /// Window is animating into view.
    Showing,
    /// Window is fully visible and interactive.
    Visible,
    /// Window is animating out of view.
    Hiding,
}

/// Configuration for the popup bar window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Window width in pixels.
    pub width: u32,
    /// Window height in pixels.
    pub height: u32,
    /// Which monitor to display on (0-indexed).
    pub monitor_index: usize,
    /// Slide animation duration in milliseconds.
    pub animation_duration_ms: u64,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 300,
            monitor_index: 0,
            animation_duration_ms: 250,
        }
    }
}

/// Manages the popup bar window state and transitions.
pub struct PopupWindowManager {
    state: WindowState,
    config: WindowConfig,
}

impl PopupWindowManager {
    /// Create a new window manager with the given configuration.
    pub fn new(config: WindowConfig) -> Self {
        Self {
            state: WindowState::Hidden,
            config,
        }
    }

    /// Slide the popup bar down into view.
    /// Phase 1: Triggers CSS animation + window show.
    pub fn show(&mut self) -> Result<(), String> {
        info!("WindowManager: show requested (stub — Phase 1)");
        self.state = WindowState::Visible;
        Ok(())
    }

    /// Slide the popup bar up out of view.
    /// Phase 1: Triggers CSS animation + window hide after animation ends.
    pub fn hide(&mut self) -> Result<(), String> {
        info!("WindowManager: hide requested (stub — Phase 1)");
        self.state = WindowState::Hidden;
        Ok(())
    }

    /// Reposition window to a specific monitor.
    /// Phase 5: Multi-monitor support.
    pub fn move_to_monitor(&mut self, monitor_index: usize) -> Result<(), String> {
        info!("WindowManager: move_to_monitor({monitor_index}) (stub — Phase 5)");
        self.config.monitor_index = monitor_index;
        Ok(())
    }

    /// Apply transparency and vibrancy effects.
    /// Handled in lib.rs setup via configure_window_vibrancy().
    pub fn apply_vibrancy(&self, _blur_intensity: f64, _tint_color: &str) -> Result<(), String> {
        info!("WindowManager: apply_vibrancy (stub — handled in lib.rs setup)");
        Ok(())
    }

    /// Get current window state.
    pub fn state(&self) -> &WindowState {
        &self.state
    }
}
