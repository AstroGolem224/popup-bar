//! OS-native drag & drop handling
//!
//! Manages drag-and-drop interactions for adding items to the shelf,
//! reordering existing items, and grouping items together.

use serde::{Deserialize, Serialize};

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
    pub source: DragSource,
    pub paths: Vec<String>,
    pub position_x: f64,
    pub position_y: f64,
}

/// Result of a completed drop operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropResult {
    pub accepted: bool,
    pub target_group_id: Option<String>,
    pub new_position_x: f64,
    pub new_position_y: f64,
}

/// Handles OS-native drag and drop events.
pub struct DndHandler;

impl DndHandler {
    /// Register drag-and-drop listeners on the window.
    pub fn register_listeners() -> Result<(), String> {
        todo!()
    }

    /// Unregister drag-and-drop listeners.
    pub fn unregister_listeners() -> Result<(), String> {
        todo!()
    }

    /// Process a drop event and determine the result.
    pub fn handle_drop(payload: DragPayload) -> Result<DropResult, String> {
        todo!()
    }
}
