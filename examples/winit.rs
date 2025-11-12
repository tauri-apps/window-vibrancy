// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use window_vibrancy::*;
#[cfg(target_os = "windows")]
use winit::platform::windows::{WindowAttributesExtWindows, WindowExtWindows};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attrs = WindowAttributes::default()
            .with_decorations(false)
            .with_transparent(true);
        #[cfg(target_os = "windows")]
        {
            window_attrs = window_attrs
                .with_undecorated_shadow(false)
                .with_inner_size(winit::dpi::PhysicalSize::new(0, 0));
        }
        let window = event_loop.create_window(window_attrs).unwrap();

        #[cfg(target_os = "windows")]
        apply_acrylic(&window, None)
            .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

        #[cfg(target_os = "macos")]
        {
            apply_liquid_glass(&window, NSGlassEffectViewStyle::Clear, None, Some(26.0)).expect(
                "Unsupported platform! 'apply_liquid_glass' is only supported on macOS 26+",
            );
        }

        #[cfg(target_os = "windows")]
        {
            window.set_undecorated_shadow(true);
            let _ = window.request_inner_size(winit::dpi::PhysicalSize::new(800, 600));
        }
        window.set_title("A fantastic window!");

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                self.window.as_ref().unwrap().drag_window().unwrap();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
