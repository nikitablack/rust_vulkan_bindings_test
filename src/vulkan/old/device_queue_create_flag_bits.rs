bitflags! {
    pub struct DeviceQueueCreateFlagBits: u32 {
        const VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT = super::vk::VkDeviceQueueCreateFlagBits::VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT as u32;
        const VK_DEVICE_QUEUE_CREATE_FLAG_BITS_MAX_ENUM = super::vk::VkDeviceQueueCreateFlagBits::VK_DEVICE_QUEUE_CREATE_FLAG_BITS_MAX_ENUM as u32;
    }
}
