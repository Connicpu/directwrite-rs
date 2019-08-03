use crate::enums::pixel_geometry::PixelGeometry;
use crate::enums::rendering_mode::RenderingMode;
use crate::error::DWResult;
use crate::factory::Factory;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use winapi::shared::windef::HMONITOR;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteRenderingParams;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(Clone, ComWrapper)]
#[com(send, sync, debug)]
/// Represents text rendering settings such as ClearType level, enhanced contrast, and gamma
/// correction for glyph rasterization and filtering.
pub struct RenderingParams {
    ptr: ComPtr<IDWriteRenderingParams>,
}

impl RenderingParams {
    /// Get the rendering parameters appropriate for rendering on the given monitor.
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Safety**
    /// This method is safe to take an `HMONITOR` because `HMONITOR` is just a handle and will
    /// therefore return an error if an invalid one is passed instead of creating memory
    /// safety errors.
    ///
    /// </div>
    pub fn create_for_monitor(factory: &Factory, monitor: HMONITOR) -> DWResult<RenderingParams> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = (*factory.get_raw()).CreateMonitorRenderingParams(monitor, &mut ptr);
            if SUCCEEDED(hr) {
                Ok(RenderingParams::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Creates a rendering parameters object with default settings for the primary monitor.
    /// Different monitors may have different rendering parameters, for more information see the
    /// [How to Add Support for Multiple Monitors][1] topic.
    ///
    /// [1]: https://msdn.microsoft.com/62274126-49da-4166-8482-73aac2b29c26
    pub fn create_default(factory: &Factory) -> DWResult<RenderingParams> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = (*factory.get_raw()).CreateRenderingParams(&mut ptr);
            if SUCCEEDED(hr) {
                Ok(RenderingParams::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets the ClearType level of the rendering parameters object.
    ///
    /// The ClearType level represents the amount of ClearType – that is, the degree to which the
    /// red, green, and blue subpixels of each pixel are treated differently. Valid values range
    /// from zero (meaning no ClearType, which is equivalent to grayscale anti-aliasing) to one
    /// (meaning full ClearType).
    pub fn cleartype_level(&self) -> f32 {
        unsafe { self.ptr.GetClearTypeLevel() }
    }

    /// Gets the enhanced contrast property of the rendering parameters object. Valid values are
    /// greater than or equal to zero.
    ///
    /// Enhanced contrast is the amount to increase the darkness of text, and typically ranges
    /// from 0 to 1. Zero means no contrast enhancement.
    pub fn enhanced_contrast(&self) -> f32 {
        unsafe { self.ptr.GetEnhancedContrast() }
    }

    /// Gets the gamma value used for gamma correction. Valid values must be greater than zero
    /// and cannot exceed 256.
    ///
    /// The gamma value is used for gamma correction, which compensates for the non-linear
    /// luminosity response of most monitors.
    pub fn gamma(&self) -> f32 {
        unsafe { self.ptr.GetGamma() }
    }

    /// Gets the pixel geometry of the rendering parameters object.
    pub fn pixel_geometry(&self) -> UncheckedEnum<PixelGeometry> {
        unsafe { self.ptr.GetPixelGeometry().into() }
    }

    /// Gets the rendering mode of the rendering parameters object.
    ///
    /// By default, the rendering mode is initialized to [`Default`][1], which means the rendering
    /// mode is determined automatically based on the font and size. To determine the recommended
    /// rendering mode to use for a given font and size and rendering parameters object, use the
    /// [`FontFace::recommended_rendering_mode`][2] method.
    ///
    /// [1]: enums/enum.RenderingMode.html#variant.Default
    /// [2]: struct.FontFace.html#method.recommended_rendering_mode
    pub fn rendering_mode(&self) -> UncheckedEnum<RenderingMode> {
        unsafe { self.ptr.GetRenderingMode().into() }
    }
}
