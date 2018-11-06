use winapi::um::d2d1::ID2D1SimplifiedGeometrySink;

pub trait ComGeometrySink {
    unsafe fn as_ptr(&self) -> *mut ID2D1SimplifiedGeometrySink;
}
