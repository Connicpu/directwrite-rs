extern crate directwrite;

use directwrite::{Factory, TextFormat, TextLayout};

fn main() {
    let factory = Factory::new().unwrap();
    let format = TextFormat::create(&factory)
        .with_family("Segoe UI Emoji")
        .with_size(16.0)
        .build()
        .unwrap();

    let text = "🦃🎃🍆📌";
    let mut layout = TextLayout::create(&factory)
        .with_str(text)
        .with_format(&format)
        .with_size(4096.0, 4096.0)
        .build()
        .unwrap();

    layout.set_underline(true, 0..2).unwrap();

    let cluster_metrics = layout.cluster_metrics();
    println!("{:#?}", cluster_metrics);
}
