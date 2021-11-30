use windows::Win32::{
    Foundation::FARPROC,
    System::LibraryLoader::{GetProcAddress, LoadLibraryA},
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
