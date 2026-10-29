include!("swapchain.rs");

#[cfg(all(windows, feature = "dx12"))]
#[allow(clippy::upper_case_acronyms, clippy::too_many_arguments)]
pub mod dx12 {
    use super::*;
    // Value types (structs, enums, handles) are shared with the rest of the D3D12 ecosystem through
    // `d3d12-agility`. COM interfaces stay opaque `c_void`s because the C API takes raw `ID3D12Foo*`
    // pointers, whereas `d3d12-agility` interface types are already pointers themselves: pass
    // `windows_core::Interface::as_raw()` for those.
    use d3d12_agility::{
        D3D12_RESOURCE_STATES, DXGI_FORMAT, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        HWND,
    };
    use std::ffi::c_void;
    use windows_core::GUID;
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
