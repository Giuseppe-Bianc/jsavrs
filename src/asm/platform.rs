use std::fmt;

/// Piattaforma target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    /// Microsoft Windows operating system
    Windows,
    /// Linux operating system
    Linux,
    /// Apple macOS operating system
    MacOS,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Windows => write!(f, "Windows"),
            Self::Linux => write!(f, "Linux"),
            Self::MacOS => write!(f, "macOS"),
        }
    }
}
