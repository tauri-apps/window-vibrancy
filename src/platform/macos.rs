//! most of the code comes from https://github.com/joboet/winit/tree/macos_blurred_background
//! just rewrote part of it.

#![cfg(target_os = "macos")]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use cocoa::{
    appkit::{
        NSAutoresizingMaskOptions, NSView, NSViewHeightSizable, NSViewWidthSizable, NSWindow,
        NSWindowOrderingMode,
    },
    base::{id, nil, BOOL},
    foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize},
};
use lazy_static::lazy_static;
use objc::{class, msg_send, runtime::Class, sel, sel_impl};

lazy_static! {
    static ref NSVISUALEFFECTVIEW: Option<&'static Class> = Class::get("NSVisualEffectView");
}

// https://developer.apple.com/documentation/appkit/nsvisualeffectview/blendingmode
#[allow(dead_code)]
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSVisualEffectViewBlendingMode {
    BehindWindow = 0,
    WithinWindow = 1,
}

// https://developer.apple.com/documentation/appkit/nsvisualeffectview/state
#[allow(dead_code)]
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSVisualEffectViewState {
    FollowsWindowActiveState = 0,
    Active = 1,
    Inactive = 2,
}

// https://developer.apple.com/documentation/appkit/nsvisualeffectview/material
#[allow(dead_code)]
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSVisualEffectViewMaterial {
    #[deprecated(
        since = "macOS 10.14",
        note = "A default material for the view’s effective appearance. But deprecated"
    )]
    AppearanceBased = 0, // Default, Deprecated macOS 10.10–10.14
    #[deprecated(since = "macOS 10.14")]
    Light = 1, // Deprecated macOS 10.10–10.14
    #[deprecated(since = "macOS 10.14")]
    Dark = 2, // Deprecated macOS 10.10–10.14
    #[deprecated(since = "macOS 10.14")]
    MediumLight = 8, // Deprecated macOS 10.11–10.14
    #[deprecated(since = "macOS 10.14")]
    UltraDark = 9, // Deprecated macOS 10.11–10.14

    Titlebar = 3,  // macOS 10.10+
    Selection = 4, // macOS 10.10+

    Menu = 5,    // macOS 10.11+
    Popover = 6, // macOS 10.11+
    Sidebar = 7, // macOS 10.11+

    HeaderView = 10,            // macOS 10.14+
    Sheet = 11,                 // macOS 10.14+
    WindowBackground = 12,      // macOS 10.14+
    HudWindow = 13,             // macOS 10.14+
    FullScreenUI = 15,          // macOS 10.14+
    Tooltip = 17,               // macOS 10.14+
    ContentBackground = 18,     // macOS 10.14+
    UnderWindowBackground = 21, // macOS 10.14+
    UnderPageBackground = 22,   // macOS 10.14+
}

// macos 10.10+
// https://developer.apple.com/documentation/appkit/nsvisualeffectview
pub trait NSVisualEffectView: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSVisualEffectView), alloc]
    }

    unsafe fn init(self) -> id;
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id;
    unsafe fn bounds(self) -> NSRect;
    unsafe fn frame(self) -> NSRect;
    unsafe fn setFrameSize(self, frameSize: NSSize);
    unsafe fn setFrameOrigin(self, frameOrigin: NSPoint);

    unsafe fn superview(self) -> id;
    unsafe fn removeFromSuperview(self);
    unsafe fn setAutoresizingMask_(self, autoresizingMask: NSAutoresizingMaskOptions);

    unsafe fn isEmphasized(self) -> BOOL;
    unsafe fn setEmphasized_(self, emphasized: BOOL);
    unsafe fn setMaterial_(self, material: NSVisualEffectViewMaterial);
    unsafe fn setState_(self, state: NSVisualEffectViewState);
    unsafe fn setBlendingMode_(self, mode: NSVisualEffectViewBlendingMode);
}

impl NSVisualEffectView for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        msg_send![self, initWithFrame: frameRect]
    }

    unsafe fn bounds(self) -> NSRect {
        msg_send![self, bounds]
    }

    unsafe fn frame(self) -> NSRect {
        msg_send![self, frame]
    }

    unsafe fn setFrameSize(self, frameSize: NSSize) {
        msg_send![self, setFrameSize: frameSize]
    }

    unsafe fn setFrameOrigin(self, frameOrigin: NSPoint) {
        msg_send![self, setFrameOrigin: frameOrigin]
    }

    unsafe fn superview(self) -> id {
        msg_send![self, superview]
    }

    unsafe fn removeFromSuperview(self) {
        msg_send![self, removeFromSuperview]
    }

    unsafe fn setAutoresizingMask_(self, autoresizingMask: NSAutoresizingMaskOptions) {
        msg_send![self, setAutoresizingMask: autoresizingMask]
    }

    unsafe fn isEmphasized(self) -> BOOL {
        msg_send![self, isEmphasized]
    }

    unsafe fn setEmphasized_(self, emphasized: BOOL) {
        msg_send![self, setEmphasized: emphasized]
    }

    unsafe fn setMaterial_(self, material: NSVisualEffectViewMaterial) {
        msg_send![self, setMaterial: material]
    }

    unsafe fn setState_(self, state: NSVisualEffectViewState) {
        msg_send![self, setState: state]
    }

    unsafe fn setBlendingMode_(self, mode: NSVisualEffectViewBlendingMode) {
        msg_send![self, setBlendingMode: mode]
    }
}

#[allow(deprecated)]
pub fn apply_blur(window: id) {
    apply_blur_with_material(window, NSVisualEffectViewMaterial::AppearanceBased)
}

pub fn apply_blur_with_material(window: id, material: NSVisualEffectViewMaterial) {
    unsafe {
        if !msg_send![class!(NSThread), isMainThread] {
            panic!("Views can only be created on the main thread on macOS");
        }

        if let Some(_) = *NSVISUALEFFECTVIEW {
            let ns_view: id = window.contentView();
            let bounds = NSView::bounds(ns_view);

            let blurred_view =
                NSVisualEffectView::initWithFrame_(NSVisualEffectView::alloc(nil), bounds);
            blurred_view.autorelease();

            blurred_view.setMaterial_(material);
            blurred_view.setBlendingMode_(NSVisualEffectViewBlendingMode::BehindWindow);
            blurred_view.setState_(NSVisualEffectViewState::FollowsWindowActiveState);
            NSVisualEffectView::setAutoresizingMask_(
                blurred_view,
                NSViewWidthSizable | NSViewHeightSizable,
            );

            let _: () = msg_send![ns_view, addSubview: blurred_view.clone() positioned: NSWindowOrderingMode::NSWindowBelow relativeTo: 0];
        }
    }
}
