fn main() {
  use window_vibrancy::*;
  use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
  };

  let event_loop = EventLoop::new();

  let window = WindowBuilder::new()
    .with_decorations(false)
    .with_transparent(true)
    .build(&event_loop)
    .unwrap();

  #[cfg(target_os = "windows")]
  apply_blur(&window, Some((18, 18, 18, 125)))
    .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

  #[cfg(target_os = "macos")]
  let _ = apply_vibrancy(&window, NSVisualEffectMaterial::WindowBackground)
    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

  window.set_title("A fantastic window!");

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => (),
    }
  });
}
