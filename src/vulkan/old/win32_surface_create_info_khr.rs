#[cfg(target_os = "windows")]
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
