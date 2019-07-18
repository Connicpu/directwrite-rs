#[auto_enum::auto_enum(u32, checked)]
/// Indicates the direction of how lines of text are placed relative to one another.
pub enum FlowDirection {
    /// Specifies that text lines are placed from top to bottom.
    TopToBottom = 0,

    /// Specifies that text lines are placed from bottom to top.
    BottomToTop = 1,

    /// Specifies that text lines are placed from left to right.
    LeftToRight = 2,

    /// Specifies that text lines are placed from right to left.
    RightToLeft = 3,
}
