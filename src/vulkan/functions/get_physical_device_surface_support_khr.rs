use crate::vk;
use crate::vulkan;

pub fn get_physical_device_surface_support_khr(
    physical_device: &vulkan::PhysicalDevice,
    queue_family_index: u32,
    surface: &vulkan::SurfaceKHR,
) -> Result<bool, vk::VkResult> {
    let mut supported = 0;

    match unsafe {
        vk::vkGetPhysicalDeviceSurfaceSupportKHR(
            physical_device.handle,
            queue_family_index,
            surface.handle,
            &mut supported,
        )
    } {
        vk::VkResult::VK_SUCCESS => Ok(supported > 0),
        result => Err(result),
    }
}
