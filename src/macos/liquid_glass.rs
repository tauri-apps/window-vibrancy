use std::{ffi::c_void, ptr::NonNull};

use objc2::MainThreadMarker;
use objc2_app_kit::{
    NSAppKitVersionNumber, NSAutoresizingMaskOptions, NSColor, NSGlassEffectViewStyle, NSView,
    NSWindowOrderingMode,
};
use objc2_foundation::NSInteger;

use crate::{macos::ns_glass_effect_view_tagged::NSGlassEffectViewTagged, Error};

/// NSView::tag for NSVisualEffectViewTagged, just a random number
pub const NS_VIEW_TAG_GLASS_VIEW: NSInteger = 96945937;

pub unsafe fn apply_liquid_glass(
    ns_view: NonNull<c_void>,
    style: super::NSGlassEffectViewStyle,
    tint_color: Option<crate::Color>,
    radius: Option<f64>,
) -> Result<(), Error> {
    let mtm = MainThreadMarker::new().ok_or(Error::NotMainThread(
        "\"apply_liquid_glass()\" can only be used on the main thread.",
    ))?;

    let tint_color = tint_color.map(|(r, g, b, a)| {
        NSColor::colorWithRed_green_blue_alpha(
            r as f64 / 255.0,
            g as f64 / 255.0,
            b as f64 / 255.0,
            a as f64 / 255.0,
        )
    });

    unsafe {
        let view: &NSView = ns_view.cast().as_ref();

        if NSAppKitVersionNumber < 2685.0 {
            return Err(Error::UnsupportedPlatformVersion(
                "\"apply_liquid_glass()\" is only available on macOS 26.0 or newer.",
            ));
        }

        let s = NSGlassEffectViewStyle(style as isize);

        let bounds = view.bounds();
        let glass_view =
            NSGlassEffectViewTagged::initWithFrame(mtm.alloc(), bounds, NS_VIEW_TAG_GLASS_VIEW);

        glass_view.setStyle(s);
        glass_view.setCornerRadius(radius.unwrap_or(0.0));
        glass_view.setTintColor(tint_color.as_deref());

        glass_view.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );

        view.addSubview_positioned_relativeTo(&glass_view, NSWindowOrderingMode::Below, None);
    }

    Ok(())
}

pub unsafe fn clear_liquid_glass(ns_view: NonNull<c_void>) -> Result<bool, Error> {
    let view: &NSView = ns_view.cast().as_ref();
    let glass_view = view.viewWithTag(NS_VIEW_TAG_GLASS_VIEW);

    if let Some(glass_view) = glass_view {
        glass_view.removeFromSuperview();
        return Ok(true);
    }

    Ok(false)
}
