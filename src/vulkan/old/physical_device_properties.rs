//#[derive(Debug)]
pub struct PhysicalDeviceProperties {
    pub api_version: super::ApiVersion,
    pub driver_version: u32,
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_type: super::vk::VkPhysicalDeviceType,
    pub device_name: String,
    pub pipeline_cache_uuid: [u8; super::vk::VK_UUID_SIZE],
    pub limits: super::vk::VkPhysicalDeviceLimits, // TODO
    pub sparse_properties: super::vk::VkPhysicalDeviceSparseProperties, // TODO
}

impl Default for PhysicalDeviceProperties {
    fn default() -> Self {
        Self {
            api_version: super::ApiVersion::default(),
            driver_version: 0,
            device_id: 0,
            vendor_id: 0,
            device_type: super::vk::VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_MAX_ENUM,
            device_name: String::new(),
            pipeline_cache_uuid: [0; super::vk::VK_UUID_SIZE],
            limits: super::vk::VkPhysicalDeviceLimits::default(),
            sparse_properties: super::vk::VkPhysicalDeviceSparseProperties::default(),
        }
    }
}
