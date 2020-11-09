use super::vulkan;
use super::AppData;

pub fn create_instance(mut app_data: AppData) -> Result<AppData, String> {
    let application_info = vulkan::ApplicationInfo {
        application_name: String::from("Vulkan Teapot"),
        application_version: 1,
        engine_name: String::from(""),
        engine_version: 0,
        api_version: vulkan::ApiVersion {
            major: 1,
            minor: 1,
            patch: 0,
        },
    };

    let instance_create_info = vulkan::InstanceCreateInfo {
        next: std::ptr::null(),
        flags: 0,
        application_info: Some(&application_info),
        enabled_layer_names: None,
        enabled_extension_names: Some(&app_data.vulkan_data.instance_extensions),
    };

    app_data.vulkan_data.instance =
        match vulkan::create_instance(&instance_create_info, std::ptr::null()) {
            // TODO
            Ok(instance) => instance,
            Err(_) => return Err(String::from("failed to create instance")),
        };

    Ok(app_data)
}
