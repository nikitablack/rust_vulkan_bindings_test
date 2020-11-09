use super::vulkan;
use super::AppData;

#[allow(dead_code)]
pub fn create_surface(mut app_data: AppData) -> Result<AppData, String> {
    app_data.vulkan_data.surface = match vulkan::create_surface(
        &app_data.window_data.window,
        &app_data.vulkan_data.instance,
        std::ptr::null(),
    ) {
        Ok(surface) => surface,
        Err(_) => return Err(String::from("failed to create surface")),
    };

    Ok(app_data)
}
