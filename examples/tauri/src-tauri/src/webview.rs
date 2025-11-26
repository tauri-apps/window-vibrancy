use objc2_app_kit::NSView;
use objc2_foundation::NSObjectProtocol;
use std::ptr::NonNull;

pub fn find_webview_recursive(view: &NSView) -> Option<NonNull<NSView>> {
    if let Some(wk_class) = objc2::runtime::AnyClass::get(c"WKWebView") {
        if view.isKindOfClass(wk_class) {
            return NonNull::new(view as *const _ as *mut NSView);
        }
    }

    let subviews = view.subviews();
    let count = subviews.count();
    for idx in 0..count {
        let subview = subviews.objectAtIndex(idx);
        if let Some(found) = find_webview_recursive(subview.as_ref()) {
            return Some(found);
        }
    }

    None
}

pub fn get_nsview_from_window(window: &tauri::WebviewWindow) -> *mut std::ffi::c_void {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    match window.window_handle() {
        Ok(handle) => match handle.as_raw() {
            RawWindowHandle::AppKit(h) => h.ns_view.as_ptr(),
            _ => std::ptr::null_mut(),
        },
        Err(_) => std::ptr::null_mut(),
    }
}
