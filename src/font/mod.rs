use winapi::um::dwrite::IDWriteFont;
use wio::com::ComPtr;

#[derive(ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Font {
    ptr: ComPtr<IDWriteFont>,
}
