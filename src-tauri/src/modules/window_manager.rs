//! Window position, size, and transparency management
//!
//! Controls the popup bar window lifecycle including slide-down animations,
//! positioning across monitors, and transparency/vibrancy effects.
//! Full animation logic in Phase 1.

use serde::{Deserialize, Serialize};
use log::info;

/// Window display state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// Errors for invalid window state transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowStateError {
    InvalidTransition { from: WindowState, action: &'static str },
}

impl std::fmt::Display for WindowStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTransition { from, action } => {
                write!(f, "invalid transition: cannot {action} from state {from:?}")
            }
        }
    }
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

/// Screen rectangle of the bar window. Used so the hotzone does not emit
/// leave while the cursor is over the bar (user can click icons).
#[derive(Debug, Clone, Default)]
pub struct BarRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl BarRect {
    /// Returns true if the point (e.g. cursor) is inside this rect.
    pub fn contains(&self, px: f64, py: f64) -> bool {
        if self.width == 0 || self.height == 0 {
            return false;
        }
        let px = px as i32;
        let py = py as i32;
        px >= self.x && px < self.x + self.width as i32 && py >= self.y && py < self.y + self.height as i32
    }
}

/// Manages the popup bar window state and transitions.
pub struct PopupWindowManager {
    state: WindowState,
    #[allow(dead_code)]
    config: WindowConfig,
    next_transition_token: u64,
    pending_transition: Option<PendingTransition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransitionKind {
    Show,
    Hide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PendingTransition {
    kind: TransitionKind,
    token: u64,
}

impl PopupWindowManager {
    /// Create a new window manager with the given configuration.
    pub fn new(config: WindowConfig) -> Self {
        Self {
            state: WindowState::Hidden,
            config,
            next_transition_token: 1,
            pending_transition: None,
        }
    }

    /// Request a transition toward visible state.
    ///
    /// Allowed:
    /// - Hidden -> Showing
    /// - Hiding -> Showing (interrupt hide)
    /// - Showing -> Showing (idempotent)
    /// - Visible -> Visible (idempotent)
    pub fn request_show(&mut self) -> Result<Option<u64>, WindowStateError> {
        match self.state {
            WindowState::Hidden | WindowState::Hiding => {
                self.state = WindowState::Showing;
                let token = self.allocate_transition_token();
                self.pending_transition = Some(PendingTransition {
                    kind: TransitionKind::Show,
                    token,
                });
                Ok(Some(token))
            }
            WindowState::Showing | WindowState::Visible => {
                // idempotent
                Ok(self.pending_token_for(TransitionKind::Show))
            }
        }
    }

    /// Confirm that the window is now visible.
    ///
    /// Returns:
    /// - `Ok(true)` when the token matched and transition was applied
    /// - `Ok(false)` when token was stale/outdated and ignored
    pub fn confirm_shown(&mut self, token: u64) -> Result<bool, WindowStateError> {
        let Some(pending) = self.pending_transition else {
            return Ok(false);
        };

        if pending.kind != TransitionKind::Show || pending.token != token {
            return Ok(false);
        }

        match self.state {
            WindowState::Showing | WindowState::Visible => {
                self.state = WindowState::Visible;
                self.pending_transition = None;
                Ok(true)
            }
            _ => Err(WindowStateError::InvalidTransition {
                from: self.state.clone(),
                action: "confirm shown",
            }),
        }
    }

    /// Request a transition toward hidden state.
    ///
    /// Allowed:
    /// - Visible -> Hiding
    /// - Showing -> Hiding (interrupt show)
    /// - Hiding -> Hiding (idempotent)
    /// - Hidden -> Hidden (idempotent)
    pub fn request_hide(&mut self) -> Result<Option<u64>, WindowStateError> {
        match self.state {
            WindowState::Visible | WindowState::Showing => {
                self.state = WindowState::Hiding;
                let token = self.allocate_transition_token();
                self.pending_transition = Some(PendingTransition {
                    kind: TransitionKind::Hide,
                    token,
                });
                Ok(Some(token))
            }
            WindowState::Hiding | WindowState::Hidden => {
                // idempotent
                Ok(self.pending_token_for(TransitionKind::Hide))
            }
        }
    }

    /// Confirm that the window is now hidden.
    ///
    /// Returns:
    /// - `Ok(true)` when the token matched and transition was applied
    /// - `Ok(false)` when token was stale/outdated and ignored
    pub fn confirm_hidden(&mut self, token: u64) -> Result<bool, WindowStateError> {
        let Some(pending) = self.pending_transition else {
            return Ok(false);
        };

        if pending.kind != TransitionKind::Hide || pending.token != token {
            return Ok(false);
        }

        match self.state {
            WindowState::Hiding | WindowState::Hidden => {
                self.state = WindowState::Hidden;
                self.pending_transition = None;
                Ok(true)
            }
            _ => Err(WindowStateError::InvalidTransition {
                from: self.state.clone(),
                action: "confirm hidden",
            }),
        }
    }

    /// Reposition window to a specific monitor.
    /// Phase 5: Multi-monitor support.
    #[allow(dead_code)]
    pub fn move_to_monitor(&mut self, monitor_index: usize) -> Result<(), String> {
        info!("WindowManager: move_to_monitor({monitor_index}) (stub — Phase 5)");
        self.config.monitor_index = monitor_index;
        Ok(())
    }

    /// Apply transparency and vibrancy effects.
    /// Handled in lib.rs setup via configure_window_vibrancy().
    #[allow(dead_code)]
    pub fn apply_vibrancy(&self, _blur_intensity: f64, _tint_color: &str) -> Result<(), String> {
        info!("WindowManager: apply_vibrancy (stub — handled in lib.rs setup)");
        Ok(())
    }

    /// Get current window state.
    #[allow(dead_code)]
    pub fn state(&self) -> &WindowState {
        &self.state
    }

    fn allocate_transition_token(&mut self) -> u64 {
        let token = self.next_transition_token;
        self.next_transition_token = self.next_transition_token.saturating_add(1);
        token
    }

    fn pending_token_for(&self, kind: TransitionKind) -> Option<u64> {
        self.pending_transition
            .filter(|pending| pending.kind == kind)
            .map(|pending| pending.token)
    }
}

#[cfg(test)]
mod tests {
    use super::{PopupWindowManager, WindowConfig, WindowState};

    #[test]
    fn hidden_to_visible_path_is_valid() {
        let mut manager = PopupWindowManager::new(WindowConfig::default());
        assert_eq!(manager.state(), &WindowState::Hidden);

        let show_token = manager
            .request_show()
            .expect("request_show should succeed")
            .expect("show should produce a transition token");
        assert_eq!(manager.state(), &WindowState::Showing);

        assert!(
            manager
                .confirm_shown(show_token)
                .expect("confirm_shown should succeed")
        );
        assert_eq!(manager.state(), &WindowState::Visible);
    }

    #[test]
    fn visible_to_hidden_path_is_valid() {
        let mut manager = PopupWindowManager::new(WindowConfig::default());
        let show_token = manager.request_show().unwrap().unwrap();
        manager.confirm_shown(show_token).unwrap();

        let hide_token = manager
            .request_hide()
            .expect("request_hide should succeed")
            .expect("hide should produce a transition token");
        assert_eq!(manager.state(), &WindowState::Hiding);

        assert!(
            manager
                .confirm_hidden(hide_token)
                .expect("confirm_hidden should succeed")
        );
        assert_eq!(manager.state(), &WindowState::Hidden);
    }

    #[test]
    fn cannot_confirm_hidden_from_visible() {
        let mut manager = PopupWindowManager::new(WindowConfig::default());
        let show_token = manager.request_show().unwrap().unwrap();
        manager.confirm_shown(show_token).unwrap();

        assert_eq!(
            manager
                .confirm_hidden(999)
                .expect("stale token should be ignored, not errored"),
            false
        );
    }

    #[test]
    fn stale_show_completion_is_ignored_after_hide_request() {
        let mut manager = PopupWindowManager::new(WindowConfig::default());

        let show_token = manager.request_show().unwrap().unwrap();
        let hide_token = manager.request_hide().unwrap().unwrap();

        assert_eq!(manager.confirm_shown(show_token).unwrap(), false);
        assert_eq!(manager.confirm_hidden(hide_token).unwrap(), true);
        assert_eq!(manager.state(), &WindowState::Hidden);
    }
}
