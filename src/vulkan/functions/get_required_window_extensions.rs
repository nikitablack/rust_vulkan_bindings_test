use crate::vk;
use raw_window_handle::HasRawWindowHandle;

#[allow(dead_code)]
pub fn get_required_window_extensions(
    window: &winit::window::Window,
) -> Result<Vec<String>, vk::VkResult> {
    match window.raw_window_handle() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Windows(_) => {
            return Ok(vec![
                String::from("VK_KHR_surface"),
                String::from("VK_KHR_win32_surface"),
            ])
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(_) => {
            return Ok(vec![
                String::from("VK_KHR_surface"),
                String::from("VK_KHR_wayland_surface"),
            ])
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(_) => {
            return Ok(vec![
                String::from("VK_KHR_surface"),
                String::from("VK_KHR_xlib_surface"),
            ])
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(_) => {
            return Ok(vec![
                String::from("VK_KHR_surface"),
                String::from("VK_KHR_xcb_surface"),
            ])
        }

        _ => return Err(vk::VkResult::VK_ERROR_EXTENSION_NOT_PRESENT),
    };
}
