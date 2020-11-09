use crate::vk;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct VkAllocationCallbacks {
    pub pUserData: *const std::ffi::c_void,
    //PFN_vkAllocationFunction                pfnAllocation;
    //PFN_vkReallocationFunction              pfnReallocation;
    //PFN_vkFreeFunction                      pfnFree;
    //PFN_vkInternalAllocationNotification    pfnInternalAllocation;
    //PFN_vkInternalFreeNotification          pfnInternalFree;
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct VkApplicationInfo {
    pub sType: vk::VkStructureType,
    pub pNext: *const std::ffi::c_void,
    pub pApplicationName: *const std::os::raw::c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const std::os::raw::c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct VkDeviceCreateInfo {
    pub sType: vk::VkStructureType,
    pub pNext: *const std::ffi::c_void,
    pub flags: vk::VkDeviceCreateFlags,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const std::os::raw::c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const std::os::raw::c_char,
    pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
    pub sType: vk::VkStructureType,
    pub pNext: *const std::ffi::c_void,
    pub flags: vk::VkDeviceQueueCreateFlagBits,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const f32,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct VkExtensionProperties {
    pub extensionName: [std::os::raw::c_char; vk::VK_MAX_EXTENSION_NAME_SIZE],
    pub specVersion: u32,
}

impl Default for VkExtensionProperties {
    fn default() -> Self {
        Self {
            extensionName: [0; vk::VK_MAX_EXTENSION_NAME_SIZE],
            specVersion: 0,
        }
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Default)]
#[repr(C)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct VkFormatProperties {
    pub linearTilingFeatures: vk::VkFormatFeatureFlagBits,
    pub optimalTilingFeatures: vk::VkFormatFeatureFlagBits,
    pub bufferFeatures: vk::VkFormatFeatureFlagBits,
}

impl Default for VkFormatProperties {
    fn default() -> Self {
        Self {
            linearTilingFeatures: vk::VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_FLAG_BITS_MAX_ENUM,
            optimalTilingFeatures:
                vk::VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_FLAG_BITS_MAX_ENUM,
            bufferFeatures: vk::VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_FLAG_BITS_MAX_ENUM,
        }
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub sType: vk::VkStructureType,
    pub pNext: *const std::ffi::c_void,
    pub flags: u32,
    pub pApplicationInfo: *const vk::VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const std::os::raw::c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const std::os::raw::c_char,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Default)]
#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
    pub robustBufferAccess: u32,
    pub fullDrawIndexUint32: u32,
    pub imageCubeArray: u32,
    pub independentBlend: u32,
    pub geometryShader: u32,
    pub tessellationShader: u32,
    pub sampleRateShading: u32,
    pub dualSrcBlend: u32,
    pub logicOp: u32,
    pub multiDrawIndirect: u32,
    pub drawIndirectFirstInstance: u32,
    pub depthClamp: u32,
    pub depthBiasClamp: u32,
    pub fillModeNonSolid: u32,
    pub depthBounds: u32,
    pub wideLines: u32,
    pub largePoints: u32,
    pub alphaToOne: u32,
    pub multiViewport: u32,
    pub samplerAnisotropy: u32,
    pub textureCompressionETC2: u32,
    pub textureCompressionASTC_LDR: u32,
    pub textureCompressionBC: u32,
    pub occlusionQueryPrecise: u32,
    pub pipelineStatisticsQuery: u32,
    pub vertexPipelineStoresAndAtomics: u32,
    pub fragmentStoresAndAtomics: u32,
    pub shaderTessellationAndGeometryPointSize: u32,
    pub shaderImageGatherExtended: u32,
    pub shaderStorageImageExtendedFormats: u32,
    pub shaderStorageImageMultisample: u32,
    pub shaderStorageImageReadWithoutFormat: u32,
    pub shaderStorageImageWriteWithoutFormat: u32,
    pub shaderUniformBufferArrayDynamicIndexing: u32,
    pub shaderSampledImageArrayDynamicIndexing: u32,
    pub shaderStorageBufferArrayDynamicIndexing: u32,
    pub shaderStorageImageArrayDynamicIndexing: u32,
    pub shaderClipDistance: u32,
    pub shaderCullDistance: u32,
    pub shaderFloat64: u32,
    pub shaderInt64: u32,
    pub shaderInt16: u32,
    pub shaderResourceResidency: u32,
    pub shaderResourceMinLod: u32,
    pub sparseBinding: u32,
    pub sparseResidencyBuffer: u32,
    pub sparseResidencyImage2D: u32,
    pub sparseResidencyImage3D: u32,
    pub sparseResidency2Samples: u32,
    pub sparseResidency4Samples: u32,
    pub sparseResidency8Samples: u32,
    pub sparseResidency16Samples: u32,
    pub sparseResidencyAliased: u32,
    pub variableMultisampleRate: u32,
    pub inheritedQueries: u32,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Default)]
#[repr(C)]
pub struct VkPhysicalDeviceLimits {
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: u64,
    pub sparseAddressSpaceSize: u64,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: f32,
    pub maxSamplerAnisotropy: f32,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2],
    pub viewportBoundsRange: [f32; 2],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: usize,
    pub minTexelBufferOffsetAlignment: u64,
    pub minUniformBufferOffsetAlignment: u64,
    pub minStorageBufferOffsetAlignment: u64,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: f32,
    pub maxInterpolationOffset: f32,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: u32,
    pub framebufferDepthSampleCounts: u32,
    pub framebufferStencilSampleCounts: u32,
    pub framebufferNoAttachmentsSampleCounts: u32,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: u32,
    pub sampledImageIntegerSampleCounts: u32,
    pub sampledImageDepthSampleCounts: u32,
    pub sampledImageStencilSampleCounts: u32,
    pub storageImageSampleCounts: u32,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: u32,
    pub timestampPeriod: f32,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [f32; 2],
    pub lineWidthRange: [f32; 2],
    pub pointSizeGranularity: f32,
    pub lineWidthGranularity: f32,
    pub strictLines: u32,
    pub standardSampleLocations: u32,
    pub optimalBufferCopyOffsetAlignment: u64,
    pub optimalBufferCopyRowPitchAlignment: u64,
    pub nonCoherentAtomSize: u64,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub deviceID: u32,
    pub vendorID: u32,
    pub deviceType: vk::VkPhysicalDeviceType,
    pub deviceName: [std::os::raw::c_char; vk::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pub pipelineCacheUUID: [u8; vk::VK_UUID_SIZE],
    pub limits: vk::VkPhysicalDeviceLimits,
    pub sparseProperties: vk::VkPhysicalDeviceSparseProperties,
}

impl Default for VkPhysicalDeviceProperties {
    fn default() -> Self {
        Self {
            apiVersion: 0,
            driverVersion: 0,
            deviceID: 0,
            vendorID: 0,
            deviceType: vk::VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_MAX_ENUM,
            deviceName: [0; vk::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
            pipelineCacheUUID: [0; vk::VK_UUID_SIZE],
            limits: vk::VkPhysicalDeviceLimits::default(),
            sparseProperties: vk::VkPhysicalDeviceSparseProperties::default(),
        }
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Default)]
#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: u32,
    pub residencyStandard2DMultisampleBlockShape: u32,
    pub residencyStandard3DBlockShape: u32,
    pub residencyAlignedMipSize: u32,
    pub residencyNonResidentStrict: u32,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct VkQueueFamilyProperties {
    pub queueFlags: vk::VkQueueFlagBits,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: vk::VkExtent3D,
}

impl Default for VkQueueFamilyProperties {
    fn default() -> Self {
        Self {
            queueFlags: vk::VkQueueFlagBits::empty(),
            queueCount: 0,
            timestampValidBits: 0,
            minImageTransferGranularity: vk::VkExtent3D::default(),
        }
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VkSurfaceFormatKHR {
    pub format: vk::VkFormat,
    pub colorSpace: vk::VkColorSpaceKHR,
}

impl Default for VkSurfaceFormatKHR {
    fn default() -> Self {
        Self {
            format: vk::VkFormat::VK_FORMAT_UNDEFINED,
            colorSpace: vk::VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
        }
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[cfg(target_os = "windows")]
#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const std::ffi::c_void,
    pub flags: u32,
    pub hinstance: *mut std::ffi::c_void,
    pub hwnd: *mut std::ffi::c_void,
}
