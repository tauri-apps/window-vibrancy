#[macro_use]
mod utils;

use tauri::Window;
use utils::{get_windows_ver, set_window_composition_attribute, AccentState};
use windows::Win32::{
    Foundation::HWND,
    Graphics::{
        Dwm::{DwmEnableBlurBehindWindow, DWM_BB_ENABLE, DWM_BLURBEHIND},
        Gdi::HRGN,
    },
};

pub trait Vibrancy {
    /// Adds Acrylic effect to you tauri window.
    ///
    /// ## WARNING:
    ///
    /// This method has poor performance on Windows 10 v1903 and above,
    /// the window will lag when resizing or dragging the window.
    /// It is an issue in the undocumented api used for this method
    /// and microsoft needs to fix it (they probably won't).
    ///
    /// ## Platform-specific
    ///
    /// - **Windows**: has no effect on Windows 10 versions below v1809.
    /// - **Linux / macOS:** Unsupported
    fn set_acrylic(&self);

    /// Adds blur effect to tauri window.
    ///
    /// ## Platform-specific
    ///
    /// - **Linux / macOS:** Unsupported
    fn set_blur(&self);
}

impl Vibrancy for Window {
    fn set_acrylic(&self) {
        unsafe {
            if let Some(v) = get_windows_ver() {
                if v.2 >= 17763 {
                    set_window_composition_attribute(
                        HWND(self.hwnd().unwrap() as _),
                        AccentState::EnableAcrylicBlurBehind,
                    );
                }
            }
        }
    }

    fn set_blur(&self) {
        unsafe {
            if let Some(v) = get_windows_ver() {
                // windows 7 is 6.1
                if v.0 == 6 && v.1 == 1 {
                    let bb = DWM_BLURBEHIND {
                        dwFlags: DWM_BB_ENABLE,
                        fEnable: true.into(),
                        hRgnBlur: HRGN::default(),
                        ..Default::default()
                    };

                    let _ = DwmEnableBlurBehindWindow(HWND(self.hwnd().unwrap() as _), &bb);
                    return;
                }
            }

            set_window_composition_attribute(
                HWND(self.hwnd().unwrap() as _),
                AccentState::EnableBlurBehind,
            );
        }
    }
}
