use super::vk;
use super::vulkan;
use super::AppData;

fn check_required_device_extensions(
    physical_device: &vulkan::PhysicalDevice,
    required_extensions: &Vec<String>,
) -> Result<(), String> {
    let supported_device_extensions =
        match vulkan::enumerate_device_extension_properies(physical_device) {
            Ok(props) => props,
            Err(_) => {
                return Err(String::from(
                    "failed to enumerate instance extension properies",
                ))
            }
        };

    let mut supported_device_extensions_set = std::collections::HashSet::new();
    for vulkan::ExtensionProperties { extension_name, .. } in &supported_device_extensions {
        supported_device_extensions_set.insert(extension_name);
    }

    for extension_name in required_extensions {
        if !supported_device_extensions_set.contains(extension_name) {
            return Err(format!(
                "device extension {} is not supported",
                extension_name
            ));
        }
    }

    Ok(())
}

fn check_device_suitability(
    physical_device: &vulkan::PhysicalDevice,
    required_extensions: &Vec<String>,
) -> Result<(), String> {
    let properties = vulkan::get_physical_device_properties(&physical_device);

    if properties.device_type != vk::VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU {
        return Err(format!(
            "the device {} is not a discrete GPU",
            properties.device_name
        ));
    }

    let features = vulkan::get_physical_device_features(physical_device);

    if !features.tessellation_shader {
        return Err(format!(
            "the device {} does not support tesselation shader",
            properties.device_name
        ));
    }

    if !features.fill_mode_non_solid {
        return Err(format!(
            "the device {} does not support fill mode non solid",
            properties.device_name
        ));
    }

    check_required_device_extensions(physical_device, required_extensions)?;

    Ok(())
}

fn get_device_surface_format(
    physical_device: &vulkan::PhysicalDevice,
    surface: &vulkan::SurfaceKHR,
) -> Result<vulkan::SurfaceFormatKHR, String> {
    let formats = match vulkan::get_physical_device_surface_formats_khr(physical_device, surface) {
        Ok(formats) => formats,
        Err(_) => {
            return Err(String::from(
                "failed to get physical device surface formats",
            ));
        }
    };

    if formats.is_empty() {
        return Err(String::from(
            "failed to get physical device surface formats",
        ));
    }

    if formats.len() == 1 && formats[0].format == vk::VkFormat::VK_FORMAT_UNDEFINED {
        return Ok(vulkan::SurfaceFormatKHR {
            format: vk::VkFormat::VK_FORMAT_B8G8R8A8_UNORM,
            color_space: vk::VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
        });
    }

    for f in &formats {
        if f.format == vk::VkFormat::VK_FORMAT_B8G8R8A8_UNORM
            && f.color_space == vk::VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
        {
            return Ok(vulkan::SurfaceFormatKHR {
                format: vk::VkFormat::VK_FORMAT_B8G8R8A8_UNORM,
                color_space: vk::VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
            });
        }
    }

    Ok(formats[0])
}

fn get_device_surface_present_mode(
    physical_device: &vulkan::PhysicalDevice,
    surface: &vulkan::SurfaceKHR,
) -> Result<vk::VkPresentModeKHR, String> {
    let modes =
        match vulkan::get_physical_device_surface_present_modes_khr(physical_device, surface) {
            Ok(formats) => formats,
            Err(_) => {
                return Err(String::from(
                    "failed to get physical device surface present modes",
                ));
            }
        };

    if modes.is_empty() {
        return Err(String::from(
            "failed to get physical device surface present modes",
        ));
    }

    if modes.contains(&vk::VkPresentModeKHR::VK_PRESENT_MODE_MAILBOX_KHR) {
        return Ok(vk::VkPresentModeKHR::VK_PRESENT_MODE_MAILBOX_KHR);
    }

    if modes.contains(&vk::VkPresentModeKHR::VK_PRESENT_MODE_IMMEDIATE_KHR) {
        return Ok(vk::VkPresentModeKHR::VK_PRESENT_MODE_IMMEDIATE_KHR);
    }

    Ok(vk::VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR)
}

fn get_device_graphics_present_queue_family(
    physical_device: &vulkan::PhysicalDevice,
    surface: &vulkan::SurfaceKHR,
) -> Result<u32, String> {
    let props = vulkan::get_physical_device_queue_family_properties(&physical_device);

    for (ind, p) in props.iter().enumerate() {
        if p.queue_count > 0
            && p.queue_flags
                .contains(vk::VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT)
        {
            let present_supported = match super::vulkan::get_physical_device_surface_support_khr(
                physical_device,
                ind as u32,
                surface,
            ) {
                Ok(result) => result,
                Err(_) => {
                    return Err(String::from(
                        "failed to get physical device surface_support",
                    ))
                }
            };

            if present_supported {
                return Ok(ind as u32);
            }
        }
    }

    Err(String::from(
        "failed to find graphics queue with present support",
    ))
}

fn get_depth_format(physical_device: &vulkan::PhysicalDevice) -> Result<vk::VkFormat, String> {
    let format_candidates = &[
        vk::VkFormat::VK_FORMAT_D24_UNORM_S8_UINT,
        vk::VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT,
        vk::VkFormat::VK_FORMAT_D16_UNORM_S8_UINT,
    ];

    for &format in format_candidates {
        let props = vulkan::get_physical_device_format_properties(physical_device, format);

        if props
            .optimal_tiling_features
            .contains(vk::VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT)
        {
            return Ok(format);
        }
    }

    Err(String::from("failed to find depth format"))
}

#[allow(dead_code)]
pub fn get_physical_device(mut app_data: AppData) -> Result<AppData, String> {
    let devices = match vulkan::enumerate_physical_devices(&app_data.vulkan_data.instance) {
        Ok(devices) => devices,
        Err(_) => return Err(String::from("failed to enumerate physical devices")),
    };

    for device in &devices {
        if let Err(_) = check_device_suitability(device, &app_data.vulkan_data.device_extensions) {
            continue;
        }

        app_data.vulkan_data.device_data.surface_format =
            match get_device_surface_format(device, &app_data.vulkan_data.surface) {
                Ok(format) => format,
                Err(_) => continue,
            };

        app_data.vulkan_data.device_data.present_mode =
            match get_device_surface_present_mode(device, &app_data.vulkan_data.surface) {
                Ok(mode) => mode,
                Err(_) => continue,
            };

        app_data.vulkan_data.device_data.queue_family =
            match get_device_graphics_present_queue_family(device, &app_data.vulkan_data.surface) {
                Ok(family) => family,
                Err(_) => continue,
            };

        app_data.vulkan_data.device_data.depth_format = match get_depth_format(device) {
            Ok(format) => format,
            Err(_) => continue,
        };

        app_data.vulkan_data.device_data.properties =
            vulkan::get_physical_device_properties(device);

        app_data.vulkan_data.device_data.physical_device = vulkan::PhysicalDevice {
            handle: device.handle,
        };

        return Ok(app_data);
    }

    Err(String::from("failed to find suitable device"))
}
