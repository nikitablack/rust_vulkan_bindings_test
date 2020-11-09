#[derive(Debug)]
pub struct InstanceCreateInfo<'a, 'b, 'c> {
    pub next: *const std::ffi::c_void,
    pub flags: u32,
    pub application_info: Option<&'a super::ApplicationInfo>,
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
