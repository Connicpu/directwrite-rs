use {TextFormat, TextLayout};
use comptr::ComPtr;
use internal::FromParams;
use error::DWriteError;
use helpers::ToWide;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::*;

pub struct Params {
    text: Vec<u16>,
    format: TextFormat,
    width: f32,
    height: f32,
    centered: bool,
}

pub struct ParamBuilder<'a> {
    text: Option<&'a str>,
    format: Option<TextFormat>,
    width: Option<f32>,
    height: Option<f32>,
    centered: bool,
}

impl<'a> ParamBuilder<'a> {
    pub fn new() -> ParamBuilder<'static> {
        ParamBuilder {
            text: None,
            format: None,
            width: None,
            height: None,
            centered: false,
        }
    }

    pub fn build(self) -> Option<Params> {
        match self {
            ParamBuilder { text: Some(text),
                           format: Some(format),
                           width: Some(width),
                           height: Some(height),
                           centered } => {
                Some(Params {
                    text: text.to_wide_null(),
                    format: format,
                    width: width,
                    height: height,
                    centered: centered,
                })
            }
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

    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }
}

unsafe impl FromParams for TextLayout {
    type Params = Params;

    fn from_params(factory: &mut IDWriteFactory, params: Params) -> Result<Self, DWriteError> {
        unsafe {
            let mut ptr: ComPtr<IDWriteTextLayout> = ComPtr::new();
            let result = factory.CreateTextLayout(params.text.as_ptr(),
                                                  params.text.len() as u32,
                                                  params.format.get_raw(),
                                                  params.width,
                                                  params.height,
                                                  ptr.raw_addr());

            if SUCCEEDED(result) {
                if params.centered {
                    ptr.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER);
                }

                Ok(TextLayout { ptr: ptr })
            } else {
                Err(From::from(result))
            }
        }
    }
}
