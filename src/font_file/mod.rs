use enums::FontFaceType;
use enums::FontFileType;
use error::DWResult;
use factory::Factory;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFontFile;
use wio::com::ComPtr;

pub use self::builder::FontFileBuilder;

pub mod builder;

#[derive(Clone, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct FontFile {
    ptr: ComPtr<IDWriteFontFile>,
}

impl FontFile {
    pub fn create(factory: &Factory) -> FontFileBuilder {
        unsafe { FontFileBuilder::new(&*factory.get_raw()) }
    }

    /// Analyzes a file and returns whether it represents a font, and whether the font type is
    /// supported by the font system.
    pub fn analyze(&self) -> DWResult<Analysis> {
        unsafe {
            let mut ex = 0;
            let mut file = 0;
            let mut face = 0;
            let mut num = 0;

            let hr = self.ptr.Analyze(&mut ex, &mut file, &mut face, &mut num);

            if SUCCEEDED(hr) {
                Ok(Analysis {
                    exists: ex != 0,
                    file_type: file.into(),
                    face_type: face.into(),
                    num_faces: num,
                })
            } else {
                Err(hr.into())
            }
        }
    }
}

pub struct Analysis {
    pub exists: bool,
    pub file_type: UncheckedEnum<FontFileType>,
    pub face_type: UncheckedEnum<FontFaceType>,
    pub num_faces: u32,
}
