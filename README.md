# tauri-plugin-vibrancy

Make your Tauri/TAO windows vibrant.

## Platform support

- **`Windows:`** Yes!
- **`macOS:`** Yes!
- **`Linux:`** No, blur effect is controlled by the compositor installed on the user system and they can enable it for your app if they want.

## Installation

Add it as a dependncy in `Cargo.toml` of your Tao/Tauri project
```toml
[dependencies]
tauri-plugin-vibrancy = { git = "https://github.com/tauri-apps/tauri-plugin-vibrancy", features = ["tauri-impl"] } # or "tao-impl" for TAO projects.
```
> You also need to use Tauri/TAO from github using the `next` branch (Only until the next release of Tauri).

## Cargo Features:

- `tauri-impl`: for Tauri projects.
- `tao-impl`: for TAO projects.

## Usage

1. Enable transparency on your window:
    - Tauri: Edit your window in `tauri.conf.json > tauri > windows` and add `"transparent": true`
      or use `tauri::WindowBuilder::transparent`.
    - TAO: Use `tao::window::WindowBuilder::with_transparent`.
2. Use the `Vibrancy` trait methods on your window type:
    - Tauri:
        ```rs
        let window = app.get_window("main").unwrap();

        use tauri_plugin_vibrancy::Vibrancy;
        #[cfg(target_os = "windows")]
        window.apply_blur();
        #[cfg(target_os = "macos")]
        {
            use tauri_plugin_vibrancy::MacOSVibrancy;
            window.apply_vibrancy(MacOSVibrancy::AppearanceBased);
        }
        ```
    - Tao:
        ```rs
        let window = WindowBuilder::new().with_transparent(true).build(&event_loop).unwrap();

        use tauri_plugin_vibrancy::Vibrancy;
        #[cfg(target_os = "windows")]
        window.apply_blur();
        #[cfg(target_os = "macos")]
        {
            use tauri_plugin_vibrancy::MacOSVibrancy;
            window.apply_vibrancy(MacOSVibrancy::AppearanceBased);
        }
        ```

## Available methods

> Please read the methods documentation in [src/lib.rs](src/lib.rs)
- `apply_blur()` - **`Windows 7/10/11`**
- `apply_acrylic()` - **`Windows 10/11`** has bad performance when resizing/dragging the window on Windows 10 v1903+ and Windows 11 build 22000
- `apply_mica()` - **Windows 11** thanks to [@sudo-carson](https://github.com/sudo-carson)
- `apply_vibrancy()` - **`macOS`** thanks to [@youngsing](https://github.com/youngsing)

