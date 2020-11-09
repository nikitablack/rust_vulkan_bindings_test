use crate::vk;
use crate::vulkan;

#[allow(dead_code)]
#[cfg(target_os = "windows")]
pub fn create_surface_win32<'a, 'b>(
    create_info: &'a vulkan::Win32SurfaceCreateInfoKHR,
    instance: &'b vulkan::Instance,
    _allocator: *const vulkan::AllocationCallbacks, // TODO
) -> Result<vulkan::SurfaceKHR, vk::VkResult> {
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
            std::ptr::null(), // TODO
            &mut vk_surface,
        )
    } {
        vk::VkResult::VK_SUCCESS => Ok(vulkan::SurfaceKHR { handle: vk_surface }),
        result => Err(result),
    }
}
