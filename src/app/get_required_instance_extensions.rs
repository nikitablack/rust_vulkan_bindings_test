use super::vulkan;
use super::AppData;

#[allow(dead_code)]
pub fn get_required_instance_extensions(mut app_data: AppData) -> Result<AppData, String> {
    app_data.vulkan_data.instance_extensions =
        match vulkan::get_required_window_extensions(&app_data.window_data.window) {
            Ok(extensions) => extensions,
            Err(_) => return Err(String::from("failed to get required window extension")),
        };

    Ok(app_data)
}
