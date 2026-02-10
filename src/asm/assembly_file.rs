// src/asm/assembly_file.rs

use super::{Abi, AbiKind, AssemblySection, DataDirective, Instruction};
use chrono::Utc;
use std::fmt;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AssemblyFile {
    abi: Abi,
    text_section: AssemblySection,
    data_section: AssemblySection,
    bss_section: Option<AssemblySection>,
    rodata_section: Option<AssemblySection>,
}

impl AssemblyFile {
    #[must_use]
    pub fn new(abi: Abi) -> Self {
        let (bss_section, rodata_section) = match abi.kind {
            AbiKind::SystemV => (Some(AssemblySection::bss_section()), None),
            AbiKind::Windows => (None, Some(AssemblySection::rodata_section())),
        };

        Self {
            abi,
            text_section: AssemblySection::text_section(),
            data_section: AssemblySection::data_section(),
            bss_section,
            rodata_section,
        }
    }

    // Getters
    /// Returns a reference to the ABI configuration.
    #[must_use]
    pub const fn abi(&self) -> &Abi {
        &self.abi
    }

    /// Returns a reference to the optional BSS section.
    ///
    /// The BSS section is present for `SystemV` ABI and absent for Windows ABI.
    #[must_use]
    pub const fn bss_section(&self) -> Option<&AssemblySection> {
        self.bss_section.as_ref()
    }

    /// Returns a reference to the optional read-only data section.
    ///
    /// The rodata section is present for Windows ABI and absent for `SystemV` ABI.
    #[must_use]
    pub const fn rodata_section(&self) -> Option<&AssemblySection> {
        self.rodata_section.as_ref()
    }

    /// Returns a reference to the data section.
    #[must_use]
    pub const fn data_section(&self) -> &AssemblySection {
        &self.data_section
    }

    /// Returns a reference to the text (code) section.
    #[must_use]
    pub const fn text_section(&self) -> &AssemblySection {
        &self.text_section
    }
    pub fn data_sec_add_data(&mut self, label: impl Into<String>, directive: DataDirective) {
        self.data_section.add_data(label, directive);
    }
    pub fn text_sec_add_instruction(&mut self, instr: Instruction) {
        self.text_section.add_instruction(instr);
    }
    pub fn text_sec_add_label(&mut self, label: impl Into<String>) {
        self.text_section.add_label(label);
    }
    pub fn text_sec_add_comment(&mut self, comment: impl Into<String>) {
        self.text_section.add_comment(comment);
    }
    pub fn text_sec_add_instruction_with_comment(&mut self, instr: Instruction, comment: impl Into<String>) {
        self.text_section.add_instruction_with_comment(instr, comment);
    }

    pub fn text_sec_add_global_label(&mut self, global_label: impl Into<String>) {
        self.text_section.add_global_label(global_label);
    }
}

impl fmt::Display for AssemblyFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "; Assembly File - ABI: {}", self.abi)?;
        let now = Utc::now();
        let iso_8601 = now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        writeln!(f, "; Generated on: {iso_8601}")?;
        writeln!(f, "{}", self.data_section)?;
        if let Some(bss) = &self.bss_section {
            writeln!(f, "{bss}")?;
        }
        if let Some(rodata) = &self.rodata_section {
            writeln!(f, "{rodata}")?;
        }
        writeln!(f, "{}", self.text_section)?;
        Ok(())
    }
}
