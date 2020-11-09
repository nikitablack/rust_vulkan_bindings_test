use crate::vk;
use crate::vulkan;

pub fn get_physical_device_surface_present_modes_khr(
    physical_device: &vulkan::PhysicalDevice,
    surface: &vulkan::SurfaceKHR,
) -> Result<Vec<vk::VkPresentModeKHR>, vk::VkResult> {
    let mut count: u32 = 0;

    match unsafe {
        vk::vkGetPhysicalDeviceSurfacePresentModesKHR(
            physical_device.handle,
            surface.handle,
            &mut count,
            std::ptr::null_mut(),
        )
    } {
        vk::VkResult::VK_SUCCESS => (),
        result => return Err(result),
    }

    let mut vk_modes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        vk_modes.push(vk::VkPresentModeKHR::VK_PRESENT_MODE_MAX_ENUM_KHR);
    }
    match unsafe {
        vk::vkGetPhysicalDeviceSurfacePresentModesKHR(
            physical_device.handle,
            surface.handle,
            &mut count,
            vk_modes.as_mut_ptr(),
        )
    } {
        vk::VkResult::VK_SUCCESS => (),
        result => return Err(result),
    }

    Ok(vk_modes)
}
