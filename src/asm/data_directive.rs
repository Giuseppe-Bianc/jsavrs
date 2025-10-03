/// Rappresenta una direttiva di dati nell'assembly
use  super::{Instruction, Section};
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DataDirective {
    /// Byte (8-bit)
    Db(Vec<u8>),
    /// Word (16-bit)
    Dw(Vec<u16>),
    /// Double word (32-bit)
    Dd(Vec<u32>),
    /// Quad word (64-bit)
    Dq(Vec<u64>),
    /// String (con null terminator)
    Asciz(String),
    /// String (senza null terminator)
    Ascii(String),
    /// Spazio riservato in byte
    Resb(usize),
    /// Spazio riservato in word
    Resw(usize),
    /// Spazio riservato in double word
    Resd(usize),
    /// Spazio riservato in quad word
    Resq(usize),
}

/// Rappresenta un elemento in una sezione assembly
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AssemblyElement {
    /// Etichetta
    Label(String),
    /// Istruzione
    Instruction(Instruction),
    /// Direttiva di dati
    Data(String, DataDirective),
    /// Commento
    Comment(String),
    /// Linea vuota
    EmptyLine,
}

/// Sezione assembly con i suoi elementi
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AssemblySection {
    pub section: Section,
    pub elements: Vec<AssemblyElement>,
}

#[allow(dead_code)]
impl AssemblySection {
    pub fn new(section: Section) -> Self {
        Self {
            section,
            elements: Vec::new(),
        }
    }

    pub fn add_label(&mut self, label: impl Into<String>) {
        self.elements.push(AssemblyElement::Label(label.into()));
    }

    pub fn add_instruction(&mut self, instr: Instruction) {
        self.elements.push(AssemblyElement::Instruction(instr));
    }

    pub fn add_data(&mut self, label: impl Into<String>, directive: DataDirective) {
        self.elements.push(AssemblyElement::Data(label.into(), directive));
    }

    pub fn add_comment(&mut self, comment: impl Into<String>) {
        self.elements.push(AssemblyElement::Comment(comment.into()));
    }

    pub fn add_empty_line(&mut self) {
        self.elements.push(AssemblyElement::EmptyLine);
    }

	pub fn tesxt_section() -> Self {
		Self::new(Section::Text)
	}

	pub fn data_section() -> Self {
		Self::new(Section::Data)
	}

	pub fn bss_section() -> Self {
		Self::new(Section::Bss)
	}

	pub fn rodata_section() -> Self {
		Self::new(Section::Rodata)
	}
}