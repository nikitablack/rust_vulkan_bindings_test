use super::vulkan;
use super::AppData;

pub fn get_device_queue(mut app_data: AppData) -> AppData {
    app_data.vulkan_data.device_data.queue = vulkan::get_device_queue(
        &app_data.vulkan_data.device_data.device,
        app_data.vulkan_data.device_data.queue_family,
        0,
    );

    app_data
}
