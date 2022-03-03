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
//! ```no_run,ignore
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
//! apply_blur(&window).unwrap();
//!
//! #[cfg(target_os = "macos")]
//! apply_vibrancy(&window, NSVisualEffectMaterial::AppearanceBased).unwrap();
//! ```

mod macos;
mod windows;

pub use macos::NSVisualEffectMaterial;

/// Applies Acrylic effect to you window.
///
/// ## WARNING:
///
/// This method has poor performance on Windows 10 v1903+ and Windows 11 build 22000,
/// the window will lag when resizing or dragging.
/// It is an issue in the undocumented api used for this method
/// and microsoft needs to fix it (they probably won't).
pub fn apply_acrylic(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => {
      windows::apply_acrylic(handle.hwnd as _);
      Ok(())
    }
    _ => Err(Error::UnsupportedPlatform(
      "apply_acrylic()".into(),
      "Windows".into(),
    )),
  }
}

/// Applies blur effect to window.
pub fn apply_blur(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => {
      windows::apply_blur(handle.hwnd as _);
      Ok(())
    }
    _ => Err(Error::UnsupportedPlatform(
      "apply_blur()".into(),
      "Windows".into(),
    )),
  }
}

/// Applies mica effect to window.
pub fn apply_mica(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => {
      windows::apply_mica(handle.hwnd as _);
      Ok(())
    }
    _ => Err(Error::UnsupportedPlatform(
      "apply_mica()".into(),
      "Windows".into(),
    )),
  }
}

pub fn clear_mica(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => {
      windows::clear_mica(handle.hwnd as _);
      Ok(())
    }
    _ => Err(Error::UnsupportedPlatform(
      "clear_mica()".into(),
      "Windows".into(),
    )),
  }
}

/// Clears all Windows effects applied to window.
pub fn clear_effects(window: impl raw_window_handle::HasRawWindowHandle) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "windows")]
    raw_window_handle::RawWindowHandle::Win32(handle) => {
      windows::clear_effects(handle.hwnd as _);
      Ok(())
    }
    _ => Err(Error::UnsupportedPlatform(
      "clear_effects()".into(),
      "Windows".into(),
    )),
  }
}

/// Applies macos vibrancy effect to window. This has no effect on macOS versions below 10.10
pub fn apply_vibrancy(
  window: impl raw_window_handle::HasRawWindowHandle,
  effect: NSVisualEffectMaterial,
) -> Result<(), Error> {
  match window.raw_window_handle() {
    #[cfg(target_os = "macos")]
    raw_window_handle::RawWindowHandle::Win32(handle) => {
      macos::apply_vibrancy(handle.hwnd as _, effect);
      Ok(())
    }
    _ => Err(Error::UnsupportedPlatform(
      "apply_vibrancy()".into(),
      "macOS".into(),
    )),
  }
}

#[derive(Debug)]
pub enum Error {
  UnsupportedPlatform(String, String),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::UnsupportedPlatform(func, supported_platform) => {
        write!(f, "{} is only supported on {} ", func, supported_platform)
      }
    }
  }
}
