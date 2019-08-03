#[auto_enum::auto_enum(u32, checked)]
/// How to apply number substitution on digits and related punctuation.
pub enum NumberSubstitutionMethod {
    /// Specifies that the substitution method should be determined based
    /// on LOCALE_IDIGITSUBSTITUTION value of the specified text culture.
    FromCulture,

    /// If the culture is Arabic or Farsi, specifies that the number shape
    /// depend on the context. Either traditional or nominal number shape
    /// are used depending on the nearest preceding strong character or (if
    /// there is none) the reading direction of the paragraph.
    Contextual,

    /// Specifies that code points 0x30-0x39 are always rendered as nominal numeral
    /// shapes (ones of the European number), i.e., no substitution is performed.
    None,

    /// Specifies that number are rendered using the national number shape
    /// as specified by the LOCALE_SNATIVEDIGITS value of the specified text culture.
    National,

    /// Specifies that number are rendered using the traditional shape
    /// for the specified culture. For most cultures, this is the same as
    /// NativeNational. However, NativeNational results in Latin number
    /// for some Arabic cultures, whereas this value results in Arabic
    /// number for all Arabic cultures.
    Traditional,
}
