use winapi::*;
use std::{ptr, mem, ffi};
use kernel32;
use helpers::*;
use comptr::ComPtr;

type CreateFactory = unsafe extern "system" fn(
    DWRITE_FACTORY_TYPE, REFIID, *mut *mut IUnknown,
) -> HRESULT;

#[derive(Debug, PartialEq)]
pub struct DWrite {
    handle: HMODULE,
}

impl DWrite {
    pub fn load() -> Result<DWrite, HRESULT> {
        unsafe {
            let dll = ffi::CString::new("DWrite.dll").unwrap();
            let mut handle = kernel32::GetModuleHandleA(dll.as_ptr());
            if handle == ptr::null_mut() {
                handle = kernel32::LoadLibraryA(dll.as_ptr());
            }
            
            if handle != ptr::null_mut() {
                Ok(DWrite { handle: handle })
            } else {
                Err(last_error_hr())
            }
        }
    }
    
    pub fn create_factory(&self, isolated: bool) -> Result<ComPtr<IDWriteFactory>, HRESULT> {
        unsafe {
            let procedure = ffi::CString::new("DWriteCreateFactory").unwrap();
            let create_factory_ptr = kernel32::GetProcAddress(self.handle, procedure.as_ptr());
            
            if create_factory_ptr == ptr::null_mut() {
                panic!("Error loading function DWriteCreateFactory: {:?}", last_error_string());
            }
            
            let create_factory: CreateFactory = mem::transmute(create_factory_ptr);
            let mut ptr = ComPtr::<IDWriteFactory>::new();
            let result = create_factory(
                if isolated { DWRITE_FACTORY_TYPE_ISOLATED } else { DWRITE_FACTORY_TYPE_SHARED },
                &ptr.iid(),
                ptr.raw_addr() as *mut *mut _,
            );
            
            if SUCCEEDED(result) {
                Ok(ptr)
            } else {
                Err(result)
            }
        }
    }
}
