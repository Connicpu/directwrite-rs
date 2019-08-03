#[repr(transparent)]
#[derive(Copy, Clone)]
/// FFI-safe boolean value wrapper for structs that have boolean values.
pub struct DBool(i32);

impl DBool {
    /// Represents `true` in both Rust and Win32
    pub const TRUE: DBool = DBool(1);
    /// Represents `false` in both Rust and Win32
    pub const FALSE: DBool = DBool(0);
}

impl std::fmt::Debug for DBool {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&bool::from(*self), fmt)
    }
}

impl From<DBool> for bool {
    fn from(b: DBool) -> bool {
        b.0 != 0
    }
}

impl From<bool> for DBool {
    fn from(b: bool) -> DBool {
        DBool(b as i32)
    }
}

impl From<DBool> for i32 {
    fn from(b: DBool) -> i32 {
        b.0
    }
}

impl From<i32> for DBool {
    fn from(b: i32) -> DBool {
        (b != 0).into()
    }
}

impl PartialEq for DBool {
    #[inline]
    fn eq(&self, rhs: &DBool) -> bool {
        bool::from(*self) == bool::from(*rhs)
    }

    #[inline]
    fn ne(&self, rhs: &DBool) -> bool {
        bool::from(*self) != bool::from(*rhs)
    }
}

impl PartialEq<bool> for DBool {
    #[inline]
    fn eq(&self, rhs: &bool) -> bool {
        bool::from(*self) == *rhs
    }

    #[inline]
    fn ne(&self, rhs: &bool) -> bool {
        bool::from(*self) != *rhs
    }
}

impl PartialEq<DBool> for bool {
    #[inline]
    fn eq(&self, rhs: &DBool) -> bool {
        *self == bool::from(*rhs)
    }

    #[inline]
    fn ne(&self, rhs: &DBool) -> bool {
        *self != bool::from(*rhs)
    }
}
