/// Assembly sections
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Section {
    Text,
    Data,
    Bss,
    Rodata,
}

impl Section {
    /// Get the name of the section as a string
    pub fn name(&self) -> &'static str {
        match self {
            Section::Text => ".text",
            Section::Data => ".data",
            Section::Bss => ".bss",
            Section::Rodata => ".rodata",
        }
    }

    /// Check if this is the text section
    pub fn is_text(&self) -> bool {
        matches!(self, Section::Text)
    }

    /// Check if this is the data section
    pub fn is_data(&self) -> bool {
        matches!(self, Section::Data)
    }

    /// Check if this is the bss section
    pub fn is_bss(&self) -> bool {
        matches!(self, Section::Bss)
    }

    /// Check if this is the rodata section
    pub fn is_rodata(&self) -> bool {
        matches!(self, Section::Rodata)
    }
}