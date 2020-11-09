#[derive(Debug, Default)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[allow(dead_code)]
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
