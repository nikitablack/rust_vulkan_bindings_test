use crate::vk;

#[derive(Debug)]
pub struct ExtensionProperties {
    pub extension_name: String,
    pub spec_version: u32,
}

#[derive(Debug)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ApiVersion {
    fn from(version: u32) -> Self {
        ApiVersion {
            major: version >> 22,
            minor: (version >> 12) & 0x3ff,
            patch: version & 0xfff,
        }
    }

    fn to_number(&self) -> u32 {
        (self.major << 22) | (self.minor << 12) | (self.patch)
    }
}

#[derive(Debug)]
pub struct ApplicationInfo {
    pub application_name: String,
    pub application_version: u32,
    pub engine_name: String,
    pub engine_version: u32,
    pub api_version: ApiVersion,
}

#[derive(Debug)]
pub struct InstanceCreateInfo<'a> {
    pub next: *const std::ffi::c_void,
    pub flags: u32,
    pub application_info: Option<&'a ApplicationInfo>,
    pub enabled_layer_names: Vec<String>,
    pub enabled_extension_names: Vec<String>,
}

#[derive(Debug)]
pub struct Instance {
    handle: *const vk::VkInstance,
}

pub fn enumerate_instance_extension_properies() -> Result<Vec<ExtensionProperties>, vk::VkResult> {
    let mut count: u32 = 0;

    match unsafe {
        vk::vkEnumerateInstanceExtensionProperties(
            std::ptr::null(),
            &mut count,
            std::ptr::null_mut(),
        )
    } {
        vk::VkResult::VkSuccess => (),
        result => return Err(result),
    }

    let mut vk_properties = Vec::with_capacity(count as usize);
    for _ in 0..count {
        vk_properties.push(vk::VkExtensionProperties {
            extensionName: [0; vk::VK_MAX_EXTENSION_NAME_SIZE],
            specVersion: 0,
        });
    }
    match unsafe {
        vk::vkEnumerateInstanceExtensionProperties(
            std::ptr::null(),
            &mut count,
            vk_properties.as_mut_ptr(),
        )
    } {
        vk::VkResult::VkSuccess => (),
        result => return Err(result),
    }

    let mut properties = Vec::with_capacity(count as usize);

    for p in &vk_properties {
        let extension_name_cstr = unsafe { std::ffi::CStr::from_ptr(p.extensionName.as_ptr()) };

        let extension_name = extension_name_cstr
            .to_str()
            .map(|s| s.to_owned())
            .unwrap_or("Failed to get a name".to_string());

        properties.push(ExtensionProperties {
            extension_name,
            spec_version: p.specVersion,
        });
    }

    Ok(properties)
}

pub fn enumerate_instance_version() -> Result<ApiVersion, vk::VkResult> {
    let mut version: u32 = 0;

    match unsafe { vk::vkEnumerateInstanceVersion(&mut version) } {
        vk::VkResult::VkSuccess => Ok(ApiVersion::from(version)),
        result => Err(result),
    }
}

fn string_to_cstr(s: &String) -> std::ffi::CString {
    std::ffi::CString::new(s.as_str()).expect("Wrong string parameter")
}

fn strings_to_cstrings(v: &Vec<String>) -> Vec<std::ffi::CString> {
    v.iter().map(|s: &String| string_to_cstr(s)).collect()
}

fn cstrings_to_ccharptrs(v: &Vec<std::ffi::CString>) -> Vec<*const std::os::raw::c_char> {
    v.iter().map(|s| s.as_ptr()).collect()
}

pub fn create_instance<'a>(create_info: &'a InstanceCreateInfo) -> Result<Instance, vk::VkResult> {
    let vk_application_info = match create_info.application_info {
        Some(info) => Some(vk::VkApplicationInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: std::ptr::null(),
            pApplicationName: string_to_cstr(&info.application_name).as_ptr(),
            applicationVersion: info.application_version,
            pEngineName: string_to_cstr(&info.engine_name).as_ptr(),
            engineVersion: info.engine_version,
            apiVersion: info.api_version.to_number(),
        }),
        None => None,
    };

    let layer_names_as_cstrings = strings_to_cstrings(&create_info.enabled_layer_names);
    let layer_names_as_ccharptrs = cstrings_to_ccharptrs(&layer_names_as_cstrings);

    let extension_names_as_cstrings = strings_to_cstrings(&create_info.enabled_extension_names);
    let extension_names_as_ccharptrs = cstrings_to_ccharptrs(&extension_names_as_cstrings);

    let vk_instance_create_info = vk::VkInstanceCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: std::ptr::null(),
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
    //let mut vk_instance: *mut vk::VkInstance = 0_usize as _;
    match unsafe {
        vk::vkCreateInstance(&vk_instance_create_info, std::ptr::null(), &mut vk_instance)
    } {
        vk::VkResult::VkSuccess => Ok(Instance {
            handle: vk_instance,
        }),
        result => Err(result),
    }
}
