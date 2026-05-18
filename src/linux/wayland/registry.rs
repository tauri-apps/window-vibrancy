// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{
    collections::HashMap,
    ffi::{c_char, c_void, CStr},
    ptr,
    sync::{Mutex, OnceLock},
};

use wayland_client::{
    backend::protocol::{wl_argument, wl_interface},
    protocol::{wl_compositor, wl_display as wl_display_proto, wl_registry},
    Proxy,
};
use wayland_protocols::ext::background_effect::v1::client::ext_background_effect_manager_v1::{
    self, ExtBackgroundEffectManagerV1,
};
use wayland_sys::client::{self, wl_display, wl_event_queue, wl_proxy};

use crate::Error;

use super::ffi;

pub(super) const FULL_SURFACE_REGION_SIZE: i32 = i32::MAX / 2;

const BLUR_CAPABILITY: u32 = 1;
const WL_COMPOSITOR_NAME: &[u8] = b"wl_compositor";
const EXT_BACKGROUND_EFFECT_MANAGER_NAME: &[u8] = b"ext_background_effect_manager_v1";

pub(super) struct DisplayState {
    display: usize,
    queue: usize,
    registry: usize,
    data: Box<RegistryData>,
    compositor: Option<BoundProxy>,
    manager: Option<BoundProxy>,
}

impl DisplayState {
    pub(super) unsafe fn new(display: *mut wl_display) -> Result<Self, Error> {
        let queue = unsafe {
            wayland_sys::ffi_dispatch!(
                client::wayland_client_handle(),
                wl_display_create_queue,
                display
            )
        };
        if queue.is_null() {
            return Err(ffi::query_failed());
        }

        let display_wrapper = unsafe {
            wayland_sys::ffi_dispatch!(
                client::wayland_client_handle(),
                wl_proxy_create_wrapper,
                display.cast::<wl_proxy>()
            )
        };
        if display_wrapper.is_null() {
            unsafe {
                wayland_sys::ffi_dispatch!(
                    client::wayland_client_handle(),
                    wl_event_queue_destroy,
                    queue
                );
            }
            return Err(ffi::query_failed());
        }

        unsafe {
            wayland_sys::ffi_dispatch!(
                client::wayland_client_handle(),
                wl_proxy_set_queue,
                display_wrapper,
                queue
            );
        }

        let registry = unsafe {
            wayland_sys::ffi_dispatch!(
                client::wayland_client_handle(),
                wl_proxy_marshal_array_constructor,
                display_wrapper,
                wl_display_proto::REQ_GET_REGISTRY_OPCODE.into(),
                ptr::null_mut(),
                ffi::c_interface(wl_registry::WlRegistry::interface())
            )
        };

        unsafe {
            wayland_sys::ffi_dispatch!(
                client::wayland_client_handle(),
                wl_proxy_wrapper_destroy,
                display_wrapper
            );
        }

        if registry.is_null() {
            unsafe {
                wayland_sys::ffi_dispatch!(
                    client::wayland_client_handle(),
                    wl_event_queue_destroy,
                    queue
                );
            }
            return Err(ffi::query_failed());
        }

        let data = Box::<RegistryData>::default();
        unsafe {
            wayland_sys::ffi_dispatch!(
                client::wayland_client_handle(),
                wl_proxy_set_queue,
                registry,
                queue
            );
            let add_listener_result = wayland_sys::ffi_dispatch!(
                client::wayland_client_handle(),
                wl_proxy_add_listener,
                registry,
                &REGISTRY_LISTENER as *const RegistryListener as *mut extern "C" fn(),
                data.as_ref() as *const RegistryData as *mut c_void
            );
            if add_listener_result != 0 {
                destroy_proxy(registry);
                destroy_queue(queue);
                return Err(ffi::query_failed());
            }
        }

        let state = Self {
            display: display as usize,
            queue: queue as usize,
            registry: registry as usize,
            data,
            compositor: None,
            manager: None,
        };

        unsafe {
            state.roundtrip()?;
        }

        Ok(state)
    }

    pub(super) unsafe fn ensure_compositor(&mut self) -> Result<*mut wl_proxy, Error> {
        unsafe {
            self.roundtrip()?;
        }

        let global = self.globals().compositor.ok_or_else(ffi::unsupported)?;
        if let Some(compositor) = self.compositor {
            if compositor.name == global.name {
                return Ok(compositor.proxy());
            }
            unsafe {
                ffi::release_compositor(compositor.proxy(), compositor.version);
            }
        }

        let version = global
            .version
            .min(wl_compositor::WlCompositor::interface().version);
        let compositor = unsafe {
            bind_global(
                self.registry(),
                global.name,
                version,
                ffi::c_interface(wl_compositor::WlCompositor::interface()),
            )?
        };
        unsafe {
            self.set_queue(compositor);
        }

        self.compositor = Some(BoundProxy {
            name: global.name,
            version,
            ptr: compositor as usize,
        });

        Ok(compositor)
    }

    pub(super) unsafe fn ensure_manager(&mut self) -> Result<*mut wl_proxy, Error> {
        unsafe {
            self.roundtrip()?;
        }

        let global = self
            .globals()
            .background_effect_manager
            .ok_or_else(ffi::unsupported)?;

        if let Some(manager) = self.manager {
            if manager.name == global.name {
                return Ok(manager.proxy());
            }
            unsafe {
                ffi::send_destructor(
                    manager.proxy(),
                    ext_background_effect_manager_v1::REQ_DESTROY_OPCODE,
                );
            }
        }

        let version = global
            .version
            .min(ExtBackgroundEffectManagerV1::interface().version);
        let manager = unsafe {
            bind_global(
                self.registry(),
                global.name,
                version,
                ffi::c_interface(ExtBackgroundEffectManagerV1::interface()),
            )?
        };
        unsafe {
            self.set_queue(manager);
        }

        *ffi::lock(&self.data.capabilities) = None;

        let add_listener_result = unsafe {
            wayland_sys::ffi_dispatch!(
                client::wayland_client_handle(),
                wl_proxy_add_listener,
                manager,
                &BACKGROUND_EFFECT_MANAGER_LISTENER as *const BackgroundEffectManagerListener
                    as *mut extern "C" fn(),
                self.data.as_ref() as *const RegistryData as *mut c_void
            )
        };
        if add_listener_result != 0 {
            unsafe {
                ffi::send_destructor(
                    manager,
                    ext_background_effect_manager_v1::REQ_DESTROY_OPCODE,
                );
            }
            return Err(ffi::query_failed());
        }

        self.manager = Some(BoundProxy {
            name: global.name,
            version,
            ptr: manager as usize,
        });

        unsafe {
            self.roundtrip()?;
        }

        Ok(manager)
    }

    pub(super) fn supports_blur(&self) -> bool {
        let capabilities = *ffi::lock(&self.data.capabilities);
        capabilities.is_some_and(|capabilities| capabilities & BLUR_CAPABILITY == BLUR_CAPABILITY)
    }

    fn globals(&self) -> Globals {
        *ffi::lock(&self.data.globals)
    }

    fn registry(&self) -> *mut wl_proxy {
        self.registry as *mut wl_proxy
    }

    unsafe fn set_queue(&self, proxy: *mut wl_proxy) {
        unsafe {
            wayland_sys::ffi_dispatch!(
                client::wayland_client_handle(),
                wl_proxy_set_queue,
                proxy,
                self.queue as *mut wl_event_queue
            );
        }
    }

    unsafe fn roundtrip(&self) -> Result<(), Error> {
        unsafe {
            ffi::roundtrip_queue(
                self.display as *mut wl_display,
                self.queue as *mut wl_event_queue,
            )
        }
    }
}

impl Drop for DisplayState {
    fn drop(&mut self) {
        unsafe {
            if let Some(manager) = self.manager.take() {
                ffi::send_destructor(
                    manager.proxy(),
                    ext_background_effect_manager_v1::REQ_DESTROY_OPCODE,
                );
            }

            if let Some(compositor) = self.compositor.take() {
                ffi::release_compositor(compositor.proxy(), compositor.version);
            }

            destroy_proxy(self.registry());
            destroy_queue(self.queue as *mut wl_event_queue);
        }
    }
}

#[derive(Clone, Copy)]
struct BoundProxy {
    name: u32,
    version: u32,
    ptr: usize,
}

impl BoundProxy {
    fn proxy(self) -> *mut wl_proxy {
        self.ptr as *mut wl_proxy
    }
}

#[derive(Default)]
struct RegistryData {
    globals: Mutex<Globals>,
    capabilities: Mutex<Option<u32>>,
}

#[derive(Default, Clone, Copy)]
struct Globals {
    compositor: Option<Global>,
    background_effect_manager: Option<Global>,
}

#[derive(Clone, Copy)]
struct Global {
    name: u32,
    version: u32,
}

#[repr(C)]
struct RegistryListener {
    global: unsafe extern "C" fn(*mut c_void, *mut wl_proxy, u32, *const c_char, u32),
    global_remove: unsafe extern "C" fn(*mut c_void, *mut wl_proxy, u32),
}

static REGISTRY_LISTENER: RegistryListener = RegistryListener {
    global: registry_global,
    global_remove: registry_global_remove,
};

#[repr(C)]
struct BackgroundEffectManagerListener {
    capabilities: unsafe extern "C" fn(*mut c_void, *mut wl_proxy, u32),
}

static BACKGROUND_EFFECT_MANAGER_LISTENER: BackgroundEffectManagerListener =
    BackgroundEffectManagerListener {
        capabilities: background_effect_manager_capabilities,
    };

unsafe extern "C" fn registry_global(
    data: *mut c_void,
    _registry: *mut wl_proxy,
    name: u32,
    interface: *const c_char,
    version: u32,
) {
    if data.is_null() || interface.is_null() {
        return;
    }

    let data = unsafe { &*(data as *const RegistryData) };
    let interface = unsafe { CStr::from_ptr(interface) }.to_bytes();

    let mut globals = ffi::lock(&data.globals);
    if interface == WL_COMPOSITOR_NAME {
        globals.compositor = Some(Global { name, version });
    } else if interface == EXT_BACKGROUND_EFFECT_MANAGER_NAME {
        globals.background_effect_manager = Some(Global { name, version });
    }
}

unsafe extern "C" fn registry_global_remove(
    data: *mut c_void,
    _registry: *mut wl_proxy,
    name: u32,
) {
    if data.is_null() {
        return;
    }

    let data = unsafe { &*(data as *const RegistryData) };
    let mut globals = ffi::lock(&data.globals);
    if globals.compositor.map(|global| global.name) == Some(name) {
        globals.compositor = None;
    }
    if globals.background_effect_manager.map(|global| global.name) == Some(name) {
        globals.background_effect_manager = None;
        *ffi::lock(&data.capabilities) = None;
    }
}

unsafe extern "C" fn background_effect_manager_capabilities(
    data: *mut c_void,
    _manager: *mut wl_proxy,
    flags: u32,
) {
    if data.is_null() {
        return;
    }

    let data = unsafe { &*(data as *const RegistryData) };
    *ffi::lock(&data.capabilities) = Some(flags);
}

unsafe fn bind_global(
    registry: *mut wl_proxy,
    name: u32,
    version: u32,
    interface: *const wl_interface,
) -> Result<*mut wl_proxy, Error> {
    let mut args = unsafe {
        [
            wl_argument { u: name },
            wl_argument {
                s: (*interface).name,
            },
            wl_argument { u: version },
        ]
    };

    let proxy = unsafe {
        wayland_sys::ffi_dispatch!(
            client::wayland_client_handle(),
            wl_proxy_marshal_array_constructor_versioned,
            registry,
            wl_registry::REQ_BIND_OPCODE.into(),
            args.as_mut_ptr(),
            interface,
            version
        )
    };

    if proxy.is_null() {
        return Err(ffi::query_failed());
    }

    Ok(proxy)
}

pub(super) fn displays() -> &'static Mutex<HashMap<usize, DisplayState>> {
    static DISPLAYS: OnceLock<Mutex<HashMap<usize, DisplayState>>> = OnceLock::new();
    DISPLAYS.get_or_init(|| Mutex::new(HashMap::new()))
}

unsafe fn destroy_proxy(proxy: *mut wl_proxy) {
    unsafe {
        wayland_sys::ffi_dispatch!(client::wayland_client_handle(), wl_proxy_destroy, proxy);
    }
}

unsafe fn destroy_queue(queue: *mut wl_event_queue) {
    unsafe {
        wayland_sys::ffi_dispatch!(
            client::wayland_client_handle(),
            wl_event_queue_destroy,
            queue
        );
    }
}
