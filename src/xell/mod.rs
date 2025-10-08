include!("xell.rs");

#[cfg(all(windows, feature = "dx12"))]
pub mod dx12 {
    use super::*;
    use std::ffi::c_void;
    type ID3D12Device = c_void;
    include!("dx12.rs");
}
