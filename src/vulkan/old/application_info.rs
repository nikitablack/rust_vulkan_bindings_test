#[derive(Debug, Default)]
pub struct ApplicationInfo {
    pub application_name: String,
    pub application_version: u32,
    pub engine_name: String,
    pub engine_version: u32,
    pub api_version: super::ApiVersion,
}
