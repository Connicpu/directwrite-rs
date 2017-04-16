use std::{ptr, mem, u32};
use winapi::*;
use error::DWriteError;
use enums::{FontStretch, FontStyle, FontWeight};
use helpers::{InternalConstructor, ToWide};
use internal::FromParams;
use comptr::ComPtr;
use text_format::TextFormat;

pub mod metrics;

/// The TextLayout interface represents a block of text after it has been fully analyzed and formatted.
#[derive(Clone, Debug, PartialEq)]
pub struct TextLayout {
    ptr: ComPtr<IDWriteTextLayout>,
}

impl TextLayout {
    pub unsafe fn from_ptr(ptr: ComPtr<IDWriteTextLayout>) -> Self {
        TextLayout { ptr: ptr }
    }

    pub unsafe fn get_ptr(&self) -> ComPtr<IDWriteTextLayout> {
        self.ptr.clone()
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteTextLayout {
        self.ptr.raw_value()
    }

    unsafe fn ptr(&self) -> &mut IDWriteTextLayout {
        &mut *self.ptr.raw_value()
    }

    pub fn as_format(&self) -> TextFormat {
        unsafe { TextFormat::from_ptr(self.ptr.query_interface().unwrap()) }
    }

    /// Determines the minimum possible width the layout can be set to without emergency breaking
    /// between the characters of whole words occurring.
    pub fn determine_min_width(&self) -> f32 {
        unsafe {
            let mut value = 0.0;
            self.ptr().DetermineMinWidth(&mut value);
            value
        }
    }

    // TODO: Perhaps look into a way that Draw can be implemented? I could create a Trait

    /// Gets the number of ClusterMetrics objects which exist for this TextLayout
    pub fn get_cluster_metrics_count(&self) -> usize {
        unsafe {
            let mut count = 0;
            self.ptr().GetClusterMetrics(ptr::null_mut(), 0, &mut count);
            count as usize
        }
    }

    /// Retrieves the ClusterMetrics for the glyph clusters in this layout. You should ensure the
    /// slice is large enough to hold all of the metrics, which can be obtained by calling
    /// `get_cluster_metrics_count`. If the slice is not large enough, it will return
    /// Err(actual_count), otherwise returns Ok(actual_count).
    pub fn get_cluster_metrics_slice(&self,
                                     buf: &mut [metrics::ClusterMetrics])
                                     -> Result<usize, usize> {
        assert!(buf.len() <= u32::MAX as usize);
        unsafe {
            let mut actual_count = 0;
            let buf_ptr = buf.as_mut_ptr() as *mut DWRITE_CLUSTER_METRICS;
            let res = self.ptr().GetClusterMetrics(buf_ptr, buf.len() as u32, &mut actual_count);

            if res == S_OK {
                Ok(actual_count as usize)
            } else {
                Err(actual_count as usize)
            }
        }
    }

    /// Fill all of the Cluster metrics into a Vec. This function will resize the Vec to fit all
    /// of the metrics structures exactly.
    pub fn get_cluster_metrics(&self, buf: &mut Vec<metrics::ClusterMetrics>) {
        let count = self.get_cluster_metrics_count();
        buf.resize(count, Default::default());
        assert_eq!(self.get_cluster_metrics_slice(buf), Ok(count));
    }

    /// Gets the font em height of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn get_font_size(&self, position: u32) -> Result<(f32, TextRange), DWriteError> {
        unsafe {
            let mut font_size = 0.0;
            let mut range = mem::uninitialized();
            let res = self.ptr().GetFontSize(position, &mut font_size, &mut range);
            if res < 0 {
                return Err(res.into());
            }
            Ok((font_size, range.into()))
        }
    }

    /// Gets the font stretch of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn get_font_stretch(&self, position: u32) -> Result<(FontStretch, TextRange), DWriteError> {
        unsafe {
            let (mut stretch, mut range) = mem::uninitialized();
            let res = self.ptr().GetFontStretch(position, &mut stretch, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((FontStretch::from_u32(stretch.0).unwrap(), range.into()))
        }
    }

    /// Gets the font style of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn get_font_style(&self, position: u32) -> Result<(FontStyle, TextRange), DWriteError> {
        unsafe {
            let (mut style, mut range) = mem::uninitialized();
            let res = self.ptr().GetFontStyle(position, &mut style, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((FontStyle::from_u32(style.0).unwrap(), range.into()))
        }
    }

    /// Gets the font weight of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn get_font_weight(&self, position: u32) -> Result<(FontWeight, TextRange), DWriteError> {
        unsafe {
            let (mut weight, mut range) = mem::uninitialized();
            let res = self.ptr().GetFontWeight(position, &mut weight, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((FontWeight::from_u32(weight.0).unwrap(), range.into()))
        }
    }

    // TODO: Inline objects somehow?

    /// Get the number of LineMetrics objects that you need room for when calling
    /// `get_line_metrics_slice`
    pub fn get_line_metrics_count(&self) -> usize {
        unsafe {
            let mut count = 0;
            self.ptr().GetLineMetrics(ptr::null_mut(), 0, &mut count);
            count as usize
        }
    }

    /// Retrieves the information about each individual text line of the text string. You should
    /// first call `get_line_metrics_count` to know how large your slice must be to fit all of
    /// the metrics objects. The return value will contain the actual number of elements in the
    /// layout, but the official documentation does *not* specify whether the array will be filled
    /// with any values in the Err case, so that behavior is not guaranteed between windows
    /// versions.
    pub fn get_line_metrics_slice(&self, buf: &mut [metrics::LineMetrics]) -> Result<usize, usize> {
        assert!(buf.len() <= u32::MAX as usize);
        unsafe {
            let mut actual_count = 0;
            let buf_ptr = buf.as_mut_ptr() as *mut DWRITE_LINE_METRICS;
            let res = self.ptr().GetLineMetrics(buf_ptr, buf.len() as u32, &mut actual_count);

            if res == S_OK {
                Ok(actual_count as usize)
            } else {
                Err(actual_count as usize)
            }
        }
    }

    /// etrieves the information about each individual text line of the text string. Resizes `buf`
    /// to fit all of the elements exactly.
    pub fn get_line_metrics(&self, buf: &mut Vec<metrics::LineMetrics>) {
        let count = self.get_line_metrics_count();
        buf.resize(count, Default::default());
        assert_eq!(self.get_line_metrics_slice(buf), Ok(count));
    }

    /// Gets the layout maximum height.
    pub fn get_max_height(&self) -> f32 {
        unsafe { self.ptr().GetMaxHeight() }
    }

    /// Gets the layout maximum width.
    pub fn get_max_width(&self) -> f32 {
        unsafe { self.ptr().GetMaxWidth() }
    }

    /// Retrieves overall metrics for the formatted string.
    pub fn get_metrics(&self) -> metrics::Metrics {
        unsafe {
            let mut metrics = mem::zeroed();
            self.ptr().GetMetrics(&mut metrics);

            metrics::Metrics::build(metrics)
        }
    }

    /// Returns the overhangs (in DIPs) of the layout and all objects contained in it, including
    /// text glyphs and inline objects.
    pub fn get_overhang_metrics(&self) -> metrics::OverhangMetrics {
        unsafe {
            let mut metrics = mem::zeroed();
            self.ptr().GetOverhangMetrics(&mut metrics);

            metrics::OverhangMetrics::build(metrics)
        }
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

#[derive(Copy, Clone, Debug)]
pub struct TextRange {
    pub start: u32,
    pub length: u32,
}

impl From<DWRITE_TEXT_RANGE> for TextRange {
    fn from(range: DWRITE_TEXT_RANGE) -> Self {
        TextRange {
            start: range.startPosition,
            length: range.length,
        }
    }
}

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
