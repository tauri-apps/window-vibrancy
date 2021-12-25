# tauri-plugin-vibrancy

Make your Tauri/TAO windows vibrant.

## Platform Note

Only Windows and macOS are supported, 
Linux blur effect is controlled by the compositor installed on the user system and they can enable it for your app if they want.

## Installation

Add it as a dependncy in `Cargo.toml` of your Tao/Tauri project
```toml
[target."cfg(any(target_os = \"windows\", target_os = \"macos\"))".dependencies]
tauri-plugin-vibrancy = { git = "https://github.com/amrbashir/tauri-plugin-vibrancy", features = ["tauri-impl"] }
```
You also need to use Tauri/TAO from github using the `next` branch (Only until the next release of Tauri).

## Crate Features:

- `tauri-impl`: for Tauri projects.
- `tao-impl`: for TAO projects.

## Usage

1. Enable transparency on your window
    - Tauri: Edit your window in `tauri.conf.json > tauri > windows` and add `"transparent": true`
      or use `tauri::WindowBuilder::transparent`
    - TAO: Use `tao::window::WindowBuilder::with_transparent`
2. Use the `Vibrancy` trait methods on your window type
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
        let window = WindowBuilder::new().with_transparent(true).build().unwrap();

        use tauri_plugin_vibrancy::Vibrancy;
        #[cfg(target_os = "windows")]
        window.apply_blur();
        #[cfg(target_os = "macos")]
        {
            use tauri_plugin_vibrancy::MacOSVibrancy;
            window.apply_vibrancy(MacOSVibrancy::AppearanceBased);
        }
        ```

## Methods

> Please read the methods documentation, it has valuable info
- `apply_blur()` - **`Windows Only`**
- `apply_acrylic()` - **`Windows Only`** works only on Windows 10 v1809 and above, it also has bad performance when resizing/dragging the window
- `apply_vibrancy()` - **`macOS Only`** thanks to [@youngsing](https://github.com/youngsing)

## TODOS

- [ ] `apply_mica()` for Windows 11

## License

[MIT](./LICENSE) License © 2021 [Amr Bashir](https://github.com/amrbashir)
