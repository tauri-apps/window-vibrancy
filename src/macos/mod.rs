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

// TODO: Find references for styles other than 0 and 1
/// Note that only the first two variants Regular (0) and Clear (1) are officially supported,
/// see <https://developer.apple.com/documentation/appkit/nsglasseffectview/style-swift.enum>.
///
/// All other variants use private macOS APIs and may change or break at any time.
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSGlassEffectViewStyle {
    /// macOS 26.0+
    Regular = 0,
    /// macOS 26.0+
    Clear = 1,
    /// macOS 26.0+
    Dock = 2,
    /// macOS 26.0+
    AppIcons = 3,
    /// macOS 26.0+
    Widgets = 4,
    /// macOS 26.0+
    Text = 5,
    /// macOS 26.0+
    AvPlayer = 6,
    /// macOS 26.0+
    FaceTime = 7,
    /// macOS 26.0+
    ControlCenter = 8,
    /// macOS 26.0+
    NotificationCenter = 9,
    /// macOS 26.0+
    Monogram = 10,
    /// macOS 26.0+
    Bubbles = 11,
    /// macOS 26.0+
    Identity = 12,
    /// macOS 26.0+
    FocusBorder = 13,
    /// macOS 26.0+
    FocusPlatter = 14,
    /// macOS 26.0+
    Keyboard = 15,
    /// macOS 26.0+
    Sidebar = 16,
    /// macOS 26.0+
    AbuttedSidebar = 17,
    /// macOS 26.0+
    Inspector = 18,
    /// macOS 26.0+
    Control = 19,
    /// macOS 26.0+
    Loupe = 20,
    /// macOS 26.0+
    Slider = 21,
    /// macOS 26.0+
    Camera = 22,
    /// macOS 26.0+
    CartouchePopover = 23,
}

#[cfg(target_os = "macos")]
mod vibrancy;

#[cfg(target_os = "macos")]
pub use vibrancy::{apply_vibrancy, clear_vibrancy};

#[cfg(target_os = "macos")]
mod ns_visual_effect_view_tagged;

#[cfg(target_os = "macos")]
pub use ns_visual_effect_view_tagged::NSVisualEffectViewTagged;

// Liquid Glass support (macOS 26.0+)
#[cfg(target_os = "macos")]
mod liquid_glass;

#[cfg(target_os = "macos")]
pub use liquid_glass::{apply_liquid_glass, clear_liquid_glass};

#[cfg(target_os = "macos")]
mod ns_glass_effect_view_tagged;

#[cfg(target_os = "macos")]
pub use ns_glass_effect_view_tagged::NSGlassEffectViewTagged;
