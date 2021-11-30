use std::ffi::c_void;
use windows::Win32::Foundation::{BOOL, HWND};

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

pub unsafe fn set_window_composition_attribute(
    hwnd: *mut c_void,
    accent_state: AccentState,
    _color: &str,
) {
    if let Some(set_window_composition_attribute) =
        get_function!("user32.dll", SetWindowCompositionAttribute)
    {
        let mut policy = ACCENT_POLICY {
            AccentState: accent_state.into(),
            AccentFlags: 2,
            GradientColor: 1,
            AnimationId: 0,
        };

        let mut data = WINDOWCOMPOSITIONATTRIBDATA {
            Attrib: 0x13,
            pvData: &mut policy as *mut _ as _,
            cbData: std::mem::size_of_val(&policy),
        };

        set_window_composition_attribute(HWND(hwnd as _), &mut data as *mut _ as _);
    }
}
