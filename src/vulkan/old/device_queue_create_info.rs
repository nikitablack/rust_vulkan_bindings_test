#[derive(Debug)]
pub struct DeviceQueueCreateInfo {
    next: *const std::ffi::c_void,
    flags: super::DeviceQueueCreateFlagBits,
    queue_family_index: u32,
    queue_priorities: Vec<f32>,
}

impl Default for DeviceQueueCreateInfo {
    fn default() -> Self {
        Self {
            next: std::ptr::null(),
            flags: super::DeviceQueueCreateFlagBits::empty(),
            queue_family_index: 0,
            queue_priorities: Vec::<f32>::default(),
        }
    }
}
