#[macro_use]
mod utils;
mod swca;

use tauri::Window;

pub trait Vibrancy {
    /// Adds Acrylic effect to you tauri window.
    ///
    /// ## WARNING:
    ///
    /// This method has poor performance on Windows 10 v1903 and above,
    /// the window will lag when resizing or dragging the window.
    /// It is an issue in the undocumented api used for this method
    /// and microsoft needs to fix it (they probably won't).
    ///
    /// ## Platform-specific
    ///
    /// - **Windows**: has no effect on Windows 10 versions below v1809.
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
            if let Some(v) = utils::get_windows10_build_ver() {
                if v >= 17763 {
                    swca::set_window_composition_attribute(
                        self.hwnd().unwrap(),
                        swca::AccentState::EnableAcrylicBlurBehind,
                        color,
                    );
                }
            }
        }
    }

    fn set_blur(&self, color: &str) {
        unsafe {
            swca::set_window_composition_attribute(
                self.hwnd().unwrap(),
                swca::AccentState::EnableBlurBehind,
                color,
            );
        }
    }
}
