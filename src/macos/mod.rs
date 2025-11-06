// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// The use of NSVisualEffectView comes from https://github.com/joboet/winit/tree/macos_blurred_background
// with a bit of rewrite by @youngsing to make it more like cocoa::appkit style.
/// <https://developer.apple.com/documentation/appkit/nsvisualeffectview/material>
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSVisualEffectMaterial {
    #[deprecated(
        since = "macOS 10.14",
        note = "A default material appropriate for the view's effectiveAppearance.  You should instead choose an appropriate semantic material."
    )]
    AppearanceBased = 0,
    #[deprecated(since = "macOS 10.14", note = "Use a semantic material instead.")]
    Light = 1,
    #[deprecated(since = "macOS 10.14", note = "Use a semantic material instead.")]
    Dark = 2,
    #[deprecated(since = "macOS 10.14", note = "Use a semantic material instead.")]
    MediumLight = 8,
    #[deprecated(since = "macOS 10.14", note = "Use a semantic material instead.")]
    UltraDark = 9,

    /// macOS 10.10+
    Titlebar = 3,
    /// macOS 10.10+
    Selection = 4,

    /// macOS 10.11+
    Menu = 5,
    /// macOS 10.11+
    Popover = 6,
    /// macOS 10.11+
    Sidebar = 7,

    /// macOS 10.14+
    HeaderView = 10,
    /// macOS 10.14+
    Sheet = 11,
    /// macOS 10.14+
    WindowBackground = 12,
    /// macOS 10.14+
    HudWindow = 13,
    /// macOS 10.14+
    FullScreenUI = 15,
    /// macOS 10.14+
    Tooltip = 17,
    /// macOS 10.14+
    ContentBackground = 18,
    /// macOS 10.14+
    UnderWindowBackground = 21,
    /// macOS 10.14+
    UnderPageBackground = 22,
}

/// <https://developer.apple.com/documentation/appkit/nsvisualeffectview/state>
#[allow(dead_code)]
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSVisualEffectState {
    /// Make window vibrancy state follow the window's active state
    FollowsWindowActiveState = 0,
    /// Make window vibrancy state always active
    Active = 1,
    /// Make window vibrancy state always inactive
    Inactive = 2,
}

#[cfg(target_os = "macos")]
mod internal;

#[cfg(target_os = "macos")]
pub use internal::apply_vibrancy;

#[cfg(target_os = "macos")]
pub use internal::clear_vibrancy;

#[cfg(target_os = "macos")]
mod ns_visual_effect_view_tagged;

#[cfg(target_os = "macos")]
pub use ns_visual_effect_view_tagged::NSVisualEffectViewTagged;

// NSGlassEffectView wrapper (macOS 15.0+ / AppKit 26.0+)
#[cfg(target_os = "macos")]
mod ns_glass_effect_view;

#[cfg(target_os = "macos")]
pub use ns_glass_effect_view::NSGlassEffectVariant;

// Liquid Glass support (macOS 26.0+)
#[cfg(target_os = "macos")]
mod liquid_glass;

#[cfg(target_os = "macos")]
pub use liquid_glass::{apply_liquid_glass, clear_liquid_glass, LiquidGlassOptions};
