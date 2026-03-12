//! Platform-specific abstractions
//!
//! Defines a common trait for platform-dependent operations and provides
//! implementations for Windows, macOS, and Linux.

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

/// Mouse position coordinates.
pub struct MousePosition {
    pub x: f64,
    pub y: f64,
}

/// Platform-agnostic interface for OS-level operations.
pub trait PlatformProvider {
    /// Register a hotzone listener at the top screen edge.
    fn register_hotzone(&self, height: u32) -> Result<(), String>;

    /// Remove the hotzone listener.
    fn unregister_hotzone(&self) -> Result<(), String>;

    /// Get the current mouse cursor position.
    fn get_mouse_position(&self) -> Result<MousePosition, String>;

    /// Apply window vibrancy/blur effect (platform-native).
    fn set_window_vibrancy(&self, blur_radius: f64, tint_color: &str) -> Result<(), String>;

    /// Extract an icon from a file, app, or URL target.
    fn extract_icon(&self, path: &str, size: u32) -> Result<Vec<u8>, String>;

    /// Launch an item using the platform's default handler.
    fn launch_item(&self, path: &str) -> Result<(), String>;
}

/// Create the appropriate platform provider for the current OS.
pub fn create_provider() -> Box<dyn PlatformProvider> {
    #[cfg(target_os = "windows")]
    return Box::new(windows::WindowsProvider::new());

    #[cfg(target_os = "macos")]
    return Box::new(macos::MacOSProvider::new());

    #[cfg(target_os = "linux")]
    return Box::new(linux::LinuxProvider::new());

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    compile_error!("Unsupported platform");
}
