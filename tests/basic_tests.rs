extern crate directwrite;

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
