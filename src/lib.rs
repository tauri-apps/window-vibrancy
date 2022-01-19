//! Make your Tauri/TAO windows vibrant.
//!
//! # Platform support:
//!
//! - **Windows:** Yes!
//! - **macOS:** Yes!
//! - **Linux:** No, blur effect is controlled by the compositor installed on the user system and they can enable it for your app if they want.
//!
//! # Usage:
//!
//! 1. Enable transparency on your window
//!     - **Tauri:** Edit your window in `tauri.conf.json > tauri > windows` and add `"transparent": true`
//!       or use [`tauri::WindowBuilder::transparent`]
//!     - **TAO:** Use [`tao::window::WindowBuilder::with_transparent`]
//! 2. Use the [`Vibrancy`] trait methods on your window type
//!     - Tauri:
//!         ```ignore
//!         let window = app.get_window("main").unwrap();
//!         #[cfg(target_os = "windows")]
//!         window.apply_blur();
//!         #[cfg(target_os = "macos")]
//!         {
//!             use tauri_plugin_vibrancy::MacOSVibrancy;
//!             window.apply_vibrancy(MacOSVibrancy::AppearanceBased);
//!         }
//!         ```
//!     - Tao:
//!         ```ignore
//!         let window = WindowBuilder::new().with_transparent(true).build(&event_loop).unwrap();
//!         use tauri_plugin_vibrancy::Vibrancy;
//!         #[cfg(target_os = "windows")]
//!         window.apply_blur();
//!         #[cfg(target_os = "macos")]
//!         {
//!             use tauri_plugin_vibrancy::MacOSVibrancy;
//!             window.apply_vibrancy(MacOSVibrancy::AppearanceBased);
//!         }
//!         ```

#![allow(unused)]

mod platform;

#[cfg(target_os = "macos")]
use crate::platform::macos;
#[cfg(target_os = "macos")]
pub use crate::platform::macos::NSVisualEffectMaterial as MacOSVibrancy;
#[cfg(target_os = "windows")]
use crate::platform::windows;

#[cfg(feature = "tauri-impl")]
use tauri::{Runtime, Window as TauriWindow};

#[cfg(all(target_os = "macos", feature = "tao-impl"))]
use tao::platform::macos::WindowExtMacOS;
#[cfg(all(target_os = "windows", feature = "tao-impl"))]
use tao::platform::windows::WindowExtWindows;
#[cfg(feature = "tao-impl")]
use tao::window::Window as TaoWindow;

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

#[cfg(feature = "tauri-impl")]
impl<R> Vibrancy for TauriWindow<R>
where
  R: Runtime,
{
  #[cfg(target_os = "windows")]
  fn apply_acrylic(&self) {
    windows::apply_acrylic(windows::HWND(self.hwnd().unwrap() as _));
  }

  #[cfg(target_os = "windows")]
  fn apply_blur(&self) {
    windows::apply_blur(windows::HWND(self.hwnd().unwrap() as _));
  }

  #[cfg(target_os = "macos")]
  fn apply_vibrancy(&self, vibrancy: MacOSVibrancy) {
    macos::apply_vibrancy(self.ns_window().unwrap() as _, vibrancy);
  }
}

#[cfg(feature = "tao-impl")]
impl Vibrancy for TaoWindow {
  #[cfg(target_os = "windows")]
  fn apply_acrylic(&self) {
    windows::apply_acrylic(windows::HWND(self.hwnd() as _));
  }

  #[cfg(target_os = "windows")]
  fn apply_blur(&self) {
    windows::apply_blur(windows::HWND(self.hwnd() as _));
  }

  #[cfg(target_os = "macos")]
  fn apply_vibrancy(&self, vibrancy: MacOSVibrancy) {
    macos::apply_vibrancy(self.ns_window() as _, vibrancy);
  }
}
