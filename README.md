# tauri-plugin-vibrancy
Add vibrancy/blur/acrylic to your tauri window.

## Installation
Add it as a dependency in `src-tauri/Cargo.toml`.
```toml
[dependencies]
tauri-plugin-vibrancy = { git = "https://github.com/amrbashir/tauri-plugin-vibrancy" }
# only needed unti the next release of tauri
tauri = { git = "https://github.com/tauri-apps/tauri", branch = "next" }
```

## Usage
1. import the vibrancy trait
    ```rs
    use tauri_plugin_vibrancy::Vibrnacy;
    ```
2. use the trait methods on the `tauri::Window` type.
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

## TODOS:
1. add `set_blur` fallback on Windows 10 versions before 1803.
2. add `set_mica` for Windows 11.
3. add `set_vibrancy` for macOS.

## License

[MIT](./LICENSE) License © 2021 [Amr Bashir](https://github.com/amrbashir)
