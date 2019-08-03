use crate::effects::DrawingEffect;

use com_wrapper::ComWrapper;
use winapi::um::unknwnbase::IUnknown;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Represents a wrapped value that could be any drawing effect type.
///
/// Use the [`DrawingEffect`][1] trait to cast this to a type that could have been passed to
/// [`TextLayout::set_drawing_effect`][2]. If you need an escape hatch to cast this to another type
/// that doesn't implement [`DrawingEffect`][1], see the [`ComWrapper`][3] trait.
///
/// [1]: trait.DrawingEffect.html
/// [2]: ../struct.TextLayout.html#method.set_drawing_effect
/// [3]: https://docs.rs/com-wrapper/*/com_wrapper/trait.ComWrapper.html
pub struct ClientEffect {
    ptr: ComPtr<IUnknown>,
}

unsafe impl DrawingEffect for ClientEffect {
    fn get_effect_ptr(&self) -> *mut IUnknown {
        self.ptr.as_raw()
    }

    fn from_client_effect(effect: &ClientEffect) -> Option<Self> {
        Some(effect.clone())
    }
}
