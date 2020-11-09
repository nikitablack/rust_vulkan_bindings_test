use crate::vk;

#[repr(C)]
pub struct AllocationCallbacks {
    pub user_data: *const std::ffi::c_void,
    //PFN_vkAllocationFunction                pfnAllocation;
    //PFN_vkReallocationFunction              pfnReallocation;
    //PFN_vkFreeFunction                      pfnFree;
    //PFN_vkInternalAllocationNotification    pfnInternalAllocation;
    //PFN_vkInternalFreeNotification          pfnInternalFree;
}

#[derive(Debug, Default)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ApiVersion {
    pub fn from(version: u32) -> Self {
        Self {
            major: version >> 22,
            minor: (version >> 12) & 0x3ff,
            patch: version & 0xfff,
        }
    }

    pub fn to_number(&self) -> u32 {
        (self.major << 22) | (self.minor << 12) | (self.patch)
    }
}

#[derive(Debug, Default)]
pub struct ApplicationInfo {
    pub application_name: String,
    pub application_version: u32,
    pub engine_name: String,
    pub engine_version: u32,
    pub api_version: ApiVersion,
}

#[derive(Debug)]
pub struct DeviceCreateInfo<'a, 'b> {
    pub next: *const std::ffi::c_void,
    pub flags: vk::VkDeviceCreateFlags,
    pub queue_create_infos: Vec<DeviceQueueCreateInfo>,
    pub enabled_layer_names: Option<&'a Vec<String>>,
    pub enabled_extension_names: Option<&'b Vec<String>>,
    pub enabled_features: PhysicalDeviceFeatures,
}

impl<'a, 'b> Default for DeviceCreateInfo<'_, '_> {
    fn default() -> Self {
        Self {
            next: std::ptr::null(),
            flags: vk::VkDeviceCreateFlags::empty(),
            queue_create_infos: Vec::default(),
            enabled_layer_names: None,
            enabled_extension_names: None,
            enabled_features: PhysicalDeviceFeatures::default(),
        }
    }
}

#[derive(Debug)]
pub struct DeviceQueueCreateInfo {
    pub next: *const std::ffi::c_void,
    pub flags: vk::VkDeviceQueueCreateFlagBits,
    pub queue_family_index: u32,
    pub queue_priorities: Vec<f32>,
}

impl Default for DeviceQueueCreateInfo {
    fn default() -> Self {
        Self {
            next: std::ptr::null(),
            flags: vk::VkDeviceQueueCreateFlagBits::empty(),
            queue_family_index: 0,
            queue_priorities: Vec::<f32>::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ExtensionProperties {
    pub extension_name: String,
    pub spec_version: u32,
}

#[derive(Debug, Default)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Debug)]
pub struct FormatProperties {
    pub linear_tiling_features: vk::VkFormatFeatureFlagBits,
    pub optimal_tiling_features: vk::VkFormatFeatureFlagBits,
    pub buffer_features: vk::VkFormatFeatureFlagBits,
}

impl Default for FormatProperties {
    fn default() -> Self {
        Self {
            linear_tiling_features: vk::VkFormatFeatureFlagBits::empty(),
            optimal_tiling_features: vk::VkFormatFeatureFlagBits::empty(),
            buffer_features: vk::VkFormatFeatureFlagBits::empty(),
        }
    }
}

#[derive(Debug)]
pub struct InstanceCreateInfo<'a, 'b, 'c> {
    pub next: *const std::ffi::c_void,
    pub flags: u32,
    pub application_info: Option<&'a ApplicationInfo>,
    pub enabled_layer_names: Option<&'b Vec<String>>,
    pub enabled_extension_names: Option<&'c Vec<String>>,
}

impl<'a, 'b, 'c> Default for InstanceCreateInfo<'_, '_, '_> {
    fn default() -> Self {
        Self {
            next: std::ptr::null(),
            flags: 0,
            application_info: None,
            enabled_layer_names: None,
            enabled_extension_names: None,
        }
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

#[derive(Debug, Default)]
pub struct PhysicalDeviceLimits {
    pub max_image_dimension_1d: u32,
    pub max_image_dimension_2d: u32,
    pub max_image_dimension_3d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: u64,
    pub sparse_address_space_size: u64,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [f32; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: u64,
    pub min_uniform_buffer_offset_alignment: u64,
    pub min_storage_buffer_offset_alignment: u64,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: u32,
    pub framebuffer_depth_sample_counts: u32,
    pub framebuffer_stencil_sample_counts: u32,
    pub framebuffer_no_attachments_sample_counts: u32,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: u32,
    pub sampled_image_integer_sample_counts: u32,
    pub sampled_image_depth_sample_counts: u32,
    pub sampled_image_stencil_sample_counts: u32,
    pub storage_image_sample_counts: u32,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: u32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: u32,
    pub standard_sample_locations: u32,
    pub optimal_buffer_copy_offset_alignment: u64,
    pub optimal_buffer_copy_row_pitch_alignment: u64,
    pub non_coherent_atom_size: u64,
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
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}

impl Default for PhysicalDeviceProperties {
    fn default() -> Self {
        Self {
            api_version: ApiVersion::default(),
            driver_version: 0,
            device_id: 0,
            vendor_id: 0,
            device_type: vk::VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_OTHER,
            device_name: String::new(),
            pipeline_cache_uuid: [0; vk::VK_UUID_SIZE],
            limits: PhysicalDeviceLimits::default(),
            sparse_properties: PhysicalDeviceSparseProperties::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: u32,
    pub residency_standard_2d_multisample_block_shape: u32,
    pub residency_standard_3d_block_shape: u32,
    pub residency_aligned_mip_size: u32,
    pub residency_non_resident_strict: u32,
}

#[derive(Debug)]
pub struct QueueFamilyProperties {
    pub queue_flags: vk::VkQueueFlagBits,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}

impl Default for QueueFamilyProperties {
    fn default() -> Self {
        Self {
            queue_flags: vk::VkQueueFlagBits::empty(),
            queue_count: 0,
            timestamp_valid_bits: 0,
            min_image_transfer_granularity: Extent3D::default(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SurfaceFormatKHR {
    pub format: vk::VkFormat,
    pub color_space: vk::VkColorSpaceKHR,
}

impl Default for SurfaceFormatKHR {
    fn default() -> Self {
        Self {
            format: vk::VkFormat::VK_FORMAT_UNDEFINED,
            color_space: vk::VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
        }
    }
}

#[derive(Debug)]
pub struct Win32SurfaceCreateInfoKHR {
    pub next: *const std::ffi::c_void,
    pub flags: u32,
    pub hinstance: *mut std::ffi::c_void,
    pub hwnd: *mut std::ffi::c_void,
}

impl Default for Win32SurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            next: std::ptr::null(),
            flags: 0,
            hinstance: std::ptr::null_mut(),
            hwnd: std::ptr::null_mut(),
        }
    }
}
