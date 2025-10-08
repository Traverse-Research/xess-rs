include!("xell.rs");

#[cfg(all(windows, feature = "dx12"))]
pub mod dx12 {
    use super::*;
    use windows::Win32::Graphics::Direct3D12::ID3D12Device;
    include!("dx12.rs");
}
