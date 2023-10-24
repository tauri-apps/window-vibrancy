// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

fn main() {
    #[cfg(target_os = "windows")]
    use tao::platform::windows::{WindowBuilderExtWindows, WindowExtWindows};
    use tao::{
        event::{ElementState, Event, MouseButton, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    use window_vibrancy::*;

    let event_loop = EventLoop::new();

    #[allow(unused_mut)]
    let mut builder = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(true);
    #[cfg(target_os = "windows")]
    {
        builder = builder.with_undecorated_shadow(false);
    }
    let window = builder.build(&event_loop).unwrap();

    #[cfg(target_os = "windows")]
    apply_acrylic(&window, None)
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    #[cfg(target_os = "macos")]
    apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    window.set_undecorated_shadow(true);
    window.set_title("A fantastic window!");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    },
                ..
            } => {
                window.drag_window().unwrap();
            }
            _ => (),
        }
    });
}
