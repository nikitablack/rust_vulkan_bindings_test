use crate::vk;
use crate::vulkan;

#[allow(dead_code)]
pub fn get_device_queue(
    device: &vulkan::Device,
    queue_family_index: u32,
    queue_index: u32,
) -> vulkan::Queue {
    let mut vk_queue = std::ptr::null_mut();

    unsafe {
        vk::vkGetDeviceQueue(
            device.handle,
            queue_family_index,
            queue_index,
            &mut vk_queue,
        )
    };

    vulkan::Queue { handle: vk_queue }
}
