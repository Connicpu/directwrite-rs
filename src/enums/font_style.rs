#[auto_enum::auto_enum(u32, checked)]
/// Represents the style of a font face as normal, italic, or oblique.
/// 
/// Three terms categorize the slant of a font: normal, italic, and oblique.
/// 
/// <table><tr>
/// <td>Font style</td>
/// <td style="text-align: center">Description</td>
/// </tr><tr>
/// <td>Normal</td>
/// <td>The characters in a normal, or roman, font are upright.</td>
/// </tr><tr>
/// <td>Italic</td>
/// <td>The characters in an italic font are truly slanted and appear as they
/// were designed.</td>
/// </tr><tr>
/// <td>Oblique</td>
/// <td>The characters in an oblique font are artificially slanted.</td>
/// </tr></table>
/// 
/// For Oblique, the slant is achieved by performing a shear transformation on
/// the characters from a normal font. When a true italic font is not available
/// on a computer or printer, an oblique style can be generated from the normal
/// font and used to simulate an italic font.
/// 
/// The following illustration shows the normal, italic, and oblique font
/// styles for the Palatino Linotype font. Notice how the italic font style
/// has a more flowing and visually appealing appearance than the oblique font
/// style, which is simply created by skewing the normal font style version
/// of the text.
/// 
/// ![Example of Normal, Italic, and Oblique fonts](https://docs.microsoft.com/en-us/windows/desktop/api/dwrite/images/fontstyle_for_palatino.png)
/// 
/// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
/// 
/// **Note**
/// Values other than the ones defined in the enumeration are considered to be
/// invalid, and they are rejected by font API functions.
/// 
/// </div>
pub enum FontStyle {
    /// The characters in a normal, or roman, font are upright. 
    Normal,

    /// The characters in an oblique font are artificially slanted.
    Oblique,

    /// The characters in an italic font are truly slanted and appear as they were designed.
    Italic,
}
