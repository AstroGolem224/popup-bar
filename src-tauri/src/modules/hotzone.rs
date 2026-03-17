#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//! Mouse tracking & hotzone detection
//!
//! Monitors cursor position and triggers the popup bar when the mouse
//! enters the configurable hotzone at the top edge of the screen.
//! Full implementation in Phase 1.

use crate::modules::window_manager::BarRect;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
    Mutex,
};
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HotzoneAction {
    None,
    Enter,
    Leave,
}

#[derive(Debug, Clone, Copy)]
struct HotzoneEvaluation {
    in_hotzone_active: bool,
    enter_candidate_since: Option<Instant>,
    action: HotzoneAction,
}

fn evaluate_hotzone_transition(
    in_hotzone_now: bool,
    in_hotzone_active: bool,
    enter_candidate_since: Option<Instant>,
    now: Instant,
    delay: Duration,
) -> HotzoneEvaluation {
    if in_hotzone_now {
        if in_hotzone_active {
            return HotzoneEvaluation {
                in_hotzone_active: true,
                enter_candidate_since: None,
                action: HotzoneAction::None,
            };
        }

        let started = enter_candidate_since.unwrap_or(now);
        if now.duration_since(started) >= delay {
            HotzoneEvaluation {
                in_hotzone_active: true,
                enter_candidate_since: None,
                action: HotzoneAction::Enter,
            }
        } else {
            HotzoneEvaluation {
                in_hotzone_active: false,
                enter_candidate_since: Some(started),
                action: HotzoneAction::None,
            }
        }
    } else if in_hotzone_active {
        HotzoneEvaluation {
            in_hotzone_active: false,
            enter_candidate_since: None,
            action: HotzoneAction::Leave,
        }
    } else {
        HotzoneEvaluation {
            in_hotzone_active: false,
            enter_candidate_since: None,
            action: HotzoneAction::None,
        }
    }
}

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
    stop_signal: Arc<AtomicBool>,
}

impl HotzoneTracker {
    /// Create a new tracker with the given config.
    pub fn new(config: HotzoneConfig) -> Self {
        Self {
            config,
            is_active: false,
            stop_signal: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start listening for mouse events in the hotzone.
    ///
    /// Current implementation: lightweight polling loop with platform cursor API.
    /// Emits:
    /// - `hotzone:enter` with `{ x, y }`
    /// - `hotzone:leave` with `null`
    pub fn start(&mut self, app_handle: AppHandle, bar_rect: Arc<Mutex<BarRect>>) -> Result<(), String> {
        if self.is_active {
            return Ok(());
        }

        info!("HotzoneTracker: starting");
        let config = self.config.clone();
        let stop_signal = Arc::clone(&self.stop_signal);
        stop_signal.store(false, Ordering::Relaxed);

        thread::spawn(move || {
            let provider = crate::modules::platform::create_provider();
            let mut in_hotzone_active = false;
            let mut enter_candidate_since: Option<Instant> = None;

            loop {
                if stop_signal.load(Ordering::Relaxed) {
                    break;
                }

                if !config.enabled {
                    thread::sleep(Duration::from_millis(200));
                    continue;
                }

                match provider.get_mouse_position() {
                    Ok(pos) => {
                        let in_hotzone_now = pos.y <= config.height as f64;
                        let now = Instant::now();
                        let evaluation = evaluate_hotzone_transition(
                            in_hotzone_now,
                            in_hotzone_active,
                            enter_candidate_since,
                            now,
                            Duration::from_millis(config.delay_ms),
                        );

                        let over_bar = bar_rect
                            .lock()
                            .map(|r| r.contains(pos.x, pos.y))
                            .unwrap_or(false);

                        // If we would leave but cursor is over the bar, keep bar open (don't emit leave).
                        let leave_suppressed = matches!(evaluation.action, HotzoneAction::Leave) && over_bar;
                        if matches!(evaluation.action, HotzoneAction::Leave) && !over_bar {
                            in_hotzone_active = false;
                            enter_candidate_since = None;
                            if let Err(err) = app_handle.emit("hotzone:leave", ()) {
                                warn!("HotzoneTracker: failed to emit hotzone:leave: {err}");
                            }
                        } else if !leave_suppressed {
                            in_hotzone_active = evaluation.in_hotzone_active;
                            enter_candidate_since = evaluation.enter_candidate_since;
                        }
                        if matches!(evaluation.action, HotzoneAction::Enter) {
                            if let Err(err) =
                                app_handle.emit("hotzone:enter", serde_json::json!({ "x": pos.x, "y": pos.y }))
                            {
                                warn!("HotzoneTracker: failed to emit hotzone:enter: {err}");
                            }
                        }

                        thread::sleep(Duration::from_millis(25));
                    }
                    Err(err) => {
                        // Avoid noisy spin-loops if a platform provider is not ready yet.
                        warn!("HotzoneTracker: cursor position unavailable: {err}");
                        thread::sleep(Duration::from_millis(500));
                    }
                }
            }
        });

        self.is_active = true;
        Ok(())
    }

    /// Stop listening for mouse events.
    pub fn stop(&mut self) -> Result<(), String> {
        if !self.is_active {
            return Ok(());
        }

        info!("HotzoneTracker: stopping");
        self.stop_signal.store(true, Ordering::Relaxed);
        self.is_active = false;
        Ok(())
    }

    /// Check if cursor is currently within the hotzone.
    /// Returns `true` while the tracker thread is running.
    pub fn is_cursor_in_hotzone(&self) -> bool {
        self.is_active
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

#[cfg(test)]
mod tests {
    use super::{evaluate_hotzone_transition, HotzoneAction};
    use std::time::{Duration, Instant};

    #[test]
    fn emits_enter_only_after_delay_elapsed() {
        let t0 = Instant::now();
        let delay = Duration::from_millis(200);

        let first = evaluate_hotzone_transition(true, false, None, t0, delay);
        assert_eq!(first.action, HotzoneAction::None);
        assert_eq!(first.in_hotzone_active, false);
        assert!(first.enter_candidate_since.is_some());

        let second = evaluate_hotzone_transition(
            true,
            first.in_hotzone_active,
            first.enter_candidate_since,
            t0 + Duration::from_millis(210),
            delay,
        );
        assert_eq!(second.action, HotzoneAction::Enter);
        assert_eq!(second.in_hotzone_active, true);
        assert!(second.enter_candidate_since.is_none());
    }

    #[test]
    fn emits_leave_immediately_when_cursor_exits_active_hotzone() {
        let now = Instant::now();
        let result =
            evaluate_hotzone_transition(false, true, None, now, Duration::from_millis(200));
        assert_eq!(result.action, HotzoneAction::Leave);
        assert_eq!(result.in_hotzone_active, false);
    }

    #[test]
    fn leaving_before_delay_resets_enter_candidate() {
        let t0 = Instant::now();
        let delay = Duration::from_millis(200);

        let entering = evaluate_hotzone_transition(true, false, None, t0, delay);
        assert!(entering.enter_candidate_since.is_some());

        let left_too_soon = evaluate_hotzone_transition(
            false,
            entering.in_hotzone_active,
            entering.enter_candidate_since,
            t0 + Duration::from_millis(50),
            delay,
        );
        assert_eq!(left_too_soon.action, HotzoneAction::None);
        assert!(left_too_soon.enter_candidate_since.is_none());
        assert_eq!(left_too_soon.in_hotzone_active, false);
    }
}
