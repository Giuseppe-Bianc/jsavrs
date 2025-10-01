use super::Platform;
pub enum Abi {
    SystemV,
    Windows,
}

#[allow(dead_code)]
impl Abi {
    pub fn from_platform(platform: Platform) -> Self {
        match platform {
            Platform::Windows => Abi::Windows,
            _ => Abi::SystemV,
        }
    }
}
