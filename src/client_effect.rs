use winapi::um::unknwnbase::IUnknown;
use wio::com::ComPtr;

#[derive(ComWrapper)]
pub struct ClientEffect {
    ptr: ComPtr<IUnknown>,
}
