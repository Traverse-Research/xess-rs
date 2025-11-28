include!("swapchain.rs");

#[cfg(all(windows, feature = "dx12"))]
pub mod dx12 {
    use super::*;
    use std::ffi::c_void;
    use windows::{
        core::GUID,
        Win32::{
            Foundation::HWND,
            Graphics::{
                Direct3D12::D3D12_RESOURCE_STATES,
                Dxgi::{DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FULLSCREEN_DESC},
            },
        },
    };
    type ID3D12CommandList = c_void;
    type ID3D12CommandQueue = c_void;
    type ID3D12DescriptorHeap = c_void;
    type ID3D12Device = c_void;
    type ID3D12Heap = c_void;
    type ID3D12PipelineLibrary = c_void;
    type ID3D12Resource = c_void;
    type IDXGIFactory2 = c_void;
    type IDXGISwapChain = c_void;
    type IID = GUID;
    include!("dx12.rs");
}
