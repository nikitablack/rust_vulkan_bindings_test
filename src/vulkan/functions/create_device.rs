use crate::utils;
use crate::vk;
use crate::vulkan;

#[allow(dead_code)]
pub fn create_device(
    physical_device: &vulkan::PhysicalDevice,
    create_info: &vulkan::DeviceCreateInfo,
    _allocator: *const vulkan::AllocationCallbacks, // TODO
) -> Result<vulkan::Device, vk::VkResult> {
    let vk_queue_create_infos = create_info
        .queue_create_infos
        .iter()
        .map(|info| vk::VkDeviceQueueCreateInfo {
            sType: vk::VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            pNext: std::ptr::null(),
            flags: info.flags,
            queueFamilyIndex: info.queue_family_index,
            queueCount: info.queue_priorities.len() as u32,
            pQueuePriorities: info.queue_priorities.as_ptr(),
        })
        .collect::<Vec<_>>();

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

    let vk_features = vk::VkPhysicalDeviceFeatures {
        robustBufferAccess: if create_info.enabled_features.robust_buffer_access {
            1
        } else {
            0
        },
        fullDrawIndexUint32: if create_info.enabled_features.full_draw_index_uint32 {
            1
        } else {
            0
        },
        imageCubeArray: if create_info.enabled_features.image_cube_array {
            1
        } else {
            0
        },
        independentBlend: if create_info.enabled_features.independent_blend {
            1
        } else {
            0
        },
        geometryShader: if create_info.enabled_features.geometry_shader {
            1
        } else {
            0
        },
        tessellationShader: if create_info.enabled_features.tessellation_shader {
            1
        } else {
            0
        },
        sampleRateShading: if create_info.enabled_features.sample_rate_shading {
            1
        } else {
            0
        },
        dualSrcBlend: if create_info.enabled_features.dual_src_blend {
            1
        } else {
            0
        },
        logicOp: if create_info.enabled_features.logic_op {
            1
        } else {
            0
        },
        multiDrawIndirect: if create_info.enabled_features.multi_draw_indirect {
            1
        } else {
            0
        },
        drawIndirectFirstInstance: if create_info.enabled_features.draw_indirect_first_instance {
            1
        } else {
            0
        },
        depthClamp: if create_info.enabled_features.depth_clamp {
            1
        } else {
            0
        },
        depthBiasClamp: if create_info.enabled_features.depth_bias_clamp {
            1
        } else {
            0
        },
        fillModeNonSolid: if create_info.enabled_features.fill_mode_non_solid {
            1
        } else {
            0
        },
        depthBounds: if create_info.enabled_features.depth_bounds {
            1
        } else {
            0
        },
        wideLines: if create_info.enabled_features.wide_lines {
            1
        } else {
            0
        },
        largePoints: if create_info.enabled_features.large_points {
            1
        } else {
            0
        },
        alphaToOne: if create_info.enabled_features.alpha_to_one {
            1
        } else {
            0
        },
        multiViewport: if create_info.enabled_features.multi_viewport {
            1
        } else {
            0
        },
        samplerAnisotropy: if create_info.enabled_features.sampler_anisotropy {
            1
        } else {
            0
        },
        textureCompressionETC2: if create_info.enabled_features.texture_compression_etc2 {
            1
        } else {
            0
        },
        textureCompressionASTC_LDR: if create_info.enabled_features.texture_compression_astc_ldr {
            1
        } else {
            0
        },
        textureCompressionBC: if create_info.enabled_features.texture_compression_bc {
            1
        } else {
            0
        },
        occlusionQueryPrecise: if create_info.enabled_features.occlusion_query_precise {
            1
        } else {
            0
        },
        pipelineStatisticsQuery: if create_info.enabled_features.pipeline_statistics_query {
            1
        } else {
            0
        },
        vertexPipelineStoresAndAtomics: if create_info
            .enabled_features
            .vertex_pipeline_stores_and_atomics
        {
            1
        } else {
            0
        },
        fragmentStoresAndAtomics: if create_info.enabled_features.fragment_stores_and_atomics {
            1
        } else {
            0
        },
        shaderTessellationAndGeometryPointSize: if create_info
            .enabled_features
            .shader_tessellation_and_geometry_point_size
        {
            1
        } else {
            0
        },
        shaderImageGatherExtended: if create_info.enabled_features.shader_image_gather_extended {
            1
        } else {
            0
        },
        shaderStorageImageExtendedFormats: if create_info
            .enabled_features
            .shader_storage_image_extended_formats
        {
            1
        } else {
            0
        },
        shaderStorageImageMultisample: if create_info
            .enabled_features
            .shader_storage_image_multisample
        {
            1
        } else {
            0
        },
        shaderStorageImageReadWithoutFormat: if create_info
            .enabled_features
            .shader_storage_image_read_without_format
        {
            1
        } else {
            0
        },
        shaderStorageImageWriteWithoutFormat: if create_info
            .enabled_features
            .shader_storage_image_write_without_format
        {
            1
        } else {
            0
        },
        shaderUniformBufferArrayDynamicIndexing: if create_info
            .enabled_features
            .shader_uniform_buffer_array_dynamic_indexing
        {
            1
        } else {
            0
        },
        shaderSampledImageArrayDynamicIndexing: if create_info
            .enabled_features
            .shader_sampled_image_array_dynamic_indexing
        {
            1
        } else {
            0
        },
        shaderStorageBufferArrayDynamicIndexing: if create_info
            .enabled_features
            .shader_storage_buffer_array_dynamic_indexing
        {
            1
        } else {
            0
        },
        shaderStorageImageArrayDynamicIndexing: if create_info
            .enabled_features
            .shader_storage_image_array_dynamic_indexing
        {
            1
        } else {
            0
        },
        shaderClipDistance: if create_info.enabled_features.shader_clip_distance {
            1
        } else {
            0
        },
        shaderCullDistance: if create_info.enabled_features.shader_cull_distance {
            1
        } else {
            0
        },
        shaderFloat64: if create_info.enabled_features.shader_float64 {
            1
        } else {
            0
        },
        shaderInt64: if create_info.enabled_features.shader_int64 {
            1
        } else {
            0
        },
        shaderInt16: if create_info.enabled_features.shader_int16 {
            1
        } else {
            0
        },
        shaderResourceResidency: if create_info.enabled_features.shader_resource_residency {
            1
        } else {
            0
        },
        shaderResourceMinLod: if create_info.enabled_features.shader_resource_min_lod {
            1
        } else {
            0
        },
        sparseBinding: if create_info.enabled_features.sparse_binding {
            1
        } else {
            0
        },
        sparseResidencyBuffer: if create_info.enabled_features.sparse_residency_buffer {
            1
        } else {
            0
        },
        sparseResidencyImage2D: if create_info.enabled_features.sparse_residency_image_2d {
            1
        } else {
            0
        },
        sparseResidencyImage3D: if create_info.enabled_features.sparse_residency_image_3d {
            1
        } else {
            0
        },
        sparseResidency2Samples: if create_info.enabled_features.sparse_residency_2_samples {
            1
        } else {
            0
        },
        sparseResidency4Samples: if create_info.enabled_features.sparse_residency_4_samples {
            1
        } else {
            0
        },
        sparseResidency8Samples: if create_info.enabled_features.sparse_residency_8_samples {
            1
        } else {
            0
        },
        sparseResidency16Samples: if create_info.enabled_features.sparse_residency_16_samples {
            1
        } else {
            0
        },
        sparseResidencyAliased: if create_info.enabled_features.sparse_residency_aliased {
            1
        } else {
            0
        },
        variableMultisampleRate: if create_info.enabled_features.variable_multisample_rate {
            1
        } else {
            0
        },
        inheritedQueries: if create_info.enabled_features.inherited_queries {
            1
        } else {
            0
        },
    };

    let vk_create_info = vk::VkDeviceCreateInfo {
        sType: vk::VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        pNext: std::ptr::null(), // TODO
        flags: create_info.flags,
        queueCreateInfoCount: vk_queue_create_infos.len() as u32,
        pQueueCreateInfos: vk_queue_create_infos.as_ptr(),
        enabledLayerCount: layer_names_as_ccharptrs.len() as u32,
        ppEnabledLayerNames: layer_names_as_ccharptrs.as_ptr(),
        enabledExtensionCount: extension_names_as_ccharptrs.len() as u32,
        ppEnabledExtensionNames: extension_names_as_ccharptrs.as_ptr(),
        pEnabledFeatures: &vk_features,
    };

    let mut vk_device = std::ptr::null_mut();

    match unsafe {
        vk::vkCreateDevice(
            physical_device.handle,
            &vk_create_info,
            std::ptr::null(),
            &mut vk_device,
        ) // TODO
    } {
        vk::VkResult::VK_SUCCESS => Ok(vulkan::Device { handle: vk_device }),
        result => Err(result),
    }
}
