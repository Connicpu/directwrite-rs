use error::DWResult;
use factory::Factory;
use font_file::FontFile;

use std::marker::PhantomData;
use std::sync::Arc;

use com_wrapper::ComWrapper;
use winapi::um::dwrite::IDWriteFontCollectionLoader;
use winapi::um::dwrite::IDWriteFontCollectionLoaderVtbl;
use winapi::um::dwrite::IDWriteFontFileEnumeratorVtbl;
use wio::com::ComPtr;

mod enum_vtbl;
mod vtbl;

pub trait FontKey: Sized + Send + Sync + 'static {}
impl<T> FontKey for T where T: Sized + Send + Sync + 'static {}

pub trait FontCollectionLoader: Send + Sync + 'static {
    type Key: FontKey;
    type Iter: Iterator<Item = DWResult<FontFile>> + 'static;

    fn get_iterator(&self, factory: &Factory, key: &Self::Key) -> DWResult<Self::Iter>;
}

#[derive(Clone)]
pub struct CollectionLoaderHandle<K: FontKey> {
    pub(crate) ptr: ComPtr<IDWriteFontCollectionLoader>,
    _marker: PhantomData<K>,
}

impl<K: FontKey> ComWrapper for CollectionLoaderHandle<K> {
    type Interface = IDWriteFontCollectionLoader;

    unsafe fn get_raw(&self) -> *mut IDWriteFontCollectionLoader {
        self.ptr.as_raw()
    }

    unsafe fn into_raw(self) -> *mut IDWriteFontCollectionLoader {
        self.ptr.into_raw()
    }

    unsafe fn from_raw(raw: *mut IDWriteFontCollectionLoader) -> Self {
        Self::from_ptr(ComPtr::from_raw(raw))
    }

    unsafe fn from_ptr(ptr: ComPtr<IDWriteFontCollectionLoader>) -> Self {
        CollectionLoaderHandle {
            ptr,
            _marker: PhantomData,
        }
    }

    unsafe fn into_ptr(self) -> ComPtr<IDWriteFontCollectionLoader> {
        self.ptr
    }
}

#[repr(C)]
pub struct KeyPayload<K: FontKey> {
    ty_id: u64,
    data: K,
}

impl<K: FontKey> KeyPayload<K> {
    pub fn new(data: K) -> Self {
        KeyPayload {
            ty_id: Self::id(),
            data,
        }
    }

    fn valid(&self) -> bool {
        self.ty_id == Self::id()
    }

    fn id() -> u64 {
        use std::hash::Hash;
        let tid = std::any::TypeId::of::<K>();
        let mut h = FnvHasher::default();
        tid.hash(&mut h);
        h.0
    }
}

#[repr(C)]
pub struct ComFontCollectionLoader<T>
where
    T: FontCollectionLoader,
{
    vtbl: *const IDWriteFontCollectionLoaderVtbl,
    loader: T,
}

impl<T> ComFontCollectionLoader<T>
where
    T: FontCollectionLoader,
{
    pub fn new(loader: T) -> Arc<Self> {
        Arc::new(ComFontCollectionLoader {
            vtbl: vtbl::loader_vtable_for::<T>(),
            loader,
        })
    }

    pub fn to_raw_loader(this: Arc<Self>) -> *mut IDWriteFontCollectionLoader {
        Arc::into_raw(this) as *mut Self as *mut _
    }
}

unsafe impl<T> Send for ComFontCollectionLoader<T> where T: FontCollectionLoader {}
unsafe impl<T> Sync for ComFontCollectionLoader<T> where T: FontCollectionLoader {}

#[repr(C)]
struct ComEnumerator<I>
where
    I: Iterator<Item = DWResult<FontFile>> + 'static,
{
    vtbl: *const IDWriteFontFileEnumeratorVtbl,
    iter: I,
    curr: Option<FontFile>,
    err: Option<i32>,
}

impl<I> ComEnumerator<I>
where
    I: Iterator<Item = DWResult<FontFile>> + 'static,
{
    fn new(iter: I) -> Arc<Self> {
        Arc::new(ComEnumerator {
            vtbl: enum_vtbl::enum_vtable_for::<I>(),
            iter: iter,
            curr: None,
            err: None,
        })
    }
}

pub struct FnvHasher(u64);

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
