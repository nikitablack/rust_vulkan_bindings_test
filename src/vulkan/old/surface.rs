#[derive(Debug)]
pub struct Surface {
    pub handle: *const super::vk::VkSurfaceKHR,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            handle: std::ptr::null(),
        }
    }
}
