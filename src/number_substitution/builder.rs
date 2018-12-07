use crate::enums::NumberSubstitutionMethod;
use crate::error::DWResult;
use crate::factory::Factory;
use crate::number_substitution::NumberSubstitution;

use std::borrow::Cow;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use wio::wide::ToWide;

/// Builder for a `NumberSubstitution` object.
pub struct NumberSubstitutionBuilder<'a> {
    factory: &'a Factory,
    method: Option<NumberSubstitutionMethod>,
    locale: Cow<'a, [u16]>,
    ignore_user_override: bool,
}

impl<'a> NumberSubstitutionBuilder<'a> {
    pub(crate) fn new(factory: &'a Factory) -> Self {
        NumberSubstitutionBuilder {
            factory,
            method: None,
            locale: Cow::Borrowed(DEFAULT_LOCALE),
            ignore_user_override: false,
        }
    }

    /// Build the number substitution object.
    pub fn build(self) -> DWResult<NumberSubstitution> {
        let method = self.method.expect("`method` must be specified");
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = (*self.factory.get_raw()).CreateNumberSubstitution(
                method as u32,
                self.locale.as_ptr(),
                self.ignore_user_override as i32,
                &mut ptr,
            );

            if SUCCEEDED(hr) {
                Ok(NumberSubstitution::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Specify the substitution method used. This is required.
    pub fn with_method(mut self, method: NumberSubstitutionMethod) -> Self {
        self.method = Some(method);
        self
    }

    /// Specify a locale string for the substitution.
    pub fn with_locale(mut self, locale: &str) -> Self {
        self.locale = Cow::Owned(locale.to_wide_null());
        self
    }

    /// Specify a locale string for the substitution.
    pub fn with_locale_wide(mut self, locale: impl Into<Cow<'a, [u16]>>) -> Self {
        let mut locale = locale.into();
        if locale.last() != Some(&0) {
            locale.to_mut().push(0);
        }
        self.locale = locale;
        self
    }

    /// Specify whether this number substitution ignores user overrides.
    pub fn with_ignore_user_override(mut self, ignore: bool) -> Self {
        self.ignore_user_override = ignore;
        self
    }
}

const DEFAULT_LOCALE: &[u16] = &[
    b'e' as u16,
    b'n' as u16,
    b'-' as u16,
    b'U' as u16,
    b'S' as u16,
    0,
];
