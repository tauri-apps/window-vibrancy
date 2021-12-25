//! Make your Tao/Tauri windows vibrant.
//!
//! # Platform Note:
//!
//! Only Windows and macOS are supported, 
//! Linux blur effect is controlled by the compositor installed on the user system and they can enable it for your app if they want.
//!
//! # Usage:
//!
//! 1. Enable transparency on your window
//!     - **Tauri:** Edit your window in `tauri.conf.json > tauri > windows` and add `"transparent": true`
//!       or use [`tauri::WindowBuilder::transparent`]
//!     - **Tao:** Use [`tao::window::WindowBuilder::with_transparent`]
//! 2. Use the [`Vibrancy`] trait methods on your window type
//!     - Tauri:
//!         ```no_run
//!         let window = app.get_window("main").unwrap();
//!
//!         use tauri_plugin_vibrancy::Vibrancy;
//!         #[cfg(target_os = "windows")]
//!         window.apply_blur();
//!         #[cfg(target_os = "macos")]
//!         {
//!             use tauri_plugin_vibrancy::MacOSVibrancy;
//!             window.apply_vibrancy(MacOSVibrancy::AppearanceBased);
//!         }
//!         ```
//!     - Tao:
//!         ```no_run
//!         let window = WindowBuilder::new().with_transparent(true).build().unwrap();
//!
//!         use tauri_plugin_vibrancy::Vibrancy;
//!         #[cfg(target_os = "windows")]
//!         window.apply_blur();
//!         #[cfg(target_os = "macos")]
//!         {
//!             use tauri_plugin_vibrancy::MacOSVibrancy;
//!             window.apply_vibrancy(MacOSVibrancy::AppearanceBased);
//!         }
//!         ```

mod platform;

use tao::window::Window as TaoWindow;
use tauri::Window as TauriWindow;

#[cfg(target_os = "macos")]
use crate::platform::macos;
#[cfg(target_os = "macos")]
pub use crate::platform::macos::NSVisualEffectMaterial as MacOSVibrancy;
#[cfg(target_os = "windows")]
use crate::platform::windows;
#[cfg(target_os = "windows")]
use ::windows::Win32::Foundation::HWND;
#[cfg(target_os = "macos")]
use tao::platform::macos::WindowExtMacOS;
#[cfg(target_os = "windows")]
use tao::platform::windows::WindowExtWindows;

pub trait Vibrancy {
    /// Applies Acrylic effect to you tao/tauri window. This has no effect on Windows versions below Windows 10 v1809
    ///
    /// ## WARNING:
    ///
    /// This method has poor performance on Windows 10 v1903 and above,
    /// the window will lag when resizing or dragging.
    /// It is an issue in the undocumented api used for this method
    /// and microsoft needs to fix it (they probably won't).
    #[cfg(target_os = "windows")]
    fn apply_acrylic(&self);

    /// Applies blur effect to tao/tauri window.
    #[cfg(target_os = "windows")]
    fn apply_blur(&self);

    /// Applies macos vibrancy effect to tao/tauri window. This has no effect on macOS versions below 10.10
    #[cfg(target_os = "macos")]
    fn apply_vibrancy(&self, vibrancy: MacOSVibrancy);
}

impl Vibrancy for TauriWindow {
    #[cfg(target_os = "windows")]
    fn apply_acrylic(&self) {
        windows::apply_acrylic(HWND(self.hwnd().unwrap() as _));
    }

    #[cfg(target_os = "windows")]
    fn apply_blur(&self) {
        windows::apply_blur(HWND(self.hwnd().unwrap() as _));
    }

    #[cfg(target_os = "macos")]
    fn apply_vibrancy(&self, vibrancy: MacOSVibrancy) {
        macos::apply_vibrancy(self.ns_window().unwrap() as _, vibrancy);
    }
}

impl Vibrancy for TaoWindow {
    #[cfg(target_os = "windows")]
    fn apply_acrylic(&self) {
        windows::apply_acrylic(HWND(self.hwnd() as _));
    }

    #[cfg(target_os = "windows")]
    fn apply_blur(&self) {
        windows::apply_blur(HWND(self.hwnd() as _));
    }

    #[cfg(target_os = "macos")]
    fn apply_vibrancy(&self, vibrancy: MacOSVibrancy) {
        macos::apply_vibrancy(self.ns_window() as _, vibrancy);
    }
}
