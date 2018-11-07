use winapi::ctypes::c_void;
use winapi::um::dwrite::IDWriteFontFace;

pub struct FontTable<'a> {
    pub(super) face: &'a IDWriteFontFace,
    pub(super) context: *mut c_void,
    pub(super) data: &'a [u8],
}

impl<'a> FontTable<'a> {
    #[inline]
    pub fn data(&self) -> &[u8] {
        self.data
    }
}

impl<'a> AsRef<[u8]> for FontTable<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.data
    }
}

impl<'a> Drop for FontTable<'a> {
    fn drop(&mut self) {
        unsafe {
            self.face.ReleaseFontTable(self.context);
        }
    }
}
