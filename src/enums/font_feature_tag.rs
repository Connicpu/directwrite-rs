#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct FontFeatureTag(pub u32);

#[cfg(target_endian = "little")]
macro_rules! feature_tag {
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr) => {
        FontFeatureTag(
            $v0 as u32 | (($v1 as u32) << 8) | (($v2 as u32) << 16) | (($v3 as u32) << 24),
        )
    };
}
#[cfg(not(target_endian = "little"))]
macro_rules! feature_tag {
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr) => {
        FontFeatureTag(
            $v3 as u32 | (($v2 as u32) << 8) | (($v1 as u32) << 16) | (($v0 as u32) << 24),
        )
    };
}

impl FontFeatureTag {
    #[inline]
    pub fn from_array(values: [u8; 4]) -> FontFeatureTag {
        feature_tag!(values[0], values[1], values[2], values[3])
    }

    #[inline]
    pub fn from_slice(b: &[u8]) -> FontFeatureTag {
        assert_eq!(b.len(), 4);
        FontFeatureTag::from_array([b[0], b[1], b[2], b[3]])
    }

    #[inline]
    pub fn from_str(s: &str) -> FontFeatureTag {
        FontFeatureTag::from_slice(s.as_bytes())
    }
}

impl From<u32> for FontFeatureTag {
    fn from(u: u32) -> FontFeatureTag {
        FontFeatureTag(u)
    }
}

impl From<[u8; 4]> for FontFeatureTag {
    fn from(a: [u8; 4]) -> FontFeatureTag {
        FontFeatureTag::from_array(a)
    }
}

impl<'a> From<&'a [u8]> for FontFeatureTag {
    fn from(b: &'a [u8]) -> FontFeatureTag {
        FontFeatureTag::from_slice(b)
    }
}

impl<'a> From<&'a str> for FontFeatureTag {
    fn from(s: &'a str) -> FontFeatureTag {
        FontFeatureTag::from_str(s)
    }
}


impl FontFeatureTag {
    /// Replaces figures separated by a slash with an alternative form.
    ///
    /// **Equivalent OpenType tag:** 'afrc'
    pub const ALTERNATIVE_FRACTIONS: FontFeatureTag = feature_tag!(b'a', b'f', b'r', b'c');

    /// Turns capital characters into petite capitals. It is generally used for words which would
    /// otherwise be set in all caps, such as acronyms, but which are desired in petite-cap form to
    /// avoid disrupting the flow of text. See the pcap feature description for notes on the
    /// relationship of caps, smallcaps and petite caps.
    ///
    /// **Equivalent OpenType tag:** 'c2pc'
    pub const PETITE_CAPITALS_FROM_CAPITALS: FontFeatureTag = feature_tag!(b'c', b'2', b'p', b'c');

    /// Turns capital characters into small capitals. It is generally used for words which would
    /// otherwise be set in all caps, such as acronyms, but which are desired in small-cap form to
    /// avoid disrupting the flow of text.
    ///
    /// **Equivalent OpenType tag:** 'c2sc'
    pub const SMALL_CAPITALS_FROM_CAPITALS: FontFeatureTag = feature_tag!(b'c', b'2', b's', b'c');

    /// In specified situations, replaces default glyphs with alternate forms which provide better
    /// joining behavior. Used in script typefaces which are designed to have some or all of their
    /// glyphs join.
    ///
    /// **Equivalent OpenType tag:** 'calt'
    pub const CONTEXTUAL_ALTERNATES: FontFeatureTag = feature_tag!(b'c', b'a', b'l', b't');

    /// Shifts various punctuation marks up to a position that works better with all-capital
    /// sequences or sets of lining figures; also changes oldstyle figures to lining figures. By
    /// default, glyphs in a text face are designed to work with lowercase characters. Some
    /// characters should be shifted vertically to fit the higher visual center of all-capital or
    /// lining text. Also, lining figures are the same height (or close to it) as capitals, and fit
    /// much better with all-capital text.
    ///
    /// **Equivalent OpenType tag:** 'case'
    pub const CASE_SENSITIVE_FORMS: FontFeatureTag = feature_tag!(b'c', b'a', b's', b'e');

    /// To minimize the number of glyph alternates, it is sometimes desired to decompose a character
    /// into two glyphs. Additionally, it may be preferable to compose two characters into a single
    /// glyph for better glyph processing. This feature permits such composition/decomposition. The
    /// feature should be processed as the first feature processed, and should be processed only
    /// when it is called.
    ///
    /// **Equivalent OpenType tag:** 'ccmp'
    pub const GLYPH_COMPOSITION_DECOMPOSITION: FontFeatureTag =
        feature_tag!(b'c', b'c', b'm', b'p');

    /// Replaces a sequence of glyphs with a single glyph which is preferred for typographic
    /// purposes. Unlike other ligature features, clig specifies the context in which the ligature
    /// is recommended. This capability is important in some script designs and for swash ligatures.
    ///
    /// **Equivalent OpenType tag:** 'clig'
    pub const CONTEXTUAL_LIGATURES: FontFeatureTag = feature_tag!(b'c', b'l', b'i', b'g');

    /// Globally adjusts inter-glyph spacing for all-capital text. Most typefaces contain capitals
    /// and lowercase characters, and the capitals are positioned to work with the lowercase. When
    /// capitals are used for words, they need more space between them for legibility and esthetics.
    /// This feature would not apply to monospaced designs. Of course the user may want to override
    /// this behavior in order to do more pronounced letterspacing for esthetic reasons.
    ///
    /// **Equivalent OpenType tag:** 'cpsp'
    pub const CAPITAL_SPACING: FontFeatureTag = feature_tag!(b'c', b'p', b's', b'p');

    /// Replaces default character glyphs with corresponding swash glyphs in a specified context.
    /// Note that there may be more than one swash alternate for a given character.
    ///
    /// **Equivalent OpenType tag:** 'cswh'
    pub const CONTEXTUAL_SWASH: FontFeatureTag = feature_tag!(b'c', b's', b'w', b'h');

    /// In cursive scripts like Arabic, this feature cursively positions adjacent glyphs.
    ///
    /// **Equivalent OpenType tag:** 'curs'
    pub const CURSIVE_POSITIONING: FontFeatureTag = feature_tag!(b'c', b'u', b'r', b's');

    /// The default.
    ///
    /// **Equivalent OpenType tag:** 'dflt'
    pub const DEFAULT: FontFeatureTag = feature_tag!(b'd', b'f', b'l', b't');

    /// Replaces a sequence of glyphs with a single glyph which is preferred for typographic
    /// purposes. This feature covers those ligatures which may be used for special effect, at the
    /// user's preference.
    ///
    /// **Equivalent OpenType tag:** 'dlig'
    pub const DISCRETIONARY_LIGATURES: FontFeatureTag = feature_tag!(b'd', b'l', b'i', b'g');

    /// Replaces standard forms in Japanese fonts with corresponding forms preferred by
    /// typographers. For example, a user would invoke this feature to replace kanji character
    /// U+5516 with U+555E.
    ///
    /// **Equivalent OpenType tag:** 'expt'
    pub const EXPERT_FORMS: FontFeatureTag = feature_tag!(b'e', b'x', b'p', b't');

    /// Replaces figures separated by a slash with 'common' (diagonal) fractions.
    ///
    /// **Equivalent OpenType tag:** 'frac'
    pub const FRACTIONS: FontFeatureTag = feature_tag!(b'f', b'r', b'a', b'c');

    /// Replaces glyphs set on other widths with glyphs set on full (usually em) widths. In a CJKV
    /// font, this may include "lower ASCII" Latin characters and various symbols. In a European
    /// font, this feature replaces proportionally-spaced glyphs with monospaced glyphs, which are
    /// generally set on widths of 0.6 em. For example, a user may invoke this feature in a Japanese
    /// font to get full monospaced Latin glyphs instead of the corresponding proportionally-spaced
    /// versions.
    ///
    /// **Equivalent OpenType tag:** 'fwid'
    pub const FULL_WIDTH: FontFeatureTag = feature_tag!(b'f', b'w', b'i', b'd');

    /// Produces the half forms of consonants in Indic scripts. For example, in Hindi (Devanagari
    /// script), the conjunct KKa, obtained by doubling the Ka, is denoted with a half form of Ka
    /// followed by the full form.
    ///
    /// **Equivalent OpenType tag:** 'half'
    pub const HALF_FORMS: FontFeatureTag = feature_tag!(b'h', b'a', b'l', b'f');

    /// Produces the halant forms of consonants in Indic scripts. For example, in Sanskrit
    /// (Devanagari script), syllable final consonants are frequently required in their halant form.
    ///
    /// **Equivalent OpenType tag:** 'haln'
    pub const HALANT_FORMS: FontFeatureTag = feature_tag!(b'h', b'a', b'l', b'n');

    /// Respaces glyphs designed to be set on full-em widths, fitting them onto half-em widths.
    /// This differs from hwid in that it does not substitute new glyphs.
    ///
    /// **Equivalent OpenType tag:** 'halt'
    pub const ALTERNATE_HALF_WIDTH: FontFeatureTag = feature_tag!(b'h', b'a', b'l', b't');

    /// Replaces the default (current) forms with the historical alternates. While some ligatures
    /// are also used for historical effect, this feature deals only with single characters. Some
    /// fonts include the historical forms as alternates, so they can be used for a 'period' effect.
    ///
    /// **Equivalent OpenType tag:** 'hist'
    pub const HISTORICAL_FORMS: FontFeatureTag = feature_tag!(b'h', b'i', b's', b't');

    /// Replaces standard kana with forms that have been specially designed for only horizontal
    /// writing. This is a typographic optimization for improved fit and more even color.
    ///
    /// **Equivalent OpenType tag:** 'hkna'
    pub const HORIZONTAL_KANA_ALTERNATES: FontFeatureTag = feature_tag!(b'h', b'k', b'n', b'a');

    /// Replaces the default (current) forms with the historical alternates. Some ligatures were in
    /// common use in the past, but appear anachronistic today. Some fonts include the historical
    /// forms as alternates, so they can be used for a 'period' effect.
    ///
    /// **Equivalent OpenType tag:** 'hlig'
    pub const HISTORICAL_LIGATURES: FontFeatureTag = feature_tag!(b'h', b'l', b'i', b'g');

    /// Replaces glyphs on proportional widths, or fixed widths other than half an em, with glyphs
    /// on half-em (en) widths. Many CJKV fonts have glyphs which are set on multiple widths; this
    /// feature selects the half-em version. There are various contexts in which this is the
    /// preferred behavior, including compatibility with older desktop documents.
    ///
    /// **Equivalent OpenType tag:** 'hwid'
    pub const HALF_WIDTH: FontFeatureTag = feature_tag!(b'h', b'w', b'i', b'd');

    /// Used to access the JIS X 0212-1990 glyphs for the cases when the JIS X 0213:2004 form is
    /// encoded. The JIS X 0212-1990 (aka, "Hojo Kanji") and JIS X 0213:2004 character sets overlap
    /// significantly. In some cases their prototypical glyphs differ. When building fonts that
    /// support both JIS X 0212-1990 and JIS X 0213:2004 (such as those supporting the Adobe-Japan
    /// 1-6 character collection), it is recommended that JIS X 0213:2004 forms be the preferred
    /// encoded form.
    ///
    /// **Equivalent OpenType tag:** 'hojo'
    pub const HOJO_KANJI_FORMS: FontFeatureTag = feature_tag!(b'h', b'o', b'j', b'o');

    /// The National Language Council (NLC) of Japan has defined new glyph shapes for a number of
    /// JIS characters, which were incorporated into JIS X 0213:2004 as new prototypical forms. The
    /// 'jp04' feature is A subset of the 'nlck' feature, and is used to access these prototypical
    /// glyphs in a manner that maintains the integrity of JIS X 0213:2004.
    ///
    /// **Equivalent OpenType tag:** 'jp04'
    pub const JIS04_FORMS: FontFeatureTag = feature_tag!(b'j', b'p', b'0', b'4');

    /// Replaces default (JIS90) Japanese glyphs with the corresponding forms from the JIS C
    /// 6226-1978 (JIS78) specification.
    ///
    /// **Equivalent OpenType tag:** 'jp78'
    pub const JIS78_FORMS: FontFeatureTag = feature_tag!(b'j', b'p', b'7', b'8');

    /// Replaces default (JIS90) Japanese glyphs with the corresponding forms from the JIS X
    /// 0208-1983 (JIS83) specification.
    ///
    /// **Equivalent OpenType tag:** 'jp83'
    pub const JIS83_FORMS: FontFeatureTag = feature_tag!(b'j', b'p', b'8', b'3');

    /// Replaces Japanese glyphs from the JIS78 or JIS83 specifications with the corresponding
    /// forms from the JIS X 0208-1990 (JIS90) specification.
    ///
    /// **Equivalent OpenType tag:** 'jp90'
    pub const JIS90_FORMS: FontFeatureTag = feature_tag!(b'j', b'p', b'9', b'0');

    /// Adjusts amount of space between glyphs, generally to provide optically consistent spacing
    /// between glyphs. Although a well-designed typeface has consistent inter-glyph spacing
    /// overall, some glyph combinations require adjustment for improved legibility. Besides
    /// standard adjustment in the horizontal direction, this feature can supply size-dependent
    /// kerning data via device tables, "cross-stream" kerning in the Y text direction, and
    /// adjustment of glyph placement independent of the advance adjustment. Note that this feature
    /// may apply to runs of more than two glyphs, and would not be used in monospaced fonts. Also
    /// note that this feature does not apply to text set vertically.
    ///
    /// **Equivalent OpenType tag:** 'kern'
    pub const KERNING: FontFeatureTag = feature_tag!(b'k', b'e', b'r', b'n');

    /// Replaces a sequence of glyphs with a single glyph which is preferred for typographic
    /// purposes. This feature covers the ligatures which the designer/manufacturer judges should
    /// be used in normal conditions.
    ///
    /// **Equivalent OpenType tag:** 'liga'
    pub const STANDARD_LIGATURES: FontFeatureTag = feature_tag!(b'l', b'i', b'g', b'a');

    /// Changes selected figures from oldstyle to the default lining form. For example, a user may
    /// invoke this feature in order to get lining figures, which fit better with all-capital text.
    /// This feature overrides results of the Oldstyle Figures feature (onum).
    ///
    /// **Equivalent OpenType tag:** 'lnum'
    pub const LINING_FIGURES: FontFeatureTag = feature_tag!(b'l', b'n', b'u', b'm');

    /// Enables localized forms of glyphs to be substituted for default forms. Many scripts used to
    /// write multiple languages over wide geographical areas have developed localized variant forms
    /// of specific letters, which are used by individual literary communities. For example, a
    /// number of letters in the Bulgarian and Serbian alphabets have forms distinct from their
    /// Russian counterparts and from each other. In some cases the localized form differs only
    /// subtly from the script 'norm', in others the forms are radically distinct.
    ///
    /// **Equivalent OpenType tag:** 'locl'
    pub const LOCALIZED_FORMS: FontFeatureTag = feature_tag!(b'l', b'o', b'c', b'l');

    /// Positions mark glyphs with respect to base glyphs. For example, in Arabic script positioning
    /// the Hamza above the Yeh.
    ///
    /// **Equivalent OpenType tag:** 'mark'
    pub const MARK_POSITIONING: FontFeatureTag = feature_tag!(b'm', b'a', b'r', b'k');

    /// Replaces standard typographic forms of Greek glyphs with corresponding forms commonly used
    /// in mathematical notation (which are a subset of the Greek alphabet).
    ///
    /// **Equivalent OpenType tag:** 'mgrk'
    pub const MATHEMATICAL_GREEK: FontFeatureTag = feature_tag!(b'm', b'g', b'r', b'k');

    /// Positions marks with respect to other marks. Required in various non-Latin scripts like
    /// Arabic. For example, in Arabic, the ligaturised mark Ha with Hamza above it can also be
    /// obtained by positioning these marks relative to one another.
    ///
    /// **Equivalent OpenType tag:** 'mkmk'
    pub const MARK_TO_MARK_POSITIONING: FontFeatureTag = feature_tag!(b'm', b'k', b'm', b'k');

    /// Replaces default glyphs with various notational forms (such as glyphs placed in open or
    /// solid circles, squares, parentheses, diamonds or rounded boxes). In some cases an annotation
    /// form may already be present, but the user may want a different one.
    ///
    /// **Equivalent OpenType tag:** 'nalt'
    pub const ALTERNATE_ANNOTATION_FORMS: FontFeatureTag = feature_tag!(b'n', b'a', b'l', b't');

    /// Used to access glyphs made from glyph shapes defined by the National Language Council (NLC)
    /// of Japan for a number of JIS characters in 2000.
    ///
    /// **Equivalent OpenType tag:** 'nlck'
    pub const NLC_KANJI_FORMS: FontFeatureTag = feature_tag!(b'n', b'l', b'c', b'k');

    /// Changes selected figures from the default lining style to oldstyle form. For example, a user
    /// may invoke this feature to get oldstyle figures, which fit better into the flow of normal
    /// upper- and lowercase text. This feature overrides results of the Lining Figures feature
    /// (lnum).
    ///
    /// **Equivalent OpenType tag:** 'onum'
    pub const OLD_STYLE_FIGURES: FontFeatureTag = feature_tag!(b'o', b'n', b'u', b'm');

    /// Replaces default alphabetic glyphs with the corresponding ordinal forms for use after
    /// figures. One exception to the follows-a-figure rule is the numero character (U+2116), which
    /// is actually a ligature substitution, but is best accessed through this feature.
    ///
    /// **Equivalent OpenType tag:** 'ordn'
    pub const ORDINALS: FontFeatureTag = feature_tag!(b'o', b'r', b'd', b'n');

    /// Respaces glyphs designed to be set on full-em widths, fitting them onto individual (more or
    /// less proportional) horizontal widths. This differs from pwid in that it does not substitute
    /// new glyphs (GPOS, not GSUB feature). The user may prefer the monospaced form, or may simply
    /// want to ensure that the glyph is well-fit and not rotated in vertical setting (Latin forms
    /// designed for proportional spacing would be rotated).
    ///
    /// **Equivalent OpenType tag:** 'palt'
    pub const PROPORTIONAL_ALTERNATE_WIDTH: FontFeatureTag = feature_tag!(b'p', b'a', b'l', b't');

    /// Turns lowercase characters into petite capitals. Forms related to petite capitals, such as
    /// specially designed figures, may be included. Some fonts contain an additional size of
    /// capital letters, shorter than the regular smallcaps and it is referred to as petite caps.
    /// Such forms are most likely to be found in designs with a small lowercase x-height, where
    /// they better harmonise with lowercase text than the taller smallcaps (for examples of petite
    /// caps, see the Emigre type families Mrs Eaves and Filosofia).
    ///
    /// **Equivalent OpenType tag:** 'pcap'
    pub const PETITE_CAPITALS: FontFeatureTag = feature_tag!(b'p', b'c', b'a', b'p');

    /// Replaces figure glyphs set on uniform (tabular) widths with corresponding glyphs set on
    /// glyph-specific (proportional) widths. Tabular widths will generally be the default, but
    /// this cannot be safely assumed. Of course this feature would not be present in monospaced
    /// designs.
    ///
    /// **Equivalent OpenType tag:** 'pnum'
    pub const PROPORTIONAL_FIGURES: FontFeatureTag = feature_tag!(b'p', b'n', b'u', b'm');

    /// Replaces glyphs set on uniform widths (typically full or half-em) with proportionally spaced
    /// glyphs. The proportional variants are often used for the Latin characters in CJKV fonts, but
    /// may also be used for Kana in Japanese fonts.
    ///
    /// **Equivalent OpenType tag:** 'pwid'
    pub const PROPORTIONAL_WIDTHS: FontFeatureTag = feature_tag!(b'p', b'w', b'i', b'd');

    /// Replaces glyphs on other widths with glyphs set on widths of one quarter of an em (half an
    /// en). The characters involved are normally figures and some forms of punctuation.
    ///
    /// **Equivalent OpenType tag:** 'qwid'
    pub const QUARTER_WIDTHS: FontFeatureTag = feature_tag!(b'q', b'w', b'i', b'd');

    /// Replaces a sequence of glyphs with a single glyph which is preferred for typographic
    /// purposes. This feature covers those ligatures, which the script determines as required to
    /// be used in normal conditions. This feature is important for some scripts to ensure correct
    /// glyph formation.
    ///
    /// **Equivalent OpenType tag:** 'rlig'
    pub const REQUIRED_LIGATURES: FontFeatureTag = feature_tag!(b'r', b'l', b'i', b'g');

    /// Identifies glyphs in the font which have been designed for "ruby", from the old typesetting
    /// term for four-point-sized type. Japanese typesetting often uses smaller kana glyphs,
    /// generally in superscripted form, to clarify the meaning of kanji which may be unfamiliar
    /// to the reader.
    ///
    /// **Equivalent OpenType tag:** 'ruby'
    pub const RUBY_NOTATION_FORMS: FontFeatureTag = feature_tag!(b'r', b'u', b'b', b'y');

    /// Replaces the default forms with the stylistic alternates. Many fonts contain alternate
    /// glyph designs for a purely esthetic effect; these don't always fit into a clear category
    /// like swash or historical. As in the case of swash glyphs, there may be more than one
    /// alternate form.
    ///
    /// **Equivalent OpenType tag:** 'salt'
    pub const STYLISTIC_ALTERNATES: FontFeatureTag = feature_tag!(b's', b'a', b'l', b't');

    /// Replaces lining or oldstyle figures with inferior figures (smaller glyphs which sit lower
    /// than the standard baseline, primarily for chemical or mathematical notation). May also
    /// replace lowercase characters with alphabetic inferiors.
    ///
    /// **Equivalent OpenType tag:** 'sinf'
    pub const SCIENTIFIC_INFERIORS: FontFeatureTag = feature_tag!(b's', b'i', b'n', b'f');

    /// Turns lowercase characters into small capitals. This corresponds to the common SC font
    /// layout. It is generally used for display lines set in Large & small caps, such as titles.
    /// Forms related to small capitals, such as oldstyle figures, may be included.
    ///
    /// **Equivalent OpenType tag:** 'smcp'
    pub const SMALL_CAPITALS: FontFeatureTag = feature_tag!(b's', b'm', b'c', b'p');

    /// Replaces 'traditional' Chinese or Japanese forms with the corresponding 'simplified' forms.
    ///
    /// **Equivalent OpenType tag:** 'smpl'
    pub const SIMPLIFIED_FORMS: FontFeatureTag = feature_tag!(b's', b'm', b'p', b'l');

    /// In addition to, or instead of, stylistic alternatives of individual glyphs (see 'salt'
    /// feature), some fonts may contain sets of stylistic variant glyphs corresponding to portions
    /// of the character set, such as multiple variants for lowercase letters in a Latin font.
    /// Glyphs in stylistic sets may be designed to harmonise visually, interract in particular
    /// ways, or otherwise work together. Examples of fonts including stylistic sets are Zapfino
    /// Linotype and Adobe's Poetica. Individual features numbered sequentially with the tag name
    /// convention 'ss01' 'ss02' 'ss03' . 'ss20' provide a mechanism for glyphs in these sets to be
    /// associated via GSUB lookup indexes to default forms and to each other, and for users to
    /// select from available stylistic sets
    ///
    /// **Equivalent OpenType tag:** 'ss01'
    pub const STYLISTIC_SET_1: FontFeatureTag = feature_tag!(b's', b's', b'0', b'1');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss02'
    pub const STYLISTIC_SET_2: FontFeatureTag = feature_tag!(b's', b's', b'0', b'2');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss03'
    pub const STYLISTIC_SET_3: FontFeatureTag = feature_tag!(b's', b's', b'0', b'3');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss04'
    pub const STYLISTIC_SET_4: FontFeatureTag = feature_tag!(b's', b's', b'0', b'4');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss05'
    pub const STYLISTIC_SET_5: FontFeatureTag = feature_tag!(b's', b's', b'0', b'5');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss06'
    pub const STYLISTIC_SET_6: FontFeatureTag = feature_tag!(b's', b's', b'0', b'6');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss07'
    pub const STYLISTIC_SET_7: FontFeatureTag = feature_tag!(b's', b's', b'0', b'7');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss08'
    pub const STYLISTIC_SET_8: FontFeatureTag = feature_tag!(b's', b's', b'0', b'8');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss09'
    pub const STYLISTIC_SET_9: FontFeatureTag = feature_tag!(b's', b's', b'0', b'9');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss10'
    pub const STYLISTIC_SET_10: FontFeatureTag = feature_tag!(b's', b's', b'1', b'0');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss11'
    pub const STYLISTIC_SET_11: FontFeatureTag = feature_tag!(b's', b's', b'1', b'1');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss12'
    pub const STYLISTIC_SET_12: FontFeatureTag = feature_tag!(b's', b's', b'1', b'2');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss13'
    pub const STYLISTIC_SET_13: FontFeatureTag = feature_tag!(b's', b's', b'1', b'3');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss14'
    pub const STYLISTIC_SET_14: FontFeatureTag = feature_tag!(b's', b's', b'1', b'4');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss15'
    pub const STYLISTIC_SET_15: FontFeatureTag = feature_tag!(b's', b's', b'1', b'5');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss16'
    pub const STYLISTIC_SET_16: FontFeatureTag = feature_tag!(b's', b's', b'1', b'6');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss17'
    pub const STYLISTIC_SET_17: FontFeatureTag = feature_tag!(b's', b's', b'1', b'7');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss18'
    pub const STYLISTIC_SET_18: FontFeatureTag = feature_tag!(b's', b's', b'1', b'8');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss19'
    pub const STYLISTIC_SET_19: FontFeatureTag = feature_tag!(b's', b's', b'1', b'9');

    /// See the description for [`STYLISTIC_SET_1`](#associatedconstant.STYLISTIC_SET_1)
    ///
    /// **Equivalent OpenType tag:** 'ss20'
    pub const STYLISTIC_SET_20: FontFeatureTag = feature_tag!(b's', b's', b'2', b'0');

    /// May replace a default glyph with a subscript glyph, or it may combine a glyph substitution
    /// with positioning adjustments for proper placement.
    ///
    /// **Equivalent OpenType tag:** 'subs'
    pub const SUBSCRIPT: FontFeatureTag = feature_tag!(b's', b'u', b'b', b's');

    /// Replaces lining or oldstyle figures with superior figures (primarily for footnote
    /// indication), and replaces lowercase letters with superior letters (primarily for
    /// abbreviated French titles).
    ///
    /// **Equivalent OpenType tag:** 'sups'
    pub const SUPERSCRIPT: FontFeatureTag = feature_tag!(b's', b'u', b'p', b's');

    /// Replaces default character glyphs with corresponding swash glyphs. Note that there may be
    /// more than one swash alternate for a given character.
    ///
    /// **Equivalent OpenType tag:** 'swsh'
    pub const SWASH: FontFeatureTag = feature_tag!(b's', b'w', b's', b'h');

    /// Replaces the default glyphs with corresponding forms designed specifically for
    /// titling. These may be all-capital and/or larger on the body, and adjusted for
    /// viewing at larger sizes.
    ///
    /// **Equivalent OpenType tag:** 'titl'
    pub const TITLING: FontFeatureTag = feature_tag!(b't', b'i', b't', b'l');

    /// Replaces 'simplified' Japanese kanji forms with the corresponding 'traditional' forms.
    /// This is equivalent to the Traditional Forms feature, but explicitly limited to the
    /// traditional forms considered proper for use in personal names (as many as 205 glyphs in
    /// some fonts).
    ///
    /// **Equivalent OpenType tag:** 'tnam'
    pub const TRADITIONAL_NAME_FORMS: FontFeatureTag = feature_tag!(b't', b'n', b'a', b'm');

    /// Replaces figure glyphs set on proportional widths with corresponding glyphs set on
    /// uniform (tabular) widths. Tabular widths will generally be the default, but this cannot
    /// be safely assumed. Of course this feature would not be present in monospaced designs.
    ///
    /// **Equivalent OpenType tag:** 'tnum'
    pub const TABULAR_FIGURES: FontFeatureTag = feature_tag!(b't', b'n', b'u', b'm');

    /// Replaces 'simplified' Chinese hanzi or Japanese kanji forms with the corresponding
    /// 'traditional' forms.
    ///
    /// **Equivalent OpenType tag:** 'trad'
    pub const TRADITIONAL_FORMS: FontFeatureTag = feature_tag!(b't', b'r', b'a', b'd');

    /// Replaces glyphs on other widths with glyphs set on widths of one third of an em. The
    /// characters involved are normally figures and some forms of punctuation.
    ///
    /// **Equivalent OpenType tag:** 'twid'
    pub const THIRD_WIDTHS: FontFeatureTag = feature_tag!(b't', b'w', b'i', b'd');

    /// Maps upper- and lowercase letters to a mixed set of lowercase and small capital forms,
    /// resulting in a single case alphabet (for an example of unicase, see the Emigre type
    /// family Filosofia). The letters substituted may vary from font to font, as appropriate
    /// to the design. If aligning to the x-height, smallcap glyphs may be substituted, or
    /// specially designed unicase forms might be used. Substitutions might also include
    /// specially designed figures.
    ///
    /// **Equivalent OpenType tag:** 'unic'
    pub const UNICASE: FontFeatureTag = feature_tag!(b'u', b'n', b'i', b'c');

    /// Indicates that the font is displayed vertically.
    ///
    /// **Equivalent OpenType tag:** 'vert'
    pub const VERTICAL_WRITING: FontFeatureTag = feature_tag!(b'v', b'e', b'r', b't');

    /// Replaces normal figures with figures adjusted for vertical display.
    ///
    /// **Equivalent OpenType tag:** 'vrt2'
    pub const VERTICAL_ALTERNATES_AND_ROTATION: FontFeatureTag =
        feature_tag!(b'v', b'r', b't', b'2');

    /// Allows the user to change from the default 0 to a slashed form. Some fonts contain both a
    /// default form of zero, and an alternative form which uses a diagonal slash through the
    /// counter. Especially in condensed designs, it can be difficult to distinguish between 0
    /// and O (zero and capital O) in any situation where capitals and lining figures may be
    /// arbitrarily mixed.
    ///
    /// **Equivalent OpenType tag:** 'zero'
    pub const SLASHED_ZERO: FontFeatureTag = feature_tag!(b'z', b'e', b'r', b'o');
}
