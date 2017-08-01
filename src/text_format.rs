use std::ptr;
use enums::*;
use error::DWriteError;
use helpers::ToWide;
use internal::FromParams;
use comptr::ComPtr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TextFormat {
    ptr: ComPtr<IDWriteTextFormat>,
}

impl TextFormat {
    pub unsafe fn from_ptr(ptr: ComPtr<IDWriteTextFormat>) -> Self {
        TextFormat { ptr: ptr }
    }

    pub unsafe fn get_ptr(&self) -> ComPtr<IDWriteTextFormat> {
        self.ptr.clone()
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteTextFormat {
        self.ptr.raw_value()
    }
}

unsafe impl FromParams for TextFormat {
    type Params = Params;

    fn from_params(factory: &mut IDWriteFactory, params: Params) -> Result<Self, DWriteError> {
        unsafe {
            let mut ptr: ComPtr<IDWriteTextFormat> = ComPtr::new();
            let result = factory.CreateTextFormat(params.family.as_ptr(),
                                                  ptr::null_mut(),
                                                  params.weight as u32,
                                                  params.style as u32,
                                                  params.stretch as u32,
                                                  params.size,
                                                  params.locale.as_ptr(),
                                                  ptr.raw_addr());

            if SUCCEEDED(result) {
                Ok(TextFormat { ptr: ptr })
            } else {
                Err(From::from(result))
            }
        }
    }
}

pub struct Params {
    family: Vec<u16>,
    weight: FontWeight,
    style: FontStyle,
    stretch: FontStretch,
    size: f32,
    locale: Vec<u16>,
}

pub struct ParamBuilder<'a> {
    family: Option<&'a str>,
    weight: FontWeight,
    style: FontStyle,
    stretch: FontStretch,
    size: Option<f32>,
    locale: Option<&'a str>,
}

impl<'a> ParamBuilder<'a> {
    pub fn new() -> ParamBuilder<'static> {
        ParamBuilder {
            family: None,
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
            stretch: FontStretch::Normal,
            size: None,
            locale: None,
        }
    }

    pub fn build(self) -> Option<Params> {
        if self.size == None || self.family == None {
            return None;
        }

        Some(Params {
            family: self.family.unwrap().to_wide_null(),
            weight: self.weight,
            style: self.style,
            stretch: self.stretch,
            size: self.size.unwrap(),
            locale: self.locale.unwrap_or("en-US").to_wide_null(),
        })
    }

    pub fn family(mut self, family: &'a str) -> Self {
        self.family = Some(family);
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn style(mut self, style: FontStyle) -> Self {
        self.style = style;
        self
    }

    pub fn stretch(mut self, stretch: FontStretch) -> Self {
        self.stretch = stretch;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn locale(mut self, locale: &'a str) -> Self {
        self.locale = Some(locale);
        self
    }
}
