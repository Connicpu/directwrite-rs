use std::mem;
use winapi::*;
use error::DWriteError;
use helpers::ToWide;
use internal::FromParams;
use comptr::ComPtr;
use text_format::TextFormat;

#[derive(Clone, Debug, PartialEq)]
pub struct TextLayout {
    ptr: ComPtr<IDWriteTextLayout>,
}

impl TextLayout {
    pub unsafe fn get_ptr(&self) -> ComPtr<IDWriteTextLayout> {
        self.ptr.clone()
    }
    
    pub unsafe fn get_raw(&self) -> *mut IDWriteTextLayout {
        self.ptr.raw_value()
    }
    
    pub fn get_measured_size(&self) -> (f32, f32) {
        unsafe {
            let mut metrics: DWRITE_TEXT_METRICS = mem::uninitialized();
            self.ptr().GetMetrics(&mut metrics);
            
            (metrics.width, metrics.height)
        }
    }
    
    unsafe fn ptr(&self) -> &mut IDWriteTextLayout {
        &mut *self.ptr.raw_value()
    }
}

unsafe impl FromParams for TextLayout {
    type Params = Params;
    
    fn from_params(factory: &mut IDWriteFactory, params: Params) -> Result<Self, DWriteError> {
        unsafe {
            let mut ptr: ComPtr<IDWriteTextLayout> = ComPtr::new();
            let result = factory.CreateTextLayout(
                params.text.as_ptr(),
                params.text.len() as u32,
                params.format.get_raw(),
                params.width,
                params.height,
                ptr.raw_addr(),
            );
            
            if SUCCEEDED(result) {
                Ok(TextLayout { ptr: ptr })
            } else {
                Err(From::from(result))
            }
        }
    }
}

pub struct Params {
    text: Vec<u16>,
    format: TextFormat,
    width: f32,
    height: f32,
}

pub struct ParamBuilder<'a> {
    text: Option<&'a str>,
    format: Option<TextFormat>,
    width: Option<f32>,
    height: Option<f32>,
}

impl<'a> ParamBuilder<'a> {
    pub fn new() -> ParamBuilder<'static> {
        ParamBuilder {
            text: None,
            format: None,
            width: None,
            height: None,
        }
    }
    
    pub fn build(self) -> Option<Params> {
        match self {
            ParamBuilder {
                text: Some(text), format: Some(format),
                width: Some(width), height: Some(height),
            } => Some(Params {
                text: text.to_wide_null(),
                format: format,
                width: width,
                height: height,
            }),
            _ => None,
        }
    }
    
    pub fn text(mut self, text: &'a str) -> Self {
        self.text = Some(text);
        self
    }
    
    pub fn font(mut self, font: TextFormat) -> Self {
        self.format = Some(font);
        self
    }
    
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
    
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }
    
    pub fn size(self, width: f32, height: f32) -> Self {
        self.width(width).height(height)
    }
}
