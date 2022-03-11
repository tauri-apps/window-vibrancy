//! Make your windows vibrant.
//!
//! # Platform support:
//!
//! - **Windows:** Yes!
//! - **macOS:** Yes!
//! - **Linux:** No, blur effect is controlled by the compositor installed on the user system and they can enable it for your app if they want.
//!
//! # Example with [`winit`](https://docs.rs/winit)
//!
//! ```no_run
//! # use winit::{event_loop::EventLoop, window::WindowBuilder};
//! # use window_vibrancy::{apply_vibrancy, apply_blur, NSVisualEffectMaterial};
//! let event_loop = EventLoop::new();
//!
//! let window = WindowBuilder::new()
//!  .with_decorations(false)
//!  .build(&event_loop)
//!  .unwrap();
//!
//! #[cfg(target_os = "windows")]
//! apply_blur(&window, Some((18, 18, 18, 125))).unwrap();
//!
//! #[cfg(target_os = "macos")]
//! apply_vibrancy(&window, NSVisualEffectMaterial::AppearanceBased).unwrap();
//! ```

mod macos;
mod windows;

pub use macos::NSVisualEffectMaterial;

/// a tuple of RGBA colors. Each value has minimum of 0 and maximum of 255.
pub type Color = (u8, u8, u8, u8);

/// Applies blur effect to window. Works only on Windows 7, Windows 10 v1809 or newer and Windows 11.
///
/// - *`color`* is ignored on Windows 7 and has no effect.
pub fn apply_blur(
  window: impl raw_window_handle::HasRawWindowHandle,
  #[allow(unused)] color: Option<Color>,
) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => {
      windows::apply_blur(handle.hwnd as _, color)
    }
    _ => Err(Error::UnsupportedPlatform(
      "\"apply_blur()\" is only supported on Windows.",
    )),
  }
}

/// Clears blur effect applied to window. Works only on Windows 7, Windows 10 v1809 or newer and Windows 11.
pub fn clear_blur(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => windows::clear_blur(handle.hwnd as _),
    _ => Err(Error::UnsupportedPlatform(
      "\"clear_blur()\" is only supported on Windows.",
    )),
  }
}

/// Applies Acrylic effect to you window. Works only on Windows 10 v1809 or newer and Windows 11
///
/// - *`color`* is ignored on Windows 11 build 22523 and newer and has no effect.
///
/// ## WARNING:
///
/// This method has poor performance on Windows 10 v1903+ and Windows 11 build 22000,
/// the window will lag when resizing or dragging.
/// It is an issue in the undocumented api used for this method
/// and microsoft needs to fix it (they probably won't).
pub fn apply_acrylic(
  window: impl raw_window_handle::HasRawWindowHandle,
  #[allow(unused)] color: Option<Color>,
) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => {
      windows::apply_acrylic(handle.hwnd as _, color)
    }
    _ => Err(Error::UnsupportedPlatform(
      "\"apply_acrylic()\" is only supported on Windows.",
    )),
  }
}

/// Clears acrylic effect applied to window. Works only on Windows 10 v1809 or newer and Windows 11.
pub fn clear_acrylic(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => windows::clear_acrylic(handle.hwnd as _),
    _ => Err(Error::UnsupportedPlatform(
      "\"clear_acrylic()\" is only supported on Windows.",
    )),
  }
}

/// Applies mica effect to window. Works only on Windows 11.
pub fn apply_mica(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => windows::apply_mica(handle.hwnd as _),
    _ => Err(Error::UnsupportedPlatform(
      "\"apply_mica()\" is only supported on Windows.",
    )),
  }
}

/// Clears mica effect applied to window. Works only on Windows 11.
pub fn clear_mica(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => windows::clear_mica(handle.hwnd as _),
    _ => Err(Error::UnsupportedPlatform(
      "\"clear_mica()\" is only supported on Windows.",
    )),
  }
}

/// Applies macos vibrancy effect to window. Works only on macOS 10.10 or newer.
pub fn apply_vibrancy(
  window: impl raw_window_handle::HasRawWindowHandle,
  #[allow(unused)] effect: NSVisualEffectMaterial,
  #[allow(unused)] radius: f64,
) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "macos")]
    raw_window_handle::RawWindowHandle::AppKit(handle) => {
      macos::apply_vibrancy(handle.ns_window as _, effect, radius)
    }
    _ => Err(Error::UnsupportedPlatform(
      "\"apply_vibrancy()\" is only supported on macOS.",
    )),
  }
}

#[derive(Debug)]
pub enum Error {
  UnsupportedPlatform(&'static str),
  UnsupportedPlatformVersion(&'static str),
  NotMainThread(&'static str),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::UnsupportedPlatform(e)
      | Error::UnsupportedPlatformVersion(e)
      | Error::NotMainThread(e) => {
        write!(f, "{}", e)
      }
    }
  }
}
