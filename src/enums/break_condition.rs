#[auto_enum::auto_enum(u32, checked)]
/// Indicates the condition at the edges of inline object or text used to
/// determine line-breaking behavior.
pub enum BreakCondition {
    /// Indicates whether a break is allowed by determining the condition of
    /// the neighboring text span or inline object.
    Neutral = 0,
    
    /// Indicates that a line break is allowed, unless overruled by the
    /// condition of the neighboring text span or inline object, either
    /// prohibited by a "may not break" condition or forced by a "must break"
    /// condition.
    CanBreak = 1,

    /// Indicates that there should be no line break, unless overruled by a
    /// "must break" condition from the neighboring text span or inline object.
    MayNotBreak = 2,

    /// Indicates that the line break must happen, regardless of the condition
    /// of the adjacent text span or inline object.
    MustBreak = 3,
}
