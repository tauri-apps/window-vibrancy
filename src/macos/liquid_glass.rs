use std::ffi::c_void;
use std::ptr::NonNull;

use objc2::rc::Retained;
use objc2::runtime::{AnyClass, AnyObject, MessageReceiver, Sel};
use objc2::{class, msg_send, sel};
use objc2_app_kit::{
    NSAppKitVersionNumber, NSAutoresizingMaskOptions, NSBox, NSColor, NSView,
    NSVisualEffectBlendingMode, NSVisualEffectMaterial,
    NSVisualEffectState as AppKitVisualEffectState, NSVisualEffectView, NSWindowOrderingMode,
};
use objc2_foundation::{MainThreadMarker, NSArray, NSRect};

use crate::macos::ns_glass_effect_view::{
    NSGlassEffectVariant, NSGlassEffectView, NSGlassEffectViewExt,
};
use crate::{Color, Error};

const GLASS_EFFECT_KEY: &str = "WindowVibrancyGlassEffectKey";
const MOVED_CONTENT_KEY: &str = "WindowVibrancyMovedContentKey";
const BACKGROUND_VIEW_KEY: &str = "WindowVibrancyBackgroundViewKey";

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
const OBJC_ASSOCIATION_RETAIN: usize = 0x301;

#[derive(Debug, Clone)]
pub struct LiquidGlassOptions {
    pub variant: NSGlassEffectVariant,
    pub tint: Option<Color>,
    pub radius: Option<f64>,
    pub opaque: Option<bool>,
    pub state: Option<crate::macos::NSVisualEffectState>,
}

impl Default for LiquidGlassOptions {
    fn default() -> Self {
        Self {
            variant: NSGlassEffectVariant::Regular,
            tint: None,
            radius: None,
            opaque: None,
            state: None,
        }
    }
}

pub unsafe fn apply_liquid_glass(
    ns_view: NonNull<c_void>,
    options: LiquidGlassOptions,
) -> Result<(), Error> {
    let mtm = MainThreadMarker::new().ok_or(Error::NotMainThread(
        "apply_liquid_glass must be called on main thread",
    ))?;

    let container: &NSView = ns_view.cast().as_ref();
    remove_existing_glass(container);
    remove_visual_effect_views(container);

    let bounds = container.bounds();
    let use_opaque = options.opaque.unwrap_or(false);
    let background_view = if use_opaque {
        let bg = create_background_box(&mtm, bounds);
        if let Some(radius) = options.radius {
            if radius > 0.0 {
                unsafe { apply_corner_radius_layer(bg.as_ref(), radius) };
            }
        }
        container.addSubview_positioned_relativeTo(
            &bg,
            NSWindowOrderingMode::Below,
            None::<&NSView>,
        );
        Some(bg)
    } else {
        None
    };

    let glass_view = if let Some(glass) =
        unsafe { <NSGlassEffectView as NSGlassEffectViewExt>::new_with_frame(bounds) }
    {
        unsafe {
            let mask = NSAutoresizingMaskOptions::ViewWidthSizable
                | NSAutoresizingMaskOptions::ViewHeightSizable;
            glass.set_autoresizing_mask(mask);
        }

        Retained::into_super(glass)
    } else {
        let visual = NSVisualEffectView::initWithFrame(mtm.alloc(), bounds);
        visual.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
        visual.setMaterial(NSVisualEffectMaterial::UnderWindowBackground);
        visual.setState(AppKitVisualEffectState::Active);

        let mask = NSAutoresizingMaskOptions::ViewWidthSizable
            | NSAutoresizingMaskOptions::ViewHeightSizable;
        let view: &NSView = visual.as_ref();
        view.setAutoresizingMask(mask);

        Retained::into_super(visual)
    };

    set_glass_style(glass_view.as_ref(), options.variant)?;
    configure_glass_appearance(glass_view.as_ref(), &options)?;
    set_view_ignores_mouse_events(glass_view.as_ref(), true)?;

    if let Some(webview) = find_webview_recursive(container) {
        prepare_webview_with_optimization(webview.as_ref())?;
    }

    move_primary_content_view(
        container,
        glass_view.as_ref(),
        options.radius.unwrap_or(0.0),
    );

    if let Some(ref bg) = background_view {
        container.addSubview_positioned_relativeTo(
            glass_view.as_ref(),
            NSWindowOrderingMode::Above,
            Some(bg.as_ref()),
        );
    } else {
        unsafe { add_glass_to_container(container, glass_view.as_ref())? };
    }

    unsafe {
        let key = GLASS_EFFECT_KEY.as_ptr() as *const c_void;
        let ptr = Retained::as_ptr(&glass_view) as *const c_void;
        objc_setAssociatedObject(
            container as *const _ as *const c_void,
            key,
            ptr,
            OBJC_ASSOCIATION_RETAIN,
        );
    }

    if let Some(bg) = background_view {
        unsafe {
            let key = BACKGROUND_VIEW_KEY.as_ptr() as *const c_void;
            let ptr = Retained::as_ptr(&bg) as *const c_void;
            objc_setAssociatedObject(
                container as *const _ as *const c_void,
                key,
                ptr,
                OBJC_ASSOCIATION_RETAIN,
            );
        }
    }

    Ok(())
}

pub unsafe fn clear_liquid_glass(ns_view: NonNull<c_void>) -> Result<bool, Error> {
    let _mtm = MainThreadMarker::new().ok_or(Error::NotMainThread(
        "clear_liquid_glass must be called on main thread",
    ))?;

    let container: &NSView = ns_view.cast().as_ref();
    Ok(remove_existing_glass(container))
}

/// Creates opaque background layer using NSBox (approach from electron-liquid-glass).
fn create_background_box(mtm: &MainThreadMarker, bounds: NSRect) -> Retained<NSView> {
    unsafe {
        let background_box = NSBox::initWithFrame(mtm.alloc(), bounds);

        const NS_BOX_CUSTOM: isize = 4;
        const NS_NO_BORDER: isize = 0;
        let _: () = msg_send![&background_box, setBoxType: NS_BOX_CUSTOM];
        let _: () = msg_send![&background_box, setBorderType: NS_NO_BORDER];

        let window_bg_color = NSColor::windowBackgroundColor();
        background_box.setFillColor(&window_bg_color);

        let _: () = msg_send![&background_box, setWantsLayer: true];

        let mask = NSAutoresizingMaskOptions::ViewWidthSizable
            | NSAutoresizingMaskOptions::ViewHeightSizable;
        let view: &NSView = background_box.as_ref();
        view.setAutoresizingMask(mask);

        Retained::into_super(background_box)
    }
}

fn configure_glass_appearance(glass: &NSView, options: &LiquidGlassOptions) -> Result<(), Error> {
    unsafe {
        if let Some(radius) = options.radius {
            if radius > 0.0 {
                let selector = sel!(setCornerRadius:);
                let responds_corner: bool = msg_send![glass, respondsToSelector: selector];

                if responds_corner {
                    let _: () = MessageReceiver::send_message(
                        glass as *const _ as *mut AnyObject,
                        selector,
                        (radius,),
                    );

                    if let Some(content_ptr) = glass_content_view(glass) {
                        let content_view = &*content_ptr;
                        apply_corner_radius_layer(content_view, radius);
                    }
                } else {
                    apply_corner_radius_layer(glass, radius);
                }
            }
        }

        if let Some(tint) = options.tint {
            let color = color_to_nscolor(tint);
            let selector = sel!(setTintColor:);
            let responds: bool = msg_send![glass, respondsToSelector: selector];

            if responds {
                let _: () = MessageReceiver::send_message(
                    glass as *const _ as *mut AnyObject,
                    selector,
                    (&*color,),
                );
            } else {
                let _: () = msg_send![glass, setWantsLayer: true];
                let layer: *mut AnyObject = msg_send![glass, layer];
                if !layer.is_null() {
                    let cg_color: *mut AnyObject = msg_send![&*color, CGColor];
                    let _: () = msg_send![layer, setBackgroundColor: cg_color];
                }
            }
        }
    }

    let state = options
        .state
        .map(|state| AppKitVisualEffectState(state as isize))
        .unwrap_or(AppKitVisualEffectState::FollowsWindowActiveState);
    set_glass_state(glass, state)?;
    Ok(())
}

fn color_to_nscolor((r, g, b, a): Color) -> Retained<NSColor> {
    let rf = r as f64 / 255.0;
    let gf = g as f64 / 255.0;
    let bf = b as f64 / 255.0;
    let af = a as f64 / 255.0;

    NSColor::colorWithRed_green_blue_alpha(rf, gf, bf, af)
}

unsafe fn apply_corner_radius_layer(view: &NSView, radius: f64) {
    let _: () = msg_send![view, setWantsLayer: true];
    let layer: *mut AnyObject = msg_send![view, layer];
    if !layer.is_null() {
        let _: () = msg_send![layer, setCornerRadius: radius];
        let _: () = msg_send![layer, setMasksToBounds: true];
    }
}

unsafe fn add_glass_to_container(container: &NSView, glass: &NSView) -> Result<(), Error> {
    container.addSubview_positioned_relativeTo(glass, NSWindowOrderingMode::Below, None::<&NSView>);
    Ok(())
}

fn retained_glass_view(container: &NSView) -> Option<Retained<NSView>> {
    unsafe {
        let key = GLASS_EFFECT_KEY.as_ptr() as *const c_void;
        let ptr = objc_getAssociatedObject(container as *const _ as *const c_void, key);
        if ptr.is_null() {
            None
        } else {
            Retained::retain(ptr as *mut NSView)
        }
    }
}

fn remove_existing_glass(container: &NSView) -> bool {
    let had_glass = if let Some(glass) = retained_glass_view(container) {
        restore_primary_content_view(container);

        let view: &NSView = glass.as_ref();
        view.removeFromSuperview();
        clear_associated_glass(container);
        true
    } else {
        false
    };

    if let Some(background) = retained_background_view(container) {
        let view: &NSView = background.as_ref();
        view.removeFromSuperview();
        clear_associated_background(container);
    }

    had_glass
}

fn retained_background_view(container: &NSView) -> Option<Retained<NSView>> {
    unsafe {
        let key = BACKGROUND_VIEW_KEY.as_ptr() as *const c_void;
        let ptr = objc_getAssociatedObject(container as *const _ as *const c_void, key);
        if ptr.is_null() {
            None
        } else {
            Retained::retain(ptr as *mut NSView)
        }
    }
}

fn remove_visual_effect_views(container: &NSView) {
    unsafe {
        let subviews_ptr: *mut AnyObject = msg_send![container, subviews];
        if subviews_ptr.is_null() {
            return;
        }

        let subviews: &NSArray<NSView> = &*(subviews_ptr as *const NSArray<NSView>);
        let count = subviews.count();
        let glass_class = AnyClass::get(c"NSGlassEffectView");
        let visual_class = class!(NSVisualEffectView);

        let mut views_to_remove: Vec<*mut NSView> = Vec::new();

        for idx in 0..count {
            let subview = subviews.objectAtIndex(idx);
            let subview_ptr = Retained::as_ptr(&subview) as *mut NSView;
            let subview: &NSView = subview.as_ref();

            let is_glass = glass_class.is_some_and(|cls| {
                let result: bool = msg_send![subview, isKindOfClass: cls];
                result
            });

            let is_visual: bool = msg_send![subview, isKindOfClass: visual_class];

            if is_glass || is_visual {
                views_to_remove.push(subview_ptr);
            }
        }

        for view_ptr in views_to_remove {
            let view = &*view_ptr;
            view.removeFromSuperview();
        }
    }
}

fn clear_associated_glass(container: &NSView) {
    unsafe {
        let key = GLASS_EFFECT_KEY.as_ptr() as *const c_void;
        objc_setAssociatedObject(
            container as *const _ as *const c_void,
            key,
            std::ptr::null(),
            OBJC_ASSOCIATION_RETAIN,
        );

        let moved_key = MOVED_CONTENT_KEY.as_ptr() as *const c_void;
        objc_setAssociatedObject(
            container as *const _ as *const c_void,
            moved_key,
            std::ptr::null(),
            OBJC_ASSOCIATION_ASSIGN,
        );
    }
}

fn clear_associated_background(container: &NSView) {
    unsafe {
        let key = BACKGROUND_VIEW_KEY.as_ptr() as *const c_void;
        objc_setAssociatedObject(
            container as *const _ as *const c_void,
            key,
            std::ptr::null(),
            OBJC_ASSOCIATION_RETAIN,
        );
    }
}

/// Optimizes WKWebView for glass blending. Catches panics to avoid propagation
/// since optimization is optional and non-critical.
fn prepare_webview_with_optimization(webview: &NSView) -> Result<(), Error> {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        let responds_to_draws_bg: bool =
            msg_send![webview, respondsToSelector: sel!(setDrawsBackground:)];
        if responds_to_draws_bg {
            let _: () = msg_send![webview, setDrawsBackground: false];
        }

        let responds_to_bg_color: bool =
            msg_send![webview, respondsToSelector: sel!(setBackgroundColor:)];
        if responds_to_bg_color {
            let clear = NSColor::clearColor();
            let _: () = msg_send![webview, setBackgroundColor: &*clear];
        }

        let responds_to_wants_layer: bool =
            msg_send![webview, respondsToSelector: sel!(setWantsLayer:)];
        if responds_to_wants_layer {
            let _: () = msg_send![webview, setWantsLayer: true];
        }

        let responds_to_layer: bool = msg_send![webview, respondsToSelector: sel!(layer)];
        if responds_to_layer {
            let layer: *mut AnyObject = msg_send![webview, layer];
            if !layer.is_null() {
                let _: () = msg_send![layer, setOpaque: false];
                let _: () = msg_send![layer, setZPosition: 1.0_f64];
                let _: () = msg_send![layer, setShouldRasterize: true];

                let main_screen: *mut AnyObject = msg_send![class!(NSScreen), mainScreen];
                if !main_screen.is_null() {
                    let backing_scale: f64 = msg_send![main_screen, backingScaleFactor];
                    let _: () = msg_send![layer, setRasterizationScale: backing_scale];
                }
            }
        }
    }));

    if result.is_err() {
        eprintln!("[WARN] Failed to optimize webview (non-critical)");
    }

    Ok(())
}

fn find_webview_recursive(view: &NSView) -> Option<Retained<NSView>> {
    unsafe {
        if let Some(wk_class) = AnyClass::get(c"WKWebView") {
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

fn set_glass_style(view: &NSView, style: NSGlassEffectVariant) -> Result<(), Error> {
    unsafe {
        if <NSGlassEffectView as NSGlassEffectViewExt>::is_available() {
            let ptr = view as *const NSView as *mut NSGlassEffectView;
            if !ptr.is_null() {
                if let Some(glass_view) = Retained::retain(ptr) {
                    if glass_view.set_style_variant(style) {
                        return Ok(());
                    }
                }
            }
        }

        let selectors = [sel!(setStyle:)];
        send_optional_i64(view, &selectors, style as i64)
    }
}

fn set_glass_state(view: &NSView, state: AppKitVisualEffectState) -> Result<(), Error> {
    unsafe {
        if <NSGlassEffectView as NSGlassEffectViewExt>::is_available() {
            let ptr = view as *const NSView as *mut NSGlassEffectView;
            if !ptr.is_null() {
                if let Some(glass_view) = Retained::retain(ptr) {
                    if glass_view.set_state(state) {
                        return Ok(());
                    }
                }
            }
        }

        let raw_state = state.0 as i64;
        let selectors = [Sel::register(c"set_state:"), sel!(setState:)];
        send_optional_i64(view, &selectors, raw_state)
    }
}

fn send_optional_i64(view: &NSView, selectors: &[Sel], value: i64) -> Result<(), Error> {
    unsafe {
        for &sel in selectors {
            let responds: bool = msg_send![view, respondsToSelector: sel];
            if responds {
                let val: isize = value as isize;
                let _: () =
                    MessageReceiver::send_message(view as *const _ as *mut AnyObject, sel, (val,));
                return Ok(());
            }
        }
    }

    Ok(())
}

fn set_view_ignores_mouse_events(view: &NSView, ignores: bool) -> Result<(), Error> {
    unsafe {
        if <NSGlassEffectView as NSGlassEffectViewExt>::is_available() {
            let ptr = view as *const NSView as *mut NSGlassEffectView;
            if !ptr.is_null() {
                if let Some(glass_view) = Retained::retain(ptr) {
                    if glass_view.set_ignores_mouse_events(ignores) {
                        return Ok(());
                    }
                }
            }
        }

        let selectors = [
            Sel::register(c"set_ignoresMouseEvents:"),
            sel!(setIgnoresMouseEvents:),
        ];
        send_optional_bool(view, &selectors, ignores)
    }
}

fn send_optional_bool(view: &NSView, selectors: &[Sel], value: bool) -> Result<(), Error> {
    unsafe {
        for &sel in selectors {
            let responds: bool = msg_send![view, respondsToSelector: sel];
            if responds {
                let objc_value = objc2::runtime::Bool::new(value);
                let _: () = MessageReceiver::send_message(
                    view as *const _ as *mut AnyObject,
                    sel,
                    (objc_value,),
                );
                return Ok(());
            }
        }
    }

    Ok(())
}

/// Reparents primary content beneath the glass view for layering effect.
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
            return;
        }

        let subviews_ptr: *mut AnyObject = msg_send![container, subviews];
        if subviews_ptr.is_null() {
            return;
        }

        let subviews: &NSArray<NSView> = &*(subviews_ptr as *const NSArray<NSView>);
        let count = subviews.count();
        if count == 0 {
            return;
        }

        let mask = NSAutoresizingMaskOptions::ViewWidthSizable
            | NSAutoresizingMaskOptions::ViewHeightSizable;

        // Move only first subview to preserve stacking order
        let subview = subviews.objectAtIndex(0);
        let subview_ptr = Retained::as_ptr(&subview) as *mut NSView;
        let subview: &NSView = subview.as_ref();

        subview.removeFromSuperview();
        target_view.addSubview(subview);

        let bounds = target_view.bounds();
        let _: () = msg_send![subview, setFrame: bounds];
        subview.setAutoresizingMask(mask);

        let moved_key = MOVED_CONTENT_KEY.as_ptr() as *const c_void;
        objc_setAssociatedObject(
            container as *const _ as *const c_void,
            moved_key,
            subview_ptr as *const c_void,
            OBJC_ASSOCIATION_ASSIGN,
        );
    }
}

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

fn glass_content_view(glass: &NSView) -> Option<*mut NSView> {
    unsafe {
        if <NSGlassEffectView as NSGlassEffectViewExt>::is_available() {
            let ptr = glass as *const NSView as *mut NSGlassEffectView;
            if !ptr.is_null() {
                if let Some(glass_view) = Retained::retain(ptr) {
                    if let Some(content) = glass_view.content_view() {
                        return Some(Retained::as_ptr(&content) as *mut NSView);
                    }
                }
            }
        }

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
