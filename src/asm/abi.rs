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

    /// Returns the required stack alignment in bytes.
    pub fn alignment(&self) -> u32 {
        16
    }


    /// Returns the size of the red zone in bytes.
    pub fn red_zone(&self) -> u32 {
        match self {
            Abi::SystemV => 128,
            Abi::Windows => 0,
        }
    }

    /// Returns the size of the shadow space in bytes.
    pub fn shadow_space(&self) -> u32 {
        match self {
            Abi::SystemV => 0,
            Abi::Windows => 32,
        }
    }
}
