use std::ffi::c_void;

use objc2::{
    ffi::{objc_getAssociatedObject, objc_setAssociatedObject},
    rc::Retained,
    runtime::AnyObject,
    MainThreadMarker,
};
use objc2_app_kit::{
    NSAppKitVersionNumber, NSAutoresizingMaskOptions, NSBox, NSBoxType, NSColor, NSGlassEffectView,
    NSGlassEffectViewStyle, NSView, NSWindowOrderingMode,
};
use objc2_foundation::{NSInteger, NSRect};

use crate::{macos::ns_glass_effect_view_tagged::NSGlassEffectViewTagged, Error};

/// NSView::tag for NSVisualEffectViewTagged, just a random number
pub const NS_VIEW_TAG_GLASS_VIEW: NSInteger = 96945937;

const MOVED_CONTENT_KEY: &str = "WindowVibrancyMovedContentKey";
const BACKGROUND_VIEW_KEY: &str = "WindowVibrancyBackgroundViewKey";

const OBJC_ASSOCIATION_ASSIGN: usize = 0x0;
const OBJC_ASSOCIATION_RETAIN: usize = 0x301;

/// Minimum NSAppKitVersionNumber for liquid glass support (macOS 26.0+)
const MIN_APPKIT_VERSION_LIQUID_GLASS: f64 = 2685.0;

#[derive(Debug, Clone)]
pub struct LiquidGlassOptions<'a> {
    pub(crate) style: super::NSGlassEffectViewStyle,
    pub(crate) tint_color: Option<crate::Color>,
    pub(crate) radius: Option<f64>,
    pub(crate) opaque: Option<bool>,
    pub(crate) content_view: Option<&'a NSView>,
}

impl<'a> LiquidGlassOptions<'a> {
    pub fn new(style: super::NSGlassEffectViewStyle) -> Self {
        Self {
            style,
            tint_color: None,
            radius: None,
            opaque: None,
            content_view: None,
        }
    }

    pub fn tint_color(mut self, color: crate::Color) -> Self {
        self.tint_color = Some(color);
        self
    }

    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn opaque(mut self, opaque: bool) -> Self {
        self.opaque = Some(opaque);
        self
    }

    pub fn content_view(mut self, view: &'a NSView) -> Self {
        self.content_view = Some(view);
        self
    }
}

impl<'a> Default for LiquidGlassOptions<'a> {
    fn default() -> Self {
        Self::new(super::NSGlassEffectViewStyle::Regular)
    }
}

pub fn apply_liquid_glass(view: &NSView, options: LiquidGlassOptions<'_>) -> Result<(), Error> {
    let mtm = MainThreadMarker::new().ok_or(Error::NotMainThread(
        "apply_liquid_glass() can only be used on the main thread.",
    ))?;

    if unsafe { NSAppKitVersionNumber } < MIN_APPKIT_VERSION_LIQUID_GLASS {
        return Err(Error::UnsupportedPlatformVersion(
            "apply_liquid_glass() is only available on macOS 26.0 or newer.",
        ));
    }

    let tint_color = options.tint_color.map(|(r, g, b, a)| {
        NSColor::colorWithRed_green_blue_alpha(
            r as f64 / 255.0,
            g as f64 / 255.0,
            b as f64 / 255.0,
            a as f64 / 255.0,
        )
    });

    let bounds = view.bounds();
    let use_opaque = options.opaque.unwrap_or(false);
    let radius = options.radius.unwrap_or(0.0);

    let background_view = if use_opaque {
        let bg = create_background_box(&mtm, bounds);
        if radius > 0.0 {
            unsafe { apply_corner_radius_layer(bg.as_ref(), radius) };
        }
        view.addSubview_positioned_relativeTo(&bg, NSWindowOrderingMode::Below, None::<&NSView>);
        Some(bg)
    } else {
        None
    };

    let style = NSGlassEffectViewStyle(options.style as isize);
    let glass_view = unsafe {
        NSGlassEffectViewTagged::initWithFrame(mtm.alloc(), bounds, NS_VIEW_TAG_GLASS_VIEW)
    };

    glass_view.setStyle(style);
    glass_view.setCornerRadius(radius);
    glass_view.setTintColor(tint_color.as_deref());
    glass_view.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable,
    );

    move_primary_content_view(view, options.content_view, &glass_view, radius);

    if let Some(ref bg) = background_view {
        view.addSubview_positioned_relativeTo(
            &glass_view,
            NSWindowOrderingMode::Above,
            Some(bg.as_ref()),
        );
    } else {
        view.addSubview_positioned_relativeTo(&glass_view, NSWindowOrderingMode::Below, None);
    }

    if let Some(bg) = background_view {
        associate_background_view(view, bg);
    }

    Ok(())
}

pub fn clear_liquid_glass(view: &NSView) -> Result<bool, Error> {
    let glass_view = view.viewWithTag(NS_VIEW_TAG_GLASS_VIEW);

    if let Some(glass_view) = glass_view {
        restore_primary_content_view(view);
        glass_view.removeFromSuperview();

        if let Some(background) = get_associated_background_view(view) {
            background.removeFromSuperview();
            clear_associated_background(view);
        }

        return Ok(true);
    }

    Ok(false)
}

unsafe fn apply_corner_radius_layer(view: &NSView, radius: f64) {
    if view.layer().is_none() {
        view.setWantsLayer(true);
    }

    if let Some(layer) = view.layer() {
        let current_radius = layer.cornerRadius();
        if (current_radius - radius).abs() > 0.001 {
            layer.setCornerRadius(radius);
        }

        if !layer.masksToBounds() {
            layer.setMasksToBounds(true);
        }
    }
}

fn move_primary_content_view(
    container: &NSView,
    content_view: Option<&NSView>,
    glass: &NSGlassEffectView,
    radius: f64,
) {
    let Some(content) = content_view else {
        return;
    };

    let content_view_retained = glass.contentView();
    let target_view: &NSView = content_view_retained
        .as_ref()
        .map(|retained| retained.as_ref())
        .unwrap_or_else(|| glass.as_ref());

    if radius > 0.0 {
        unsafe { apply_corner_radius_layer(target_view, radius) };
    }

    content.removeFromSuperview();
    target_view.addSubview(content);

    let bounds = target_view.bounds();
    content.setFrame(bounds);
    content.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable,
    );

    unsafe {
        let key = MOVED_CONTENT_KEY.as_ptr() as *const c_void;
        objc_setAssociatedObject(
            container as *const _ as *mut AnyObject,
            key,
            content as *const _ as *mut AnyObject,
            OBJC_ASSOCIATION_ASSIGN,
        );
    }
}

fn restore_primary_content_view(container: &NSView) {
    unsafe {
        let moved_key = MOVED_CONTENT_KEY.as_ptr() as *const c_void;
        let moved_ptr =
            objc_getAssociatedObject(container as *const _ as *const AnyObject, moved_key)
                as *mut NSView;

        if moved_ptr.is_null() {
            return;
        }

        let view = &*moved_ptr;
        view.removeFromSuperview();
        container.addSubview(view);

        let bounds = container.bounds();
        view.setFrame(bounds);
        let mask = NSAutoresizingMaskOptions::ViewWidthSizable
            | NSAutoresizingMaskOptions::ViewHeightSizable;
        view.setAutoresizingMask(mask);

        objc_setAssociatedObject(
            container as *const _ as *mut AnyObject,
            moved_key,
            std::ptr::null_mut(),
            OBJC_ASSOCIATION_ASSIGN,
        );
    }
}

fn create_background_box(mtm: &MainThreadMarker, bounds: NSRect) -> Retained<NSView> {
    let background_box = NSBox::initWithFrame(mtm.alloc(), bounds);

    background_box.setBoxType(NSBoxType::Custom);
    background_box.setTransparent(false);
    background_box.setBorderWidth(0.0);

    let window_bg_color = NSColor::windowBackgroundColor();
    background_box.setFillColor(&window_bg_color);

    let view: &NSView = background_box.as_ref();
    view.setWantsLayer(true);

    let mask =
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable;
    view.setAutoresizingMask(mask);

    Retained::into_super(background_box)
}

fn associate_background_view(container: &NSView, background: Retained<NSView>) {
    unsafe {
        let key = BACKGROUND_VIEW_KEY.as_ptr() as *const c_void;
        let ptr = Retained::as_ptr(&background) as *mut AnyObject;
        objc_setAssociatedObject(
            container as *const _ as *mut AnyObject,
            key,
            ptr,
            OBJC_ASSOCIATION_RETAIN,
        );
    }
}

fn get_associated_background_view(container: &NSView) -> Option<Retained<NSView>> {
    unsafe {
        let key = BACKGROUND_VIEW_KEY.as_ptr() as *const c_void;
        let ptr = objc_getAssociatedObject(container as *const _ as *const AnyObject, key);
        if ptr.is_null() {
            None
        } else {
            Retained::retain(ptr as *mut NSView)
        }
    }
}

fn clear_associated_background(container: &NSView) {
    unsafe {
        let key = BACKGROUND_VIEW_KEY.as_ptr() as *const c_void;
        objc_setAssociatedObject(
            container as *const _ as *mut AnyObject,
            key,
            std::ptr::null_mut(),
            OBJC_ASSOCIATION_RETAIN,
        );
    }
}
