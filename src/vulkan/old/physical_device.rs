#[derive(Debug)]
pub struct PhysicalDevice {
    pub handle: *const super::vk::VkPhysicalDevice,
}

impl Default for PhysicalDevice {
    fn default() -> Self {
        Self {
            handle: std::ptr::null(),
        }
    }
}
