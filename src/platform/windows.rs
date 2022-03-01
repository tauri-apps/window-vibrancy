#![cfg(target_os = "windows")]

use std::ffi::c_void;
pub use windows::Win32::{
  Foundation::{BOOL, ERROR_SUCCESS, FARPROC, HWND, PSTR, PWSTR},
  Graphics::{
    Dwm::{DwmEnableBlurBehindWindow, DwmSetWindowAttribute, DWMWINDOWATTRIBUTE, DWM_BB_ENABLE, DWM_BLURBEHIND},
    Gdi::HRGN,
  },
  System::{
    Registry::{HKEY_CURRENT_USER, RegGetValueW, RRF_RT_REG_DWORD},
    LibraryLoader::{GetProcAddress, LoadLibraryA},
    SystemInformation::OSVERSIONINFOW,
  },
};

const DWMWA_USE_IMMERSIVE_DARK_MODE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(20i32);
const DWMWA_MICA_EFFECT: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(1029i32);
const DWMWA_SYSTEMBACKDROP_TYPE: DWMWINDOWATTRIBUTE = DWMWINDOWATTRIBUTE(38i32);

pub enum SystemBackdropType {
  Auto,
  Disable,
  Mica,
  Acrylic,
  TabbedMica,
}

pub fn apply_acrylic(hwnd: HWND, tint: Option<u32>) {
  if let Some(v) = get_windows_ver() {
    if v.2 < 17763 {
      eprintln!("\"apply_acrylic\" is only available on Windows 10 v1809 or newer");
      return;
    }

    unsafe {
      set_window_composition_attribute(hwnd, AccentState::EnableAcrylicBlurBehind, tint.unwrap_or(16777216));
    }
  }
}
pub fn apply_blur(hwnd: HWND, tint: Option<u32>) {
  if let Some(v) = get_windows_ver() {
    // windows 7 is 6.1
    if v.0 == 6 && v.1 == 1 {
      let bb = DWM_BLURBEHIND {
        dwFlags: DWM_BB_ENABLE,
        fEnable: true.into(),
        hRgnBlur: HRGN::default(),
        ..Default::default()
      };
      unsafe {
        let _ = DwmEnableBlurBehindWindow(hwnd, &bb);
      }
    } else {
      unsafe {
        set_window_composition_attribute(hwnd, AccentState::EnableBlurBehind, tint.unwrap_or(16777216));
      }
    }
  }
}
pub fn apply_mica(hwnd: HWND) {
  if let Some(v) = get_windows_ver() {
    if v.2 >= 22523 {
      unsafe {
        DwmSetWindowAttribute(hwnd, DWMWA_USE_IMMERSIVE_DARK_MODE, &is_dark_theme() as *const _ as _, 4);
        DwmSetWindowAttribute(hwnd, DWMWA_SYSTEMBACKDROP_TYPE, &(SystemBackdropType::Mica as i32) as *const _ as _, 4);
      }
    } else if v.2 >= 22000 {
      unsafe {
        DwmSetWindowAttribute(hwnd, DWMWA_USE_IMMERSIVE_DARK_MODE, &is_dark_theme() as *const _ as _, 4);
        DwmSetWindowAttribute(hwnd, DWMWA_MICA_EFFECT, &1i32 as *const _ as _, 4);
      }
    } else {
      eprintln!("\"apply_mica\" is only available on Windows 11");
    }
  }
}

fn get_function_impl(library: &str, function: &str) -> Option<FARPROC> {
  assert_eq!(library.chars().last(), Some('\0'));
  assert_eq!(function.chars().last(), Some('\0'));

  let module = unsafe { LoadLibraryA(PSTR(library.as_ptr() as _)) };
  if module.0 == 0 {
    return None;
  }
  Some(unsafe { GetProcAddress(module, PSTR(function.as_ptr() as _)) })
}

macro_rules! get_function {
  ($lib:expr, $func:ident) => {
    get_function_impl(concat!($lib, '\0'), concat!(stringify!($func), '\0'))
      .map(|f| unsafe { std::mem::transmute::<windows::Win32::Foundation::FARPROC, $func>(f) })
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

pub fn is_dark_theme() -> bool {
  let mut buffer: [u8; 4] = [0; 4];
  let mut cb_data: u32 = (buffer.len()).try_into().unwrap();
  let res = unsafe {
    RegGetValueW(
      HKEY_CURRENT_USER,
      r#"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"#.to_wide().as_pwstr(),
      "AppsUseLightTheme".to_wide().as_pwstr(),
      RRF_RT_REG_DWORD,
      std::ptr::null_mut(),
      buffer.as_mut_ptr() as _,
      &mut cb_data as *mut _,
    )
  };
  if res == ERROR_SUCCESS {
    i32::from_le_bytes(buffer) == 0
  } else {
    false
  }
}

#[derive(Default)]
pub struct WideString(pub Vec<u16>);

pub trait ToWide {
  fn to_wide(&self) -> WideString;
}

impl ToWide for &str {
  fn to_wide(&self) -> WideString {
    let mut result: Vec<u16> = self.encode_utf16().collect();
    result.push(0);
    WideString(result)
  }
}

impl ToWide for String {
  fn to_wide(&self) -> WideString {
    let mut result: Vec<u16> = self.encode_utf16().collect();
    result.push(0);
    WideString(result)
  }
}

impl WideString {
  pub fn as_pwstr(&self) -> PWSTR {
    PWSTR(self.0.as_ptr() as *mut _)
  }
}

type SetWindowCompositionAttribute =
  unsafe extern "system" fn(HWND, *mut WINDOWCOMPOSITIONATTRIBDATA) -> BOOL;

#[allow(non_snake_case)]
type WINDOWCOMPOSITIONATTRIB = u32;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]
struct ACCENT_POLICY {
  AccentState: u32,
  AccentFlags: u32,
  GradientColor: u32,
  AnimationId: u32,
}

#[allow(non_snake_case)]
#[repr(C)]
struct WINDOWCOMPOSITIONATTRIBDATA {
  Attrib: WINDOWCOMPOSITIONATTRIB,
  pvData: *mut c_void,
  cbData: usize,
}

pub enum AccentState {
  EnableBlurBehind,
  EnableAcrylicBlurBehind,
}

impl From<AccentState> for u32 {
  fn from(state: AccentState) -> Self {
    match state {
      AccentState::EnableBlurBehind => 3,
      AccentState::EnableAcrylicBlurBehind => 4,
    }
  }
}

unsafe fn set_window_composition_attribute(hwnd: HWND, accent_state: AccentState, tint: u32) {
  if let Some(set_window_composition_attribute) =
    get_function!("user32.dll", SetWindowCompositionAttribute)
  {
    let mut policy = ACCENT_POLICY {
      AccentState: accent_state.into(),
      AccentFlags: 2,
      GradientColor: tint,
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
