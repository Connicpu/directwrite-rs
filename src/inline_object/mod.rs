use enums::BreakCondition;
use error::DWResult;

use std::any::Any;
use std::sync::Arc;

use winapi::ctypes::c_void;
use winapi::um::dwrite::{IDWriteInlineObject, IDWriteInlineObjectVtbl, IDWriteTextRenderer};
use winapi::um::unknwnbase::IUnknown;
use wio::com::ComPtr;

mod vtbl;

#[repr(C)]
pub struct InlineObjectContainer {
    com_vtbl: *const IDWriteInlineObjectVtbl,
    obj: Box<InlineObject>,
}

impl InlineObjectContainer {
    pub fn new(obj: impl Into<Box<InlineObject>>) -> Arc<InlineObjectContainer> {
        Arc::new(InlineObjectContainer {
            com_vtbl: &vtbl::INLINE_OBJECT_VTBL,
            obj: obj.into(),
        })
    }

    pub unsafe fn try_from_ptr(
        ptr: *mut IDWriteInlineObject,
    ) -> Option<Arc<InlineObjectContainer>> {
        let ptr = ptr as *const InlineObjectContainer;

        let qi_fn = (*(*ptr).com_vtbl).parent.QueryInterface as *mut c_void;
        if qi_fn == vtbl::query_interface as *mut c_void {
            Some(Arc::from_raw(ptr))
        } else {
            None
        }
    }

    pub unsafe fn into_raw(container: Arc<InlineObjectContainer>) -> *mut IDWriteInlineObject {
        Arc::into_raw(container) as *mut _
    }
}

pub trait InlineObject: Any {
    fn draw(&self, context: &DrawingContext) -> DWResult<()>;
    fn get_metrics(&self) -> DWResult<InlineObjectMetrics>;
    fn get_overhang_metrics(&self) -> DWResult<InlineObjectOverhang>;
    fn get_break_conditions(&self) -> DWResult<(BreakCondition, BreakCondition)>;
}

pub struct DrawingContext {
    pub client_context: *mut c_void,
    pub renderer: ComPtr<IDWriteTextRenderer>,
    pub origin_x: f32,
    pub origin_y: f32,
    pub is_sideways: bool,
    pub is_right_to_left: bool,
    pub client_effect: Option<ComPtr<IUnknown>>,
}

pub struct InlineObjectMetrics {
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub supports_sideways: bool,
}

pub struct InlineObjectOverhang {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

pub unsafe trait IntoInlineObject {
    unsafe fn into_iobj(self) -> *mut IDWriteInlineObject;
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RawInlineObject(*mut IDWriteInlineObject);

impl RawInlineObject {
    #[inline]
    pub unsafe fn new(ptr: *mut IDWriteInlineObject) -> Self {
        RawInlineObject(ptr)
    }

    #[inline]
    pub unsafe fn into_raw(self) -> *mut IDWriteInlineObject {
        self.0
    }
}

unsafe impl IntoInlineObject for RawInlineObject {
    unsafe fn into_iobj(self) -> *mut IDWriteInlineObject {
        self.into_raw()
    }
}

unsafe impl IntoInlineObject for Arc<InlineObjectContainer> {
    unsafe fn into_iobj(self) -> *mut IDWriteInlineObject {
        InlineObjectContainer::into_raw(self)
    }
}
