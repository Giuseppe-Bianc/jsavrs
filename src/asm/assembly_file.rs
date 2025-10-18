// src/asm/assembly_file.rs

use super::*;
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
    pub fn new(abi: Abi) -> Self {
        let (bss_section, rodata_section) = match abi {
            Abi::SYSTEM_V_MACOS | Abi::SYSTEM_V_LINUX => (Some(AssemblySection::bss_section()), None),
            Abi::WINDOWS => (None, Some(AssemblySection::rodata_section())),
            _ => (None, None),
        };

        Self {
            abi,
            text_section: AssemblySection::text_section(),
            data_section: AssemblySection::data_section(),
            bss_section,
            rodata_section,
        }
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
}


impl fmt::Display for AssemblyFile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "; Assembly File - ABI: {}", self.abi)?;
		writeln!(f, "{}", self.data_section)?;
		if let Some(bss) = &self.bss_section {
			writeln!(f, "{}", bss)?;
		}
		if let Some(rodata) = &self.rodata_section {
			writeln!(f, "{}", rodata)?;
		}
		writeln!(f, "{}", self.text_section)?;
		Ok(())
	}
}