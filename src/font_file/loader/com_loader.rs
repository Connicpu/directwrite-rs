use descriptions::KeyPayload;
use font_file::loader::com_stream::ComFontFileStream;
use font_file::loader::FontFileLoader;

use std::mem;

use com_impl::Refcount;
use com_impl::VTable;
use winapi::ctypes::c_void;
use winapi::shared::winerror::HRESULT;
use winapi::shared::winerror::S_OK;
use winapi::shared::winerror::{E_FAIL, E_INVALIDARG};
use winapi::um::dwrite::IDWriteFontFileStream;
use winapi::um::dwrite::{IDWriteFontFileLoader, IDWriteFontFileLoaderVtbl};
use wio::com::ComPtr;

#[repr(C)]
#[derive(ComImpl)]
pub struct ComFontFileLoader<T>
where
    T: FontFileLoader,
{
    vtable: VTable<IDWriteFontFileLoaderVtbl>,
    refcount: Refcount,
    loader: T,
}

impl<T> ComFontFileLoader<T>
where
    T: FontFileLoader,
{
    pub fn new(loader: T) -> ComPtr<IDWriteFontFileLoader> {
        let ptr = Self::create_raw(loader);
        let ptr = ptr as *mut IDWriteFontFileLoader;
        unsafe { ComPtr::from_raw(ptr) }
    }
}

#[com_impl]
unsafe impl<T> IDWriteFontFileLoader for ComFontFileLoader<T>
where
    T: FontFileLoader,
{
    #[panic(result = "E_FAIL")]
    unsafe fn create_stream_from_key(
        &self,
        key: *const c_void,
        key_size: u32,
        out_stream: *mut *mut IDWriteFontFileStream,
    ) -> HRESULT {
        if key_size as usize != mem::size_of::<KeyPayload<T::Key>>() {
            return E_INVALIDARG;
        }

        let key = &*(key as *const KeyPayload<T::Key>);
        if !key.valid() {
            return E_INVALIDARG;
        }

        let stream = match self.loader.create_stream(&key.data) {
            Ok(stream) => stream,
            Err(e) => return e.0,
        };

        let com_stream = ComFontFileStream::new(stream);
        *out_stream = com_stream.into_raw();

        S_OK
    }
}
