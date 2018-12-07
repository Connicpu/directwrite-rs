use crate::font_file::FontFile;

use std::ptr;

use com_impl::Refcount;
use com_impl::VTable;
use com_wrapper::ComWrapper;
use crate::error::DWResult;
use winapi::shared::minwindef::BOOL;
use winapi::shared::winerror::{E_FAIL, HRESULT, S_OK};
use winapi::um::dwrite::IDWriteFontFile;
use winapi::um::dwrite::{IDWriteFontFileEnumerator, IDWriteFontFileEnumeratorVtbl};
use wio::com::ComPtr;

#[repr(C)]
#[derive(ComImpl)]
pub struct ComEnumerator<I>
where
    I: Iterator<Item = DWResult<FontFile>> + 'static,
{
    vtbl: VTable<IDWriteFontFileEnumeratorVtbl>,
    refcount: Refcount,
    curr: Option<FontFile>,
    err: Option<i32>,
    iter: I,
}

impl<I> ComEnumerator<I>
where
    I: Iterator<Item = DWResult<FontFile>> + 'static,
{
    pub fn new(iter: I) -> ComPtr<IDWriteFontFileEnumerator> {
        let ptr = ComEnumerator::create_raw(None, None, iter);
        let ptr = ptr as *mut IDWriteFontFileEnumerator;
        unsafe { ComPtr::from_raw(ptr) }
    }
}

#[com_impl]
unsafe impl<I> IDWriteFontFileEnumerator for ComEnumerator<I>
where
    I: Iterator<Item = DWResult<FontFile>> + 'static,
{
    unsafe fn get_current_font_file(&self, file: *mut *mut IDWriteFontFile) -> HRESULT {
        if let Some(err) = self.err {
            return err;
        }

        let ptr = self
            .curr
            .clone()
            .map(|f| f.into_raw())
            .unwrap_or(ptr::null_mut());
        *file = ptr;

        match ptr.is_null() {
            false => S_OK,
            true => E_FAIL,
        }
    }

    unsafe fn move_next(&mut self, has_next: *mut BOOL) -> HRESULT {
        if let Some(err) = self.err {
            return err;
        }

        let item = self.iter.next();
        let item = match item {
            Some(item) => item,
            None => {
                *has_next = 0;
                return S_OK;
            }
        };

        let item = match item {
            Ok(item) => item,
            Err(e) => {
                self.err = Some(e.0);
                *has_next = 0;
                return e.0;
            }
        };

        self.curr = Some(item);
        *has_next = 1;
        S_OK
    }
}
