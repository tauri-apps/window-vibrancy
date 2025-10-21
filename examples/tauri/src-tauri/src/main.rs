// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use window_vibrancy::*;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            {
                // Use liquid glass effect with Regular variant (automatically falls back to NSVisualEffectView on older macOS)
                let options = LiquidGlassOptions {
                    variant: NSGlassEffectVariant::Clear,
                    radius: Some(26.0),
                    ..Default::default()
                };

                apply_liquid_glass(&window, options)
                    .expect("Unsupported platform! 'apply_liquid_glass' is only supported on macOS");
            }

            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
