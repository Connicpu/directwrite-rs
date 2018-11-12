extern crate directwrite;

use directwrite::enums::{FontStretch, FontStyle, FontWeight, InformationalStringId};
use directwrite::font_collection::FontCollection;
use directwrite::Factory;

fn main() {
    let factory = Factory::new().unwrap();

    let collection = FontCollection::system_font_collection(&factory, false).unwrap();
    let segoe_id = collection.find_family_by_name("Segoe UI").unwrap();
    let segoe = collection.family(segoe_id).unwrap();
    let segoe_fonts = segoe
        .matching_fonts(FontWeight::NORMAL, FontStretch::Normal, FontStyle::Normal)
        .unwrap();
    for font in segoe_fonts.all_fonts() {
        println!("Font name: {:#?}", font.informational_strings(InformationalStringId::FullName));
        println!("Face name: {:#?}", font.face_name());
    }
}
