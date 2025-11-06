use objc2_app_kit::{
    NSAppKitVersionNumber, NSAppKitVersionNumber10_10, NSAppKitVersionNumber10_11,
    NSAppKitVersionNumber10_14, NSAutoresizingMaskOptions, NSView, NSVisualEffectBlendingMode,
    NSVisualEffectMaterial, NSVisualEffectState, NSWindowOrderingMode,
};
use objc2_foundation::{MainThreadMarker, NSInteger};
use std::{ffi::c_void, ptr::NonNull};

use crate::macos::NSVisualEffectViewTagged;
use crate::Error;

/// NSView::tag for NSVisualEffectViewTagged, just a random number
pub const NS_VIEW_TAG_BLUR_VIEW: NSInteger = 91376254;

#[allow(deprecated)]
pub unsafe fn apply_vibrancy(
    ns_view: NonNull<c_void>,
    appearance: super::NSVisualEffectMaterial,
    state: Option<super::NSVisualEffectState>,
    radius: Option<f64>,
) -> Result<(), Error> {
    let mtm = MainThreadMarker::new().ok_or(Error::NotMainThread(
        "\"apply_vibrancy()\" can only be used on the main thread.",
    ))?;

    unsafe {
        let view: &NSView = ns_view.cast().as_ref();

        if NSAppKitVersionNumber < NSAppKitVersionNumber10_10 {
            return Err(Error::UnsupportedPlatformVersion(
                "\"apply_vibrancy()\" is only available on macOS 10.0 or newer.",
            ));
        }

        let mut m = NSVisualEffectMaterial(appearance as isize);
        if (appearance as u32 > 9 && NSAppKitVersionNumber < NSAppKitVersionNumber10_14)
            || (appearance as u32 > 4 && NSAppKitVersionNumber < NSAppKitVersionNumber10_11)
        {
            m = NSVisualEffectMaterial::AppearanceBased;
        }

        let bounds = view.bounds();
        let blurred_view =
            NSVisualEffectViewTagged::initWithFrame(mtm.alloc(), bounds, NS_VIEW_TAG_BLUR_VIEW);

        blurred_view.setMaterial(m);
        blurred_view.setCornerRadius(radius.unwrap_or(0.0));
        blurred_view.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
        blurred_view.setState(
            state
                .map(|state| NSVisualEffectState(state as isize))
                .unwrap_or(NSVisualEffectState::FollowsWindowActiveState),
        );
        blurred_view.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable,
        );

        view.addSubview_positioned_relativeTo(&blurred_view, NSWindowOrderingMode::Below, None);
    }

    Ok(())
}

pub unsafe fn clear_vibrancy(ns_view: NonNull<c_void>) -> Result<bool, Error> {
    let view: &NSView = ns_view.cast().as_ref();
    let blurred_view = view.viewWithTag(NS_VIEW_TAG_BLUR_VIEW);

    if let Some(blurred_view) = blurred_view {
        blurred_view.removeFromSuperview();
        return Ok(true);
    }

    Ok(false)
}
