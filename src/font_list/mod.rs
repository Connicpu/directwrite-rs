use winapi::um::dwrite::IDWriteFontList;
use wio::com::ComPtr;

#[derive(ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct FontList {
    ptr: ComPtr<IDWriteFontList>,
}
