#[enum_flags(u32)]
/// Specifies algorithmic style simulations to be applied to the font face.
/// Bold and oblique simulations can be combined via bitwise OR operation.
pub enum FontSimulations {
    /// Indicates that algorithmic emboldening is applied to the font face.
    /// `BOLD` increases weight by applying a widening algorithm to the glyph
    /// outline. This may be used to simulate a bold weight where no designed
    /// bold weight is available.
    BOLD = 1,

    /// Indicates that algorithmic italicization is applied to the font face.
    /// `OBLIQUE` applies obliquing (shear) to the glyph outline. This may be
    /// used to simulate an oblique/italic style where no designed 
    /// blique/italic style is available.
    OBLIQUE = 2,
}
