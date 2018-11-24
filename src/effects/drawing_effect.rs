use effects::ClientEffect;

use winapi::um::unknwnbase::IUnknown;

/// This trait represents types which can be used in [`TextLayout::set_drawing_effect`][1].
///
/// Implementing this trait is unsafe because you must take care to only return valid
/// pointer values from `get_effect_ptr`. It must also be safe for your type to have multiple
/// pointers to it i.e. no mutation methods on shared state.
///
/// [1]: ../struct.TextLayout.html#method.set_drawing_effect
pub unsafe trait DrawingEffect: Send + Sync + Sized {
    /// Return a pointer to the underlying interface of this effect.
    fn get_effect_ptr(&self) -> *mut IUnknown;

    /// Attempt to convert back to your type from a `client_effect`.
    fn from_client_effect(effect: &ClientEffect) -> Option<Self>;
}
