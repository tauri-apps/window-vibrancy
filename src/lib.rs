// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Make your windows vibrant.
//!
//! ## Platform-specific
//!
//! - **Linux**: Blur is supported on Wayland when the compositor exposes `ext-background-effect-v1` with the blur capability.
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

#[cfg(target_os = "windows")]
use windows_sys::core::HRESULT;

mod macos;
mod windows;

#[cfg(target_os = "linux")]
mod linux;

pub use macos::{NSGlassEffectViewStyle, NSVisualEffectMaterial, NSVisualEffectState};

#[cfg(target_os = "macos")]
pub use macos::{NSGlassEffectViewTagged, NSVisualEffectViewTagged};

/// a tuple of RGBA colors. Each value has minimum of 0 and maximum of 255.
pub type Color = (u8, u8, u8, u8);

/// Applies blur effect to window. Works on Windows 7, Windows 10 v1809 or newer, and supported Linux Wayland compositors.
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
/// - **Linux**: *`color`* is ignored. Only Wayland compositors that expose `ext-background-effect-v1` with the blur capability are supported.
/// - **macOS**: Unsupported.
#[cfg(not(target_os = "linux"))]
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

/// Applies blur effect to window. Works on Windows 7, Windows 10 v1809 or newer, and supported Linux Wayland compositors.
///
/// ## Platform-specific
///
/// - **Linux**: *`color`* is ignored. Only Wayland compositors that expose `ext-background-effect-v1` with the blur capability are supported.
/// - **macOS**: Unsupported.
#[cfg(target_os = "linux")]
pub fn apply_blur(
    window: impl raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle,
    #[allow(unused)] color: Option<Color>,
) -> Result<(), Error> {
    match (window.display_handle()?.as_raw(), window.window_handle()?.as_raw()) {
        (
            raw_window_handle::RawDisplayHandle::Wayland(display),
            raw_window_handle::RawWindowHandle::Wayland(handle),
        ) => linux::apply_blur(display, handle),
        _ => Err(Error::UnsupportedPlatform(
            "\"apply_blur()\" is only supported on Wayland compositors with ext-background-effect-v1 blur support.",
        )),
    }
}

/// Clears blur effect applied to window. Works on Windows 7, Windows 10 v1809 or newer, and supported Linux Wayland compositors.
///
/// ## Platform-specific
///
/// - **macOS**: Unsupported.
pub fn clear_blur(window: impl raw_window_handle::HasWindowHandle) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows::clear_blur(handle.hwnd.get() as _)
        }
        #[cfg(target_os = "linux")]
        raw_window_handle::RawWindowHandle::Wayland(handle) => linux::clear_blur(handle),
        _ => Err(Error::UnsupportedPlatform(
            "\"clear_blur()\" is only supported on Windows and Wayland.",
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
        raw_window_handle::RawWindowHandle::AppKit(handle) => unsafe {
            macos::apply_vibrancy(handle.ns_view, effect, state, radius)
        },
        _ => Err(Error::UnsupportedPlatform(
            "\"apply_vibrancy()\" is only supported on macOS.",
        )),
    }
}

/// Clears vibrancy effect applied to window. Works only on macOS 10.10 or newer.
///
/// ## Platform-specific
///
/// - **Linux / Windows**: Unsupported.
///
/// # Returns
///
/// - `Ok(true)` if the vibrancy effect was cleared
/// - `Ok(false)` if the vibrancy effect was not previously applied by this crate.
pub fn clear_vibrancy(window: impl raw_window_handle::HasWindowHandle) -> Result<bool, Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "macos")]
        raw_window_handle::RawWindowHandle::AppKit(handle) => unsafe {
            macos::clear_vibrancy(handle.ns_view)
        },
        _ => Err(Error::UnsupportedPlatform(
            "\"clear_vibrancy()\" is only supported on macOS.",
        )),
    }
}

/// Applies liquid glass effect to window. Works only on macOS 26.0+.
///
/// ## Platform-specific
///
/// - **Linux / Windows**: Unsupported.
///
/// # Example
///
/// ```no_run
/// use window_vibrancy::{apply_liquid_glass, NSGlassEffectViewStyle};
///
/// # let window: &dyn raw_window_handle::HasWindowHandle = unsafe { std::mem::zeroed() };
/// apply_liquid_glass(&window, NSGlassEffectViewStyle::Regular, None, Some(12.0));
/// ```
#[cfg(target_os = "macos")]
pub fn apply_liquid_glass(
    window: impl raw_window_handle::HasWindowHandle,
    #[allow(unused)] style: NSGlassEffectViewStyle,
    #[allow(unused)] tint_color: Option<Color>,
    #[allow(unused)] radius: Option<f64>,
) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "macos")]
        raw_window_handle::RawWindowHandle::AppKit(handle) => unsafe {
            macos::apply_liquid_glass(handle.ns_view, style, tint_color, radius)
        },
        _ => Err(Error::UnsupportedPlatform(
            "\"apply_vibrancy()\" is only supported on macOS.",
        )),
    }
}

/// Clears liquid glass effect applied to window. Works only on macOS 26.0+.
///
/// ## Platform-specific
///
/// - **Linux / Windows**: Unsupported.
///
/// # Returns
///
/// - `Ok(true)` if the liquid glass effect was cleared
/// - `Ok(false)` if the liquid glass effect was not previously applied by this crate.
#[cfg(target_os = "macos")]
pub fn clear_liquid_glass(window: impl raw_window_handle::HasWindowHandle) -> Result<bool, Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "macos")]
        raw_window_handle::RawWindowHandle::AppKit(handle) => unsafe {
            macos::clear_liquid_glass(handle.ns_view)
        },
        _ => Err(Error::UnsupportedPlatform(
            "\"clear_liquid_glass()\" is only supported on macOS.",
        )),
    }
}

#[derive(Debug)]
pub enum Error {
    UnsupportedPlatform(&'static str),
    UnsupportedPlatformVersion(&'static str),
    NotMainThread(&'static str),
    NoWindowHandle(raw_window_handle::HandleError),
    #[cfg(target_os = "windows")]
    Win32Error {
        api: &'static str,
        result: HRESULT,
    },
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
            #[cfg(target_os = "windows")]
            Error::Win32Error { api, result } => {
                write!(
                    f,
                    "Win32 API {api}() returned the error result 0x{:x}.",
                    result,
                )
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
