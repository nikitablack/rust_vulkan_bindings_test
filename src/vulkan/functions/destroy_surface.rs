use crate::vk;
use crate::vulkan;

#[allow(dead_code)]
pub fn destroy_surface(
    instance: &vulkan::Instance,
    surface: &vulkan::SurfaceKHR,
    _allocator: *const vulkan::AllocationCallbacks, // TODO
) {
    unsafe { vk::vkDestroySurfaceKHR(instance.handle, surface.handle, std::ptr::null()) };
    // TODO
}
