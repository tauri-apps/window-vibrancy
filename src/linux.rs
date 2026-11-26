// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg(target_os = "linux")]

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::{Mutex, OnceLock};

use wayland_client::backend::{Backend, ObjectId};
use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_client::protocol::{
    wl_compositor::WlCompositor, wl_region::WlRegion, wl_registry::WlRegistry,
    wl_surface::WlSurface,
};
use wayland_client::{delegate_noop, Connection, Dispatch, EventQueue, Proxy, QueueHandle, WEnum};
use wayland_protocols::ext::background_effect::v1::client::{
    ext_background_effect_manager_v1::{self, Capability, ExtBackgroundEffectManagerV1},
    ext_background_effect_surface_v1::ExtBackgroundEffectSurfaceV1,
};

use crate::{Error, RoundedRect};

/// What the compositor said it can do.
#[derive(Default)]
struct State {
    blur: bool,
}

impl Dispatch<WlRegistry, GlobalListContents> for State {
    fn event(
        _: &mut Self,
        _: &WlRegistry,
        _: <WlRegistry as Proxy>::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ExtBackgroundEffectManagerV1, ()> for State {
    fn event(
        state: &mut Self,
        _: &ExtBackgroundEffectManagerV1,
        event: ext_background_effect_manager_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let ext_background_effect_manager_v1::Event::Capabilities {
            flags: WEnum::Value(flags),
        } = event
        {
            state.blur = flags.contains(Capability::Blur);
        }
    }
}

delegate_noop!(State: WlCompositor);
delegate_noop!(State: WlRegion);
delegate_noop!(State: ExtBackgroundEffectSurfaceV1);

/// Our own queue on the application's `wl_display`.
struct Display {
    connection: Connection,
    events: EventQueue<State>,
    state: State,
    compositor: WlCompositor,
    manager: Option<ExtBackgroundEffectManagerV1>,
    /// By surface pointer and protocol id, as a destroyed surface's pointer can be reused.
    effects: HashMap<(usize, u32), ExtBackgroundEffectSurfaceV1>,
}

impl Display {
    fn new(display: NonNull<c_void>) -> Result<Self, Error> {
        // The application owns the display; this only adds an event queue to it.
        let connection = Connection::from_backend(unsafe {
            Backend::from_foreign_display(display.as_ptr().cast())
        });
        let (globals, mut events) = registry_queue_init::<State>(&connection)
            .map_err(|_| Error::UnsupportedPlatform("Failed to read the Wayland globals."))?;
        let queue = events.handle();
        let compositor = globals
            .bind::<WlCompositor, _, _>(&queue, 1..=6, ())
            .map_err(|_| Error::UnsupportedPlatform("The compositor has no wl_compositor."))?;
        let manager = globals
            .bind::<ExtBackgroundEffectManagerV1, _, _>(&queue, 1..=1, ())
            .ok();
        let mut state = State::default();
        if manager.is_some() {
            // Brings the capabilities, sent when the global is bound.
            events
                .roundtrip(&mut state)
                .map_err(|_| Error::UnsupportedPlatform("The Wayland connection failed."))?;
        }
        Ok(Self {
            connection,
            events,
            state,
            compositor,
            manager,
            effects: HashMap::new(),
        })
    }
}

fn displays() -> &'static Mutex<HashMap<usize, Display>> {
    static DISPLAYS: OnceLock<Mutex<HashMap<usize, Display>>> = OnceLock::new();
    DISPLAYS.get_or_init(Default::default)
}

/// The application's surface as a proxy on `connection`.
fn surface(connection: &Connection, surface: NonNull<c_void>) -> Result<WlSurface, Error> {
    let id = unsafe { ObjectId::from_ptr(WlSurface::interface(), surface.as_ptr().cast()) }
        .map_err(|_| Error::UnsupportedPlatform("The window's surface is not a wl_surface."))?;
    WlSurface::from_id(connection, id)
        .map_err(|_| Error::UnsupportedPlatform("The window's surface is not a wl_surface."))
}

pub fn apply_blur(
    display: NonNull<c_void>,
    surface_ptr: NonNull<c_void>,
    rect: Option<RoundedRect>,
) -> Result<(), Error> {
    let mut displays = displays().lock().unwrap_or_else(|e| e.into_inner());
    let display = match displays.entry(display.as_ptr() as usize) {
        Entry::Occupied(entry) => entry.into_mut(),
        Entry::Vacant(entry) => entry.insert(Display::new(display)?),
    };
    // Capabilities can change at any time; the application's event loop reads them into our queue.
    let _ = display.events.dispatch_pending(&mut display.state);
    let Some(manager) = display.manager.as_ref().filter(|_| display.state.blur) else {
        return Err(Error::UnsupportedPlatform(
            "\"apply_blur()\" needs a compositor with blur through ext-background-effect-v1.",
        ));
    };

    let surface = surface(&display.connection, surface_ptr)?;
    let queue = display.events.handle();
    let effect = display
        .effects
        .entry((surface_ptr.as_ptr() as usize, surface.id().protocol_id()))
        .or_insert_with(|| manager.get_background_effect(&surface, &queue, ()));

    let region = display.compositor.create_region(&queue, ());
    match rect {
        // The compositor clips the region to the surface, so this follows every resize.
        None => region.add(0, 0, i32::MAX, i32::MAX),
        Some(RoundedRect {
            width,
            height,
            radius,
        }) => {
            let (width, height) = (width as i32, height as i32);
            let radius = (radius as i32).min(width / 2).min(height / 2);
            region.add(0, radius, width, height - 2 * radius);
            // One row per pixel along the arcs.
            for y in 0..radius {
                let dy = radius as f64 - (y as f64 + 0.5);
                let inset =
                    radius - (radius as f64 * radius as f64 - dy * dy).sqrt().round() as i32;
                region.add(inset, y, width - 2 * inset, 1);
                region.add(inset, height - 1 - y, width - 2 * inset, 1);
            }
        }
    }
    effect.set_blur_region(Some(&region));
    region.destroy();
    surface.commit();
    display
        .connection
        .flush()
        .map_err(|_| Error::UnsupportedPlatform("The Wayland connection failed."))
}

pub fn clear_blur(display: NonNull<c_void>, surface_ptr: NonNull<c_void>) -> Result<(), Error> {
    let mut displays = displays().lock().unwrap_or_else(|e| e.into_inner());
    let Some(display) = displays.get_mut(&(display.as_ptr() as usize)) else {
        return Ok(());
    };
    let surface = surface(&display.connection, surface_ptr)?;
    let Some(effect) = display
        .effects
        .remove(&(surface_ptr.as_ptr() as usize, surface.id().protocol_id()))
    else {
        return Ok(());
    };
    effect.destroy();
    surface.commit();
    display
        .connection
        .flush()
        .map_err(|_| Error::UnsupportedPlatform("The Wayland connection failed."))
}
