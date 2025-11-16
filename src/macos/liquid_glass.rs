use std::{ffi::c_void, ptr::NonNull};

use objc2::{msg_send, rc::Retained, runtime::AnyObject, sel, MainThreadMarker};
use objc2_app_kit::{
    NSAppKitVersionNumber, NSAutoresizingMaskOptions, NSColor, NSGlassEffectViewStyle, NSView,
    NSWindowOrderingMode,
};
use objc2_foundation::{NSArray, NSInteger};

use crate::{macos::ns_glass_effect_view_tagged::NSGlassEffectViewTagged, Error};

/// NSView::tag for NSVisualEffectViewTagged, just a random number
pub const NS_VIEW_TAG_GLASS_VIEW: NSInteger = 96945937;

const MOVED_CONTENT_KEY: &str = "WindowVibrancyMovedContentKey";

extern "C" {
    fn objc_setAssociatedObject(
        object: *const c_void,
        key: *const c_void,
        value: *const c_void,
        policy: usize,
    );
    fn objc_getAssociatedObject(object: *const c_void, key: *const c_void) -> *mut c_void;
}

const OBJC_ASSOCIATION_ASSIGN: usize = 0x0;

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

        // Move primary content view into glass view's contentView before adding to hierarchy
        move_primary_content_view(view, &glass_view, radius.unwrap_or(0.0));

        view.addSubview_positioned_relativeTo(&glass_view, NSWindowOrderingMode::Below, None);
    }

    Ok(())
}

pub unsafe fn clear_liquid_glass(ns_view: NonNull<c_void>) -> Result<bool, Error> {
    let view: &NSView = ns_view.cast().as_ref();
    let glass_view = view.viewWithTag(NS_VIEW_TAG_GLASS_VIEW);

    if let Some(glass_view) = glass_view {
        restore_primary_content_view(view);
        glass_view.removeFromSuperview();
        return Ok(true);
    }

    Ok(false)
}

/// Gets the contentView of NSGlassEffectView
fn glass_content_view(glass: &NSView) -> Option<*mut NSView> {
    unsafe {
        let has_content_view: bool = msg_send![glass, respondsToSelector: sel!(contentView)];
        if has_content_view {
            let content_ptr: *mut NSView = msg_send![glass, contentView];
            if !content_ptr.is_null() {
                return Some(content_ptr);
            }
        }
        None
    }
}

/// Applies corner radius to a view's layer
unsafe fn apply_corner_radius_layer(view: &NSView, radius: f64) {
    let _: () = msg_send![view, setWantsLayer: true];
    let layer: *mut AnyObject = msg_send![view, layer];
    if !layer.is_null() {
        let _: () = msg_send![layer, setCornerRadius: radius];
        let _: () = msg_send![layer, setMasksToBounds: true];
    }
}

/// Finds WKWebView recursively in the view hierarchy
fn find_webview_recursive(view: &NSView) -> Option<Retained<NSView>> {
    unsafe {
        if let Some(wk_class) = objc2::runtime::AnyClass::get(c"WKWebView") {
            let is_webview: bool = msg_send![view, isKindOfClass: wk_class];
            if is_webview {
                return Retained::retain(view as *const _ as *mut NSView);
            }
        }

        let subviews_ptr: *mut AnyObject = msg_send![view, subviews];
        if subviews_ptr.is_null() {
            return None;
        }

        let subviews: &NSArray<NSView> = &*(subviews_ptr as *const NSArray<NSView>);
        let count = subviews.count();
        for idx in 0..count {
            let subview = subviews.objectAtIndex(idx);
            if let Some(found) = find_webview_recursive(subview.as_ref()) {
                return Some(found);
            }
        }
    }

    None
}

/// Moves primary content view beneath the glass view for layering effect
fn move_primary_content_view(container: &NSView, glass: &NSView, radius: f64) {
    unsafe {
        let target_ptr = glass_content_view(glass).unwrap_or(glass as *const _ as *mut NSView);
        if target_ptr.is_null() {
            return;
        }

        let target_view = &*target_ptr;
        if radius > 0.0 {
            apply_corner_radius_layer(target_view, radius);
        }

        if let Some(webview) = find_webview_recursive(container) {
            let mask = NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable;
            let webview_ptr = Retained::as_ptr(&webview) as *mut NSView;
            let view_ref: &NSView = webview.as_ref();

            view_ref.removeFromSuperview();
            target_view.addSubview(view_ref);

            let bounds = target_view.bounds();
            let _: () = msg_send![view_ref, setFrame: bounds];
            view_ref.setAutoresizingMask(mask);

            let moved_key = MOVED_CONTENT_KEY.as_ptr() as *const c_void;
            objc_setAssociatedObject(
                container as *const _ as *const c_void,
                moved_key,
                webview_ptr as *const c_void,
                OBJC_ASSOCIATION_ASSIGN,
            );
        }
    }
}

/// Restores primary content view to original position
fn restore_primary_content_view(container: &NSView) {
    unsafe {
        let moved_key = MOVED_CONTENT_KEY.as_ptr() as *const c_void;
        let moved_ptr = objc_getAssociatedObject(container as *const _ as *const c_void, moved_key)
            as *mut NSView;

        if moved_ptr.is_null() {
            return;
        }

        let view = &*moved_ptr;
        view.removeFromSuperview();
        container.addSubview(view);

        let bounds = container.bounds();
        let _: () = msg_send![view, setFrame: bounds];
        let mask = NSAutoresizingMaskOptions::ViewWidthSizable
            | NSAutoresizingMaskOptions::ViewHeightSizable;
        view.setAutoresizingMask(mask);

        objc_setAssociatedObject(
            container as *const _ as *const c_void,
            moved_key,
            std::ptr::null(),
            OBJC_ASSOCIATION_ASSIGN,
        );
    }
}
