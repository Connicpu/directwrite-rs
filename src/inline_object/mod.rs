//! InlineObject and types for creating your own instances.

use effects::ClientEffect;
use enums::BreakCondition;
use error::DWResult;
use factory::Factory;
use helpers::unwrap_opt_com;
use inline_object::custom::CustomInlineObject;
use metrics::overhang::OverhangMetrics;
use metrics::InlineObjectMetrics;
use text_format::TextFormat;
use text_renderer::DrawContext;
use text_renderer::TextRenderer;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use math2d::Point2f;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteInlineObject;
use wio::com::ComPtr;

pub mod custom;

#[repr(transparent)]
#[derive(Clone, ComWrapper)]
#[com(send, sync)]
/// Inline text object that can be layed out in the middle of text. Can be implemented custom
/// by an application or an implementation provided by another library.
pub struct InlineObject {
    ptr: ComPtr<IDWriteInlineObject>,
}

impl InlineObject {
    /// Create an InlineObject from a Rust application-implemented object.
    pub fn create_custom(object: impl CustomInlineObject) -> InlineObject {
        let ptr = custom::com_obj::ComInlineObject::new(object);
        unsafe { InlineObject::from_ptr(ptr) }
    }

    /// Creates an inline object for trimming, using an ellipsis as the ommission sign.
    pub fn create_trimming_ellipsis(
        factory: &Factory,
        format: &TextFormat,
    ) -> DWResult<InlineObject> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = (*factory.get_raw()).CreateEllipsisTrimmingSign(format.get_raw(), &mut ptr);
            if SUCCEEDED(hr) {
                Ok(InlineObject::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Get the metrics reported by this inline object.
    pub fn metrics(&self) -> InlineObjectMetrics {
        unsafe {
            let mut metrics = std::mem::zeroed();
            self.ptr.GetMetrics(&mut metrics);
            metrics.into()
        }
    }

    /// Get the overhang reported by this inline object.
    pub fn overhang_metrics(&self) -> OverhangMetrics {
        unsafe {
            let mut metrics = std::mem::zeroed();
            self.ptr.GetOverhangMetrics(&mut metrics);
            metrics.into()
        }
    }

    /// Get the object's requested line breaking behavior.
    pub fn break_conditions(&self) -> BreakConditions {
        unsafe {
            let (mut before, mut after) = std::mem::zeroed();
            self.ptr.GetBreakConditions(&mut before, &mut after);
            BreakConditions {
                preceding: before.into(),
                following: after.into(),
            }
        }
    }

    /// Requests the inline object to draw itself.
    pub fn draw(&self, context: DrawingContext) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.Draw(
                context.client_context.ptr(),
                context.renderer.get_raw(),
                context.origin.x,
                context.origin.y,
                context.is_sideways as i32,
                context.is_right_to_left as i32,
                unwrap_opt_com(context.client_effect),
            );

            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }
}

/// The pair of break conditions for an inline object.
pub struct BreakConditions {
    /// The condition with regards to the text that precedes the object.
    pub preceding: UncheckedEnum<BreakCondition>,

    /// The condition with regards to the text that follows the object.
    pub following: UncheckedEnum<BreakCondition>,
}

/// Data describing the context your object is being drawn in.
pub struct DrawingContext<'a> {
    /// An application or renderer determined context value associated with this drawing
    /// state. This may be a pointer or an integer value.
    pub client_context: DrawContext,

    /// The text renderer drawing this object.
    pub renderer: &'a mut TextRenderer,

    /// The origin point of the rendering.
    pub origin: Point2f,

    /// Indicates whether the object's baseline runs alongside the baseline axis of the line.
    pub is_sideways: bool,

    /// Indicates whether the object is in a right-to-left context and should be drawn flipped.
    pub is_right_to_left: bool,

    /// The drawing effect set in [`TextLayout::set_drawing_effect`][1]. Usually this effect is
    /// a foreground brush that is used in glyph drawing.
    ///
    /// [1]: ../../struct.TextLayout.html#method.set_drawing_effect
    pub client_effect: Option<&'a ClientEffect>,
}
