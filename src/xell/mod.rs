include!("xell.rs");

#[cfg(feature = "dx12")]
pub mod xell_d3d12 {
    use super::*;
    use windows::Win32::Graphics::Direct3D12::ID3D12Device;
    include!("xell_d3d12.rs");
}
