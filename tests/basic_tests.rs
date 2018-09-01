extern crate directwrite;

use directwrite::{Factory, TextFormat, TextLayout};
use directwrite::font_collection::FontCollection;

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
        .with_text("This is some test text!")
        .with_font(&font)
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

    let layout = TextLayout::create(&factory)
        .with_text(text)
        .with_font(&font)
        .with_width(300.0)
        .with_height(200.0)
        .build()
        .unwrap();

    layout.set_underline(true, ..text.len() as u32).unwrap();
    let (is_underlined, range) = layout.get_underline(0).unwrap();
    assert!(is_underlined);
    assert_eq!(range.start, 0);
    assert_eq!(range.length as usize, text.len());
}

#[test]
fn query_fonts(){
    let factory = Factory::new().unwrap();

    let collection = FontCollection::get_system_font_collection(&factory, true).unwrap();
    let count = collection.get_font_family_count().unwrap();

    for i in 0..count{
        let family = collection.get_font_family(i).unwrap();
        let family_name = family.get_family_name().unwrap();
        assert_eq!(collection.find_family_name(&family_name).unwrap().unwrap(), i);
    }
}