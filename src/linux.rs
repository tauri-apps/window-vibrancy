// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg(target_os = "linux")]

use raw_window_handle::{WaylandDisplayHandle, WaylandWindowHandle};
use wayland_sys::client::{wl_display, wl_proxy};

use crate::Error;

mod wayland;

pub fn apply_blur(
    display: WaylandDisplayHandle,
    surface: WaylandWindowHandle,
) -> Result<(), Error> {
    let display = display.display.as_ptr().cast::<wl_display>();
    let surface = surface.surface.as_ptr().cast::<wl_proxy>();

    if display.is_null() || surface.is_null() {
        return Err(wayland::unsupported());
    }

    unsafe { wayland::apply_blur(display, surface) }
}

pub fn clear_blur(surface: WaylandWindowHandle) -> Result<(), Error> {
    let surface = surface.surface.as_ptr().cast::<wl_proxy>();

    if surface.is_null() {
        return Err(wayland::unsupported());
    }

    unsafe { wayland::clear_blur(surface) }
}
