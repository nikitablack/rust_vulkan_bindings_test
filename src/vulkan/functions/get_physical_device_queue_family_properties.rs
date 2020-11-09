use crate::vk;
use crate::vulkan;

#[allow(dead_code)]
pub fn get_physical_device_queue_family_properties(
    physical_device: &vulkan::PhysicalDevice,
) -> Vec<vulkan::QueueFamilyProperties> {
    let mut count = 0;
    unsafe {
        vk::vkGetPhysicalDeviceQueueFamilyProperties(
            physical_device.handle,
            &mut count,
            std::ptr::null_mut(),
        )
    };

    let mut vk_families = Vec::with_capacity(count as usize);
    for _ in 0..count {
        vk_families.push(vk::VkQueueFamilyProperties::default());
    }

    unsafe {
        vk::vkGetPhysicalDeviceQueueFamilyProperties(
            physical_device.handle,
            &mut count,
            vk_families.as_mut_ptr(),
        )
    };

    vk_families
        .iter()
        .map(|prop| vulkan::QueueFamilyProperties {
            queue_flags: prop.queueFlags,
            queue_count: prop.queueCount,
            timestamp_valid_bits: prop.timestampValidBits,
            min_image_transfer_granularity: vulkan::Extent3D {
                width: prop.minImageTransferGranularity.width,
                height: prop.minImageTransferGranularity.height,
                depth: prop.minImageTransferGranularity.depth,
            },
        })
        .collect()
}
