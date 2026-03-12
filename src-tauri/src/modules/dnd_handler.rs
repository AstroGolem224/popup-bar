//! OS-native drag & drop handling
//!
//! Manages drag-and-drop interactions for adding items to the shelf,
//! reordering existing items, and grouping items together.
//! Full implementation in Phase 3.

use serde::{Deserialize, Serialize};
use log::info;

/// The source of a drag operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DragSource {
    /// Item dragged from outside the app (e.g., desktop, file manager).
    External,
    /// Item being reordered within the shelf.
    Internal { item_id: String },
}

/// Payload carried during a drag operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragPayload {
    /// Where the drag originated.
    pub source: DragSource,
    /// File/folder paths being dragged.
    pub paths: Vec<String>,
    /// Drop position X coordinate.
    pub position_x: f64,
    /// Drop position Y coordinate.
    pub position_y: f64,
}

/// Result of a completed drop operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropResult {
    /// Whether the drop was accepted.
    pub accepted: bool,
    /// Target group ID if dropped into a group.
    pub target_group_id: Option<String>,
    /// Final X position of the dropped item.
    pub new_position_x: f64,
    /// Final Y position of the dropped item.
    pub new_position_y: f64,
}

/// Handles OS-native drag and drop events.
/// Phase 3: Tauri file-drop integration + custom URL handling.
pub struct DndHandler;

impl DndHandler {
    /// Register drag-and-drop listeners on the window.
    pub fn register_listeners() -> Result<(), String> {
        info!("DndHandler: register_listeners (stub — Phase 3)");
        Ok(())
    }

    /// Unregister drag-and-drop listeners.
    pub fn unregister_listeners() -> Result<(), String> {
        info!("DndHandler: unregister_listeners (stub — Phase 3)");
        Ok(())
    }

    /// Process a drop event and determine the result.
    pub fn handle_drop(_payload: DragPayload) -> Result<DropResult, String> {
        info!("DndHandler: handle_drop (stub — Phase 3)");
        Ok(DropResult {
            accepted: false,
            target_group_id: None,
            new_position_x: 0.0,
            new_position_y: 0.0,
        })
    }
}
