use objc2::{msg_send, runtime::AnyObject};
use objc2_app_kit::NSView;
use objc2_foundation::NSArray;
use std::ptr::NonNull;

pub fn find_webview_recursive(view: &NSView) -> Option<NonNull<NSView>> {
    unsafe {
        if let Some(wk_class) = objc2::runtime::AnyClass::get(c"WKWebView") {
            let is_webview: bool = msg_send![view, isKindOfClass: wk_class];
            if is_webview {
                return NonNull::new(view as *const _ as *mut NSView);
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
