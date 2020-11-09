#[derive(Debug)]
pub struct Device {
    pub handle: *const super::vk::VkDevice,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            handle: std::ptr::null(),
        }
    }
}
