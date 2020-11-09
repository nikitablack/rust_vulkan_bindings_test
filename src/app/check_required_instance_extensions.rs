use super::vulkan;
use super::AppData;

#[allow(dead_code)]
pub fn check_required_instance_extensions(app_data: AppData) -> Result<AppData, String> {
    let supported_instance_extensions = match vulkan::enumerate_instance_extension_properies() {
        Ok(props) => props,
        Err(_) => {
            return Err(String::from(
                "failed to enumerate instance extension properies",
            ))
        }
    };

    let mut supported_instance_extensions_set = std::collections::HashSet::new();
    for vulkan::ExtensionProperties { extension_name, .. } in &supported_instance_extensions {
        supported_instance_extensions_set.insert(extension_name);
    }

    for extension_name in &app_data.vulkan_data.instance_extensions {
        if !supported_instance_extensions_set.contains(extension_name) {
            return Err(format!(
                "instance extension {} is not supported",
                extension_name
            ));
        }
    }

    Ok(app_data)
}
