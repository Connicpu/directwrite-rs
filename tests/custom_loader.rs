extern crate directwrite;
extern crate winapi;

use directwrite::error::DWResult;
use directwrite::font_collection::loader::FontCollectionLoader;
use directwrite::font_file::loader::{FileLoaderHandle, FontFileLoader, StaticDataStream};
use directwrite::{Factory, FontCollection, FontFile, TextFormat, TextLayout};
use winapi::shared::winerror::{ERROR_NOT_FOUND, HRESULT_FROM_WIN32};

pub struct DataFileLoader;
impl FontFileLoader for DataFileLoader {
    type Key = str;
    type Stream = StaticDataStream;

    fn create_stream(&self, key: &str) -> DWResult<StaticDataStream> {
        static OPENSANS_REGULAR: &[u8] = include_bytes!("test_fonts/OpenSans-Regular.ttf");

        match key {
            "OpenSans-Regular" => Ok(StaticDataStream {
                // Sunday, November 11, 2018 18:30:45
                last_modified: 636775578456076107,
                data: OPENSANS_REGULAR,
            }),
            _ => Err(HRESULT_FROM_WIN32(ERROR_NOT_FOUND).into()),
        }
    }
}

pub struct DataCollectionLoader(FileLoaderHandle<str>);
impl FontCollectionLoader for DataCollectionLoader {
    type Key = ();
    type Iter = Box<Iterator<Item = DWResult<FontFile>>>;

    fn get_iterator(&self, factory: &Factory, _key: &()) -> DWResult<Self::Iter> {
        static FONTS: &[&str] = &["OpenSans-Regular"];

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

    let format = TextFormat::create(&factory)
        .with_collection(&collection)
        .with_family("OpenSans")
        .with_size(12.0)
        .build()
        .unwrap();

    let layout = TextLayout::create(&factory)
        .with_format(&format)
        .with_str("It works! O:")
        .with_size(1200.0, 500.0)
        .with_centered(true)
        .build()
        .unwrap();

    let _clusters = layout.cluster_metrics();
}
