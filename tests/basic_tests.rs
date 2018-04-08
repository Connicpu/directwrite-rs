extern crate directwrite;

use directwrite::Factory;
use directwrite::text_format::{self, TextFormat};
use directwrite::text_layout::{self, TextLayout};

#[test]
fn create_factory() {
    Factory::new().unwrap();
}

#[test]
fn create_format() {
    let factory = Factory::new().unwrap();

    factory
        .create::<TextFormat>(
            text_format::ParamBuilder::new()
                .family("Segoe UI")
                .size(16.0)
                .build()
                .unwrap(),
        )
        .unwrap();
}

#[test]
fn create_layout() {
    let factory = Factory::new().unwrap();

    let format = factory
        .create::<TextFormat>(
            text_format::ParamBuilder::new()
                .family("Segoe UI")
                .size(16.0)
                .build()
                .unwrap(),
        )
        .unwrap();

    factory
        .create::<TextLayout>(
            text_layout::ParamBuilder::new()
                .text("This is some test text!")
                .font(format)
                .width(300.0)
                .height(200.0)
                .build()
                .unwrap(),
        )
        .unwrap();
}
