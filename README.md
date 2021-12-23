# tauri-plugin-vibrancy
 Make your Tao/Tauri windows vibrant.

## Note:

 This plugin is an experiment to gather enough feedback that will help me
 decide how and whether this will be included in Tao/Tauri directly or kept as a plugin.

## Usage:

1. Enable transparency on your window
    - Tauri: Edit you window in `tauri.conf.json > tauri > windows` and add `"transparent": true`
    or use `tauri::WindowBuilder::transparent`
    - Tao: use `tao::window::WindowBuilder::with_transparent`
2. Import the vibrancy trait
    ```rs
    use tauri_plugin_vibrancy::Vibrancy;
    ```
3. Use the `Vibrancy` trait methods on your window
    - Tauri:
        ```rs
        let window = app.get_window("main").unwrap();
        window.apply_blur();
        ```
    - Tao:
        ```rs
        let window = WindowBuilder::new().with_transparent(true).build().unwrap();
        window.apply_blur();
        ```
## Methods:
> Please read the methods documentation, it has valuable info
- `apply_blur()` - **`Windows Only`**
- `apply_acrylic()` - **`Windows Only`**: works only on Windows 10 v1809 and above, it also has bad performance when resizing/dragging the window
- `apply_vibrancy()` - **`macOS Only`**, thanks to @youngsing 

## TODOS:
- [ ] `apply_mica()` for Windows 11

## License
[MIT](./LICENSE) License © 2021 [Amr Bashir](https://github.com/amrbashir)
