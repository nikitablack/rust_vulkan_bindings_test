use crate::vk;
use crate::vulkan;
use raw_window_handle::HasRawWindowHandle;

#[allow(dead_code)]
pub fn create_surface(
    window: &winit::window::Window,
    instance: &vulkan::Instance,
    _allocator: *const vulkan::AllocationCallbacks, // TODO
) -> Result<vulkan::SurfaceKHR, vk::VkResult> {
    match window.raw_window_handle() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Windows(handle) => {
            let create_info = vulkan::Win32SurfaceCreateInfoKHR {
                next: std::ptr::null(),
                flags: 0,
                hinstance: handle.hinstance,
                hwnd: handle.hwnd,
            };

            match vulkan::create_surface_win32(&create_info, &instance, std::ptr::null()) {
                // TODO
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
