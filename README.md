# window-vibrancy

Make your windows vibrant.

## Platform support

- **`Windows:`** Yes!
- **`macOS:`** Yes!
- **`Linux:`** No, blur effect is controlled by the compositor installed on the user system and they can enable it for your app if they want.

## Installation

Add it as a dependncy in `Cargo.toml`
```toml
[dependencies]
window-vibrancy = { git = "https://github.com/tauri-apps/window-vibrancy" }
```

## Available methods

> Please read the methods documentation in [src/lib.rs](src/lib.rs)
- `apply_blur()` - **`Windows 7/10/11`**
- `apply_acrylic()` - **`Windows 10/11`** has bad performance when resizing/dragging the window on Windows 10 v1903+ and Windows 11 build 22000
- `apply_mica()` - **`Windows 11`** thanks to [@sudo-carson](https://github.com/sudo-carson)
- `apply_vibrancy()` - **`macOS`** thanks to [@youngsing](https://github.com/youngsing)

## Examples

- with `winit`:
    ```rs
    use winit::{event_loop::EventLoop, window::WindowBuilder};
    use window_vibrancy::{apply_vibrancy, apply_blur, NSVisualEffectMaterial};

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
    .with_decorations(false)
    .build(&event_loop)
    .unwrap();

    #[cfg(target_os = "macos")]
    apply_vibrancy(&window, NSVisualEffectMaterial::AppearanceBased).unwrap();

    #[cfg(target_os = "windows")]
    apply_blur(&window).unwrap();
    ```

- with `tauri`:
    ```rs
    use window_vibrancy::{apply_vibrancy, apply_blur, NSVisualEffectMaterial};

    let window = app.get_window("main").unwrap();

    #[cfg(target_os = "macos")]
    apply_vibrancy(&window, NSVisualEffectMaterial::AppearanceBased).unwrap();

    #[cfg(target_os = "windows")]
    apply_blur(&window).unwrap();
    ```
