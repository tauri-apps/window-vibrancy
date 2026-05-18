// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{
    ptr,
    sync::{Mutex, MutexGuard},
};

use wayland_client::{
    backend::protocol::{wl_interface, Interface},
    protocol::{wl_compositor, wl_surface},
};
use wayland_sys::client::{self, wl_display, wl_event_queue, wl_proxy};

use crate::Error;

const APPLY_BLUR_UNSUPPORTED: &str =
    "\"apply_blur()\" requires a Wayland compositor with ext-background-effect-v1 blur support.";
const WAYLAND_QUERY_FAILED: &str =
    "\"apply_blur()\" could not query Wayland compositor blur support.";
const WAYLAND_REQUEST_FAILED: &str = "\"apply_blur()\" could not request Wayland compositor blur.";

pub(super) fn ensure_wayland_client() -> Result<(), Error> {
    if client::is_lib_available() {
        Ok(())
    } else {
        Err(unsupported())
    }
}

pub(super) unsafe fn release_compositor(proxy: *mut wl_proxy, version: u32) {
    if version >= wl_compositor::REQ_RELEASE_SINCE {
        unsafe {
            send_destructor(proxy, wl_compositor::REQ_RELEASE_OPCODE);
        }
    } else {
        unsafe {
            wayland_sys::ffi_dispatch!(client::wayland_client_handle(), wl_proxy_destroy, proxy);
        }
    }
}

pub(super) unsafe fn send_destructor(proxy: *mut wl_proxy, opcode: u16) {
    unsafe {
        wayland_sys::ffi_dispatch!(
            client::wayland_client_handle(),
            wl_proxy_marshal_array,
            proxy,
            opcode.into(),
            ptr::null_mut()
        );
        wayland_sys::ffi_dispatch!(client::wayland_client_handle(), wl_proxy_destroy, proxy);
    }
}

pub(super) unsafe fn commit_surface(surface: *mut wl_proxy) {
    unsafe {
        wayland_sys::ffi_dispatch!(
            client::wayland_client_handle(),
            wl_proxy_marshal_array,
            surface,
            wl_surface::REQ_COMMIT_OPCODE.into(),
            ptr::null_mut()
        );
    }
}

pub(super) unsafe fn flush_display(display: *mut wl_display) -> Result<(), Error> {
    let result = unsafe {
        wayland_sys::ffi_dispatch!(client::wayland_client_handle(), wl_display_flush, display)
    };
    if result < 0 {
        return Err(request_failed());
    }

    Ok(())
}

pub(super) unsafe fn roundtrip_queue(
    display: *mut wl_display,
    queue: *mut wl_event_queue,
) -> Result<(), Error> {
    let result = unsafe {
        wayland_sys::ffi_dispatch!(
            client::wayland_client_handle(),
            wl_display_roundtrip_queue,
            display,
            queue
        )
    };
    if result < 0 {
        return Err(query_failed());
    }

    Ok(())
}

pub(super) unsafe fn proxy_id(proxy: *mut wl_proxy) -> u32 {
    unsafe { wayland_sys::ffi_dispatch!(client::wayland_client_handle(), wl_proxy_get_id, proxy) }
}

pub(super) fn c_interface(interface: &'static Interface) -> *const wl_interface {
    interface
        .c_ptr
        .expect("generated Wayland interfaces include C metadata")
}

pub(super) fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|error| error.into_inner())
}

pub(super) fn unsupported() -> Error {
    Error::UnsupportedPlatformVersion(APPLY_BLUR_UNSUPPORTED)
}

pub(super) fn query_failed() -> Error {
    Error::UnsupportedPlatformVersion(WAYLAND_QUERY_FAILED)
}

pub(super) fn request_failed() -> Error {
    Error::UnsupportedPlatformVersion(WAYLAND_REQUEST_FAILED)
}
