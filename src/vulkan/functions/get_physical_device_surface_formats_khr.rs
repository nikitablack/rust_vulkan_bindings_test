use crate::vk;
use crate::vulkan;

pub fn get_physical_device_surface_formats_khr(
    physical_device: &vulkan::PhysicalDevice,
    surface: &vulkan::SurfaceKHR,
) -> Result<Vec<vulkan::SurfaceFormatKHR>, vk::VkResult> {
    let mut count: u32 = 0;

    match unsafe {
        vk::vkGetPhysicalDeviceSurfaceFormatsKHR(
            physical_device.handle,
            surface.handle,
            &mut count,
            std::ptr::null_mut(),
        )
    } {
        vk::VkResult::VK_SUCCESS => (),
        result => return Err(result),
    }

    let mut vk_formats = Vec::with_capacity(count as usize);
    for _ in 0..count {
        vk_formats.push(vk::VkSurfaceFormatKHR::default());
    }
    match unsafe {
        vk::vkGetPhysicalDeviceSurfaceFormatsKHR(
            physical_device.handle,
            surface.handle,
            &mut count,
            vk_formats.as_mut_ptr(),
        )
    } {
        vk::VkResult::VK_SUCCESS => (),
        result => return Err(result),
    }

    Ok(vk_formats
        .iter()
        .map(|f| vulkan::SurfaceFormatKHR {
            format: f.format,
            color_space: f.colorSpace,
        })
        .collect())
}
