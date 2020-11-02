use crate::utils;
use crate::vk;

use raw_window_handle::HasRawWindowHandle;

#[derive(Debug, Default)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ApiVersion {
    pub fn from(version: u32) -> Self {
        ApiVersion {
            major: version >> 22,
            minor: (version >> 12) & 0x3ff,
            patch: version & 0xfff,
        }
    }

    pub fn to_number(&self) -> u32 {
        (self.major << 22) | (self.minor << 12) | (self.patch)
    }
}

#[derive(Debug)]
pub struct ExtensionProperties {
    pub extension_name: String,
    pub spec_version: u32,
}

#[derive(Debug)]
pub struct Instance {
    handle: *const vk::VkInstance,
}

impl Default for Instance {
    fn default() -> Instance {
        Instance {
            handle: std::ptr::null(),
        }
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
    pub enabled_extension_names: &'a Vec<String>,
}

pub fn create_instance<'a>(create_info: &'a InstanceCreateInfo) -> Result<Instance, vk::VkResult> {
    let vk_application_info = match create_info.application_info {
        Some(info) => Some(vk::VkApplicationInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: std::ptr::null(),
            pApplicationName: utils::string_to_cstr(&info.application_name).as_ptr(),
            applicationVersion: info.application_version,
            pEngineName: utils::string_to_cstr(&info.engine_name).as_ptr(),
            engineVersion: info.engine_version,
            apiVersion: info.api_version.to_number(),
        }),
        None => None,
    };

    let layer_names_as_cstrings = utils::strings_to_cstrings(&create_info.enabled_layer_names);
    let layer_names_as_ccharptrs = utils::cstrings_to_ccharptrs(&layer_names_as_cstrings);

    let extension_names_as_cstrings =
        utils::strings_to_cstrings(&create_info.enabled_extension_names);
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
    //let mut vk_instance: *mut vk::VkInstance = 0_usize as _;
    match unsafe {
        vk::vkCreateInstance(&vk_instance_create_info, std::ptr::null(), &mut vk_instance)
    } {
        vk::VkResult::VK_SUCCESS => Ok(Instance {
            handle: vk_instance,
        }),
        result => Err(result),
    }
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
        vk::VkResult::VK_SUCCESS => (),
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
        vk::VkResult::VK_SUCCESS => (),
        result => return Err(result),
    }

    let mut properties = Vec::with_capacity(count as usize);

    for p in &vk_properties {
        let extension_name_cstr = unsafe { std::ffi::CStr::from_ptr(p.extensionName.as_ptr()) };

        let extension_name = extension_name_cstr
            .to_str()
            .map(|s| s.to_owned())
            .unwrap_or("failed to get instance extension name".to_string());

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
        vk::VkResult::VK_SUCCESS => Ok(ApiVersion::from(version)),
        result => Err(result),
    }
}

#[derive(Debug)]
pub struct Surface {
    pub handle: *const vk::VkSurfaceKHR,
}

impl Default for Surface {
    fn default() -> Surface {
        Surface {
            handle: std::ptr::null(),
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Debug)]
struct Win32SurfaceCreateInfoKHR {
    next: *const std::ffi::c_void,
    flags: u32,
    hinstance: *mut std::ffi::c_void,
    hwnd: *mut std::ffi::c_void,
}

#[cfg(target_os = "windows")]
fn create_surface_win32<'a, 'b>(
    create_info: &'a Win32SurfaceCreateInfoKHR,
    instance: &'b Instance,
) -> Result<Surface, vk::VkResult> {
    let create_info = vk::VkWin32SurfaceCreateInfoKHR {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR,
        pNext: create_info.next,
        flags: create_info.flags,
        hinstance: create_info.hinstance,
        hwnd: create_info.hwnd,
    };

    let mut vk_surface = std::ptr::null_mut();

    match unsafe {
        vk::vkCreateWin32SurfaceKHR(
            instance.handle,
            &create_info,
            std::ptr::null(),
            &mut vk_surface,
        )
    } {
        vk::VkResult::VK_SUCCESS => Ok(Surface { handle: vk_surface }),
        result => Err(result),
    }
}

pub fn get_required_window_extensions(
    window: &winit::window::Window,
) -> Result<Vec<String>, vk::VkResult> {
    match window.raw_window_handle() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Windows(_) => {
            return Ok(vec![
                String::from("VK_KHR_surface"),
                String::from("VK_KHR_win32_surface"),
            ])
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(_) => {
            return Ok(vec![
                String::from("VK_KHR_surface"),
                String::from("VK_KHR_wayland_surface"),
            ])
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(_) => {
            return Ok(vec![
                String::from("VK_KHR_surface"),
                String::from("VK_KHR_xlib_surface"),
            ])
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(_) => {
            return Ok(vec![
                String::from("VK_KHR_surface"),
                String::from("VK_KHR_xcb_surface"),
            ])
        }

        _ => return Err(vk::VkResult::VK_ERROR_EXTENSION_NOT_PRESENT),
    };
}

pub fn create_surface(
    window: &winit::window::Window,
    instance: &Instance,
) -> Result<Surface, vk::VkResult> {
    match window.raw_window_handle() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Windows(handle) => {
            let create_info = Win32SurfaceCreateInfoKHR {
                next: std::ptr::null(),
                flags: 0,
                hinstance: handle.hinstance,
                hwnd: handle.hwnd,
            };

            match create_surface_win32(&create_info, &instance) {
                Ok(surface) => return Ok(surface),
                Err(err) => return Err(err),
            };
        }

        _ => return Err(vk::VkResult::VK_ERROR_SURFACE_LOST_KHR),
    };

    /*#if defined(VK_USE_PLATFORM_WIN32_KHR)

    #elif defined(VK_USE_PLATFORM_WAYLAND_KHR)
        VkWaylandSurfaceCreateInfoKHR createInfo;
        createInfo.sType = VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR;
        createInfo.pNext = NULL;
        createInfo.flags = 0;
        createInfo.display = demo->display;
        createInfo.surface = demo->window;

        err = vkCreateWaylandSurfaceKHR(demo->inst, &createInfo, NULL, &demo->surface);
    #elif defined(VK_USE_PLATFORM_ANDROID_KHR)
        VkAndroidSurfaceCreateInfoKHR createInfo;
        createInfo.sType = VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR;
        createInfo.pNext = NULL;
        createInfo.flags = 0;
        createInfo.window = (struct ANativeWindow *)(demo->window);

        err = vkCreateAndroidSurfaceKHR(demo->inst, &createInfo, NULL, &demo->surface);
    #elif defined(VK_USE_PLATFORM_XLIB_KHR)
        VkXlibSurfaceCreateInfoKHR createInfo;
        createInfo.sType = VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR;
        createInfo.pNext = NULL;
        createInfo.flags = 0;
        createInfo.dpy = demo->display;
        createInfo.window = demo->xlib_window;

        err = vkCreateXlibSurfaceKHR(demo->inst, &createInfo, NULL, &demo->surface);
    #elif defined(VK_USE_PLATFORM_XCB_KHR)
        VkXcbSurfaceCreateInfoKHR createInfo;
        createInfo.sType = VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR;
        createInfo.pNext = NULL;
        createInfo.flags = 0;
        createInfo.connection = demo->connection;
        createInfo.window = demo->xcb_window;

        err = vkCreateXcbSurfaceKHR(demo->inst, &createInfo, NULL, &demo->surface);
    #elif defined(VK_USE_PLATFORM_DIRECTFB_EXT)
        VkDirectFBSurfaceCreateInfoEXT createInfo;
        createInfo.sType = VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT;
        createInfo.pNext = NULL;
        createInfo.flags = 0;
        createInfo.dfb = demo->dfb;
        createInfo.surface = demo->window;

        err = vkCreateDirectFBSurfaceEXT(demo->inst, &createInfo, NULL, &demo->surface);
    #elif defined(VK_USE_PLATFORM_DISPLAY_KHR)
        err = demo_create_display_surface(demo);
    #elif defined(VK_USE_PLATFORM_METAL_EXT)
        VkMetalSurfaceCreateInfoEXT surface;
        surface.sType = VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT;
        surface.pNext = NULL;
        surface.flags = 0;
        surface.pLayer = demo->caMetalLayer;

        err = vkCreateMetalSurfaceEXT(demo->inst, &surface, NULL, &demo->surface);
    #endif
        assert(!err);*/
}

pub fn destroy_surface(
    instance: &Instance,
    surface: &Surface,
    allocator: *const vk::VkAllocationCallbacks,
) {
    unsafe { vk::vkDestroySurfaceKHR(instance.handle, surface.handle, allocator) };
}

#[derive(Debug)]
pub struct PhysicalDevice {
    handle: *const vk::VkPhysicalDevice,
}

impl Default for PhysicalDevice {
    fn default() -> PhysicalDevice {
        PhysicalDevice {
            handle: std::ptr::null(),
        }
    }
}

pub fn enumerate_physical_devices(
    instance: &Instance,
) -> Result<Vec<PhysicalDevice>, vk::VkResult> {
    let mut count: u32 = 0;

    match unsafe {
        vk::vkEnumeratePhysicalDevices(instance.handle, &mut count, std::ptr::null_mut())
    } {
        vk::VkResult::VK_SUCCESS => (),
        result => return Err(result),
    }

    let mut vk_physical_devices = Vec::with_capacity(count as usize);
    for _ in 0..count {
        vk_physical_devices.push(std::ptr::null_mut());
    }
    match unsafe {
        vk::vkEnumeratePhysicalDevices(
            instance.handle,
            &mut count,
            vk_physical_devices.as_mut_ptr(),
        )
    } {
        vk::VkResult::VK_SUCCESS => (),
        result => return Err(result),
    }

    Ok(vk_physical_devices
        .into_iter()
        .map(|device| PhysicalDevice { handle: device })
        .collect())
}

#[derive(Debug)]
pub struct PhysicalDeviceProperties {
    pub api_version: ApiVersion,
    pub driver_version: u32,
    pub device_id: u32,
    pub vendor_id: u32,
    pub device_type: vk::VkPhysicalDeviceType,
    pub device_name: String,
    pub pipeline_cache_uuid: [u8; vk::VK_UUID_SIZE],
    pub limits: vk::VkPhysicalDeviceLimits,
    pub sparse_properties: vk::VkPhysicalDeviceSparseProperties,
}

impl Default for PhysicalDeviceProperties {
    fn default() -> PhysicalDeviceProperties {
        PhysicalDeviceProperties {
            api_version: ApiVersion::default(),
            driver_version: 0,
            device_id: 0,
            vendor_id: 0,
            device_type: vk::VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_MAX_ENUM,
            device_name: String::new(),
            pipeline_cache_uuid: [0; vk::VK_UUID_SIZE],
            limits: vk::VkPhysicalDeviceLimits::default(),
            sparse_properties: vk::VkPhysicalDeviceSparseProperties::default(),
        }
    }
}

pub fn get_physical_device_properties(
    physical_device: &PhysicalDevice,
) -> PhysicalDeviceProperties {
    let mut vk_props = vk::VkPhysicalDeviceProperties::default();

    unsafe { vk::vkGetPhysicalDeviceProperties(physical_device.handle, &mut vk_props) };

    let device_name_cstr = unsafe { std::ffi::CStr::from_ptr(vk_props.deviceName.as_ptr()) };

    let device_name = device_name_cstr
        .to_str()
        .map(|s| s.to_owned())
        .unwrap_or("failed to get physical device name".to_string());

    PhysicalDeviceProperties {
        api_version: ApiVersion::from(vk_props.apiVersion),
        driver_version: vk_props.driverVersion,
        device_id: vk_props.deviceID,
        vendor_id: vk_props.vendorID,
        device_type: vk_props.deviceType,
        device_name: device_name,
        pipeline_cache_uuid: vk_props.pipelineCacheUUID,
        limits: vk_props.limits,
        sparse_properties: vk_props.sparseProperties,
    }
}

#[derive(Debug, Default)]
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: bool,
    pub full_draw_index_uint32: bool,
    pub image_cube_array: bool,
    pub independent_blend: bool,
    pub geometry_shader: bool,
    pub tessellation_shader: bool,
    pub sample_rate_shading: bool,
    pub dual_src_blend: bool,
    pub logic_op: bool,
    pub multi_draw_indirect: bool,
    pub draw_indirect_first_instance: bool,
    pub depth_clamp: bool,
    pub depth_bias_clamp: bool,
    pub fill_mode_non_solid: bool,
    pub depth_bounds: bool,
    pub wide_lines: bool,
    pub large_points: bool,
    pub alpha_to_one: bool,
    pub multi_viewport: bool,
    pub sampler_anisotropy: bool,
    pub texture_compression_etc2: bool,
    pub texture_compression_astc_ldr: bool,
    pub texture_compression_bc: bool,
    pub occlusion_query_precise: bool,
    pub pipeline_statistics_query: bool,
    pub vertex_pipeline_stores_and_atomics: bool,
    pub fragment_stores_and_atomics: bool,
    pub shader_tessellation_and_geometry_point_size: bool,
    pub shader_image_gather_extended: bool,
    pub shader_storage_image_extended_formats: bool,
    pub shader_storage_image_multisample: bool,
    pub shader_storage_image_read_without_format: bool,
    pub shader_storage_image_write_without_format: bool,
    pub shader_uniform_buffer_array_dynamic_indexing: bool,
    pub shader_sampled_image_array_dynamic_indexing: bool,
    pub shader_storage_buffer_array_dynamic_indexing: bool,
    pub shader_storage_image_array_dynamic_indexing: bool,
    pub shader_clip_distance: bool,
    pub shader_cull_distance: bool,
    pub shader_float64: bool,
    pub shader_int64: bool,
    pub shader_int16: bool,
    pub shader_resource_residency: bool,
    pub shader_resource_min_lod: bool,
    pub sparse_binding: bool,
    pub sparse_residency_buffer: bool,
    pub sparse_residency_image_2d: bool,
    pub sparse_residency_image_3d: bool,
    pub sparse_residency_2_samples: bool,
    pub sparse_residency_4_samples: bool,
    pub sparse_residency_8_samples: bool,
    pub sparse_residency_16_samples: bool,
    pub sparse_residency_aliased: bool,
    pub variable_multisample_rate: bool,
    pub inherited_queries: bool,
}

pub fn get_physical_device_features(physical_device: &PhysicalDevice) -> PhysicalDeviceFeatures {
    let mut vk_features = vk::VkPhysicalDeviceFeatures::default();

    unsafe { vk::vkGetPhysicalDeviceFeatures(physical_device.handle, &mut vk_features) };

    PhysicalDeviceFeatures {
        robust_buffer_access: vk_features.robustBufferAccess > 0,
        full_draw_index_uint32: vk_features.fullDrawIndexUint32 > 0,
        image_cube_array: vk_features.imageCubeArray > 0,
        independent_blend: vk_features.independentBlend > 0,
        geometry_shader: vk_features.geometryShader > 0,
        tessellation_shader: vk_features.tessellationShader > 0,
        sample_rate_shading: vk_features.sampleRateShading > 0,
        dual_src_blend: vk_features.dualSrcBlend > 0,
        logic_op: vk_features.logicOp > 0,
        multi_draw_indirect: vk_features.multiDrawIndirect > 0,
        draw_indirect_first_instance: vk_features.drawIndirectFirstInstance > 0,
        depth_clamp: vk_features.depthClamp > 0,
        depth_bias_clamp: vk_features.depthBiasClamp > 0,
        fill_mode_non_solid: vk_features.fillModeNonSolid > 0,
        depth_bounds: vk_features.depthBounds > 0,
        wide_lines: vk_features.wideLines > 0,
        large_points: vk_features.largePoints > 0,
        alpha_to_one: vk_features.alphaToOne > 0,
        multi_viewport: vk_features.multiViewport > 0,
        sampler_anisotropy: vk_features.samplerAnisotropy > 0,
        texture_compression_etc2: vk_features.textureCompressionETC2 > 0,
        texture_compression_astc_ldr: vk_features.textureCompressionASTC_LDR > 0,
        texture_compression_bc: vk_features.textureCompressionBC > 0,
        occlusion_query_precise: vk_features.occlusionQueryPrecise > 0,
        pipeline_statistics_query: vk_features.pipelineStatisticsQuery > 0,
        vertex_pipeline_stores_and_atomics: vk_features.vertexPipelineStoresAndAtomics > 0,
        fragment_stores_and_atomics: vk_features.fragmentStoresAndAtomics > 0,
        shader_tessellation_and_geometry_point_size: vk_features
            .shaderTessellationAndGeometryPointSize
            > 0,
        shader_image_gather_extended: vk_features.shaderImageGatherExtended > 0,
        shader_storage_image_extended_formats: vk_features.shaderStorageImageExtendedFormats > 0,
        shader_storage_image_multisample: vk_features.shaderStorageImageMultisample > 0,
        shader_storage_image_read_without_format: vk_features.shaderStorageImageReadWithoutFormat
            > 0,
        shader_storage_image_write_without_format: vk_features.shaderStorageImageWriteWithoutFormat
            > 0,
        shader_uniform_buffer_array_dynamic_indexing: vk_features
            .shaderUniformBufferArrayDynamicIndexing
            > 0,
        shader_sampled_image_array_dynamic_indexing: vk_features
            .shaderSampledImageArrayDynamicIndexing
            > 0,
        shader_storage_buffer_array_dynamic_indexing: vk_features
            .shaderStorageBufferArrayDynamicIndexing
            > 0,
        shader_storage_image_array_dynamic_indexing: vk_features
            .shaderStorageImageArrayDynamicIndexing
            > 0,
        shader_clip_distance: vk_features.shaderClipDistance > 0,
        shader_cull_distance: vk_features.shaderCullDistance > 0,
        shader_float64: vk_features.shaderFloat64 > 0,
        shader_int64: vk_features.shaderInt64 > 0,
        shader_int16: vk_features.shaderInt16 > 0,
        shader_resource_residency: vk_features.shaderResourceResidency > 0,
        shader_resource_min_lod: vk_features.shaderResourceMinLod > 0,
        sparse_binding: vk_features.sparseBinding > 0,
        sparse_residency_buffer: vk_features.sparseResidencyBuffer > 0,
        sparse_residency_image_2d: vk_features.sparseResidencyImage2D > 0,
        sparse_residency_image_3d: vk_features.sparseResidencyImage3D > 0,
        sparse_residency_2_samples: vk_features.sparseResidency2Samples > 0,
        sparse_residency_4_samples: vk_features.sparseResidency4Samples > 0,
        sparse_residency_8_samples: vk_features.sparseResidency8Samples > 0,
        sparse_residency_16_samples: vk_features.sparseResidency16Samples > 0,
        sparse_residency_aliased: vk_features.sparseResidencyAliased > 0,
        variable_multisample_rate: vk_features.variableMultisampleRate > 0,
        inherited_queries: vk_features.inheritedQueries > 0,
    }
}

pub fn enumerate_device_extension_properies(
    physical_device: &PhysicalDevice,
) -> Result<Vec<ExtensionProperties>, vk::VkResult> {
    let mut count: u32 = 0;

    match unsafe {
        vk::vkEnumerateDeviceExtensionProperties(
            physical_device.handle,
            std::ptr::null(),
            &mut count,
            std::ptr::null_mut(),
        )
    } {
        vk::VkResult::VK_SUCCESS => (),
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
        vk::vkEnumerateDeviceExtensionProperties(
            physical_device.handle,
            std::ptr::null(),
            &mut count,
            vk_properties.as_mut_ptr(),
        )
    } {
        vk::VkResult::VK_SUCCESS => (),
        result => return Err(result),
    }

    let mut properties = Vec::with_capacity(count as usize);

    for p in &vk_properties {
        let extension_name_cstr = unsafe { std::ffi::CStr::from_ptr(p.extensionName.as_ptr()) };

        let extension_name = extension_name_cstr
            .to_str()
            .map(|s| s.to_owned())
            .unwrap_or("failed to get device extension name".to_string());

        properties.push(ExtensionProperties {
            extension_name,
            spec_version: p.specVersion,
        });
    }

    Ok(properties)
}
