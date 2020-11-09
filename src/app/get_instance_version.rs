use super::vulkan;
use super::AppData;

#[allow(dead_code)]
pub fn get_instance_version(mut app_data: AppData) -> Result<AppData, String> {
    let api_version = match vulkan::enumerate_instance_version() {
        Ok(version) => version,
        Err(_) => return Err(String::from("failed to enumerate instance version")),
    };

    if api_version.major < 1 || api_version.minor < 1 {
        return Err(format!(
            "the current instance version is {}.{}.{}, the minimum supported version is 1.1.0",
            api_version.major, api_version.minor, api_version.patch
        ));
    }

    app_data.vulkan_data.api_version = api_version;

    Ok(app_data)
}
