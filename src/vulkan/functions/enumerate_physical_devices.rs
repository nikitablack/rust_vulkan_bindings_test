use crate::vk;
use crate::vulkan;

#[allow(dead_code)]
pub fn enumerate_physical_devices(
    instance: &vulkan::Instance,
) -> Result<Vec<vulkan::PhysicalDevice>, vk::VkResult> {
    let mut count: u32 = 0;

    match unsafe {
        vk::vkEnumeratePhysicalDevices(instance.handle, &mut count, std::ptr::null_mut())
    } {
        vk::VkResult::VK_SUCCESS => (),
        result => return Err(result),
    }

    let mut vk_physical_devices = Vec::with_capacity(count as usize);
    for _ in 0..count {
        vk_physical_devices.push(std::ptr::null_mut());
    }
    match unsafe {
        vk::vkEnumeratePhysicalDevices(
            instance.handle,
            &mut count,
            vk_physical_devices.as_mut_ptr(),
        )
    } {
        vk::VkResult::VK_SUCCESS => (),
        result => return Err(result),
    }

    Ok(vk_physical_devices
        .into_iter()
        .map(|device| vulkan::PhysicalDevice { handle: device })
        .collect())
}
