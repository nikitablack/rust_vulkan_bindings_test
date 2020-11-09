use crate::vk;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[cfg_attr(
    target_os = "windows",
    link(name = "C:/VulkanSDK/1.2.154.1/Lib/vulkan-1")
)]
#[cfg_attr(target_os = "linux", link(name = "vulkan"))]
extern "C" {
    pub fn vkCreateDevice(
        physicalDevice: *const vk::VkPhysicalDevice,
        pCreateInfo: *const vk::VkDeviceCreateInfo,
        pAllocator: *const vk::VkAllocationCallbacks,
        pDevice: *mut *mut vk::VkDevice,
    ) -> vk::VkResult;

    pub fn vkCreateInstance(
        pCreateInfo: *const vk::VkInstanceCreateInfo,
        pAllocator: *const vk::VkAllocationCallbacks,
        pInstance: *mut *mut vk::VkInstance,
    ) -> vk::VkResult;

    #[cfg(target_os = "windows")]
    pub fn vkCreateWin32SurfaceKHR(
        instance: *const vk::VkInstance,
        pCreateInfo: *const vk::VkWin32SurfaceCreateInfoKHR,
        pAllocator: *const vk::VkAllocationCallbacks,
        pSurface: *mut *mut vk::VkSurfaceKHR,
    ) -> vk::VkResult;

    pub fn vkDestroySurfaceKHR(
        instance: *const vk::VkInstance,
        surface: *const vk::VkSurfaceKHR,
        pAllocator: *const vk::VkAllocationCallbacks,
    );

    pub fn vkEnumerateDeviceExtensionProperties(
        physicalDevice: *const vk::VkPhysicalDevice,
        pLayerName: *const std::os::raw::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::VkExtensionProperties,
    ) -> vk::VkResult;

    pub fn vkEnumerateInstanceExtensionProperties(
        pLayerName: *const std::os::raw::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::VkExtensionProperties,
    ) -> vk::VkResult;

    pub fn vkEnumerateInstanceVersion(pApiVersion: *mut u32) -> vk::VkResult;

    pub fn vkEnumeratePhysicalDevices(
        instance: *const vk::VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut *mut vk::VkPhysicalDevice,
    ) -> vk::VkResult;

    pub fn vkGetDeviceQueue(
        device: *const vk::VkDevice,
        queueFamilyIndex: u32,
        queueIndex: u32,
        pQueue: *mut *mut vk::VkQueue,
    );

    pub fn vkGetPhysicalDeviceFeatures(
        physicalDevice: *const vk::VkPhysicalDevice,
        pFeatures: *mut vk::VkPhysicalDeviceFeatures,
    );

    pub fn vkGetPhysicalDeviceFormatProperties(
        physicalDevice: *const vk::VkPhysicalDevice,
        format: vk::VkFormat,
        pFormatProperties: *mut vk::VkFormatProperties,
    );

    pub fn vkGetPhysicalDeviceProperties(
        physicalDevice: *const vk::VkPhysicalDevice,
        pProperties: *mut vk::VkPhysicalDeviceProperties,
    );

    pub fn vkGetPhysicalDeviceQueueFamilyProperties(
        physicalDevice: *const vk::VkPhysicalDevice,
        pQueueFamilyPropertyCount: *mut u32,
        pQueueFamilyProperties: *mut vk::VkQueueFamilyProperties,
    );

    pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(
        physicalDevice: *const vk::VkPhysicalDevice,
        surface: *const vk::VkSurfaceKHR,
        pSurfaceFormatCount: *mut u32,
        pSurfaceFormats: *mut vk::VkSurfaceFormatKHR,
    ) -> vk::VkResult;

    pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(
        physicalDevice: *const vk::VkPhysicalDevice,
        surface: *const vk::VkSurfaceKHR,
        pPresentModeCount: *mut u32,
        pPresentModes: *mut vk::VkPresentModeKHR,
    ) -> vk::VkResult;

    pub fn vkGetPhysicalDeviceSurfaceSupportKHR(
        physicalDevice: *const vk::VkPhysicalDevice,
        queueFamilyIndex: u32,
        surface: *const vk::VkSurfaceKHR,
        pSupported: *mut u32,
    ) -> vk::VkResult;
}
