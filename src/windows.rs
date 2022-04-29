#![cfg(target_os = "windows")]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::c_void;
pub use windows_sys::Win32::{
  Foundation::*,
  Graphics::{Dwm::*, Gdi::*},
  System::{LibraryLoader::*, SystemInformation::*},
};

use crate::{Color, Error};

pub fn apply_blur(hwnd: HWND, color: Option<Color>) -> Result<(), Error> {
  if is_win7() {
    let bb = DWM_BLURBEHIND {
      dwFlags: DWM_BB_ENABLE,
      fEnable: true.into(),
      hRgnBlur: HRGN::default(),
      fTransitionOnMaximized: 0,
    };
    unsafe {
      let _ = DwmEnableBlurBehindWindow(hwnd, &bb);
    }
  } else if is_win10_swca() || is_win11() {
    unsafe {
      SetWindowCompositionAttribute(hwnd, ACCENT_STATE::ACCENT_ENABLE_BLURBEHIND, color);
    }
  } else {
    return Err(Error::UnsupportedPlatformVersion(
      "\"apply_blur()\" is only available on Windows 7, Windows 10 v1809 or newer and Windows 11.",
    ));
  }
  Ok(())
}

pub fn clear_blur(hwnd: HWND) -> Result<(), Error> {
  if is_win7() {
    let bb = DWM_BLURBEHIND {
      dwFlags: DWM_BB_ENABLE,
      fEnable: false.into(),
      hRgnBlur: HRGN::default(),
      fTransitionOnMaximized: 0,
    };
    unsafe {
      let _ = DwmEnableBlurBehindWindow(hwnd, &bb);
    }
  } else if is_win10_swca() || is_win11() {
    unsafe {
      SetWindowCompositionAttribute(hwnd, ACCENT_STATE::ACCENT_DISABLED, None);
    }
  } else {
    return Err(Error::UnsupportedPlatformVersion(
      "\"clear_blur()\" is only available on Windows 7, Windows 10 v1809 or newer and Windows 11.",
    ));
  }
  Ok(())
}

pub fn apply_acrylic(hwnd: HWND, color: Option<Color>) -> Result<(), Error> {
  if is_win11_dwmsbt() {
    unsafe {
      DwmSetWindowAttribute(
        hwnd,
        DWMWA_USE_IMMERSIVE_DARK_MODE,
        &DWM_SYSTEMBACKDROP_TYPE::DWMSBT_TRANSIENTWINDOW as *const _ as _,
        4,
      );
    }
  } else if is_win10_swca() || is_win11() {
    unsafe {
      SetWindowCompositionAttribute(hwnd, ACCENT_STATE::ACCENT_ENABLE_ACRYLICBLURBEHIND, color);
    }
  } else {
    return Err(Error::UnsupportedPlatformVersion(
      "\"apply_acrylic()\" is only available on Windows 10 v1809 or newer and Windows 11.",
    ));
  }
  Ok(())
}

pub fn clear_acrylic(hwnd: HWND) -> Result<(), Error> {
  if is_win11_dwmsbt() {
    unsafe {
      DwmSetWindowAttribute(
        hwnd,
        DWMWA_USE_IMMERSIVE_DARK_MODE,
        &DWM_SYSTEMBACKDROP_TYPE::DWMSBT_DISABLE as *const _ as _,
        4,
      );
    }
  } else if is_win10_swca() || is_win11() {
    unsafe {
      SetWindowCompositionAttribute(hwnd, ACCENT_STATE::ACCENT_DISABLED, None);
    }
  } else {
    return Err(Error::UnsupportedPlatformVersion(
      "\"clear_acrylic()\" is only available on Windows 10 v1809 or newer and Windows 11.",
    ));
  }
  Ok(())
}

pub fn apply_mica(hwnd: HWND) -> Result<(), Error> {
  if is_win11_dwmsbt() {
    unsafe {
      DwmSetWindowAttribute(
        hwnd,
        DWMWA_SYSTEMBACKDROP_TYPE,
        &DWM_SYSTEMBACKDROP_TYPE::DWMSBT_MAINWINDOW as *const _ as _,
        4,
      );
    }
  } else if is_win11() {
    unsafe {
      DwmSetWindowAttribute(hwnd, DWMWA_MICA_EFFECT, &1 as *const _ as _, 4);
    }
  } else {
    return Err(Error::UnsupportedPlatformVersion(
      "\"apply_mica()\" is only available on Windows 11.",
    ));
  }
  Ok(())
}

pub fn clear_mica(hwnd: HWND) -> Result<(), Error> {
  if is_win11_dwmsbt() {
    unsafe {
      DwmSetWindowAttribute(
        hwnd,
        DWMWA_SYSTEMBACKDROP_TYPE,
        &DWM_SYSTEMBACKDROP_TYPE::DWMSBT_DISABLE as *const _ as _,
        4,
      );
    }
  } else if is_win11() {
    unsafe {
      DwmSetWindowAttribute(hwnd, DWMWA_MICA_EFFECT, &0 as *const _ as _, 4);
    }
  } else {
    return Err(Error::UnsupportedPlatformVersion(
      "\"clear_mica()\" is only available on Windows 11.",
    ));
  }
  Ok(())
}

fn get_function_impl(library: &str, function: &str) -> Option<FARPROC> {
  assert_eq!(library.chars().last(), Some('\0'));
  assert_eq!(function.chars().last(), Some('\0'));

  let module = unsafe { LoadLibraryA(library.as_ptr()) };
  if module == 0 {
    return None;
  }
  Some(unsafe { GetProcAddress(module, function.as_ptr()) })
}

macro_rules! get_function {
  ($lib:expr, $func:ident) => {
    get_function_impl(concat!($lib, '\0'), concat!(stringify!($func), '\0')).map(|f| unsafe {
      std::mem::transmute::<::windows_sys::Win32::Foundation::FARPROC, $func>(f)
    })
  };
}

/// Returns a tuple of (major, minor, buildnumber)
fn get_windows_ver() -> Option<(u32, u32, u32)> {
  type RtlGetVersion = unsafe extern "system" fn(*mut OSVERSIONINFOW) -> i32;
  let handle = get_function!("ntdll.dll", RtlGetVersion);
  if let Some(rtl_get_version) = handle {
    unsafe {
      let mut vi = OSVERSIONINFOW {
        dwOSVersionInfoSize: 0,
        dwMajorVersion: 0,
        dwMinorVersion: 0,
        dwBuildNumber: 0,
        dwPlatformId: 0,
        szCSDVersion: [0; 128],
      };

      let status = (rtl_get_version)(&mut vi as _);

      if status >= 0 {
        Some((vi.dwMajorVersion, vi.dwMinorVersion, vi.dwBuildNumber))
      } else {
        None
      }
    }
  } else {
    None
  }
}

#[repr(C)]
struct ACCENT_POLICY {
  AccentState: u32,
  AccentFlags: u32,
  GradientColor: u32,
  AnimationId: u32,
}

type WINDOWCOMPOSITIONATTRIB = u32;

#[repr(C)]
struct WINDOWCOMPOSITIONATTRIBDATA {
  Attrib: WINDOWCOMPOSITIONATTRIB,
  pvData: *mut c_void,
  cbData: usize,
}

#[derive(PartialEq)]
#[repr(C)]
enum ACCENT_STATE {
  ACCENT_DISABLED = 0,
  ACCENT_ENABLE_BLURBEHIND = 3,
  ACCENT_ENABLE_ACRYLICBLURBEHIND = 4,
}

unsafe fn SetWindowCompositionAttribute(
  hwnd: HWND,
  accent_state: ACCENT_STATE,
  color: Option<Color>,
) {
  type SetWindowCompositionAttribute =
    unsafe extern "system" fn(HWND, *mut WINDOWCOMPOSITIONATTRIBDATA) -> BOOL;

  if let Some(set_window_composition_attribute) =
    get_function!("user32.dll", SetWindowCompositionAttribute)
  {
    let mut color = color.unwrap_or_default();

    let is_acrylic = accent_state == ACCENT_STATE::ACCENT_ENABLE_ACRYLICBLURBEHIND;
    if is_acrylic && color.3 == 0 {
      // SetWindowCompositionAttribute doesn't like acrylic to have 0 alpha
      color.3 = 1;
    }

    let mut policy = ACCENT_POLICY {
      AccentState: accent_state as _,
      AccentFlags: if is_acrylic { 0 } else { 2 },
      GradientColor: (color.0 as u32)
        | (color.1 as u32) << 8
        | (color.2 as u32) << 16
        | (color.3 as u32) << 24,
      AnimationId: 0,
    };

    let mut data = WINDOWCOMPOSITIONATTRIBDATA {
      Attrib: 0x13,
      pvData: &mut policy as *mut _ as _,
      cbData: std::mem::size_of_val(&policy),
    };

    set_window_composition_attribute(hwnd, &mut data as *mut _ as _);
  }
}

const DWMWA_MICA_EFFECT: DWMWINDOWATTRIBUTE = 1029i32;
const DWMWA_SYSTEMBACKDROP_TYPE: DWMWINDOWATTRIBUTE = 38i32;

#[allow(unused)]
#[repr(C)]
enum DWM_SYSTEMBACKDROP_TYPE {
  DWMSBT_DISABLE = 1,         // None
  DWMSBT_MAINWINDOW = 2,      // Mica
  DWMSBT_TRANSIENTWINDOW = 3, // Acrylic
  DWMSBT_TABBEDWINDOW = 4,    // Tabbed
}

fn is_win7() -> bool {
  let v = get_windows_ver().unwrap_or_default();
  v.0 == 6 && v.1 == 1
}

fn is_win10_swca() -> bool {
  let v = get_windows_ver().unwrap_or_default();
  v.2 >= 17763 && v.2 < 22000
}

fn is_win11() -> bool {
  is_at_least_build(22000)
}

fn is_win11_dwmsbt() -> bool {
  is_at_least_build(22523)
}

fn is_at_least_build(build: u32) -> bool {
  let v = get_windows_ver().unwrap_or_default();
  v.2 >= build
}
