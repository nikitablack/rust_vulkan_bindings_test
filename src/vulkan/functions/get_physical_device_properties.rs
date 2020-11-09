use crate::vk;
use crate::vulkan;

#[allow(dead_code)]
pub fn get_physical_device_properties(
    physical_device: &vulkan::PhysicalDevice,
) -> vulkan::PhysicalDeviceProperties {
    let mut vk_props = vk::VkPhysicalDeviceProperties::default();

    unsafe { vk::vkGetPhysicalDeviceProperties(physical_device.handle, &mut vk_props) };

    let device_name_cstr = unsafe { std::ffi::CStr::from_ptr(vk_props.deviceName.as_ptr()) };

    let device_name = device_name_cstr
        .to_str()
        .map(|s| s.to_owned())
        .unwrap_or("failed to get physical device name".to_string());

    let limits = vulkan::PhysicalDeviceLimits {
        max_image_dimension_1d: vk_props.limits.maxImageDimension1D,
        max_image_dimension_2d: vk_props.limits.maxImageDimension2D,
        max_image_dimension_3d: vk_props.limits.maxImageDimension3D,
        max_image_dimension_cube: vk_props.limits.maxImageDimensionCube,
        max_image_array_layers: vk_props.limits.maxImageArrayLayers,
        max_texel_buffer_elements: vk_props.limits.maxTexelBufferElements,
        max_uniform_buffer_range: vk_props.limits.maxUniformBufferRange,
        max_storage_buffer_range: vk_props.limits.maxStorageBufferRange,
        max_push_constants_size: vk_props.limits.maxPushConstantsSize,
        max_memory_allocation_count: vk_props.limits.maxMemoryAllocationCount,
        max_sampler_allocation_count: vk_props.limits.maxSamplerAllocationCount,
        buffer_image_granularity: vk_props.limits.bufferImageGranularity,
        sparse_address_space_size: vk_props.limits.sparseAddressSpaceSize,
        max_bound_descriptor_sets: vk_props.limits.maxBoundDescriptorSets,
        max_per_stage_descriptor_samplers: vk_props.limits.maxPerStageDescriptorSamplers,
        max_per_stage_descriptor_uniform_buffers: vk_props
            .limits
            .maxPerStageDescriptorUniformBuffers,
        max_per_stage_descriptor_storage_buffers: vk_props
            .limits
            .maxPerStageDescriptorStorageBuffers,
        max_per_stage_descriptor_sampled_images: vk_props.limits.maxPerStageDescriptorSampledImages,
        max_per_stage_descriptor_storage_images: vk_props.limits.maxPerStageDescriptorStorageImages,
        max_per_stage_descriptor_input_attachments: vk_props
            .limits
            .maxPerStageDescriptorInputAttachments,
        max_per_stage_resources: vk_props.limits.maxPerStageResources,
        max_descriptor_set_samplers: vk_props.limits.maxDescriptorSetSamplers,
        max_descriptor_set_uniform_buffers: vk_props.limits.maxDescriptorSetUniformBuffers,
        max_descriptor_set_uniform_buffers_dynamic: vk_props
            .limits
            .maxDescriptorSetUniformBuffersDynamic,
        max_descriptor_set_storage_buffers: vk_props.limits.maxDescriptorSetStorageBuffers,
        max_descriptor_set_storage_buffers_dynamic: vk_props
            .limits
            .maxDescriptorSetStorageBuffersDynamic,
        max_descriptor_set_sampled_images: vk_props.limits.maxDescriptorSetSampledImages,
        max_descriptor_set_storage_images: vk_props.limits.maxDescriptorSetStorageImages,
        max_descriptor_set_input_attachments: vk_props.limits.maxDescriptorSetInputAttachments,
        max_vertex_input_attributes: vk_props.limits.maxVertexInputAttributes,
        max_vertex_input_bindings: vk_props.limits.maxVertexInputBindings,
        max_vertex_input_attribute_offset: vk_props.limits.maxVertexInputAttributeOffset,
        max_vertex_input_binding_stride: vk_props.limits.maxVertexInputBindingStride,
        max_vertex_output_components: vk_props.limits.maxVertexOutputComponents,
        max_tessellation_generation_level: vk_props.limits.maxTessellationGenerationLevel,
        max_tessellation_patch_size: vk_props.limits.maxTessellationPatchSize,
        max_tessellation_control_per_vertex_input_components: vk_props
            .limits
            .maxTessellationControlPerVertexInputComponents,
        max_tessellation_control_per_vertex_output_components: vk_props
            .limits
            .maxTessellationControlPerVertexOutputComponents,
        max_tessellation_control_per_patch_output_components: vk_props
            .limits
            .maxTessellationControlPerPatchOutputComponents,
        max_tessellation_control_total_output_components: vk_props
            .limits
            .maxTessellationControlTotalOutputComponents,
        max_tessellation_evaluation_input_components: vk_props
            .limits
            .maxTessellationEvaluationInputComponents,
        max_tessellation_evaluation_output_components: vk_props
            .limits
            .maxTessellationEvaluationOutputComponents,
        max_geometry_shader_invocations: vk_props.limits.maxGeometryShaderInvocations,
        max_geometry_input_components: vk_props.limits.maxGeometryInputComponents,
        max_geometry_output_components: vk_props.limits.maxGeometryOutputComponents,
        max_geometry_output_vertices: vk_props.limits.maxGeometryOutputVertices,
        max_geometry_total_output_components: vk_props.limits.maxGeometryTotalOutputComponents,
        max_fragment_input_components: vk_props.limits.maxFragmentInputComponents,
        max_fragment_output_attachments: vk_props.limits.maxFragmentOutputAttachments,
        max_fragment_dual_src_attachments: vk_props.limits.maxFragmentDualSrcAttachments,
        max_fragment_combined_output_resources: vk_props.limits.maxFragmentCombinedOutputResources,
        max_compute_shared_memory_size: vk_props.limits.maxComputeSharedMemorySize,
        max_compute_work_group_count: vk_props.limits.maxComputeWorkGroupCount,
        max_compute_work_group_invocations: vk_props.limits.maxComputeWorkGroupInvocations,
        max_compute_work_group_size: vk_props.limits.maxComputeWorkGroupSize,
        sub_pixel_precision_bits: vk_props.limits.subPixelPrecisionBits,
        sub_texel_precision_bits: vk_props.limits.subTexelPrecisionBits,
        mipmap_precision_bits: vk_props.limits.mipmapPrecisionBits,
        max_draw_indexed_index_value: vk_props.limits.maxDrawIndexedIndexValue,
        max_draw_indirect_count: vk_props.limits.maxDrawIndirectCount,
        max_sampler_lod_bias: vk_props.limits.maxSamplerLodBias,
        max_sampler_anisotropy: vk_props.limits.maxSamplerAnisotropy,
        max_viewports: vk_props.limits.maxViewports,
        max_viewport_dimensions: vk_props.limits.maxViewportDimensions,
        viewport_bounds_range: vk_props.limits.viewportBoundsRange,
        viewport_sub_pixel_bits: vk_props.limits.viewportSubPixelBits,
        min_memory_map_alignment: vk_props.limits.minMemoryMapAlignment,
        min_texel_buffer_offset_alignment: vk_props.limits.minTexelBufferOffsetAlignment,
        min_uniform_buffer_offset_alignment: vk_props.limits.minUniformBufferOffsetAlignment,
        min_storage_buffer_offset_alignment: vk_props.limits.minStorageBufferOffsetAlignment,
        min_texel_offset: vk_props.limits.minTexelOffset,
        max_texel_offset: vk_props.limits.maxTexelOffset,
        min_texel_gather_offset: vk_props.limits.minTexelGatherOffset,
        max_texel_gather_offset: vk_props.limits.maxTexelGatherOffset,
        min_interpolation_offset: vk_props.limits.minInterpolationOffset,
        max_interpolation_offset: vk_props.limits.maxInterpolationOffset,
        sub_pixel_interpolation_offset_bits: vk_props.limits.subPixelInterpolationOffsetBits,
        max_framebuffer_width: vk_props.limits.maxFramebufferWidth,
        max_framebuffer_height: vk_props.limits.maxFramebufferHeight,
        max_framebuffer_layers: vk_props.limits.maxFramebufferLayers,
        framebuffer_color_sample_counts: vk_props.limits.framebufferColorSampleCounts,
        framebuffer_depth_sample_counts: vk_props.limits.framebufferDepthSampleCounts,
        framebuffer_stencil_sample_counts: vk_props.limits.framebufferStencilSampleCounts,
        framebuffer_no_attachments_sample_counts: vk_props
            .limits
            .framebufferNoAttachmentsSampleCounts,
        max_color_attachments: vk_props.limits.maxColorAttachments,
        sampled_image_color_sample_counts: vk_props.limits.sampledImageColorSampleCounts,
        sampled_image_integer_sample_counts: vk_props.limits.sampledImageIntegerSampleCounts,
        sampled_image_depth_sample_counts: vk_props.limits.sampledImageDepthSampleCounts,
        sampled_image_stencil_sample_counts: vk_props.limits.sampledImageStencilSampleCounts,
        storage_image_sample_counts: vk_props.limits.storageImageSampleCounts,
        max_sample_mask_words: vk_props.limits.maxSampleMaskWords,
        timestamp_compute_and_graphics: vk_props.limits.timestampComputeAndGraphics,
        timestamp_period: vk_props.limits.timestampPeriod,
        max_clip_distances: vk_props.limits.maxClipDistances,
        max_cull_distances: vk_props.limits.maxCullDistances,
        max_combined_clip_and_cull_distances: vk_props.limits.maxCombinedClipAndCullDistances,
        discrete_queue_priorities: vk_props.limits.discreteQueuePriorities,
        point_size_range: vk_props.limits.pointSizeRange,
        line_width_range: vk_props.limits.lineWidthRange,
        point_size_granularity: vk_props.limits.pointSizeGranularity,
        line_width_granularity: vk_props.limits.lineWidthGranularity,
        strict_lines: vk_props.limits.strictLines,
        standard_sample_locations: vk_props.limits.standardSampleLocations,
        optimal_buffer_copy_offset_alignment: vk_props.limits.optimalBufferCopyOffsetAlignment,
        optimal_buffer_copy_row_pitch_alignment: vk_props.limits.optimalBufferCopyRowPitchAlignment,
        non_coherent_atom_size: vk_props.limits.nonCoherentAtomSize,
    };

    let sparse_properties = vulkan::PhysicalDeviceSparseProperties {
        residency_standard_2d_block_shape: vk_props.sparseProperties.residencyStandard2DBlockShape,
        residency_standard_2d_multisample_block_shape: vk_props
            .sparseProperties
            .residencyStandard2DMultisampleBlockShape,
        residency_standard_3d_block_shape: vk_props.sparseProperties.residencyStandard3DBlockShape,
        residency_aligned_mip_size: vk_props.sparseProperties.residencyAlignedMipSize,
        residency_non_resident_strict: vk_props.sparseProperties.residencyNonResidentStrict,
    };

    vulkan::PhysicalDeviceProperties {
        api_version: vulkan::ApiVersion::from(vk_props.apiVersion),
        driver_version: vk_props.driverVersion,
        device_id: vk_props.deviceID,
        vendor_id: vk_props.vendorID,
        device_type: vk_props.deviceType,
        device_name: device_name,
        pipeline_cache_uuid: vk_props.pipelineCacheUUID,
        limits: limits,
        sparse_properties: sparse_properties,
    }
}
