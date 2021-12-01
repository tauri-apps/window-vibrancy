use windows::Win32::{
    Foundation::FARPROC,
    System::{
        LibraryLoader::{GetProcAddress, LoadLibraryA},
        SystemInformation::OSVERSIONINFOW,
    },
};

pub fn get_function_impl(library: &str, function: &str) -> Option<FARPROC> {
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
        crate::utils::get_function_impl(concat!($lib, '\0'), concat!(stringify!($func), '\0')).map(
            |f| unsafe { std::mem::transmute::<windows::Win32::Foundation::FARPROC, $func>(f) },
        )
    };
}

pub fn get_windows10_build_ver() -> Option<u32> {
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

            if status >= 0 && vi.dwMajorVersion == 10 && vi.dwMinorVersion == 0 {
                Some(vi.dwBuildNumber)
            } else {
                None
            }
        }
    } else {
        None
    }
}
