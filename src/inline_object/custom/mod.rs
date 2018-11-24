//! Traits and Structs for implementing custom inline text objects that can be layed
//! out inline with text.

use error::DWResult;
use inline_object::BreakConditions;
use inline_object::DrawingContext;
use metrics::InlineObjectMetrics;
use metrics::OverhangMetrics;


pub(crate) mod com_obj;

/// Custom implementation of an inline text object in Rust.
pub trait CustomInlineObject: Send + Sync + 'static {
    /// Report metrics about your inline object to the runtime.
    fn metrics(&self) -> InlineObjectMetrics;

    /// Report your object's overhang values to the runtime. See the documentation on
    /// `OverhangMetrics` for more information.
    fn overhang_metrics(&self) -> OverhangMetrics;

    /// Layout uses this to determine the line-breaking behavior of the inline
    /// object among the text.
    fn break_conditions(&self) -> BreakConditions;

    /// Called by the text renderer to render your object within the text.
    fn draw(&self, context: &DrawingContext) -> DWResult<()>;
}
