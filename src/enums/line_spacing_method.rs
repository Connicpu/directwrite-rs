#[auto_enum(u32, checked)]
/// The method used for line spacing in a text layout.
pub enum LineSpacingMethod {
    /// Line spacing depends solely on the content, adjusting to accommodate
    /// the size of fonts and inline objects.
    Default = 0,

    /// Lines are explicitly set to uniform spacing, regardless of the size
    /// of fonts and inline objects. This can be useful to avoid the uneven
    /// appearance that can occur from font fallback.
    Uniform = 1,

    /// Line spacing and baseline distances are proportional to the computed
    /// values based on the content, the size of the fonts and inline objects.
    /// 
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    /// 
    /// **Note**
    /// This value is only available on Windows 10 or later and it can be used
    /// with `IDWriteTextLayout3::SetLineSpacing`, but can not be used with
    /// `IDWriteTextFormat::SetLineSpacing`. This means if you would like to
    /// use this line spacing method with this crate, you need to manually
    /// retrieve the inner DWrite pointer and QueryInterface it to a Layout3.
    /// 
    /// </div>
    Proportional = 2,
}
