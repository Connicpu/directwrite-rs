//! Representation of a string that may have multiple separate representations in different locales.

use std::ffi::{OsStr, OsString};
use std::fmt;
use std::os::windows::ffi::{OsStrExt, OsStringExt};

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteLocalizedStrings;
use wio::com::ComPtr;

#[derive(Clone, ComWrapper)]
#[com(send, sync)]
#[repr(transparent)]
/// Represents a collection of strings indexed by locale name.
pub struct LocalizedStrings {
    ptr: ComPtr<IDWriteLocalizedStrings>,
}

impl LocalizedStrings {
    /// Gets the number of locales which have values for this string.
    pub fn count(&self) -> u32 {
        unsafe { self.ptr.GetCount() }
    }

    /// Gets all localized strings in this collection.
    pub fn all_strings<'a>(&'a self) -> impl Iterator<Item = LocalizedString<'a>> + 'a {
        (0..self.count()).map(move |i| self.unchecked_locale(i))
    }

    /// Get the localized string value at the specified index.
    pub fn get(&self, index: u32) -> Option<LocalizedString> {
        if index < self.count() {
            Some(self.unchecked_locale(index))
        } else {
            None
        }
    }

    /// Get the localized string value with the specified locale name.
    pub fn get_by_name(&self, name: impl AsRef<OsStr>) -> Option<LocalizedString> {
        let name: Vec<u16> = name.as_ref().encode_wide().chain(Some(0)).collect();

        let mut index = 0;
        let mut exists = 0;
        unsafe {
            let name = name.as_ptr();
            let hr = self.ptr.FindLocaleName(name, &mut index, &mut exists);
            if SUCCEEDED(hr) && exists != 0 {
                Some(self.unchecked_locale(index))
            } else {
                None
            }
        }
    }

    fn unchecked_locale(&self, index: u32) -> LocalizedString {
        LocalizedString {
            ptr: &self.ptr,
            idx: index,
        }
    }
}

impl fmt::Debug for LocalizedStrings {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        struct LocalizedStringsMap<'a>(&'a LocalizedStrings);
        impl<'a> fmt::Debug for LocalizedStringsMap<'a> {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.debug_map().entries(self.0.all_strings().map(|s| (s.locale(), s.string()))).finish()
            }
        }

        fmt.debug_tuple("LocalizedStrings")
            .field(&LocalizedStringsMap(self))
            .finish()
    }
}

#[derive(Copy, Clone)]
/// Represents an untranslated string
pub struct LocalizedString<'a> {
    ptr: &'a IDWriteLocalizedStrings,
    idx: u32,
}

impl<'a> LocalizedString<'a> {
    /// Get the name of the locale associated with this string.
    pub fn locale(&self) -> String {
        unsafe {
            let mut length = 0;
            let hr = self.ptr.GetLocaleNameLength(self.idx, &mut length);
            if !SUCCEEDED(hr) {
                // This should never fail, but it's better to return a weird
                // string than crashing.
                return "[failed to retrieve locale name]".into();
            }

            let mut data = vec![0u16; length as usize + 1];
            let ptr = data.as_mut_ptr();
            let hr = self.ptr.GetLocaleName(self.idx, ptr, length + 1);
            if !SUCCEEDED(hr) {
                // This should never fail, but it's better to return a weird
                // string than crashing.
                return "[failed to retrieve locale name]".into();
            }

            OsString::from_wide(&data[..length as usize])
                .into_string()
                .unwrap_or_else(|s| s.to_string_lossy().into_owned())
        }
    }

    /// Get the string value associated with this locale.
    pub fn string(&self) -> String {
        unsafe {
            let mut length = 0;
            let hr = self.ptr.GetStringLength(self.idx, &mut length);
            if !SUCCEEDED(hr) {
                // This should never fail, but it's better to return a weird
                // string than crashing.
                return "[failed to retrieve string value]".into();
            }

            let mut data = vec![0u16; length as usize + 1];
            let hr = self.ptr.GetString(self.idx, data.as_mut_ptr(), length + 1);
            if !SUCCEEDED(hr) {
                // This should never fail, but it's better to return a weird
                // string than crashing.
                return "[failed to retrieve string value]".into();
            }

            OsString::from_wide(&data[..length as usize])
                .into_string()
                .unwrap_or_else(|s| s.to_string_lossy().into_owned())
        }
    }
}

impl<'a> fmt::Debug for LocalizedString<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("LocalizedString")
            .field("locale", &self.locale())
            .field("string", &self.string())
            .finish()
    }
}

impl<'a> From<LocalizedString<'a>> for String {
    fn from(ls: LocalizedString<'a>) -> String {
        ls.string()
    }
}
