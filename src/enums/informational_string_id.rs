#[auto_enum::auto_enum(u32, checked)]
/// The informational string enumeration which identifies a string embedded
/// in a font file.
pub enum InformationalStringId {
    /// Indicates the string containing the unspecified name ID.
    None,

    /// Indicates the string containing the copyright notice
    /// provided by the font.
    CopyrightNotice,

    /// Indicates the string containing a version number.
    VersionStrings,

    /// Indicates the string containing the trademark information
    /// provided by the font.
    Trademark,

    /// Indicates the string containing the name of the font manufacturer.
    Manufacturer,

    /// Indicates the string containing the name of the font designer.
    Designer,

    /// Indicates the string containing the URL of the font designer
    /// (with protocol, e.g., http://, ftp://).
    DesignerUrl,

    /// Indicates the string containing the description of the font.
    /// This may also contain revision information, usage recommendations,
    /// history, features, and so on.
    Description,

    /// Indicates the string containing the URL of the font vendor
    /// (with protocol, e.g., http://, ftp://). If a unique serial number
    /// is embedded in the URL, it can be used to register the font.
    FontVendorUrl,

    /// Indicates the string containing the description of how the font may
    /// be legally used, or different example scenarios for licensed use.
    LicenseDescription,

    /// Indicates the string containing the URL where additional licensing
    /// information can be found.
    LicenseInfoUrl,

    /// Indicates the string containing the GDI-compatible family name. Since
    /// GDI allows a maximum of four fonts per family, fonts in the same family
    /// may have different GDI-compatible family names (e.g., "Arial",
    /// "Arial Narrow", "Arial Black").
    Win32FamilyNames,

    /// Indicates the string containing a GDI-compatible subfamily name.
    Win32SubfamilyNames,

    /// [no description of this tag is provided]
    TypographicFamilyNames,

    /// [no description of this tag is provided]
    TypographicSubfamilyNames,

    /// Contains sample text for display in font lists. This can be the font
    /// name or any other text that the designer thinks is the best example
    /// to display the font in.
    SampleText,

    /// The full name of the font, like Arial Bold, from name id 4 in the
    /// name table
    FullName,

    /// The postscript name of the font, like GillSans-Bold, from name id 6
    /// in the name table.
    PostscriptName,

    /// The postscript CID findfont name, from name id 20 in the name table.
    PostscriptCidName,

    /// [no description of this tag is provided]
    WeightStretchStyleFamilyName,

    /// [no description of this tag is provided]
    DesignScriptLanguageTag,

    /// [no description of this tag is provided]
    SupportedScriptLanguageTag,

    /// Indicates the string containing the family name preferred by the designer.
    /// This enables font designers to group more than four fonts in a single
    /// family without losing compatibility with GDI. This name is typically only
    /// present if it differs from the GDI-compatible family name.
    PreferredFamilyNames,

    /// Indicates the string containing the subfamily name preferred by the designer.
    /// This name is typically only present if it differs from the GDI-compatible
    /// subfamily name.
    PreferredSubfamilyNames,

    /// [no description of this tag is provided]
    WwsFamilyName,
}
