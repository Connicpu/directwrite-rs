use helpers::{wrap_opt_ref_to_raw_com, wrap_ref_to_raw_mut_com};
use inline_object::custom::CustomInlineObject;
use inline_object::DrawingContext;
use text_renderer::DrawContext;

use com_impl::Refcount;
use com_impl::VTable;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::BOOL;
use winapi::shared::winerror::{E_FAIL, HRESULT, S_OK};
use winapi::um::dwrite::DWRITE_INLINE_OBJECT_METRICS;
use winapi::um::dwrite::DWRITE_OVERHANG_METRICS;
use winapi::um::dwrite::{IDWriteInlineObject, IDWriteInlineObjectVtbl, IDWriteTextRenderer};
use winapi::um::unknwnbase::IUnknown;
use wio::com::ComPtr;

#[repr(C)]
#[derive(ComImpl)]
pub struct ComInlineObject<T: CustomInlineObject> {
    vtable: VTable<IDWriteInlineObjectVtbl>,
    refcount: Refcount,
    object: T,
}

impl<T: CustomInlineObject> ComInlineObject<T> {
    pub fn new(object: T) -> ComPtr<IDWriteInlineObject> {
        let ptr = Self::create_raw(object);
        let ptr = ptr as *mut IDWriteInlineObject;
        unsafe { ComPtr::from_raw(ptr) }
    }
}

#[com_impl]
unsafe impl<T: CustomInlineObject> IDWriteInlineObject for ComInlineObject<T> {
    #[panic(result = "E_FAIL")]
    unsafe fn draw(
        &self,
        context: *mut c_void,
        mut renderer: *mut IDWriteTextRenderer,
        origin_x: f32,
        origin_y: f32,
        is_sideways: BOOL,
        is_rtl: BOOL,
        client_effect: *mut IUnknown,
    ) -> HRESULT {
        let context = DrawingContext {
            client_context: DrawContext::from_ptr(context),
            renderer: wrap_ref_to_raw_mut_com(&mut renderer),
            origin: (origin_x, origin_y).into(),
            is_sideways: is_sideways != 0,
            is_right_to_left: is_rtl != 0,
            client_effect: wrap_opt_ref_to_raw_com(&client_effect),
        };

        match self.object.draw(&context) {
            Ok(_) => S_OK,
            Err(e) => e.0,
        }
    }

    #[panic(result = "E_FAIL")]
    unsafe fn get_metrics(&self, metrics: *mut DWRITE_INLINE_OBJECT_METRICS) -> HRESULT {
        let result = self.object.metrics();
        let metrics = &mut *metrics;
        metrics.width = result.size.width;
        metrics.height = result.size.height;
        metrics.baseline = result.baseline;
        metrics.supportsSideways = result.supports_sideways.into();
        S_OK
    }

    #[panic(result = "E_FAIL")]
    unsafe fn get_overhang_metrics(&self, metrics: *mut DWRITE_OVERHANG_METRICS) -> HRESULT {
        let result = self.object.overhang_metrics();
        let metrics = &mut *metrics;
        metrics.left = result.left;
        metrics.top = result.top;
        metrics.right = result.right;
        metrics.bottom = result.bottom;
        S_OK
    }

    #[panic(result = "E_FAIL")]
    unsafe fn get_break_conditions(&self, preceding: *mut u32, following: *mut u32) -> HRESULT {
        let result = self.object.break_conditions();
        *preceding = result.preceding.value;
        *following = result.following.value;
        S_OK
    }
}
