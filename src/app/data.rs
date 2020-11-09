use super::vk;
use super::vulkan;

#[allow(dead_code)]
pub struct WindowData {
    pub window: winit::window::Window,
}

#[allow(dead_code)]
pub struct DeviceData {
    pub physical_device: vulkan::PhysicalDevice,
    pub device: vulkan::Device,
    pub surface_format: vulkan::SurfaceFormatKHR,
    pub present_mode: vk::VkPresentModeKHR,
    pub queue_family: u32,
    pub depth_format: vk::VkFormat,
    pub properties: vulkan::PhysicalDeviceProperties,
    pub queue: vulkan::Queue,
}

impl Default for DeviceData {
    fn default() -> Self {
        Self {
            physical_device: vulkan::PhysicalDevice::default(),
            device: vulkan::Device::default(),
            surface_format: vulkan::SurfaceFormatKHR::default(),
            present_mode: vk::VkPresentModeKHR::VK_PRESENT_MODE_IMMEDIATE_KHR,
            queue_family: 0,
            depth_format: vk::VkFormat::VK_FORMAT_UNDEFINED,
            properties: vulkan::PhysicalDeviceProperties::default(),
            queue: vulkan::Queue::default(),
        }
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct VulkanData {
    pub api_version: vulkan::ApiVersion,
    pub instance_extensions: Vec<String>,
    pub device_extensions: Vec<String>,
    pub instance: vulkan::Instance,
    pub surface: vulkan::SurfaceKHR,
    pub device_data: DeviceData,
}

#[allow(dead_code)]
pub struct AppData {
    pub window_data: WindowData,
    pub vulkan_data: VulkanData,
}
