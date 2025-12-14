use std::fmt;

/// Assembly sections
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Section {
    Text,
    Data,
    Bss,
    Rodata,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "section {}", self.name())
    }
}

#[allow(dead_code)]
impl Section {
    /// Get the name of the section as a string
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Text => ".text",
            Self::Data => ".data",
            Self::Bss => ".bss",
            Self::Rodata => ".rodata",
        }
    }

    /// Check if this is the text section
    #[must_use]
    pub const fn is_text(&self) -> bool {
        matches!(self, Self::Text)
    }

    /// Check if this is the data section
    #[must_use]
    pub const fn is_data(&self) -> bool {
        matches!(self, Self::Data)
    }

    /// Check if this is the bss section
    #[must_use]
    pub const fn is_bss(&self) -> bool {
        matches!(self, Self::Bss)
    }

    /// Check if this is the rodata section
    #[must_use]
    pub const fn is_rodata(&self) -> bool {
        matches!(self, Self::Rodata)
    }
}
