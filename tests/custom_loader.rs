extern crate directwrite;
extern crate winapi;

use directwrite::error::DWResult;
use directwrite::font_collection::loader::FontCollectionLoader;
use directwrite::font_file::loader::{FileLoaderHandle, FontFileLoader, StaticDataStream};
use directwrite::{Factory, FontCollection, FontFile, TextFormat, TextLayout};
use winapi::shared::winerror::{ERROR_NOT_FOUND, HRESULT_FROM_WIN32};

const OPENSANS_REGULAR: StaticDataStream = StaticDataStream {
    // Sunday, November 11, 2018 18:30:45
    last_modified: 636775578456076107,
    data: include_bytes!("test_fonts/OpenSans-Regular.ttf"),
};
const FIRACODE_REGULAR: StaticDataStream = StaticDataStream {
    // Sunday, November 11, 2018 18:30:45
    last_modified: 636775578456076107,
    data: include_bytes!("test_fonts/FiraCode-Regular.ttf"),
};
const FIRACODE_BOLD: StaticDataStream = StaticDataStream {
    // Sunday, November 11, 2018 18:30:45
    last_modified: 636775578456076107,
    data: include_bytes!("test_fonts/FiraCode-Bold.ttf"),
};
const FIRACODE_LIGHT: StaticDataStream = StaticDataStream {
    // Sunday, November 11, 2018 18:30:45
    last_modified: 636775578456076107,
    data: include_bytes!("test_fonts/FiraCode-Light.ttf"),
};
const FIRACODE_MEDIUM: StaticDataStream = StaticDataStream {
    // Sunday, November 11, 2018 18:30:45
    last_modified: 636775578456076107,
    data: include_bytes!("test_fonts/FiraCode-Medium.ttf"),
};

pub struct DataFileLoader;
impl FontFileLoader for DataFileLoader {
    type Key = str;
    type Stream = StaticDataStream;

    fn create_stream(&self, key: &str) -> DWResult<StaticDataStream> {
        match key {
            "OpenSans-Regular" => Ok(OPENSANS_REGULAR),
            "FiraCode-Regular" => Ok(FIRACODE_REGULAR),
            "FiraCode-Bold" => Ok(FIRACODE_BOLD),
            "FiraCode-Medium" => Ok(FIRACODE_MEDIUM),
            "FiraCode-Light" => Ok(FIRACODE_LIGHT),
            _ => Err(HRESULT_FROM_WIN32(ERROR_NOT_FOUND).into()),
        }
    }
}

pub struct DataCollectionLoader(FileLoaderHandle<str>);
impl FontCollectionLoader for DataCollectionLoader {
    type Key = ();
    type Iter = Box<Iterator<Item = DWResult<FontFile>>>;

    fn get_iterator(&self, factory: &Factory, _key: &()) -> DWResult<Self::Iter> {
        static FONTS: &[&str] = &[
            "OpenSans-Regular",

            "FiraCode-Regular",
            "FiraCode-Bold",
            "FiraCode-Medium",
            "FiraCode-Light",
        ];

        let factory = factory.clone();
        let loader = self.0.clone();
        Ok(Box::new(FONTS.iter().map(move |font| {
            FontFile::create(&factory)
                .with_loader(&loader)
                .with_key(font)
                .build()
        })))
    }
}

#[test]
fn load_custom_font() {
    let factory = Factory::new().unwrap();
    let file_loader = DataFileLoader.register(&factory).unwrap();
    let collection_loader = DataCollectionLoader(file_loader)
        .register(&factory)
        .unwrap();

    let collection = FontCollection::create(&factory)
        .with_loader(&collection_loader)
        .with_key(&())
        .build()
        .unwrap();

    assert_eq!(collection.find_family_by_name("Open Sans"), Some(0));
    assert_eq!(collection.find_family_by_name("Fira Code"), Some(1));

    let opensans = TextFormat::create(&factory)
        .with_collection(&collection)
        .with_family("Open Sans")
        .with_size(12.0)
        .build()
        .unwrap();

    assert_eq!(opensans.font_collection().as_ref(), Some(&collection));
    assert_eq!(opensans.font_family_name().as_ref().map(|s| &s[..]), Some("Open Sans"));

    let firacode = TextFormat::create(&factory)
        .with_collection(&collection)
        .with_family("Fira Code")
        .with_size(12.0)
        .build()
        .unwrap();

    assert_eq!(firacode.font_collection().as_ref(), Some(&collection));
    assert_eq!(firacode.font_family_name().as_ref().map(|s| &s[..]), Some("Fira Code"));

    fn test_layout(factory: &Factory, format: &TextFormat, text: &str) {
        TextLayout::create(&factory)
            .with_format(&format)
            .with_str(text)
            .with_size(1200.0, 500.0)
            .with_centered(true)
            .build()
            .unwrap();
    }

    test_layout(&factory, &opensans, "Lay this out in OpenSans ;3");
    test_layout(&factory, &firacode, "Lay this out in Fira Code O:");
}
