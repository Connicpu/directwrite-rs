use enums::{FontFaceType, FontSimulations};
use error::DWriteError;
use font_face::FontFace;
use font_file::FontFile;

use std::ptr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{IDWriteFactory, IDWriteFontFace, IDWriteFontFile};
use wio::com::ComPtr;

#[must_use]
/// Facilitates construction of FontFace objects.
///
/// `font_face_type`, `files`, and `face_index` are all required.
/// `font_face_simulation_flags` defaults to NONE
pub struct FontFaceBuilder<'a, 'b> {
    factory: &'a IDWriteFactory,
    font_face_type: Option<FontFaceType>,
    files: Option<&'b [FontFile]>,
    face_index: Option<u32>,
    font_face_simulation_flags: FontSimulations,
}

impl<'a, 'b> FontFaceBuilder<'a, 'b> {
    pub(super) fn new(factory: &'a IDWriteFactory) -> FontFaceBuilder<'a, 'b> {
        FontFaceBuilder {
            factory,
            font_face_type: None,
            files: None,
            face_index: None,
            font_face_simulation_flags: FontSimulations::NONE,
        }
    }

    /// Finalizes construction of the FontFace.
    pub fn build(self) -> Result<FontFace, DWriteError> {
        unsafe {
            let font_face_type = self
                .font_face_type
                .expect("`font_face_type` must be specified");
            let files = self.files.expect("`files` must be specified");
            let face_index = self.face_index.expect("`face_index` must be specified");

            let mut ptr: *mut IDWriteFontFace = ptr::null_mut();
            let result = self.factory.CreateFontFace(
                font_face_type.to_u32(),
                files.len() as u32,
                // FontFile is a repr(C) wrapper of a single *mut IDWriteFontFile,
                // so a *const [FontFace] is safely castable to pointer to an array
                // of fontfile pointers.
                files.as_ptr() as *const *mut IDWriteFontFile,
                face_index,
                self.font_face_simulation_flags.0,
                &mut ptr,
            );

            if SUCCEEDED(result) {
                let ptr = ComPtr::from_raw(ptr);
                Ok(FontFace { ptr: ptr })
            } else {
                Err(From::from(result))
            }
        }
    }

    /// Specify the font face type
    pub fn with_font_face_type(mut self, font_face_type: FontFaceType) -> Self {
        self.font_face_type = Some(font_face_type);
        self
    }

    /// Specify the list of font files to be loaded
    pub fn with_files(mut self, files: &'b [FontFile]) -> Self {
        self.files = Some(files);
        self
    }

    /// Specify the index of the face to be loaded out of the collection defined by the files.
    pub fn with_face_index(mut self, face_index: u32) -> Self {
        self.face_index = Some(face_index);
        self
    }

    /// Specify the font simulations that should be applied.
    pub fn with_font_face_simulation_flags(
        mut self,
        font_face_simulation_flags: FontSimulations,
    ) -> Self {
        self.font_face_simulation_flags = font_face_simulation_flags;
        self
    }
}
