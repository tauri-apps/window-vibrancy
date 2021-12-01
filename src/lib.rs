//! Add vibrancy/blur/acrylic to your tauri window.
//!
//! # Note:
//!
//! This plugin is an experiment to gather enough feedback that will help me
//! decide how and whether this will be included in Tao/Tauri directly or kept as a plugin.
//!
//! # Usage:
//!
//! 1. Enable transparency on your window, either through `tauri.conf.json` or programmatically. It is also recommended to disable decorations.
//! 2. Import the vibrancy trait
//!     ```no_run
//!     use tauri_plugin_vibrancy::Vibrancy;
//!     ```
//! 3. Use the [`Vibrancy`] trait methods on the `tauri::Window` type.
//!     ```no_run
//!     tauri::Builder::default()
//!         .setup(|app|{
//!             let window = app.get_window("main").unwrap();
//!             window.set_blur();
//!             Ok(())
//!         })
//!         .run(tauri::generate_context!())
//!         .expect("error while running tauri application");
//!     ```

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
