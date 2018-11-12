pub trait FontKey: Send + Sync + 'static {}
impl<T> FontKey for T where T: Send + Sync + ?Sized + 'static {}

#[repr(C)]
pub(crate) struct KeyPayload<'a, K: FontKey + ?Sized> {
    ty_id: u64,
    pub(crate) data: &'a K,
}

impl<'a, K: FontKey + ?Sized> KeyPayload<'a, K> {
    pub(crate) fn new(data: &'a K) -> Self {
        KeyPayload {
            ty_id: Self::id(),
            data,
        }
    }

    pub(crate) fn valid(&self) -> bool {
        self.ty_id == Self::id()
    }

    pub(crate) fn id() -> u64 {
        use std::hash::Hash;
        let tid = std::any::TypeId::of::<K>();
        let mut h = FnvHasher::default();
        tid.hash(&mut h);
        h.0
    }
}

struct FnvHasher(u64);

impl Default for FnvHasher {
    #[inline]
    fn default() -> FnvHasher {
        FnvHasher(0xcbf29ce484222325)
    }
}

impl std::hash::Hasher for FnvHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let FnvHasher(mut hash) = *self;

        for byte in bytes.iter() {
            hash = hash ^ (*byte as u64);
            hash = hash.wrapping_mul(0x100000001b3);
        }

        *self = FnvHasher(hash);
    }
}
