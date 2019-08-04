//! PixelSnapping is for objects that describe pixel snapping behaviors, such as TextRenderer.

use crate::text_renderer::DrawContext;

use com_wrapper::ComWrapper;
use dcommon::Error;
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

pub unsafe trait IPixelSnapping {
    /// Determines whether this object wants pixel snapping to be disabled.
    fn pixel_snapping_disabled(&self, context: &DrawContext) -> Result<bool, Error> {
        unsafe {
            let mut disabled = 0;
            let hr = self
                .raw_psnap()
                .IsPixelSnappingDisabled(context.ptr(), &mut disabled);
            if SUCCEEDED(hr) {
                Ok(disabled != 0)
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets the transform that should be applied to this text.
    fn current_transform(&self, context: &DrawContext) -> Result<Matrix3x2f, Error> {
        unsafe {
            let mut transform = std::mem::zeroed();
            let hr = self
                .raw_psnap()
                .GetCurrentTransform(context.ptr(), &mut transform);
            if SUCCEEDED(hr) {
                Ok(transform.into())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets the number of physical pixels per DIP currently being used in this renderer.
    fn pixels_per_dip(&self, context: &DrawContext) -> Result<f32, Error> {
        unsafe {
            let mut ppd = 0.0;
            let hr = self.raw_psnap().GetPixelsPerDip(context.ptr(), &mut ppd);
            if SUCCEEDED(hr) {
                Ok(ppd)
            } else {
                Err(hr.into())
            }
        }
    }

    unsafe fn raw_psnap(&self) -> &IDWritePixelSnapping;
}

unsafe impl IPixelSnapping for PixelSnapping {
    unsafe fn raw_psnap(&self) -> &IDWritePixelSnapping {
        &self.ptr
    }
}
