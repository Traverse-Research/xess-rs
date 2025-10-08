include!("swapchain.rs");

#[cfg(feature = "dx12")]
pub mod dx12 {
    use super::*;
    use windows::{
        core::GUID,
        Win32::{
            Foundation::HWND,
            Graphics::{
                Direct3D12::{
                    ID3D12CommandList, ID3D12CommandQueue, ID3D12DescriptorHeap, ID3D12Device,
                    ID3D12Heap, ID3D12PipelineLibrary, ID3D12Resource, D3D12_RESOURCE_STATES,
                },
                Dxgi::{
                    IDXGIFactory2, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC1,
                    DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
                },
            },
        },
    };
    type IID = GUID;
    include!("dx12.rs");
}
