use client_effect::ClientEffect;
use enums::BreakCondition;
use error::DWResult;
use text_renderer::TextRenderer;

pub(crate) mod com_obj;

pub trait CustomInlineObject: Send + Sync + 'static {
    fn metrics(&self) -> InlineObjectMetrics;
    fn overhang_metrics(&self) -> InlineObjectOverhang;
    fn break_conditions(&self) -> BreakConditions;

    fn draw(&self, context: &DrawingContext) -> DWResult<()>;
}

pub struct DrawingContext<'a> {
    pub client_context: usize,
    pub renderer: &'a TextRenderer,
    pub origin_x: f32,
    pub origin_y: f32,
    pub is_sideways: bool,
    pub is_right_to_left: bool,
    pub client_effect: Option<&'a ClientEffect>,
}

pub struct InlineObjectMetrics {
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub supports_sideways: bool,
}

pub struct InlineObjectOverhang {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

pub struct BreakConditions {
    pub preceding: BreakCondition,
    pub following: BreakCondition,
}
