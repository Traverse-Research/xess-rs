include!("xess.rs");

#[cfg(all(windows, feature = "dx11"))]
pub mod dx11 {
    use super::*;
    use std::ffi::c_void;
    type ID3D11Device = c_void;
    type ID3D11Resource = c_void;
    include!("dx11.rs");
}
#[cfg(all(windows, feature = "dx12"))]
pub mod dx12 {
    use super::*;
    use std::ffi::c_void;
    type ID3D12DescriptorHeap = c_void;
    type ID3D12Device = c_void;
    type ID3D12GraphicsCommandList = c_void;
    type ID3D12Heap = c_void;
    type ID3D12PipelineLibrary = c_void;
    type ID3D12Resource = c_void;
    include!("dx12.rs");
}
#[cfg(feature = "vk")]
pub mod vk {
    use super::*;
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
    include!("vk.rs");
}
