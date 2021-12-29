#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri_plugin_vibrancy::Vibrancy;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      #[cfg(target_os = "windows")]
      window.apply_blur();
      #[cfg(target_os = "macos")]
      {
        use tauri_plugin_vibrancy::MacOSVibrancy;
        window.apply_vibrancy(MacOSVibrancy::AppearanceBased);
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
