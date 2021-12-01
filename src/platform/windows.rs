use std::ffi::c_void;
use windows::Win32::{
    Foundation::{BOOL, FARPROC, HWND},
    Graphics::{
        Dwm::{DwmEnableBlurBehindWindow, DWM_BB_ENABLE, DWM_BLURBEHIND},
        Gdi::HRGN,
    },
    System::{
        LibraryLoader::{GetProcAddress, LoadLibraryA},
        SystemInformation::OSVERSIONINFOW,
    },
};

pub unsafe fn set_acrylic(hwnd: HWND) {
    if let Some(v) = get_windows_ver() {
        if v.2 >= 17763 {
            set_window_composition_attribute(hwnd, AccentState::EnableAcrylicBlurBehind);
        }
    }
}
pub unsafe fn set_blur(hwnd: HWND) {
    if let Some(v) = get_windows_ver() {
        // windows 7 is 6.1
        if v.0 == 6 && v.1 == 1 {
            let bb = DWM_BLURBEHIND {
                dwFlags: DWM_BB_ENABLE,
                fEnable: true.into(),
                hRgnBlur: HRGN::default(),
                ..Default::default()
            };

            let _ = DwmEnableBlurBehindWindow(hwnd, &bb);
            return;
        }
    }

    set_window_composition_attribute(hwnd, AccentState::EnableBlurBehind);
}

fn get_function_impl(library: &str, function: &str) -> Option<FARPROC> {
    assert_eq!(library.chars().last(), Some('\0'));
    assert_eq!(function.chars().last(), Some('\0'));

    let module = unsafe { LoadLibraryA(library) };
    if module.0 == 0 {
        return None;
    }
    unsafe { GetProcAddress(module, function) }
}

macro_rules! get_function {
    ($lib:expr, $func:ident) => {
        get_function_impl(concat!($lib, '\0'), concat!(stringify!($func), '\0')).map(|f| unsafe {
            std::mem::transmute::<windows::Win32::Foundation::FARPROC, $func>(f)
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

unsafe fn set_window_composition_attribute(hwnd: HWND, accent_state: AccentState) {
    if let Some(set_window_composition_attribute) =
        get_function!("user32.dll", SetWindowCompositionAttribute)
    {
        let mut policy = ACCENT_POLICY {
            AccentState: accent_state.into(),
            AccentFlags: 2,
            GradientColor: 0,
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
