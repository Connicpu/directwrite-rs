//! PixelSnapping is for objects that describe pixel snapping behaviors, such as TextRenderer.

use error::DWResult;
use text_renderer::DrawContext;

use math2d::Matrix3x2f;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWritePixelSnapping;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
/// An object that describes how to handle pixel snapping.
pub struct PixelSnapping {
    ptr: ComPtr<IDWritePixelSnapping>,
}

impl PixelSnapping {
    /// Determines whether this object wants pixel snapping to be disabled.
    pub fn pixel_snapping_disabled(&self, context: &DrawContext) -> DWResult<bool> {
        unsafe {
            let mut disabled = 0;
            let hr = self
                .ptr
                .IsPixelSnappingDisabled(context.ptr(), &mut disabled);
            if SUCCEEDED(hr) {
                Ok(disabled != 0)
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets the transform that should be applied to this text.
    pub fn current_transform(&self, context: &DrawContext) -> DWResult<Matrix3x2f> {
        unsafe {
            let mut transform = std::mem::zeroed();
            let hr = self.ptr.GetCurrentTransform(context.ptr(), &mut transform);
            if SUCCEEDED(hr) {
                Ok(transform.into())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets the number of physical pixels per DIP currently being used in this renderer.
    pub fn pixels_per_dip(&self, context: &DrawContext) -> DWResult<f32> {
        unsafe {
            let mut ppd = 0.0;
            let hr = self.ptr.GetPixelsPerDip(context.ptr(), &mut ppd);
            if SUCCEEDED(hr) {
                Ok(ppd)
            } else {
                Err(hr.into())
            }
        }
    }
}
