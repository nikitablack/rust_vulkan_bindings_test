#[derive(Debug)]
pub struct Instance {
    pub handle: *const super::vk::VkInstance,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            handle: std::ptr::null(),
        }
    }
}
