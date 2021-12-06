//! Make your Tao/Tauri windows vibrant.
//!
//! # Note:
//!
//! This plugin is an experiment to gather enough feedback that will help me
//! decide how and whether this will be included in Tao/Tauri directly or kept as a plugin.
//!
//! # Usage:
//!
//! 1. Enable transparency on your window
//!     - **Tauri:** edit your window in `tauri.conf.json > tauri > windows` and add `"transparent": true`
//!       or use [`tauri::WindowBuilder::transparent`]
//!     - **Tao:** use [`tao::window::WindowBuilder::with_transparent`]
//! 2. Import the vibrancy trait
//!     ```no_run
//!     use tauri_plugin_vibrancy::Vibrancy;
//!     ```
//! 3. Use the [`Vibrancy`] trait methods on your window type
//!     - Tauri:
//!         ```no_run
//!         let window = app.get_window("main").unwrap();
//!         window.apply_blur();
//!         ```
//!     - Tao:
//!         ```no_run
//!         let window = WindowBuilder::new().with_transparent(true).build().unwrap();
//!         window.apply_blur();
//!         ```

mod platform;

use tao::window::Window as TaoWindow;
use tauri::Window as TauriWindow;

#[cfg(target_os = "windows")]
use crate::platform::windows;
#[cfg(target_os = "windows")]
use ::windows::Win32::Foundation::HWND;
#[cfg(target_os = "windows")]
use tao::platform::windows::WindowExtWindows;

pub trait Vibrancy {
    /// Applies Acrylic effect to you tao/tauri window.
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
    fn apply_acrylic(&self);

    /// Applies blur effect to tao/tauri window.
    ///
    /// ## Platform-specific
    ///
    /// - **Linux / macOS:** has no effect
    fn apply_blur(&self);
}

impl Vibrancy for TauriWindow {
    fn apply_acrylic(&self) {
        #[cfg(target_os = "windows")]
        windows::apply_acrylic(HWND(self.hwnd().unwrap() as _));
    }

    fn apply_blur(&self) {
        #[cfg(target_os = "windows")]
        windows::apply_blur(HWND(self.hwnd().unwrap() as _));
    }
}

impl Vibrancy for TaoWindow {
    fn apply_acrylic(&self) {
        #[cfg(target_os = "windows")]
        windows::apply_acrylic(HWND(self.hwnd() as _));
    }

    fn apply_blur(&self) {
        #[cfg(target_os = "windows")]
        windows::apply_blur(HWND(self.hwnd() as _));
    }
}
