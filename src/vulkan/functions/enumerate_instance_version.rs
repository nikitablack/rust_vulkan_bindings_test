use crate::vk;
use crate::vulkan;

#[allow(dead_code)]
pub fn enumerate_instance_version() -> Result<vulkan::ApiVersion, vk::VkResult> {
    let mut version: u32 = 0;

    match unsafe { vk::vkEnumerateInstanceVersion(&mut version) } {
        vk::VkResult::VK_SUCCESS => Ok(vulkan::ApiVersion::from(version)),
        result => Err(result),
    }
}
