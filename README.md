# tauri-plugin-vibrancy
Add vibrancy/blur/acrylic to your tauri window.

## Note:
This plugin is an experiment to gather enough feedback that will help me decide how and whether this will be directly included in Tao/Tauri directly or kept as a plugin.

## Installation
Add it as a dependency in `src-tauri/Cargo.toml`.
```toml
[dependencies]
tauri-plugin-vibrancy = { git = "https://github.com/amrbashir/tauri-plugin-vibrancy" }
# only needed unti the next release of tauri
tauri = { git = "https://github.com/tauri-apps/tauri", branch = "next" }
```

## Usage
1. enable transparency on your window, either through `tauri.conf.json` or programmatically. It is also recommended to disable decorations.
2. import the vibrancy trait
    ```rs
    use tauri_plugin_vibrancy::Vibrnacy;
    ```
3. use the trait [methods](src/lib.rs:) on the `tauri::Window` type.
    ```rs
    tauri::Builder::default()
        .setup(|app|{
            let window = app.get_window("main").unwrap();
            window.set_blur("#1f1f1f");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    ```

## Available methods:
1. `set_acrylic()` - works only on Windows 10 v1809 and above.
2. `set_blur()`. - Need someone to confirm if it works in Windows 10 versions below v1809.

## TODOS:
- [ ] actually parse the color instead of using a dummy value internally.
- [ ] add `set_blur` fallback on Windows 10 versions before 1803.
- [ ] add `set_mica` for Windows 11.
- [ ] add `set_vibrancy` for macOS.

## License
[MIT](./LICENSE) License © 2021 [Amr Bashir](https://github.com/amrbashir)
