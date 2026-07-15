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
            let window_ = window.clone();

            #[cfg(target_os = "macos")]
            window.with_webview(move |webview| {
                use objc2_web_kit::WKWebView;
                let webview: &WKWebView = unsafe {&*webview.inner().cast()};

                let mut options = LiquidGlassOptions::new(NSGlassEffectViewStyle::Sidebar)
                    .radius(26.0)
                    .opaque(true)
                    .content_view(webview);

                apply_liquid_glass(&window_, options)
                    .expect("Unsupported platform! 'apply_liquid_glass' is only supported on macOS 26+");
            });

            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
