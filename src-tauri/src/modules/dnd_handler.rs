//! OS-native drag & drop handling
//!
//! Manages drag-and-drop interactions for adding items to the shelf,
//! reordering existing items, and grouping items together.

#![allow(dead_code)]

use crate::modules::shelf_store::{ItemType, ShelfItem, ShelfStore};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::path::Path;

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
    /// Classify a dropped path into a shelf item type.
    pub fn classify_path(path: &Path) -> ItemType {
        if path.is_dir() {
            #[cfg(target_os = "macos")]
            {
                if path.extension().and_then(|e| e.to_str()) == Some("app") {
                    return ItemType::App;
                }
            }
            return ItemType::Folder;
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let lower_ext = ext.to_ascii_lowercase();
                if lower_ext == "lnk" || lower_ext == "exe" {
                    return ItemType::App;
                }
            }
        }

        ItemType::File
    }

    /// Percent-decode (e.g. %3A -> :, %2F -> /) for file URLs.
    fn percent_decode(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '%' {
                let a = chars.next().unwrap_or('\0');
                let b = chars.next().unwrap_or('\0');
                if a.is_ascii_hexdigit() && b.is_ascii_hexdigit() {
                    let byte = u8::from_str_radix(&format!("{a}{b}"), 16).unwrap_or(0);
                    out.push(char::from(byte));
                    continue;
                }
                out.push('%');
                out.push(a);
                out.push(b);
            } else {
                out.push(c);
            }
        }
        out
    }

    /// Normalize path: strip file:// prefix, percent-decode, use OS separator.
    pub fn normalize_path(path: &str) -> String {
        let s = path.trim();
        let decoded = if s.contains('%') {
            Self::percent_decode(s)
        } else {
            s.to_string()
        };
        let stripped = if decoded.starts_with("file:///") {
            decoded[7..].replace('/', std::path::MAIN_SEPARATOR_STR)
        } else if decoded.starts_with("file://") {
            decoded[6..].to_string()
        } else {
            decoded
        };
        stripped.trim().to_string()
    }

    /// Validate dropped paths before persistence.
    pub fn validate_paths(paths: &[String]) -> Result<(), String> {
        if paths.is_empty() {
            return Err("drop payload is empty".into());
        }

        for path in paths {
            let normalized = Self::normalize_path(path);
            if normalized.is_empty() {
                return Err("Pfad ist leer".into());
            }
            if !Path::new(&normalized).exists() {
                warn!("DndHandler: path does not exist (normalized): {}", normalized);
                return Err(format!("Pfad existiert nicht: {}", normalized));
            }
        }

        Ok(())
    }

    /// Build shelf items from validated dropped paths (files, folders, apps).
    pub fn build_items_from_paths(paths: Vec<String>, container: &str) -> Result<Vec<ShelfItem>, String> {
        let normalized: Vec<String> = paths
            .into_iter()
            .map(|p| Self::normalize_path(&p))
            .collect();
        Self::validate_paths(&normalized)?;
        Ok(normalized
            .into_iter()
            .map(|path| {
                let item_type = Self::classify_path(Path::new(&path));
                ShelfStore::build_item_from_inputs(path, item_type, container)
            })
            .collect())
    }

    /// Register drag-and-drop listeners on the window.
    pub fn register_listeners() -> Result<(), String> {
        info!("DndHandler: register_listeners (Phase 3 baseline)");
        Ok(())
    }

    /// Unregister drag-and-drop listeners.
    pub fn unregister_listeners() -> Result<(), String> {
        info!("DndHandler: unregister_listeners (stub — Phase 3)");
        Ok(())
    }

    /// Process a drop event and determine the result.
    pub fn handle_drop(_payload: DragPayload) -> Result<DropResult, String> {
        info!("DndHandler: handle_drop (Phase 3 baseline)");
        Ok(DropResult {
            accepted: false,
            target_group_id: None,
            new_position_x: 0.0,
            new_position_y: 0.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    fn temp_file() -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push("popup_bar_dnd_test.tmp");
        let mut file = fs::File::create(&path).expect("create temp file");
        writeln!(file, "test").expect("write temp file");
        path
    }

    #[test]
    fn classify_path_treats_dirs_and_files() {
        let file_path = temp_file();
        let ty = DndHandler::classify_path(&file_path);
        assert!(matches!(ty, ItemType::File));
    }

    #[test]
    fn validate_paths_rejects_empty_and_missing() {
        assert!(DndHandler::validate_paths(&[]).is_err());
        assert!(DndHandler::validate_paths(&["Z:/definitely/not/here".into()]).is_err());
    }

    #[test]
    fn validate_paths_accepts_existing_file() {
        let p = temp_file();
        let s = p.to_string_lossy().to_string();
        assert!(DndHandler::validate_paths(&[s]).is_ok());
    }
}
