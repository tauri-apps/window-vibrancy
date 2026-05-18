// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{
    collections::HashMap,
    ffi::c_void,
    ptr,
    sync::{Mutex, OnceLock},
};

use wayland_client::{
    backend::protocol::wl_argument,
    protocol::{wl_compositor, wl_region},
    Proxy,
};
use wayland_protocols::ext::background_effect::v1::client::{
    ext_background_effect_manager_v1,
    ext_background_effect_surface_v1::{self, ExtBackgroundEffectSurfaceV1},
};
use wayland_sys::client::{self, wl_display, wl_proxy};

use crate::Error;

mod ffi;
mod registry;

pub(super) fn unsupported() -> Error {
    ffi::unsupported()
}

pub(super) unsafe fn apply_blur(
    display: *mut wl_display,
    surface: *mut wl_proxy,
) -> Result<(), Error> {
    ffi::ensure_wayland_client()?;

    let _operation = ffi::lock(operations());
    let mut displays = ffi::lock(registry::displays());
    let display_state = if let Some(display_state) = displays.get_mut(&(display as usize)) {
        display_state
    } else {
        let display_state = unsafe { registry::DisplayState::new(display)? };
        displays.insert(display as usize, display_state);
        displays
            .get_mut(&(display as usize))
            .expect("display was inserted")
    };

    let compositor = unsafe { display_state.ensure_compositor()? };
    let manager = unsafe { display_state.ensure_manager()? };

    if !display_state.supports_blur() {
        return Err(unsupported());
    }

    let effect = unsafe { ensure_effect(display, surface, manager)? };
    unsafe {
        set_full_blur_region(compositor, effect)?;
        ffi::commit_surface(surface);
        ffi::flush_display(display)?;
    }

    Ok(())
}

pub(super) unsafe fn clear_blur(surface: *mut wl_proxy) -> Result<(), Error> {
    ffi::ensure_wayland_client()?;

    let _operation = ffi::lock(operations());
    let effect = ffi::lock(effects()).remove(&(surface as usize));
    let Some(effect) = effect else {
        return Ok(());
    };

    unsafe {
        ffi::send_destructor(
            effect.effect as *mut wl_proxy,
            ext_background_effect_surface_v1::REQ_DESTROY_OPCODE,
        );
        ffi::commit_surface(surface);
        ffi::flush_display(effect.display as *mut wl_display)?;
    }

    Ok(())
}

unsafe fn ensure_effect(
    display: *mut wl_display,
    surface: *mut wl_proxy,
    manager: *mut wl_proxy,
) -> Result<*mut wl_proxy, Error> {
    let mut effects = ffi::lock(effects());
    if let Some(effect) = effects.get(&(surface as usize)) {
        return Ok(effect.effect as *mut wl_proxy);
    }

    let mut args = [wl_argument {
        o: surface.cast::<c_void>(),
    }];
    let effect = unsafe {
        wayland_sys::ffi_dispatch!(
            client::wayland_client_handle(),
            wl_proxy_marshal_array_constructor_versioned,
            manager,
            ext_background_effect_manager_v1::REQ_GET_BACKGROUND_EFFECT_OPCODE.into(),
            args.as_mut_ptr(),
            ffi::c_interface(ExtBackgroundEffectSurfaceV1::interface()),
            1
        )
    };

    if effect.is_null() {
        return Err(ffi::request_failed());
    }

    effects.insert(
        surface as usize,
        EffectState {
            display: display as usize,
            effect: effect as usize,
        },
    );

    Ok(effect)
}

unsafe fn set_full_blur_region(
    compositor: *mut wl_proxy,
    effect: *mut wl_proxy,
) -> Result<(), Error> {
    let region = unsafe {
        wayland_sys::ffi_dispatch!(
            client::wayland_client_handle(),
            wl_proxy_marshal_array_constructor,
            compositor,
            wl_compositor::REQ_CREATE_REGION_OPCODE.into(),
            ptr::null_mut(),
            ffi::c_interface(wl_region::WlRegion::interface())
        )
    };

    if region.is_null() {
        return Err(ffi::request_failed());
    }

    let mut add_args = [
        wl_argument { i: 0 },
        wl_argument { i: 0 },
        wl_argument {
            i: registry::FULL_SURFACE_REGION_SIZE,
        },
        wl_argument {
            i: registry::FULL_SURFACE_REGION_SIZE,
        },
    ];

    unsafe {
        wayland_sys::ffi_dispatch!(
            client::wayland_client_handle(),
            wl_proxy_marshal_array,
            region,
            wl_region::REQ_ADD_OPCODE.into(),
            add_args.as_mut_ptr()
        );

        let mut set_args = [wl_argument {
            o: region.cast::<c_void>(),
        }];
        wayland_sys::ffi_dispatch!(
            client::wayland_client_handle(),
            wl_proxy_marshal_array,
            effect,
            ext_background_effect_surface_v1::REQ_SET_BLUR_REGION_OPCODE.into(),
            set_args.as_mut_ptr()
        );
        ffi::send_destructor(region, wl_region::REQ_DESTROY_OPCODE);
    }

    Ok(())
}

struct EffectState {
    display: usize,
    effect: usize,
}

fn effects() -> &'static Mutex<HashMap<usize, EffectState>> {
    static EFFECTS: OnceLock<Mutex<HashMap<usize, EffectState>>> = OnceLock::new();
    EFFECTS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn operations() -> &'static Mutex<()> {
    static OPERATIONS: OnceLock<Mutex<()>> = OnceLock::new();
    OPERATIONS.get_or_init(|| Mutex::new(()))
}
