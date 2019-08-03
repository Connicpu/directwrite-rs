#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents the density of a typeface, in terms of the lightness or
/// heaviness of the strokes. The enumerated values correspond to the
/// usWeightClass definition in the OpenType specification. The usWeightClass
/// represents an integer value between 1 and 999. Lower values indicate
/// lighter weights; higher values indicate heavier weights.
///
/// Weight differences are generally differentiated by an increased stroke or
/// thickness that is associated with a given character in a typeface, as
/// compared to a "normal" character from that same typeface. The following
/// illustration shows an example of Normal and UltraBold weights for the
/// Palatino Linotype typeface.
///
/// ![Font Weight Example](https://docs.microsoft.com/en-us/windows/desktop/api/dwrite/images/fontweight_for_palatino.png)
///
/// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
///
/// **Note**
/// Not all weights are available for all typefaces. When a weight is not available for a typeface, the closest matching weight is returned.
///
/// </div>
///
/// Font weight values less than 1 or greater than 999 are considered invalid,
/// and they are rejected by font API functions.
pub struct FontWeight(pub u32);

impl FontWeight {
    /// Predefined font weight : Thin (100).
    pub const THIN: FontWeight = FontWeight(100);

    /// Predefined font weight : Extra-light (200).
    pub const EXTRA_LIGHT: FontWeight = FontWeight(200);

    /// Predefined font weight : Ultra-light (200).
    pub const ULTRA_LIGHT: FontWeight = FontWeight(200);

    /// Predefined font weight : Light (300).
    pub const LIGHT: FontWeight = FontWeight(300);

    /// Predefined font weight : Semi-light (350).
    pub const SEMI_LIGHT: FontWeight = FontWeight(350);

    /// Predefined font weight : Normal (400).
    pub const NORMAL: FontWeight = FontWeight(400);

    /// Predefined font weight : Regular (400).
    pub const REGULAR: FontWeight = FontWeight(400);

    /// Predefined font weight : Medium (500).
    pub const MEDIUM: FontWeight = FontWeight(500);

    /// Predefined font weight : Demi-bold (600).
    pub const DEMI_BOLD: FontWeight = FontWeight(600);

    /// Predefined font weight : Semi-bold (600).
    pub const SEMI_BOLD: FontWeight = FontWeight(600);

    /// Predefined font weight : Bold (700).
    pub const BOLD: FontWeight = FontWeight(700);

    /// Predefined font weight : Extra-bold (800).
    pub const EXTRA_BOLD: FontWeight = FontWeight(800);

    /// Predefined font weight : Ultra-bold (800).
    pub const ULTRA_BOLD: FontWeight = FontWeight(800);

    /// Predefined font weight : Black (900).
    pub const BLACK: FontWeight = FontWeight(900);

    /// Predefined font weight : Heavy (900).
    pub const HEAVY: FontWeight = FontWeight(900);

    /// Predefined font weight : Extra-black (950).
    pub const EXTRA_BLACK: FontWeight = FontWeight(950);

    /// Predefined font weight : Ultra-black (950).
    pub const ULTRA_BLACK: FontWeight = FontWeight(950);
}
