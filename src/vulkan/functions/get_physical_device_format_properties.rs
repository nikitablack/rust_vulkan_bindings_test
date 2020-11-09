use crate::vk;
use crate::vulkan;

#[allow(dead_code)]
pub fn get_physical_device_format_properties(
    physical_device: &vulkan::PhysicalDevice,
    format: vk::VkFormat,
) -> vulkan::FormatProperties {
    let mut props = vk::VkFormatProperties::default();

    unsafe { vk::vkGetPhysicalDeviceFormatProperties(physical_device.handle, format, &mut props) };

    vulkan::FormatProperties {
        linear_tiling_features: props.linearTilingFeatures,
        optimal_tiling_features: props.optimalTilingFeatures,
        buffer_features: props.bufferFeatures,
    }
}
