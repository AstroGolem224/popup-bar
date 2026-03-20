//! Mouse tracking & hotzone detection
//!
//! Monitors cursor position and triggers the popup bar when the mouse
//! enters the configurable hotzone at the top edge of the screen.
//! Full implementation in Phase 1.

use log::{info, warn};
use tauri::Manager;
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
    Mutex,
};
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Edge {
    Top,
}

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

/// Defines the rectangular hotzone regions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotzoneConfig {
    /// Height/Width of the hotzone trigger area in pixels.
    pub size: u32,
    /// Whether the top hotzone is enabled (always true for Top per user requirement).
    pub top_enabled: bool,
    /// Delay before triggering (debounce) in milliseconds.
    pub delay_ms: u64,
}

impl Default for HotzoneConfig {
    fn default() -> Self {
        Self {
            size: 5,
            top_enabled: true,
            delay_ms: 200,
        }
    }
}

/// Tracks mouse position across multiple edges and manages hotzone activation state.
pub struct HotzoneTracker {
    config: Arc<Mutex<HotzoneConfig>>,
    is_active: bool,
    stop_signal: Arc<AtomicBool>,
}

#[derive(Default)]
struct EdgeState {
    in_hotzone_active: bool,
    enter_candidate_since: Option<Instant>,
}

impl HotzoneTracker {
    /// Create a new tracker with the given config.
    pub fn new(config: HotzoneConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            is_active: false,
            stop_signal: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start listening for mouse events in the hotzones.
    ///
    /// Current implementation: lightweight polling loop with platform cursor API.
    /// Emits:
    /// - `hotzone:enter` with `{ edge, x, y }`
    /// - `hotzone:leave` with `{ edge }`
    pub fn start(
        &mut self,
        app_handle: AppHandle,
    ) -> Result<(), String> {
        if self.is_active {
            return Ok(());
        }

        info!("HotzoneTracker: starting");
        let config_shared = Arc::clone(&self.config);
        let stop_signal = Arc::clone(&self.stop_signal);
        stop_signal.store(false, Ordering::Relaxed);

        thread::spawn(move || {
            let provider = crate::modules::platform::create_provider();
            let mut top_state = EdgeState::default();

            loop {
                if stop_signal.load(Ordering::Relaxed) {
                    break;
                }

                match provider.get_mouse_position() {
                    Ok(pos) => {
                        let (delay, size) = {
                            let config = config_shared.lock().unwrap();
                            (Duration::from_millis(config.delay_ms), config.size)
                        };

                        let monitor = provider.get_primary_monitor();
                        let (screen_x, screen_y, screen_w, _screen_h) = monitor
                            .map(|m| (m.x as f64, m.y as f64, m.width as f64, m.height as f64))
                            .unwrap_or((0.0, 0.0, 1920.0, 1080.0));

                        let rel_x = pos.x - screen_x;
                        let rel_y = pos.y - screen_y;
                        let now = Instant::now();

                        let in_now = rel_y >= 0.0 && rel_y <= size as f64 && rel_x >= 0.0 && rel_x <= screen_w;
                        Self::process_edge(
                            Edge::Top,
                            in_now,
                            &mut top_state,
                            &pos,
                            &app_handle,
                            now,
                            delay,
                        );

                        // Dynamic sleep: if far from top, sleep longer
                        let sleep_dur = if rel_y > 200.0 {
                            Duration::from_millis(100)
                        } else {
                            Duration::from_millis(25)
                        };
                        thread::sleep(sleep_dur);
                    }
                    Err(err) => {
                        warn!("HotzoneTracker: cursor position unavailable: {err}");
                        thread::sleep(Duration::from_millis(500));
                    }
                }
            }
        });

        self.is_active = true;
        Ok(())
    }

    fn process_edge(
        edge: Edge,
        in_now: bool,
        state: &mut EdgeState,
        pos: &crate::modules::platform::MousePosition,
        app_handle: &AppHandle,
        now: Instant,
        delay: Duration,
    ) {
        let evaluation = evaluate_hotzone_transition(
            in_now,
            state.in_hotzone_active,
            state.enter_candidate_since,
            now,
            delay,
        );

        let over_bar = app_handle.try_state::<crate::BarRectState>()
            .and_then(|s| s.0.lock().ok())
            .map(|r| r.contains(pos.x, pos.y))
            .unwrap_or(false);

        let leave_suppressed = matches!(evaluation.action, HotzoneAction::Leave) && over_bar;

        if matches!(evaluation.action, HotzoneAction::Leave) && !over_bar {
            state.in_hotzone_active = false;
            state.enter_candidate_since = None;
            let _ = app_handle.emit("hotzone:leave", serde_json::json!({ "edge": edge }));
        } else if !leave_suppressed {
            state.in_hotzone_active = evaluation.in_hotzone_active;
            state.enter_candidate_since = evaluation.enter_candidate_since;
        }
 
        if matches!(evaluation.action, HotzoneAction::Enter) {
            let _ = app_handle.emit(
                "hotzone:enter",
                serde_json::json!({ "edge": edge, "x": pos.x, "y": pos.y }),
            );
        }
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
        info!("[hotzone] update_config called: size={}", config.size);
        if let Ok(mut c) = self.config.lock() {
            *c = config;
            info!("[hotzone] config updated successfully");
        } else {
            warn!("[hotzone] FAILED to lock config for update!");
        }
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
