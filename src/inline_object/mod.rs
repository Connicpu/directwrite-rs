use com_wrapper::ComWrapper;
use winapi::um::dwrite::IDWriteInlineObject;
use wio::com::ComPtr;

use inline_object::custom::CustomInlineObject;

pub mod custom;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync)]
pub struct InlineObject {
    ptr: ComPtr<IDWriteInlineObject>,
}

impl InlineObject {
    pub fn create_custom(object: impl CustomInlineObject) -> InlineObject {
        let ptr = custom::com_obj::ComInlineObject::new(object);
        unsafe { InlineObject::from_ptr(ptr) }
    }
}
