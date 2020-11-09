use super::vk;
use super::vulkan;
use super::AppData;

pub fn create_logical_device(mut app_data: AppData) -> Result<AppData, String> {
    let queue_indices = vec![app_data.vulkan_data.device_data.queue_family];

    let mut queue_priorities = Vec::with_capacity(queue_indices.len());
    for _ in &queue_indices {
        queue_priorities.push(vec![1.0f32]);
    }

    let mut queue_create_infos = Vec::with_capacity(queue_indices.len());

    for (ind, &family_index) in queue_indices.iter().enumerate() {
        let info = vulkan::DeviceQueueCreateInfo {
            next: std::ptr::null(),
            flags: vk::VkDeviceQueueCreateFlagBits::empty(),
            queue_family_index: family_index,
            queue_priorities: queue_priorities[ind].clone(),
        };

        queue_create_infos.push(info);
    }

    let mut features = vulkan::PhysicalDeviceFeatures::default();
    features.tessellation_shader = true;
    features.fill_mode_non_solid = true;

    let create_info = vulkan::DeviceCreateInfo {
        next: std::ptr::null(),
        flags: vk::VkDeviceCreateFlags::empty(),
        queue_create_infos: queue_create_infos,
        enabled_layer_names: None,
        enabled_extension_names: Some(&app_data.vulkan_data.device_extensions),
        enabled_features: features,
    };

    app_data.vulkan_data.device_data.device = match vulkan::create_device(
        &app_data.vulkan_data.device_data.physical_device,
        &create_info,
        std::ptr::null(),
    ) {
        // TODO
        Ok(device) => device,
        Err(_) => return Err(String::from("failed to create device")),
    };

    Ok(app_data)
}
