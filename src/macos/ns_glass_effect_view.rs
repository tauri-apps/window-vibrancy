use objc2::rc::Retained;
use objc2::{msg_send, sel};
use objc2_app_kit::{NSAutoresizingMaskOptions, NSView, NSVisualEffectState};
use objc2_foundation::NSRect;

pub use objc2_app_kit::NSGlassEffectView;

#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NSGlassEffectVariant {
    Regular = 0,
    Clear = 1,
    Dock = 2,
    AppIcons = 3,
    Widgets = 4,
    Text = 5,
    AvPlayer = 6,
    FaceTime = 7,
    ControlCenter = 8,
    NotificationCenter = 9,
    Monogram = 10,
    Bubbles = 11,
    Identity = 12,
    FocusBorder = 13,
    FocusPlatter = 14,
    Keyboard = 15,
    Sidebar = 16,
    AbuttedSidebar = 17,
    Inspector = 18,
    Control = 19,
    Loupe = 20,
    Slider = 21,
    Camera = 22,
    CartouchePopover = 23,
}

impl Default for NSGlassEffectVariant {
    fn default() -> Self {
        Self::Regular
    }
}

/// Extends official NSGlassEffectView with additional style variants and controls.
///
/// Provides unified interface for both official (contentView, autoresizing) and
/// extended functionality (style variants, state control, mouse event handling).
/// Extended methods use Objective-C runtime messaging to access private APIs.
pub trait NSGlassEffectViewExt {
    fn is_available() -> bool;

    /// Returns None if NSGlassEffectView unavailable (macOS < 26.0).
    unsafe fn new_with_frame(frame: NSRect) -> Option<Retained<Self>>;

    /// Applies extended style variants via setStyle: selector with integer values.
    /// Returns false if selector not found (API unavailable).
    unsafe fn set_style_variant(&self, style: NSGlassEffectVariant) -> bool;

    unsafe fn set_state(&self, state: NSVisualEffectState) -> bool;

    unsafe fn set_ignores_mouse_events(&self, ignores: bool) -> bool;

    /// Content view is where subviews should be added to appear with glass effect.
    unsafe fn content_view(&self) -> Option<Retained<NSView>>;

    unsafe fn set_autoresizing_mask(&self, mask: NSAutoresizingMaskOptions);
}

impl NSGlassEffectViewExt for NSGlassEffectView {
    fn is_available() -> bool {
        objc2::runtime::AnyClass::get(c"NSGlassEffectView").is_some()
    }

    unsafe fn new_with_frame(frame: NSRect) -> Option<Retained<Self>> {
        use objc2_foundation::MainThreadMarker;

        let _mtm = MainThreadMarker::new()?;

        let cls = objc2::runtime::AnyClass::get(c"NSGlassEffectView")?;
        let obj: *mut NSGlassEffectView = msg_send![cls, alloc];
        let initialized: *mut NSGlassEffectView = msg_send![obj, initWithFrame: frame];

        Retained::from_raw(initialized)
    }

    unsafe fn set_style_variant(&self, style: NSGlassEffectVariant) -> bool {
        let sel = sel!(setStyle:);
        let responds: bool = msg_send![self, respondsToSelector: sel];
        if !responds {
            return false;
        }

        let val = style as i64 as isize;
        let _: () = msg_send![self, setStyle: val];
        true
    }

    unsafe fn set_state(&self, state: NSVisualEffectState) -> bool {
        let sel = sel!(setState:);
        let responds: bool = msg_send![self, respondsToSelector: sel];
        if !responds {
            return false;
        }

        let raw_state = state.0 as i64 as isize;
        let _: () = msg_send![self, setState: raw_state];
        true
    }

    unsafe fn set_ignores_mouse_events(&self, ignores: bool) -> bool {
        let sel = sel!(setIgnoresMouseEvents:);
        let responds: bool = msg_send![self, respondsToSelector: sel];
        if !responds {
            return false;
        }

        let objc_value = objc2::runtime::Bool::new(ignores);
        let _: () = msg_send![self, setIgnoresMouseEvents: objc_value];
        true
    }

    unsafe fn content_view(&self) -> Option<Retained<NSView>> {
        self.contentView()
    }

    unsafe fn set_autoresizing_mask(&self, mask: NSAutoresizingMaskOptions) {
        let view_retained = objc2::rc::Retained::retain(self as *const _ as *mut NSView).unwrap();
        let view: &NSView = view_retained.as_ref();
        view.setAutoresizingMask(mask);
    }
}
