use crate::vk;
use crate::vulkan;

#[allow(dead_code)]
pub fn get_physical_device_features(
    physical_device: &vulkan::PhysicalDevice,
) -> vulkan::PhysicalDeviceFeatures {
    let mut vk_features = vk::VkPhysicalDeviceFeatures::default();

    unsafe { vk::vkGetPhysicalDeviceFeatures(physical_device.handle, &mut vk_features) };

    vulkan::PhysicalDeviceFeatures {
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
