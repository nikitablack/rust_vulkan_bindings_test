use crate::vk;
use crate::vulkan;

#[allow(dead_code)]
pub fn enumerate_device_extension_properies(
    physical_device: &vulkan::PhysicalDevice,
) -> Result<Vec<vulkan::ExtensionProperties>, vk::VkResult> {
    let mut count: u32 = 0;

    match unsafe {
        vk::vkEnumerateDeviceExtensionProperties(
            physical_device.handle,
            std::ptr::null(), // TODO
            &mut count,
            std::ptr::null_mut(),
        )
    } {
        vk::VkResult::VK_SUCCESS => (),
        result => return Err(result),
    }

    let mut vk_properties = Vec::with_capacity(count as usize);
    for _ in 0..count {
        vk_properties.push(vk::VkExtensionProperties::default());
    }
    match unsafe {
        vk::vkEnumerateDeviceExtensionProperties(
            physical_device.handle,
            std::ptr::null(), // TODO
            &mut count,
            vk_properties.as_mut_ptr(),
        )
    } {
        vk::VkResult::VK_SUCCESS => (),
        result => return Err(result),
    }

    let mut properties = Vec::with_capacity(count as usize);

    for p in &vk_properties {
        let extension_name_cstr = unsafe { std::ffi::CStr::from_ptr(p.extensionName.as_ptr()) };

        let extension_name = extension_name_cstr
            .to_str()
            .map(|s| s.to_owned())
            .unwrap_or("failed to get device extension name".to_string());

        properties.push(vulkan::ExtensionProperties {
            extension_name,
            spec_version: p.specVersion,
        });
    }

    Ok(properties)
}
