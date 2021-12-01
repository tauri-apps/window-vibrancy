use tauri::Window;
mod platform;

#[cfg(target_os = "windows")]
use ::windows::Win32::Foundation::HWND;
#[cfg(target_os = "windows")]
use platform::windows;

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
    /// - **Windows**: has no effect on Windows 10 versions below v1809
    /// - **Linux / macOS:** has no effect
    fn set_acrylic(&self);

    /// Adds blur effect to tauri window.
    ///
    /// ## Platform-specific
    ///
    /// - **Linux / macOS:** has no effect
    fn set_blur(&self);
}

impl Vibrancy for Window {
    fn set_acrylic(&self) {
        #[cfg(target_os = "windows")]
        unsafe {
            windows::set_acrylic(HWND(self.hwnd().unwrap() as _));
        }
    }

    fn set_blur(&self) {
        #[cfg(target_os = "windows")]
        unsafe {
            windows::set_blur(HWND(self.hwnd().unwrap() as _));
        }
    }
}
