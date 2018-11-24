#[auto_enum(u32, checked)]
/// Represents the internal structure of a device pixel (that is, the physical arrangement of
/// red, green, and blue color components) that is assumed for purposes of rendering text.
pub enum PixelGeometry {
    /// The red, green, and blue color components of each pixel are assumed to occupy the same point.
    Flat,

    /// Each pixel is composed of three vertical stripes, with red on the left, green in the
    /// center, and blue on the right. This is the most common pixel geometry for LCD monitors.
    Rgb,

    /// Each pixel is composed of three vertical stripes, with blue on the left, green in the
    /// center, and red on the right.
    Bgr,
}
