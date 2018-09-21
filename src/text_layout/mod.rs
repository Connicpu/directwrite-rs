use drawing_effect::DrawingEffect;
use enums::{FontStretch, FontStyle, FontWeight};
use error::DWResult;
use factory::Factory;
use helpers::InternalConstructor;
use inline_object::IntoInlineObject;
use font_collection::FontCollection;
use text_format::TextFormat;
use text_renderer::{Context, TextRenderer, TextRendererComRef};

use std::{mem, ops, ptr, u32};

use winapi::shared::winerror::{SUCCEEDED, S_OK};
use winapi::um::dwrite::*;
use wio::com::ComPtr;
use wio::wide::ToWide;

pub use self::builder::TextLayoutBuilder;

pub mod builder;
pub mod metrics;

/// The TextLayout interface represents a block of text after it has been fully analyzed and formatted.
#[derive(Clone, PartialEq)]
pub struct TextLayout {
    ptr: ComPtr<IDWriteTextLayout>,
}

impl TextLayout {
    pub fn create<'a>(factory: &'a Factory) -> TextLayoutBuilder<'a> {
        unsafe { TextLayoutBuilder::new(&*factory.get_raw()) }
    }

    pub fn as_format(&self) -> TextFormat {
        unsafe { TextFormat::from_raw(self.ptr.clone().up().into_raw()) }
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

    pub fn draw(
        &self,
        renderer: &mut TextRenderer,
        origin_x: f32,
        origin_y: f32,
        context: Context,
    ) -> DWResult<()> {
        unsafe {
            let mut renderer = TextRendererComRef::new(renderer);

            let hr = self.ptr
                .Draw(context.0, renderer.as_raw(), origin_x, origin_y);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets the number of ClusterMetrics objects which exist for this TextLayout
    pub fn get_cluster_metrics_count(&self) -> usize {
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
    pub fn get_cluster_metrics_slice(
        &self,
        buf: &mut [metrics::ClusterMetrics],
    ) -> Result<usize, usize> {
        assert!(buf.len() <= u32::MAX as usize);
        unsafe {
            let mut actual_count = 0;
            let buf_ptr = buf.as_mut_ptr() as *mut DWRITE_CLUSTER_METRICS;
            let res = self.ptr
                .GetClusterMetrics(buf_ptr, buf.len() as u32, &mut actual_count);

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

    /// Gets the font collection of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn get_font_collection(&self, position: u32) -> DWResult<(FontCollection, TextRange)> {
        unsafe {
            let mut collection = ptr::null_mut();
            let mut range = mem::uninitialized();
            let res = self.ptr.GetFontCollection(position, &mut collection, &mut range);
            if res < 0 {
                return Err(res.into())
            }
            Ok((FontCollection::from_raw(collection), range.into()))
        }
    }

    /// Gets the font em height of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn get_font_size(&self, position: u32) -> DWResult<(f32, TextRange)> {
        unsafe {
            let mut font_size = 0.0;
            let mut range = mem::uninitialized();
            let res = self.ptr.GetFontSize(position, &mut font_size, &mut range);
            if res < 0 {
                return Err(res.into());
            }
            Ok((font_size, range.into()))
        }
    }

    /// Gets the font stretch of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn get_font_stretch(&self, position: u32) -> DWResult<(FontStretch, TextRange)> {
        unsafe {
            let (mut stretch, mut range) = mem::uninitialized();
            let res = self.ptr.GetFontStretch(position, &mut stretch, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((FontStretch::from_u32(stretch).unwrap(), range.into()))
        }
    }

    /// Gets the font style of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn get_font_style(&self, position: u32) -> DWResult<(FontStyle, TextRange)> {
        unsafe {
            let (mut style, mut range) = mem::uninitialized();
            let res = self.ptr.GetFontStyle(position, &mut style, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((FontStyle::from_u32(style).unwrap(), range.into()))
        }
    }

    /// Gets the font weight of the text at the specified position. Also returns the text range
    /// which has identical formatting to the current character.
    pub fn get_font_weight(&self, position: u32) -> DWResult<(FontWeight, TextRange)> {
        unsafe {
            let (mut weight, mut range) = mem::uninitialized();
            let res = self.ptr.GetFontWeight(position, &mut weight, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((FontWeight::from_u32(weight).unwrap(), range.into()))
        }
    }

    /// Gets the inline object at the position as-is. May return ptr::null_mut()
    pub fn get_inline_object(
        &self,
        position: u32,
    ) -> DWResult<(*mut IDWriteInlineObject, TextRange)> {
        unsafe {
            let mut range = mem::uninitialized();
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetInlineObject(position, &mut ptr, &mut range);
            if SUCCEEDED(hr) {
                Ok((ptr, range.into()))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Get the number of LineMetrics objects that you need room for when calling
    /// `get_line_metrics_slice`
    pub fn get_line_metrics_count(&self) -> usize {
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
    pub fn get_line_metrics_slice(&self, buf: &mut [metrics::LineMetrics]) -> Result<usize, usize> {
        assert!(buf.len() <= u32::MAX as usize);
        unsafe {
            let mut actual_count = 0;
            let buf_ptr = buf.as_mut_ptr() as *mut DWRITE_LINE_METRICS;
            let res = self.ptr
                .GetLineMetrics(buf_ptr, buf.len() as u32, &mut actual_count);

            if res == S_OK {
                Ok(actual_count as usize)
            } else {
                Err(actual_count as usize)
            }
        }
    }

    /// Retrieves the information about each individual text line of the text string. Resizes `buf`
    /// to fit all of the elements.
    pub fn get_line_metrics(&self, buf: &mut Vec<metrics::LineMetrics>) {
        let count = self.get_line_metrics_count();
        unsafe {
            buf.clear();
            buf.reserve(count);
            buf.set_len(count);
        }
        assert_eq!(self.get_line_metrics_slice(buf), Ok(count));
    }

    /// Gets the layout maximum height.
    pub fn get_max_height(&self) -> f32 {
        unsafe { self.ptr.GetMaxHeight() }
    }

    /// Gets the layout maximum width.
    pub fn get_max_width(&self) -> f32 {
        unsafe { self.ptr.GetMaxWidth() }
    }

    /// Retrieves overall metrics for the formatted string.
    pub fn get_metrics(&self) -> metrics::Metrics {
        unsafe {
            let mut metrics = mem::zeroed();
            self.ptr.GetMetrics(&mut metrics);

            metrics::Metrics::build(metrics)
        }
    }

    /// Returns the overhangs (in DIPs) of the layout and all objects contained in it, including
    /// text glyphs and inline objects.
    pub fn get_overhang_metrics(&self) -> metrics::OverhangMetrics {
        unsafe {
            let mut metrics = mem::zeroed();
            self.ptr.GetOverhangMetrics(&mut metrics);

            metrics::OverhangMetrics::build(metrics)
        }
    }

    pub fn get_strikethrough(&self, position: u32) -> DWResult<(bool, TextRange)> {
        unsafe {
            let (mut strikethrough, mut range) = mem::uninitialized();
            let res = self.ptr
                .GetStrikethrough(position, &mut strikethrough, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((strikethrough != 0, range.into()))
        }
    }

    // TODO: Typography

    pub fn get_underline(&self, position: u32) -> DWResult<(bool, TextRange)> {
        unsafe {
            let (mut underline, mut range) = mem::uninitialized();
            let res = self.ptr.GetUnderline(position, &mut underline, &mut range);
            if res < 0 {
                return Err(res.into());
            }

            Ok((underline != 0, range.into()))
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
    pub fn hit_test_text_position(
        &self,
        position: u32,
        trailing: bool,
    ) -> Option<HitTestTextPosition> {
        let trailing = if trailing { 0 } else { 1 };
        unsafe {
            let (mut x, mut y) = (0.0, 0.0);
            let mut metrics = mem::uninitialized();
            let res =
                self.ptr
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
    pub fn hit_test_text_range(
        &self,
        position: u32,
        length: u32,
        origin_x: f32,
        origin_y: f32,
        metrics: &mut Vec<metrics::HitTestMetrics>,
    ) -> bool {
        unsafe {
            // Calculate the total number of items we need
            let mut actual_count = 0;
            let res = self.ptr.HitTestTextRange(
                position,
                length,
                origin_x,
                origin_y,
                ptr::null_mut(),
                0,
                &mut actual_count,
            );
            if res != S_OK {
                return false;
            }

            metrics.set_len(actual_count as usize);
            let buf_ptr = metrics[..].as_mut_ptr() as *mut _;
            let len = metrics.len() as u32;
            let res = self.ptr.HitTestTextRange(
                position,
                length,
                origin_x,
                origin_y,
                buf_ptr,
                len,
                &mut actual_count,
            );
            if res != S_OK {
                metrics.set_len(0);
                return false;
            }

            metrics.set_len(actual_count as usize);
            true
        }
    }

    /// Sets the drawing style for text within a text range.
    pub fn set_drawing_effect<E, T>(&self, effect: &E, range: T) -> DWResult<()>
    where
        E: DrawingEffect,
        T: Into<TextRange>,
    {
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
    pub fn set_font_collection<T>(&self, collection: FontCollection, range: T) -> DWResult<()>
    where
        T: Into<TextRange>,
    {
        let range = range.into();
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let hr = self.ptr.SetFontCollection(collection.get_raw(), range); 
            if SUCCEEDED(hr){
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets the font style for text within a text range.
    pub fn set_font_style<T>(&self, style: FontStyle, range: T) -> DWResult<()>
    where
        T: Into<TextRange>,
    {
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
    pub fn set_font_weight<T>(&self, weight: FontWeight, range: T) -> DWResult<()>
    where
        T: Into<TextRange>,
    {
        let range = range.into();
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let hr = self.ptr.SetFontWeight(weight as u32, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_inline_object<I, T>(&self, iobj: I, range: T) -> DWResult<()>
    where
        I: IntoInlineObject,
        T: Into<TextRange>,
    {
        let range = range.into();
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let iobj = iobj.into_iobj();
            let hr = self.ptr.SetInlineObject(iobj, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_locale_name<T>(&self, locale: &str, range: T) -> DWResult<()>
    where
        T: Into<TextRange>,
    {
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

    pub fn set_max_height(&self, maxh: f32) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.SetMaxHeight(maxh);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_max_width(&self, maxw: f32) -> DWResult<()> {
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
    pub fn set_strikethrough<T>(&self, strikethrough: bool, range: T) -> DWResult<()>
    where
        T: Into<TextRange>,
    {
        let range = range.into();

        let strikethrough = if strikethrough { 1 } else { 0 };
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let hr = self.ptr.SetStrikethrough(strikethrough, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    // TODO: Typography

    /// Sets underlining for text within a specified text range.
    pub fn set_underline<T>(&self, underline: bool, range: T) -> DWResult<()>
    where
        T: Into<TextRange>,
    {
        let range = range.into();

        let underline = if underline { 1 } else { 0 };
        let range = DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        };

        unsafe {
            let hr = self.ptr.SetUnderline(underline, range);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub unsafe fn from_raw(raw: *mut IDWriteTextLayout) -> Self {
        TextLayout {
            ptr: ComPtr::from_raw(raw),
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteTextLayout {
        self.ptr.as_raw()
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

impl From<ops::Range<u32>> for TextRange {
    fn from(range: ops::Range<u32>) -> Self {
        assert!(
            range.end >= range.start,
            "Range end cannot come before range start"
        );
        TextRange {
            start: range.start,
            length: range.end - range.start,
        }
    }
}

impl From<ops::RangeTo<u32>> for TextRange {
    fn from(range: ops::RangeTo<u32>) -> Self {
        TextRange {
            start: 0,
            length: range.end,
        }
    }
}

impl From<ops::RangeFrom<u32>> for TextRange {
    fn from(range: ops::RangeFrom<u32>) -> Self {
        TextRange {
            start: range.start,
            length: u32::MAX,
        }
    }
}

impl From<ops::RangeFull> for TextRange {
    fn from(_range: ops::RangeFull) -> Self {
        TextRange {
            start: 0,
            length: u32::MAX,
        }
    }
}

// TODO: Re-enable when 1.26 drops
/*impl From<ops::RangeInclusive<u32>> for TextRange {
    fn from(mut range: ops::RangeInclusive<u32>) -> Self {
        /*assert!(range.end + 1 >= range.start, "Range end cannot come before range start");
        TextRange {
            start: range.start,
            length: (range.end + 1) - range.start,
        }*/

        // TODO: Accessing `RangeInclusive` directly is nightly-only, so
        // I'm relying on the implementation of size_hint for RangeInclusive
        // to get the values I want.
        let length = range.size_hint().0 as u32;
        let start = range.nth(0).unwrap_or(0);

        TextRange { start, length }
    }
}*/

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

unsafe impl Send for TextLayout {}
unsafe impl Sync for TextLayout {}
