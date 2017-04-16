use winapi::IUnknown;

/// This trait represents types which can be used in set_drawing_effect on a text layout.
pub unsafe trait DrawingEffect {
    unsafe fn get_effect_ptr(&self) -> *mut IUnknown;
}
