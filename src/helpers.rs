use std::{ptr, slice};

use winapi::ctypes::wchar_t;
use winapi::shared::minwindef::*;
use winapi::um::winbase::*;
use winapi::um::winnls::GetUserDefaultLocaleName;
use winapi::um::winnt::*;

pub trait InternalConstructor {
    type Arguments;
    fn build(args: Self::Arguments) -> Self;
}

pub fn get_system_locale() -> Vec<wchar_t> {
    unsafe {
        let mut locale: Vec<wchar_t> = vec![0; 85];
        GetUserDefaultLocaleName(locale.as_mut_ptr(), locale.len() as i32 - 1);
        locale
    }
}

pub fn hresult_to_string(hr: HRESULT) -> Option<String> {
    unsafe {
        let mut buffer: *mut u8 = ptr::null_mut();
        let num_chars = FormatMessageA(
            FORMAT_MESSAGE_ALLOCATE_BUFFER
                | FORMAT_MESSAGE_FROM_SYSTEM
                | FORMAT_MESSAGE_IGNORE_INSERTS,
            ptr::null_mut(),
            hr as DWORD,
            0, // unknown lang-id, use default
            (&mut buffer) as *mut *mut u8 as *mut i8,
            0, // minimum buffer size
            ptr::null_mut(),
        );
        if num_chars == 0 {
            return None;
        }

        let bytes = slice::from_raw_parts(buffer, num_chars as usize);
        let message = String::from_utf8_lossy(bytes).into_owned();
        LocalFree(buffer as *mut _);

        Some(message)
    }
}
