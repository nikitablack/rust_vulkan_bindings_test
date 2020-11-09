use crate::vk;

pub struct Device {
    pub handle: *const vk::VkDevice,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            handle: std::ptr::null(),
        }
    }
}

pub struct Instance {
    pub handle: *const vk::VkInstance,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            handle: std::ptr::null(),
        }
    }
}

pub struct PhysicalDevice {
    pub handle: *const vk::VkPhysicalDevice,
}

impl Default for PhysicalDevice {
    fn default() -> Self {
        Self {
            handle: std::ptr::null(),
        }
    }
}

pub struct Queue {
    pub handle: *const vk::VkQueue,
}

impl Default for Queue {
    fn default() -> Self {
        Self {
            handle: std::ptr::null(),
        }
    }
}

pub struct SurfaceKHR {
    pub handle: *const vk::VkSurfaceKHR,
}

impl Default for SurfaceKHR {
    fn default() -> Self {
        Self {
            handle: std::ptr::null(),
        }
    }
}
