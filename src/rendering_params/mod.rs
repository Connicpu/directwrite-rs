use winapi::um::dwrite::IDWriteRenderingParams;
use wio::com::ComPtr;

#[derive(Clone, ComWrapper)]
#[com(send, sync, debug)]
pub struct RenderingParams {
    ptr: ComPtr<IDWriteRenderingParams>,
}
