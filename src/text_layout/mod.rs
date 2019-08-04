//! TextLayout and types for building new ones.

use crate::descriptions::TextRange;
use crate::effects::client_effect::ClientEffect;
use crate::effects::DrawingEffect;
use crate::enums::{FontStretch, FontStyle, FontWeight};
use crate::factory::Factory;
use crate::font_collection::FontCollection;
use crate::inline_object::InlineObject;
use crate::metrics::cluster::ClusterMetrics;
use crate::metrics::hit_test::HitTestMetrics;
use crate::metrics::line::LineMetrics;
use crate::metrics::overhang::OverhangMetrics;
use crate::metrics::text::TextMetrics;
use crate::text_format::TextFormat;
use crate::text_renderer::DrawContext;
use crate::text_renderer::TextRenderer;
use crate::typography::Typography;

use std::ops::{Deref, DerefMut};
use std::{mem, ptr, u32};

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use dcommon::helpers::{deref_com_wrapper, deref_com_wrapper_mut};
use dcommon::Error;
use winapi::shared::winerror::{SUCCEEDED, S_OK};
use winapi::um::dwrite::*;
use wio::com::ComPtr;
use wio::wide::ToWide;

const E_NOT_SUFFICIENT_BUFFER: i32 = -2147024774;

#[doc(inline)]
pub use self::builder::TextLayoutBuilder;

#[doc(hidden)]
pub mod builder;

#[derive(Copy, Clone, Debug)]
/// Represents a value that has an associated range for which the text has the
/// same formatting applied to it.
pub struct RangeValue<T> {
    /// The range of text that has the same formatting as the text at the position specified by
    /// position.
    pub range: TextRange,

    /// The value that was found at the requested position.
    pub value: T,
}

impl<T> From<(T, TextRange)> for RangeValue<T> {
    fn from((value, range): (T, TextRange)) -> Self {
        RangeValue { value, range }
    }
}

impl<T> Into<(T, TextRange)> for RangeValue<T> {
    fn into(self) -> (T, TextRange) {
        (self.value, self.range)
    }
}

impl<T> std::ops::Deref for RangeValue<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

/// A function result that is either a pair of T and an associated text range, or a DWriteError.
pub type RangeResult<T> = Result<RangeValue<T>, Error>;

#[derive(ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// The TextLayout interface represents a block of text after it has been fully
/// analyzed and formatted.
pub struct TextLayout {
    ptr: ComPtr<IDWriteTextLayout>,
}

impl Deref for TextLayout {
    type Target = TextFormat;
    fn deref(&self) -> &TextFormat {
        unsafe { deref_com_wrapper(self) }
    }
}

impl DerefMut for TextLayout {
    fn deref_mut(&mut self) -> &mut TextFormat {
        unsafe { deref_com_wrapper_mut(self) }
    }
}

impl TextLayout {
    /// Initialize a builder for a new TextLayout.
    pub fn create<'a>(factory: &'a Factory) -> TextLayoutBuilder<'a> {
        unsafe { TextLayoutBuilder::new(&*factory.get_raw()) }
    }

    /// Determines the minimum possible width the layout can be set to without emergency breaking
    /// between the characters of whole words occurring.
    pub fn determine_min_width(&self) -> f32 {
        unsafe {
            let mut value = 0.0;
            self.ptr.DetermineMinWidth(&mut value);
            value
        }
    }

    /// Draws text using the specified client drawing context.
    pub fn draw(
        &self,
        renderer: &mut TextRenderer,
        origin_x: f32,
        origin_y: f32,
        context: &DrawContext,
    ) -> Result<(), Error> {
        unsafe {
            let hr = self
                .ptr
                .Draw(context.ptr(), renderer.get_raw(), origin_x, origin_y);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets the number of ClusterMetrics objects which exist for this TextLayout
    pub fn cluster_metrics_count(&self) -> usize {
        unsafe {
            let mut count = 0;
            self.ptr.GetClusterMetrics(ptr::null_mut(), 0, &mut count);
            count as usize
        }
    }

    /// Retrieves the ClusterMetrics for the glyph clusters in this layout. You should ensure the
    /// slice is large enough to hold all of the metrics, which can be obtained by calling
    /// `get_cluster_metrics_count`. If the slice is not large enough, it will return
    /// Err(actual_count), otherwise returns Ok(actual_count).
    pub fn cluster_metrics_slice(&self, buf: &mut [ClusterMetrics]) -> Result<usize, usize> {
        assert!(buf.len() <= u32::MAX as usize);
        unsafe {
            let mut actual_count = 0;
            let buf_ptr = buf.as_mut_ptr() as *mut DWRITE_CLUSTER_METRICS;
            let res = self
                .ptr
                .GetClusterMetrics(buf_ptr, buf.len() as u32, &mut actual_count);

            if res == S_OK {
                Ok(actual_count as usize)
            } else {
                Err(actual_count as usize)
            }
        }
    }

    /// Fill all of the Cluster metrics into a Vec.
    pub fn cluster_metrics(&self) -> Vec<ClusterMetrics> {
        let count = self.cluster_metrics_count();
        let mut buf = Vec::with_capacity(count);
        unsafe { buf.set_len(count) };
        assert_eq!(self.cluster_metrics_slice(&mut buf), Ok(count));
        buf
    }

    /// Get the drawing effect applied at the specified position
    pub fn drawing_effect(&self, position: u32) -> RangeResult<Option<ClientEffect>> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let mut range = std::mem::zeroed();
            let hr = self.ptr.GetDrawingEffect(position, &mut ptr, &mut range);
            if SUCCEEDED(hr) {
                let effect = if ptr.is_null() {
                    None
                } else {
                    Some(ClientEffect::from_raw(ptr))
                };
                Ok((effect, range.into()).into())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets the font collection of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn font_collection(&self, position: u32) -> RangeResult<FontCollection> {
        unsafe {
            let mut collection = ptr::null_mut();
            let mut range = mem::zeroed();
            let res = self
                .ptr
                .GetFontCollection(position, &mut collection, &mut range);
            if res < 0 {
                return Err(res.into());
            }
            Ok((FontCollection::from_raw(collection), range.into()).into())
        }
    }

    /// Get the font family name applied at the specified text position.
    pub fn font_family_name(&self, position: u32) -> RangeResult<String> {
        unsafe {
            let mut len = 0;
            let mut range = mem::zeroed();
            let hr = self
                .ptr
                .GetFontFamilyNameLength(position, &mut len, &mut range);
            if !SUCCEEDED(hr) {
                return Err(hr.into());
            }

            let mut buf = vec![0u16; len as usize + 1];
            let hr = self.ptr.GetFontFamilyName(
                position,
                buf.as_mut_ptr(),
                buf.len() as u32,
                &mut range,
            );
            if !SUCCEEDED(hr) {
                return Err(hr.into());
            }

            Ok((String::from_utf16_lossy(&buf), range.into()).into())
        }
    }

    /// Gets the font em height of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn font_size(&self, position: u32) -> RangeResult<f32> {
        unsafe {
            let mut font_size = 0.0;
            let mut range = mem::uninitialized();
            let res = self.ptr.GetFontSize(position, &mut font_size, &mut range);
            if res < 0 {
                return Err(res.into());
            }
            Ok((font_size, range.into()).into())
        }
    }

    /// Gets the font stretch of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn font_stretch(&self, position: u32) -> RangeResult<UncheckedEnum<FontStretch>> {
        unsafe {
            let (mut stretch, mut range) = mem::uninitialized();
            let res = self.ptr.GetFontStretch(position, &mut stretch, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((stretch.into(), range.into()).into())
        }
    }

    /// Gets the font style of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn font_style(&self, position: u32) -> RangeResult<UncheckedEnum<FontStyle>> {
        unsafe {
            let (mut style, mut range) = mem::uninitialized();
            let res = self.ptr.GetFontStyle(position, &mut style, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((style.into(), range.into()).into())
        }
    }

    /// Gets the font weight of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn font_weight(&self, position: u32) -> RangeResult<FontWeight> {
        unsafe {
            let (mut weight, mut range) = mem::uninitialized();
            let res = self.ptr.GetFontWeight(position, &mut weight, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((FontWeight(weight), range.into()).into())
        }
    }

    /// Gets the inline object at the position as-is. May return ptr::null_mut()
    pub fn inline_object(&self, position: u32) -> RangeResult<Option<InlineObject>> {
        unsafe {
            let mut range = mem::zeroed();
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetInlineObject(position, &mut ptr, &mut range);
            if SUCCEEDED(hr) {
                let obj = if !ptr.is_null() {
                    Some(InlineObject::from_raw(ptr))
                } else {
                    None
                };
                Ok((obj, range.into()).into())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Get the number of LineMetrics objects that you need room for when calling
    /// `get_line_metrics_slice`
    pub fn line_metrics_count(&self) -> usize {
        unsafe {
            let mut count = 0;
            self.ptr.GetLineMetrics(ptr::null_mut(), 0, &mut count);
            count as usize
        }
    }

    /// Retrieves the information about each individual text line of the text string. You should
    /// first call `get_line_metrics_count` to know how large your slice must be to fit all of
    /// the metrics objects. The return value will contain the actual number of elements in the
    /// layout, but the official documentation does *not* specify whether the array will be filled
    /// with any values in the Err case, so that behavior is not guaranteed between windows
    /// versions.
    pub fn line_metrics_slice(&self, buf: &mut [LineMetrics]) -> Result<usize, usize> {
        assert!(buf.len() <= u32::MAX as usize);
        unsafe {
            let mut actual_count = 0;
            let buf_ptr = buf.as_mut_ptr() as *mut DWRITE_LINE_METRICS;
            let res = self
                .ptr
                .GetLineMetrics(buf_ptr, buf.len() as u32, &mut actual_count);

            if res == S_OK {
                Ok(actual_count as usize)
            } else {
                Err(actual_count as usize)
            }
        }
    }

    /// Retrieves the information about each individual text line of the text string.
    pub fn line_metrics(&self) -> Vec<LineMetrics> {
        let count = self.line_metrics_count();
        let mut buf = Vec::with_capacity(count);
        unsafe { buf.set_len(count) };
        assert_eq!(self.line_metrics_slice(&mut buf), Ok(count));
        buf
    }

    /// Gets the locale name applied to the text at the specified text position.
    pub fn locale_name(&self, position: u32) -> RangeResult<String> {
        unsafe {
            let mut len = 0;
            let mut range = mem::zeroed();
            let hr = self.ptr.GetLocaleNameLength(position, &mut len, &mut range);
            if !SUCCEEDED(hr) {
                return Err(hr.into());
            }

            let mut buf = vec![0u16; len as usize + 1];
            let hr =
                self.ptr
                    .GetLocaleName(position, buf.as_mut_ptr(), buf.len() as u32, &mut range);
            if !SUCCEEDED(hr) {
                return Err(hr.into());
            }

            Ok((String::from_utf16_lossy(&buf), range.into()).into())
        }
    }

    /// Gets the layout maximum height.
    pub fn max_height(&self) -> f32 {
        unsafe { self.ptr.GetMaxHeight() }
    }

    /// Gets the layout maximum width.
    pub fn max_width(&self) -> f32 {
        unsafe { self.ptr.GetMaxWidth() }
    }

    /// Retrieves overall metrics for the formatted string.
    pub fn metrics(&self) -> TextMetrics {
        unsafe {
            let mut metrics = mem::zeroed();
            self.ptr.GetMetrics(&mut metrics);
            metrics.into()
        }
    }

    /// Returns the overhangs (in DIPs) of the layout and all objects contained in it, including
    /// text glyphs and inline objects.
    pub fn overhang_metrics(&self) -> OverhangMetrics {
        unsafe {
            let mut metrics = mem::zeroed();
            self.ptr.GetOverhangMetrics(&mut metrics);
            metrics.into()
        }
    }

    /// Returns whether the text at the specified position has strikethrough applied.
    pub fn strikethrough(&self, position: u32) -> RangeResult<bool> {
        unsafe {
            let (mut strikethrough, mut range) = mem::zeroed();
            let res = self
                .ptr
                .GetStrikethrough(position, &mut strikethrough, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((strikethrough != 0, range.into()).into())
        }
    }

    /// Returns whether the text at the specified position has underline applied.
    pub fn underline(&self, position: u32) -> RangeResult<bool> {
        unsafe {
            let (mut underline, mut range) = mem::uninitialized();
            let res = self.ptr.GetUnderline(position, &mut underline, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((underline != 0, range.into()).into())
        }
    }

    /// Gets the typography description applied to the text at the specified text position.
    pub fn typography(&self, position: u32) -> RangeResult<Typography> {
        unsafe {
            let (mut ptr, mut range) = mem::zeroed();
            let hr = self.ptr.GetTypography(position, &mut ptr, &mut range);
            if SUCCEEDED(hr) {
                Ok((Typography::from_raw(ptr), range.into()).into())
            } else {
                Err(hr.into())
            }
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
            self.ptr
                .HitTestPoint(point_x, point_y, &mut trail, &mut inside, &mut metrics);

            HitTestPoint {
                metrics: metrics.into(),
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
    pub fn hit_test_text_position(
        &self,
        position: u32,
        trailing: bool,
    ) -> Option<HitTestTextPosition> {
        let trailing = if trailing { 0 } else { 1 };
        unsafe {
            let (mut x, mut y) = (0.0, 0.0);
            let mut metrics = mem::zeroed();
            let res =
                self.ptr
                    .HitTestTextPosition(position, trailing, &mut x, &mut y, &mut metrics);
            if res != S_OK {
                return None;
            }

            Some(HitTestTextPosition {
                metrics: metrics.into(),
                point_x: x,
                point_y: y,
            })
        }
    }

    /// The application calls this function to get a set of hit-test metrics corresponding to a
    /// range of text positions. One of the main usages is to implement highlight selection of
    /// the text string. origin_x and origin_y are added to the hit-test metrics returned.
    pub fn hit_test_text_range(
        &self,
        position: u32,
        length: u32,
        origin_x: f32,
        origin_y: f32,
        metrics: &mut Vec<HitTestMetrics>,
    ) -> Result<(), Error> {
        unsafe {
            metrics.clear();

            // Calculate the total number of items we need
            let mut actual_count = 0;
            let hr = self.ptr.HitTestTextRange(
                position,
                length,
                origin_x,
                origin_y,
                ptr::null_mut(),
                0,
                &mut actual_count,
            );
            match hr {
                E_NOT_SUFFICIENT_BUFFER => (),
                S_OK => return Ok(()),
                hr => return Err(hr.into()),
            }

            metrics.reserve(actual_count as usize);
            let hr = self.ptr.HitTestTextRange(
                position,
                length,
                origin_x,
                origin_y,
                metrics.as_mut_ptr() as *mut _,
                metrics.capacity() as u32,
                &mut actual_count,
            );
            if hr != S_OK {
                metrics.set_len(0);
                return Err(hr.into());
            }

            metrics.set_len(actual_count as usize);
            Ok(())
        }
    }

    /// Sets the drawing style for text within a text range.
    pub fn set_drawing_effect(
        &mut self,
        effect: &impl DrawingEffect,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        let range = range.into();
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let hr = self.ptr.SetDrawingEffect(effect.get_effect_ptr(), range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font collection for text within a text range.
    pub fn set_font_collection(
        &mut self,
        collection: FontCollection,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        let range = range.into();
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let hr = self.ptr.SetFontCollection(collection.get_raw(), range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font family used for the specified range of text.
    pub fn set_font_family_name(
        &mut self,
        name: &str,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        unsafe {
            let name = name.to_wide_null();
            let range = range.into();

            let hr = self.ptr.SetFontFamilyName(name.as_ptr(), range.into());
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font size used for the specified range of text.
    pub fn set_font_size(&mut self, size: f32, range: impl Into<TextRange>) -> Result<(), Error> {
        unsafe {
            let range = range.into();

            let hr = self.ptr.SetFontSize(size, range.into());
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font stretch for text within a text range.
    pub fn set_font_stretch(
        &mut self,
        stretch: FontStretch,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        let range = range.into();
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let hr = self.ptr.SetFontStretch(stretch as u32, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font style for text within a text range.
    pub fn set_font_style(
        &mut self,
        style: FontStyle,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        let range = range.into();
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let hr = self.ptr.SetFontStyle(style as u32, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font weight for text within a text range.
    pub fn set_font_weight(
        &mut self,
        weight: FontWeight,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        let range = range.into();
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let hr = self.ptr.SetFontWeight(weight.0, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the inline object used for a range of text.
    pub fn set_inline_object(
        &mut self,
        obj: &InlineObject,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        let range = range.into();
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let hr = self.ptr.SetInlineObject(obj.get_raw(), range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the locale used for a range of text.
    pub fn set_locale_name(
        &mut self,
        locale: &str,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        let range = range.into();

        let locale = locale.to_wide_null();
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let hr = self.ptr.SetLocaleName(locale.as_ptr(), range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the max height in DIPs for this text layout.
    pub fn set_max_height(&mut self, maxh: f32) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetMaxHeight(maxh);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the max width in DIPs for this text layout.
    pub fn set_max_width(&mut self, maxw: f32) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetMaxWidth(maxw);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets strikethrough for text within a specified text range.
    pub fn set_strikethrough(
        &mut self,
        strikethrough: bool,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        let range = range.into().into();

        unsafe {
            let hr = self.ptr.SetStrikethrough(strikethrough as i32, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets underlining for text within a specified text range.
    pub fn set_underline(
        &mut self,
        underline: bool,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        let range = range.into().into();

        unsafe {
            let hr = self.ptr.SetUnderline(underline as i32, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the typography object controlling the font face settings for a range of text.
    pub fn set_typography(
        &mut self,
        typography: &Typography,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        let range = range.into().into();

        unsafe {
            let hr = self.ptr.SetTypography(typography.get_raw(), range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }
}

#[derive(Copy, Clone)]
/// Results from calling `hit_test_point` on a TextLayout.
pub struct HitTestPoint {
    /// The output geometry fully enclosing the hit-test location. When is_inside is set to false,
    /// this structure represents the geometry enclosing the edge closest to the hit-test location.
    pub metrics: HitTestMetrics,
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
/// Results from calling `hit_test_text_position` on a TextLayout.
pub struct HitTestTextPosition {
    /// The output pixel location X, relative to the top-left location of the layout box.
    pub point_x: f32,
    /// The output pixel location Y, relative to the top-left location of the layout box.
    pub point_y: f32,

    /// The output geometry fully enclosing the specified text position.
    pub metrics: HitTestMetrics,
}
