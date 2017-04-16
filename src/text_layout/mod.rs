use std::{ptr, mem, u32};
use winapi::*;
use error::DWriteError;
use enums::{FontStretch, FontStyle, FontWeight};
use helpers::InternalConstructor;
use comptr::ComPtr;
use text_format::TextFormat;
use drawing_effect::DrawingEffect;

pub use self::builder::{Params, ParamBuilder};

pub mod builder;
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
        unsafe {
            buf.set_len(count);
        }
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

    /// The application calls this function passing in a specific pixel location relative to the
    /// top-left location of the layout box and obtains the information about the correspondent
    /// hit-test metrics of the text string where the hit-test has occurred. Returns None if the
    /// specified pixel location is outside the string.
    pub fn hit_test_point(&self, point_x: f32, point_y: f32) -> HitTestPoint {
        unsafe {
            let mut trail = 0;
            let mut inside = 0;
            let mut metrics = mem::uninitialized();
            self.ptr().HitTestPoint(point_x, point_y, &mut trail, &mut inside, &mut metrics);

            HitTestPoint {
                metrics: InternalConstructor::build(metrics),
                is_inside: inside != 0,
                is_trailing_hit: trail != 0,
            }
        }
    }

    /// The application calls this function to get the pixel location relative to the top-left of
    /// the layout box given the text position and the logical side of the position. This function
    /// is normally used as part of caret positioning of text where the caret is drawn at the
    /// location corresponding to the current text editing position. It may also be used as a way
    /// to programmatically obtain the geometry of a particular text position in UI automation.
    pub fn hit_test_text_position(&self,
                                  position: u32,
                                  trailing: bool)
                                  -> Option<HitTestTextPosition> {
        let trailing = if trailing { 0 } else { 1 };
        unsafe {
            let (mut x, mut y) = (0.0, 0.0);
            let mut metrics = mem::uninitialized();
            let res = self.ptr()
                .HitTestTextPosition(position, trailing, &mut x, &mut y, &mut metrics);
            if res != S_OK {
                return None;
            }

            Some(HitTestTextPosition {
                metrics: InternalConstructor::build(metrics),
                point_x: x,
                point_y: y,
            })
        }
    }

    /// The application calls this function to get a set of hit-test metrics corresponding to a
    /// range of text positions. One of the main usages is to implement highlight selection of
    /// the text string. origin_x and origin_y are added to the hit-test metrics returned.
    pub fn hit_test_text_range(&self,
                               position: u32,
                               length: u32,
                               origin_x: f32,
                               origin_y: f32,
                               metrics: &mut Vec<metrics::HitTestMetrics>)
                               -> bool {


        unsafe {
            // Calculate the total number of items we need
            let mut actual_count = 0;
            let res = self.ptr().HitTestTextRange(position,
                                                  length,
                                                  origin_x,
                                                  origin_y,
                                                  ptr::null_mut(),
                                                  0,
                                                  &mut actual_count);
            if res != S_OK {
                return false;
            }

            metrics.set_len(actual_count as usize);
            let buf_ptr = metrics[..].as_mut_ptr() as *mut _;
            let len = metrics.len() as u32;
            let res = self.ptr().HitTestTextRange(position,
                                                  length,
                                                  origin_x,
                                                  origin_y,
                                                  buf_ptr,
                                                  len,
                                                  &mut actual_count);
            if res != S_OK {
                metrics.set_len(0);
                return false;
            }

            metrics.set_len(actual_count as usize);
            true
        }
    }

    /// Sets the drawing style for text within a text range.
    pub fn set_drawing_effect<E>(&self, effect: &E, range: TextRange)
        where E: DrawingEffect
    {
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            self.ptr().SetDrawingEffect(effect.get_effect_ptr(), range);
        }
    }

    /// Sets the font style for text within a text range.
    pub fn set_font_style(&self, style: FontStyle, range: TextRange) {
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            self.ptr().SetFontStyle(DWRITE_FONT_STYLE(style as u32), range);
        }
    }

    /// Sets the font weight for text within a text range.
    pub fn set_font_weight(&self, weight: FontWeight, range: TextRange) {
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            self.ptr().SetFontWeight(DWRITE_FONT_WEIGHT(weight as u32), range);
        }
    }

    /// Sets underlining for text within a specified text range.
    pub fn set_underline(&self, underline: bool, range: TextRange) {
        let underline = if underline { 0 } else { 1 };
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            self.ptr().SetUnderline(underline, range);
        }
    }
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub struct HitTestPoint {
    /// The output geometry fully enclosing the hit-test location. When is_inside is set to false,
    /// this structure represents the geometry enclosing the edge closest to the hit-test location.
    pub metrics: metrics::HitTestMetrics,
    /// An output flag that indicates whether the hit-test location is inside the text string. When
    /// false, the position nearest the text's edge is returned.
    pub is_inside: bool,
    /// An output flag that indicates whether the hit-test location is at the leading or the
    /// trailing side of the character. When is_inside is set to false, this value is set according
    /// to the output hitTestMetrics->textPosition value to represent the edge closest to the
    /// hit-test location.
    pub is_trailing_hit: bool,
}

#[derive(Copy, Clone)]
pub struct HitTestTextPosition {
    /// The output pixel location X, relative to the top-left location of the layout box.
    pub point_x: f32,
    /// The output pixel location Y, relative to the top-left location of the layout box.
    pub point_y: f32,

    /// The output geometry fully enclosing the specified text position.
    pub metrics: metrics::HitTestMetrics,
}
