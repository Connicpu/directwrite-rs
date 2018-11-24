use font_file::loader::FontFileStream;

use com_impl::Refcount;
use com_impl::VTable;
use winapi::ctypes::c_void;
use winapi::shared::winerror::E_FAIL;
use winapi::shared::winerror::{HRESULT, S_OK};
use winapi::um::dwrite::{IDWriteFontFileStream, IDWriteFontFileStreamVtbl};
use wio::com::ComPtr;

#[repr(C)]
#[derive(ComImpl)]
pub struct ComFontFileStream<T>
where
    T: FontFileStream,
{
    vtable: VTable<IDWriteFontFileStreamVtbl>,
    refcount: Refcount,
    stream: T,
}

impl<T> ComFontFileStream<T>
where
    T: FontFileStream,
{
    pub fn new(stream: T) -> ComPtr<IDWriteFontFileStream> {
        let ptr = Self::create_raw(stream);
        let ptr = ptr as *mut IDWriteFontFileStream;
        unsafe { ComPtr::from_raw(ptr) }
    }
}

#[com_impl]
unsafe impl<T> IDWriteFontFileStream for ComFontFileStream<T>
where
    T: FontFileStream,
{
    #[panic(result = "E_FAIL")]
    unsafe fn get_file_size(&self, size: *mut u64) -> HRESULT {
        *size = self.stream.file_size();
        S_OK
    }

    #[panic(result = "E_FAIL")]
    unsafe fn get_last_write_time(&self, time: *mut u64) -> HRESULT {
        *time = self.stream.last_write_time();
        S_OK
    }

    #[panic(result = "E_FAIL")]
    unsafe fn read_file_fragment(
        &self,
        start: *mut *const c_void,
        offset: u64,
        length: u64,
        ctx: *mut *mut c_void,
    ) -> HRESULT {
        let fragment = match self.stream.read_fragment(offset, length) {
            Ok(frag) => frag,
            Err(e) => return e.0,
        };

        *start = fragment.data as *const c_void;
        *ctx = fragment.key as *mut c_void;

        S_OK
    }

    #[panic(abort)]
    unsafe fn release_file_fragment(&self, context: *mut c_void) {
        let key = context as usize;
        self.stream.release_fragment(key);
    }
}
