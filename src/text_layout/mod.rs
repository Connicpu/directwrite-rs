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
use crate::text_format::ITextFormat;
use crate::text_renderer::DrawContext;
use crate::text_renderer::ITextRenderer;
use crate::typography::Typography;

use std::mem::MaybeUninit;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
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

impl TextLayout {
    /// Initialize a builder for a new TextLayout.
    pub fn create<'a>(factory: &'a Factory) -> TextLayoutBuilder<'a> {
        unsafe { TextLayoutBuilder::new(&*factory.get_raw()) }
    }
}

pub unsafe trait ITextLayout: ITextFormat {
    /// Determines the minimum possible width the layout can be set to without emergency breaking
    /// between the characters of whole words occurring.
    fn determine_min_width(&self) -> f32 {
        unsafe {
            let mut value = 0.0;
            self.raw_tl().DetermineMinWidth(&mut value);
            value
        }
    }

    /// Draws text using the specified client drawing context.
    fn draw(
        &self,
        renderer: &mut dyn ITextRenderer,
        origin_x: f32,
        origin_y: f32,
        context: &DrawContext,
    ) -> Result<(), Error> {
        unsafe {
            let hr = self.raw_tl().Draw(
                context.ptr(),
                renderer.raw_tr() as *const _ as *mut _,
                origin_x,
                origin_y,
            );
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets the number of ClusterMetrics objects which exist for this TextLayout
    fn cluster_metrics_count(&self) -> usize {
        unsafe {
            let mut count = 0;
            self.raw_tl()
                .GetClusterMetrics(std::ptr::null_mut(), 0, &mut count);
            count as usize
        }
    }

    /// Retrieves the ClusterMetrics for the glyph clusters in this layout. You should ensure the
    /// slice is large enough to hold all of the metrics, which can be obtained by calling
    /// `get_cluster_metrics_count`. If the slice is not large enough, it will return
    /// Err(actual_count), otherwise returns Ok(actual_count).
    fn cluster_metrics_slice(&self, buf: &mut [ClusterMetrics]) -> Result<usize, usize> {
        assert!(buf.len() <= std::u32::MAX as usize);
        unsafe {
            let mut actual_count = 0;
            let buf_ptr = buf.as_mut_ptr() as *mut DWRITE_CLUSTER_METRICS;
            let res = self
                .raw_tl()
                .GetClusterMetrics(buf_ptr, buf.len() as u32, &mut actual_count);

            if res == S_OK {
                Ok(actual_count as usize)
            } else {
                Err(actual_count as usize)
            }
        }
    }

    /// Fill all of the Cluster metrics into a Vec.
    fn cluster_metrics(&self) -> Vec<ClusterMetrics> {
        let count = self.cluster_metrics_count();
        let mut buf = Vec::with_capacity(count);
        unsafe { buf.set_len(count) };
        assert_eq!(self.cluster_metrics_slice(&mut buf), Ok(count));
        buf
    }

    /// Get the drawing effect applied at the specified position
    fn drawing_effect(&self, position: u32) -> RangeResult<Option<ClientEffect>> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let mut range = std::mem::zeroed();
            let hr = self
                .raw_tl()
                .GetDrawingEffect(position, &mut ptr, &mut range);
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
    fn font_collection(&self, position: u32) -> RangeResult<FontCollection> {
        unsafe {
            let mut collection = std::ptr::null_mut();
            let mut range = std::mem::zeroed();
            let res = self
                .raw_tl()
                .GetFontCollection(position, &mut collection, &mut range);
            if res < 0 {
                return Err(res.into());
            }
            Ok((FontCollection::from_raw(collection), range.into()).into())
        }
    }

    /// Get the font family name applied at the specified text position.
    fn font_family_name(&self, position: u32) -> RangeResult<String> {
        unsafe {
            let mut len = 0;
            let mut range = std::mem::zeroed();
            let hr = self
                .raw_tl()
                .GetFontFamilyNameLength(position, &mut len, &mut range);
            if !SUCCEEDED(hr) {
                return Err(hr.into());
            }

            let mut buf = vec![0u16; len as usize + 1];
            let hr = self.raw_tl().GetFontFamilyName(
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
    fn font_size(&self, position: u32) -> RangeResult<f32> {
        unsafe {
            let mut font_size = 0.0;
            let mut range = MaybeUninit::uninit();
            let res = self
                .raw_tl()
                .GetFontSize(position, &mut font_size, range.as_mut_ptr());
            if res < 0 {
                return Err(res.into());
            }
            Ok((font_size, range.assume_init().into()).into())
        }
    }

    /// Gets the font stretch of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    fn font_stretch(&self, position: u32) -> RangeResult<UncheckedEnum<FontStretch>> {
        unsafe {
            let mut stretch = MaybeUninit::uninit();
            let mut range = MaybeUninit::uninit();
            let res =
                self.raw_tl()
                    .GetFontStretch(position, stretch.as_mut_ptr(), range.as_mut_ptr());
            if res < 0 {
                return Err(res.into());
            }

            Ok((stretch.assume_init().into(), range.assume_init().into()).into())
        }
    }

    /// Gets the font style of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    fn font_style(&self, position: u32) -> RangeResult<UncheckedEnum<FontStyle>> {
        unsafe {
            let mut style = MaybeUninit::uninit();
            let mut range = MaybeUninit::uninit();
            let res = self
                .raw_tl()
                .GetFontStyle(position, style.as_mut_ptr(), range.as_mut_ptr());
            if res < 0 {
                return Err(res.into());
            }

            Ok((style.assume_init().into(), range.assume_init().into()).into())
        }
    }

    /// Gets the font weight of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    fn font_weight(&self, position: u32) -> RangeResult<FontWeight> {
        unsafe {
            let mut weight = 0;
            let mut range = MaybeUninit::uninit();
            let res = self
                .raw_tl()
                .GetFontWeight(position, &mut weight, range.as_mut_ptr());
            if res < 0 {
                return Err(res.into());
            }

            Ok((FontWeight(weight), range.assume_init().into()).into())
        }
    }

    /// Gets the inline object at the position as-is. May return std::ptr::null_mut()
    fn inline_object(&self, position: u32) -> RangeResult<Option<InlineObject>> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let mut range = MaybeUninit::uninit();
            let hr = self
                .raw_tl()
                .GetInlineObject(position, &mut ptr, range.as_mut_ptr());
            if SUCCEEDED(hr) {
                let obj = if !ptr.is_null() {
                    Some(InlineObject::from_raw(ptr))
                } else {
                    None
                };
                Ok((obj, range.assume_init().into()).into())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Get the number of LineMetrics objects that you need room for when calling
    /// `get_line_metrics_slice`
    fn line_metrics_count(&self) -> usize {
        unsafe {
            let mut count = 0;
            self.raw_tl()
                .GetLineMetrics(std::ptr::null_mut(), 0, &mut count);
            count as usize
        }
    }

    /// Retrieves the information about each individual text line of the text string. You should
    /// first call `get_line_metrics_count` to know how large your slice must be to fit all of
    /// the metrics objects. The return value will contain the actual number of elements in the
    /// layout, but the official documentation does *not* specify whether the array will be filled
    /// with any values in the Err case, so that behavior is not guaranteed between windows
    /// versions.
    fn line_metrics_slice(&self, buf: &mut [LineMetrics]) -> Result<usize, usize> {
        assert!(buf.len() <= std::u32::MAX as usize);
        unsafe {
            let mut actual_count = 0;
            let buf_ptr = buf.as_mut_ptr() as *mut DWRITE_LINE_METRICS;
            let res = self
                .raw_tl()
                .GetLineMetrics(buf_ptr, buf.len() as u32, &mut actual_count);

            if res == S_OK {
                Ok(actual_count as usize)
            } else {
                Err(actual_count as usize)
            }
        }
    }

    /// Retrieves the information about each individual text line of the text string.
    fn line_metrics(&self) -> Vec<LineMetrics> {
        let count = self.line_metrics_count();
        let mut buf = Vec::with_capacity(count);
        unsafe { buf.set_len(count) };
        assert_eq!(self.line_metrics_slice(&mut buf), Ok(count));
        buf
    }

    /// Gets the locale name applied to the text at the specified text position.
    fn locale_name(&self, position: u32) -> RangeResult<String> {
        unsafe {
            let mut len = 0;
            let mut range = std::mem::zeroed();
            let hr = self
                .raw_tl()
                .GetLocaleNameLength(position, &mut len, &mut range);
            if !SUCCEEDED(hr) {
                return Err(hr.into());
            }

            let mut buf = vec![0u16; len as usize + 1];
            let hr = self.raw_tl().GetLocaleName(
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

    /// Gets the layout maximum height.
    fn max_height(&self) -> f32 {
        unsafe { self.raw_tl().GetMaxHeight() }
    }

    /// Gets the layout maximum width.
    fn max_width(&self) -> f32 {
        unsafe { self.raw_tl().GetMaxWidth() }
    }

    /// Retrieves overall metrics for the formatted string.
    fn metrics(&self) -> TextMetrics {
        unsafe {
            let mut metrics = std::mem::zeroed();
            self.raw_tl().GetMetrics(&mut metrics);
            metrics.into()
        }
    }

    /// Returns the overhangs (in DIPs) of the layout and all objects contained in it, including
    /// text glyphs and inline objects.
    fn overhang_metrics(&self) -> OverhangMetrics {
        unsafe {
            let mut metrics = std::mem::zeroed();
            self.raw_tl().GetOverhangMetrics(&mut metrics);
            metrics.into()
        }
    }

    /// Returns whether the text at the specified position has strikethrough applied.
    fn strikethrough(&self, position: u32) -> RangeResult<bool> {
        unsafe {
            let (mut strikethrough, mut range) = std::mem::zeroed();
            let res = self
                .raw_tl()
                .GetStrikethrough(position, &mut strikethrough, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((strikethrough != 0, range.into()).into())
        }
    }

    /// Returns whether the text at the specified position has underline applied.
    fn underline(&self, position: u32) -> RangeResult<bool> {
        unsafe {
            let mut underline = 0;
            let mut range = MaybeUninit::uninit();
            let res = self
                .raw_tl()
                .GetUnderline(position, &mut underline, range.as_mut_ptr());
            if res < 0 {
                return Err(res.into());
            }

            Ok((underline != 0, range.assume_init().into()).into())
        }
    }

    /// Gets the typography description applied to the text at the specified text position.
    fn typography(&self, position: u32) -> RangeResult<Typography> {
        unsafe {
            let (mut ptr, mut range) = std::mem::zeroed();
            let hr = self.raw_tl().GetTypography(position, &mut ptr, &mut range);
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
    fn hit_test_point(&self, point_x: f32, point_y: f32) -> HitTestPoint {
        unsafe {
            let mut trail = 0;
            let mut inside = 0;
            let mut metrics = MaybeUninit::uninit();
            self.raw_tl().HitTestPoint(
                point_x,
                point_y,
                &mut trail,
                &mut inside,
                metrics.as_mut_ptr(),
            );

            HitTestPoint {
                metrics: metrics.assume_init().into(),
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
    fn hit_test_text_position(&self, position: u32, trailing: bool) -> Option<HitTestTextPosition> {
        let trailing = if trailing { 0 } else { 1 };
        unsafe {
            let (mut x, mut y) = (0.0, 0.0);
            let mut metrics = std::mem::zeroed();
            let res =
                self.raw_tl()
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
    fn hit_test_text_range(
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
            let hr = self.raw_tl().HitTestTextRange(
                position,
                length,
                origin_x,
                origin_y,
                std::ptr::null_mut(),
                0,
                &mut actual_count,
            );
            match hr {
                E_NOT_SUFFICIENT_BUFFER => (),
                S_OK => return Ok(()),
                hr => return Err(hr.into()),
            }

            metrics.reserve(actual_count as usize);
            let hr = self.raw_tl().HitTestTextRange(
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
    fn set_drawing_effect(
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
            let hr = self
                .raw_tl()
                .SetDrawingEffect(effect.get_effect_ptr(), range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font collection for text within a text range.
    fn set_font_collection(
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
            let hr = self.raw_tl().SetFontCollection(collection.get_raw(), range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font family used for the specified range of text.
    fn set_font_family_name(
        &mut self,
        name: &str,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        unsafe {
            let name = name.to_wide_null();
            let range = range.into();

            let hr = self.raw_tl().SetFontFamilyName(name.as_ptr(), range.into());
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font size used for the specified range of text.
    fn set_font_size(&mut self, size: f32, range: impl Into<TextRange>) -> Result<(), Error> {
        unsafe {
            let range = range.into();

            let hr = self.raw_tl().SetFontSize(size, range.into());
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font stretch for text within a text range.
    fn set_font_stretch(
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
            let hr = self.raw_tl().SetFontStretch(stretch as u32, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font style for text within a text range.
    fn set_font_style(
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
            let hr = self.raw_tl().SetFontStyle(style as u32, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font weight for text within a text range.
    fn set_font_weight(
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
            let hr = self.raw_tl().SetFontWeight(weight.0, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the inline object used for a range of text.
    fn set_inline_object(
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
            let hr = self.raw_tl().SetInlineObject(obj.get_raw(), range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the locale used for a range of text.
    fn set_locale_name(&mut self, locale: &str, range: impl Into<TextRange>) -> Result<(), Error> {
        let range = range.into();

        let locale = locale.to_wide_null();
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let hr = self.raw_tl().SetLocaleName(locale.as_ptr(), range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the max height in DIPs for this text layout.
    fn set_max_height(&mut self, maxh: f32) -> Result<(), Error> {
        unsafe {
            let hr = self.raw_tl().SetMaxHeight(maxh);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the max width in DIPs for this text layout.
    fn set_max_width(&mut self, maxw: f32) -> Result<(), Error> {
        unsafe {
            let hr = self.raw_tl().SetMaxWidth(maxw);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets strikethrough for text within a specified text range.
    fn set_strikethrough(
        &mut self,
        strikethrough: bool,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        let range = range.into().into();

        unsafe {
            let hr = self.raw_tl().SetStrikethrough(strikethrough as i32, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets underlining for text within a specified text range.
    fn set_underline(&mut self, underline: bool, range: impl Into<TextRange>) -> Result<(), Error> {
        let range = range.into().into();

        unsafe {
            let hr = self.raw_tl().SetUnderline(underline as i32, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the typography object controlling the font face settings for a range of text.
    fn set_typography(
        &mut self,
        typography: &Typography,
        range: impl Into<TextRange>,
    ) -> Result<(), Error> {
        let range = range.into().into();

        unsafe {
            let hr = self.raw_tl().SetTypography(typography.get_raw(), range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    unsafe fn raw_tl(&self) -> &IDWriteTextLayout;
}

unsafe impl ITextFormat for TextLayout {
    unsafe fn raw_tf(&self) -> &IDWriteTextFormat {
        &self.ptr
    }
}

unsafe impl ITextLayout for TextLayout {
    unsafe fn raw_tl(&self) -> &IDWriteTextLayout {
        &self.ptr
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
