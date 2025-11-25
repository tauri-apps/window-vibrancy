// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(target_os = "macos")]
mod webview;

use tauri::Manager;
use window_vibrancy::*;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            {
                use webview::{find_webview_recursive, get_nsview_from_window};
                use objc2_app_kit::NSView;

                let nsview_ptr = get_nsview_from_window(&window);
                let webview = if !nsview_ptr.is_null() {
                    unsafe {
                        let nsview = &*(nsview_ptr as *const NSView);
                        find_webview_recursive(nsview)
                    }
                } else {
                    None
                };

                let mut options = LiquidGlassOptions::new(NSGlassEffectViewStyle::Clear)
                    .radius(26.0)
                    .opaque(false);

                if let Some(webview) = webview {
                    options = options.content_view(webview.cast());
                }

                apply_liquid_glass(&window, options)
                    .expect("Unsupported platform! 'apply_liquid_glass' is only supported on macOS 26+");
            }

            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
