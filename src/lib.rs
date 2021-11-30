#[macro_use]
mod utils;
mod swca;

use swca::{set_window_composition_attribute, AccentState};
use tauri::Window;

pub trait Vibrancy {
    /// Adds Acrylic effect to you tauri window.
    ///
    /// ## WARNING:
    ///
    /// This method has poor performance when resizing or moving the window.
    /// It is an issue in the undocumented api used for this method
    /// and microsoft needs to fix it (they probably won't).
    ///
    /// ## Platform-specific
    ///
    /// - **Windows**: works only on Windows 10 v1803 and greater.
    /// - **Linux / macOS:** Unsupported
    fn set_acrylic(&self, color: &str);

    /// Adds blur effect to tauri window.
    ///
    /// ## Platform-specific
    ///
    /// - **Linux / macOS:** Unsupported
    fn set_blur(&self, color: &str);
}

impl Vibrancy for Window {
    fn set_acrylic(&self, color: &str) {
        unsafe {
            set_window_composition_attribute(
                self.hwnd().unwrap(),
                AccentState::EnableAcrylicBlurBehind,
                color,
            );
        }
    }

    fn set_blur(&self, color: &str) {
        unsafe {
            set_window_composition_attribute(
                self.hwnd().unwrap(),
                AccentState::EnableBlurBehind,
                color,
            );
        }
    }
}
