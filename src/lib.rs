// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Make your windows vibrant.
//!
//! ## Platform-specific
//!
//! - **Linux**: Unsupported, Blur and any vibrancy effects are controlled by the compositor installed on the end-user system.
//!
//! # Example
//!
//! ```no_run
//! use window_vibrancy::{apply_vibrancy, apply_blur, NSVisualEffectMaterial};
//!
//! # let window: &dyn raw_window_handle::HasWindowHandle = unsafe { std::mem::zeroed() };
//! #[cfg(target_os = "macos")]
//! apply_vibrancy(&window, NSVisualEffectMaterial::AppearanceBased, None, None).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
//!
//! #[cfg(target_os = "windows")]
//! apply_blur(&window, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");
//! ```

#![allow(clippy::deprecated_semver)]

mod macos;
mod windows;

pub use macos::{NSVisualEffectMaterial, NSVisualEffectState};

/// a tuple of RGBA colors. Each value has minimum of 0 and maximum of 255.
pub type Color = (u8, u8, u8, u8);

/// Applies blur effect to window. Works only on Windows 7, Windows 10 v1809 or newer.
///
/// ## WARNING:
///
/// This method has poor performance on Windows 11 build 22621,
/// the window will lag when resizing or dragging.
/// It is an issue in the undocumented api used for this method
/// and microsoft needs to fix it (they probably won't).
///
/// ## Platform-specific
///
/// - **Windows**: *`color`* is ignored on Windows 7 and has no effect.
/// - **Linux / macOS**: Unsupported.
pub fn apply_blur(
    window: impl raw_window_handle::HasWindowHandle,
    #[allow(unused)] color: Option<Color>,
) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows::apply_blur(handle.hwnd.get() as _, color)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"apply_blur()\" is only supported on Windows.",
        )),
    }
}

/// Clears blur effect applied to window. Works only on Windows 7, Windows 10 v1809 or newer.
///
/// ## Platform-specific
///
/// - **Linux / macOS**: Unsupported.
pub fn clear_blur(window: impl raw_window_handle::HasWindowHandle) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows::clear_blur(handle.hwnd.get() as _)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"clear_blur()\" is only supported on Windows.",
        )),
    }
}

/// Applies acrylic effect to window. Works only on Windows 10 v1809 or newer.
///
/// ## WARNING:
///
/// This method has poor performance on Windows 10 v1903+ and Windows 11 build 22000,
/// the window will lag when resizing or dragging.
/// It is an issue in the undocumented api used for this method
/// and microsoft needs to fix it (they probably won't).
///
/// ## Platform-specific
///
/// - **Windows**: *`color`* is ignored on Windows 7 and has no effect.
/// - **Linux / macOS**: Unsupported.
pub fn apply_acrylic(
    window: impl raw_window_handle::HasWindowHandle,
    #[allow(unused)] color: Option<Color>,
) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows::apply_acrylic(handle.hwnd.get() as _, color)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"apply_acrylic()\" is only supported on Windows.",
        )),
    }
}

/// Clears acrylic effect applied to window. Works only on Windows 10 v1809 or newer.
///
/// ## Platform-specific
///
/// - **Linux / macOS**: Unsupported.
pub fn clear_acrylic(window: impl raw_window_handle::HasWindowHandle) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows::clear_acrylic(handle.hwnd.get() as _)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"clear_acrylic()\" is only supported on Windows.",
        )),
    }
}

/// Applies mica effect to window. Works only on Windows 11.
///
/// ## Arguments
///
/// - `dark`: If `None` is provide, it will match the system preference
///
/// ## Platform-specific
///
/// - **Linux / macOS**: Unsupported.
pub fn apply_mica(
    window: impl raw_window_handle::HasWindowHandle,
    dark: Option<bool>,
) -> Result<(), Error> {
    #[cfg(not(target_os = "windows"))]
    let _ = dark;
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows::apply_mica(handle.hwnd.get() as _, dark)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"apply_mica()\" is only supported on Windows.",
        )),
    }
}

/// Clears mica effect applied to window. Works only on Windows 11.
///
/// ## Platform-specific
///
/// - **Linux / macOS**: Unsupported.
pub fn clear_mica(window: impl raw_window_handle::HasWindowHandle) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows::clear_mica(handle.hwnd.get() as _)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"clear_mica()\" is only supported on Windows.",
        )),
    }
}

/// Applies mica tabbed effect to window. Works only on Windows 11.
///
/// ## Arguments
///
/// - `dark`: If `None` is provide, it will match the system preference
///
/// ## Platform-specific
///
/// - **Linux / macOS**: Unsupported.
pub fn apply_tabbed(
    window: impl raw_window_handle::HasWindowHandle,
    dark: Option<bool>,
) -> Result<(), Error> {
    #[cfg(not(target_os = "windows"))]
    let _ = dark;
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows::apply_tabbed(handle.hwnd.get() as _, dark)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"apply_tabbed()\" is only supported on Windows.",
        )),
    }
}

/// Clears mica tabbed effect applied to window. Works only on Windows 11.
///
/// ## Platform-specific
///
/// - **Linux / macOS**: Unsupported.
pub fn clear_tabbed(window: impl raw_window_handle::HasWindowHandle) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows::clear_tabbed(handle.hwnd.get() as _)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"clear_tabbed()\" is only supported on Windows.",
        )),
    }
}

/// Applies macos vibrancy effect to window. Works only on macOS 10.10 or newer.
///
/// ## Platform-specific
///
/// - **Linux / Windows**: Unsupported.
pub fn apply_vibrancy(
    window: impl raw_window_handle::HasWindowHandle,
    #[allow(unused)] effect: NSVisualEffectMaterial,
    #[allow(unused)] state: Option<NSVisualEffectState>,
    #[allow(unused)] radius: Option<f64>,
) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "macos")]
        raw_window_handle::RawWindowHandle::AppKit(handle) => {
            macos::apply_vibrancy(handle.ns_view.as_ptr() as _, effect, state, radius)
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
    NoWindowHandle(raw_window_handle::HandleError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnsupportedPlatform(e)
            | Error::UnsupportedPlatformVersion(e)
            | Error::NotMainThread(e) => {
                write!(f, "{}", e)
            }
            Error::NoWindowHandle(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<raw_window_handle::HandleError> for Error {
    fn from(err: raw_window_handle::HandleError) -> Self {
        Error::NoWindowHandle(err)
    }
}
