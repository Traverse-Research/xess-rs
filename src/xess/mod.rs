pub mod xess {
    include!("xess.rs");
}

#[cfg(feature = "dx11")]
pub mod xess_d3d11 {
    use super::xess::*;
    use windows::Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11Resource};
    include!("xess_d3d11.rs");
}
#[cfg(feature = "dx12")]
pub mod xess_d3d12 {
    use super::xess::*;
    use windows::Win32::Graphics::Direct3D12::{
        ID3D12DescriptorHeap, ID3D12Device, ID3D12GraphicsCommandList, ID3D12Heap,
        ID3D12PipelineLibrary, ID3D12Resource,
    };
    include!("xess_d3d12.rs");
}
#[cfg(feature = "vk")]
pub mod xess_vk {
    use super::xess::*;
    use ash::vk::{
        Buffer, CommandBuffer, Device, DeviceMemory, Format, Image, ImageLayout,
        ImageSubresourceRange, ImageView, Instance, PhysicalDevice, PipelineCache,
    };
    type VkBuffer = Buffer;
    type VkCommandBuffer = CommandBuffer;
    type VkDevice = Device;
    type VkDeviceMemory = DeviceMemory;
    type VkFormat = Format;
    type VkImage = Image;
    type VkImageLayout = ImageLayout;
    type VkImageSubresourceRange = ImageSubresourceRange;
    type VkImageView = ImageView;
    type VkInstance = Instance;
    type VkPhysicalDevice = PhysicalDevice;
    type VkPipelineCache = PipelineCache;
    include!("xess_vk.rs");
}
