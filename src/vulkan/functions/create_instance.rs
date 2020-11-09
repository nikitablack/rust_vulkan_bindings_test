use crate::utils;
use crate::vk;
use crate::vulkan;

#[allow(dead_code)]
pub fn create_instance(
    create_info: &vulkan::InstanceCreateInfo,
    _allocator: *const vulkan::AllocationCallbacks, // TODO
) -> Result<vulkan::Instance, vk::VkResult> {
    let vk_application_info = match create_info.application_info {
        Some(info) => Some(vk::VkApplicationInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: std::ptr::null(), // TODO
            pApplicationName: utils::string_to_cstr(&info.application_name).as_ptr(),
            applicationVersion: info.application_version,
            pEngineName: utils::string_to_cstr(&info.engine_name).as_ptr(),
            engineVersion: info.engine_version,
            apiVersion: info.api_version.to_number(),
        }),
        None => None,
    };

    let layer_names_as_cstrings = match create_info.enabled_layer_names {
        Some(names) => utils::strings_to_cstrings(names),
        None => Vec::default(),
    };

    let layer_names_as_ccharptrs = utils::cstrings_to_ccharptrs(&layer_names_as_cstrings);

    let extension_names_as_cstrings = match create_info.enabled_extension_names {
        Some(names) => utils::strings_to_cstrings(names),
        None => Vec::default(),
    };

    let extension_names_as_ccharptrs = utils::cstrings_to_ccharptrs(&extension_names_as_cstrings);

    let vk_instance_create_info = vk::VkInstanceCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: create_info.next,
        flags: create_info.flags,
        pApplicationInfo: match vk_application_info {
            Some(info) => &info,
            _ => std::ptr::null(),
        },
        enabledLayerCount: layer_names_as_ccharptrs.len() as u32,
        ppEnabledLayerNames: layer_names_as_ccharptrs.as_ptr(),
        enabledExtensionCount: extension_names_as_ccharptrs.len() as u32,
        ppEnabledExtensionNames: extension_names_as_ccharptrs.as_ptr(),
    };
    let mut vk_instance = std::ptr::null_mut();

    match unsafe {
        vk::vkCreateInstance(&vk_instance_create_info, std::ptr::null(), &mut vk_instance)
        // TODO
    } {
        vk::VkResult::VK_SUCCESS => Ok(vulkan::Instance {
            handle: vk_instance,
        }),
        result => Err(result),
    }
}
