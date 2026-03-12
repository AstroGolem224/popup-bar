//! Linux platform implementation

use super::{MousePosition, PlatformProvider};

pub struct LinuxProvider;

impl LinuxProvider {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformProvider for LinuxProvider {
    fn register_hotzone(&self, _height: u32) -> Result<(), String> {
        todo!()
    }

    fn unregister_hotzone(&self) -> Result<(), String> {
        todo!()
    }

    fn get_mouse_position(&self) -> Result<MousePosition, String> {
        todo!()
    }

    fn set_window_vibrancy(&self, _blur_radius: f64, _tint_color: &str) -> Result<(), String> {
        todo!()
    }

    fn extract_icon(&self, _path: &str, _size: u32) -> Result<Vec<u8>, String> {
        todo!()
    }

    fn launch_item(&self, _path: &str) -> Result<(), String> {
        todo!()
    }
}
