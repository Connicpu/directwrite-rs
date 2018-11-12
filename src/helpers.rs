use std::ops::Deref;
use std::{ptr, slice};

use com_wrapper::ComWrapper;
use winapi::shared::minwindef::*;
use winapi::um::winbase::*;
use winapi::um::winnt::*;

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

pub unsafe fn wrap_ref_to_raw_com<T>(ptr: &*mut T::Interface) -> &T
where
    T: ComWrapper,
{
    assert_eq!(
        std::mem::size_of::<T>(),
        std::mem::size_of::<*mut T::Interface>()
    );
    assert!(!ptr.is_null());
    std::mem::transmute::<&*mut _, &T>(ptr)
}

pub unsafe fn wrap_opt_ref_to_raw_com<T>(ptr: &*mut T::Interface) -> Option<&T>
where
    T: ComWrapper,
{
    assert_eq!(
        std::mem::size_of::<T>(),
        std::mem::size_of::<*mut T::Interface>(),
    );
    if ptr.is_null() {
        None
    } else {
        Some(std::mem::transmute::<&*mut _, &T>(ptr))
    }
}

pub unsafe fn deref_com_wrapper<T, U>(wrapper: &T) -> &U
where
    T: ComWrapper,
    U: ComWrapper,
    T::Interface: Deref<Target = U::Interface>,
{
    assert_eq!(
        std::mem::size_of::<T>(),
        std::mem::size_of::<*mut T::Interface>(),
    );
    assert_eq!(
        std::mem::size_of::<U>(),
        std::mem::size_of::<*mut U::Interface>(),
    );

    std::mem::transmute::<&T, &U>(wrapper)
}

pub unsafe fn deref_com_wrapper_mut<T, U>(wrapper: &mut T) -> &mut U
where
    T: ComWrapper,
    U: ComWrapper,
    T::Interface: Deref<Target = U::Interface>,
{
    assert_eq!(std::mem::size_of::<U>(), std::mem::size_of::<T>());
    assert_eq!(
        std::mem::size_of::<T>(),
        std::mem::size_of::<*mut T::Interface>(),
    );
    assert_eq!(
        std::mem::size_of::<U>(),
        std::mem::size_of::<*mut U::Interface>(),
    );

    std::mem::transmute::<&mut T, &mut U>(wrapper)
}
