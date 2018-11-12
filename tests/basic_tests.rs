extern crate directwrite;

use directwrite::enums::*;
use directwrite::font_collection::FontCollection;
use directwrite::font_face::FontFace;
use directwrite::font_file::FontFile;
use directwrite::{Factory, TextFormat, TextLayout};

#[test]
fn create_factory() {
    Factory::new().unwrap();
}

#[test]
fn create_format() {
    let factory = Factory::new().unwrap();

    TextFormat::create(&factory)
        .with_family("Segoe UI")
        .with_size(16.0)
        .build()
        .unwrap();
}

#[test]
fn create_layout() {
    let factory = Factory::new().unwrap();

    let font = TextFormat::create(&factory)
        .with_family("Segoe UI")
        .with_size(16.0)
        .build()
        .unwrap();

    TextLayout::create(&factory)
        .with_str("This is some test text!")
        .with_format(&font)
        .with_width(300.0)
        .with_height(200.0)
        .build()
        .unwrap();
}

#[test]
fn set_attributes() {
    let factory = Factory::new().unwrap();

    let font = TextFormat::create(&factory)
        .with_family("Segoe UI")
        .with_size(16.0)
        .build()
        .unwrap();

    let text = "This is some test text!";

    let mut layout = TextLayout::create(&factory)
        .with_str(text)
        .with_format(&font)
        .with_width(300.0)
        .with_height(200.0)
        .build()
        .unwrap();

    layout.set_underline(true, ..text.len() as u32).unwrap();

    let (is_underlined, range) = layout.underline(0).unwrap();
    assert!(is_underlined);
    assert_eq!(range.start, 0);
    assert_eq!(range.length as usize, text.len());

    layout.set_underline(false, 0).unwrap();

    let (is_underlined, range) = layout.underline(0).unwrap();
    assert!(!is_underlined);
    assert_eq!(range.start, 0);
    assert_eq!(range.length, 1);
    
    let (is_underlined, range) = layout.underline(1).unwrap();
    assert!(is_underlined);
    assert_eq!(range.start, 1);
    assert_eq!(range.length as usize, text.len() - 1);
}

#[test]
fn query_fonts() {
    let factory = Factory::new().unwrap();

    let collection = FontCollection::system_font_collection(&factory, true).unwrap();
    let count = collection.family_count();
    assert!(count > 0);

    for i in 0..count {
        let family = collection.family(i).unwrap();
        let family_name = family
            .family_name()
            .unwrap()
            .locale_by_name("en-US")
            .unwrap()
            .string();
        assert_eq!(collection.find_family_by_name(&family_name).unwrap(), i);
    }

    let ffile = FontFile::create(&factory)
        .with_file_path("tests/test_fonts/OpenSans-Regular.ttf")
        .build()
        .unwrap();

    let fface = FontFace::create(&factory)
        .with_files(&[ffile])
        .with_font_face_type(FontFaceType::TrueType)
        .with_face_index(0)
        .with_font_face_simulation_flags(FontSimulations::NONE)
        .build()
        .unwrap();

    let fmetrics = fface.metrics();
    assert_eq!(fmetrics.design_units_per_em, 2048);
    assert_eq!(fmetrics.descent, 600);
    let gmetrics = fface.design_glyph_metrics(&[0, 25, 96], true).unwrap();
    assert_eq!(gmetrics[0].advance_width, 1229);
    assert_eq!(gmetrics[1].advance_width, 1171);
}
