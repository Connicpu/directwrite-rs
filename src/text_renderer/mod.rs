use winapi::um::dwrite::IDWriteTextRenderer;
use wio::com::ComPtr;

pub mod custom;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send)]
pub struct TextRenderer {
    ptr: ComPtr<IDWriteTextRenderer>,
}

impl TextRenderer {
    pub fn new_custom(renderer: impl custom::CustomTextRenderer) -> TextRenderer {
        custom::com_renderer::ComRenderer::new(renderer)
    }
}
