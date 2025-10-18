/// Rappresenta una direttiva di dati nell'assembly
use super::{Instruction, Section};
use std::fmt;

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
    /// String (con null terminator o altro terminatore)
    Asciz(String, u8),
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
    /// EQU - costante calcolata (es: len equ $ - msg)
    Equ(EquExpression),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum EquExpression {
    /// Valore costante
    Constant(i64),
    /// Calcolo di lunghezza: $ - label
    LengthOf(String),
    /// Espressione generica (per casi più complessi)
    Generic(String),
}



impl DataDirective {
    pub fn new_asciz(s: impl Into<String>) -> Self {
        DataDirective::Asciz(s.into(), 0)
    }

    pub fn new_asciiz_with_terminator(s: impl Into<String>, terminator: u8) -> Self {
        DataDirective::Asciz(s.into(), terminator)
    }

    pub fn new_equ_constant(value: i64) -> Self {
        DataDirective::Equ(EquExpression::Constant(value))
    }

    pub fn new_equ_length_of(label: impl Into<String>) -> Self {
        DataDirective::Equ(EquExpression::LengthOf(label.into()))
    }

    pub fn new_equ_generic(expr: impl Into<String>) -> Self {
        DataDirective::Equ(EquExpression::Generic(expr.into()))
    }
}

impl fmt::Display for EquExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EquExpression::Constant(value) => write!(f, "{}", value),
            EquExpression::LengthOf(label) => write!(f, "$ - {}", label),
            EquExpression::Generic(expr) => write!(f, "{}", expr),
        }
    }
}

impl fmt::Display for DataDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataDirective::Db(bytes) => {
                write!(f, "db ")?;
                for (i, byte) in bytes.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "0x{:02x}", byte)?;
                }
                Ok(())
            }
            DataDirective::Dw(words) => {
                write!(f, "dw ")?;
                for (i, word) in words.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "0x{:04x}", word)?;
                }
                Ok(())
            }
            DataDirective::Dd(dwords) => {
                write!(f, "dd ")?;
                for (i, dword) in dwords.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "0x{:08x}", dword)?;
                }
                Ok(())
            }
            DataDirective::Dq(qwords) => {
                write!(f, "dq ")?;
                for (i, qword) in qwords.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "0x{:016x}", qword)?;
                }
                Ok(())
            }
            DataDirective::Asciz(s, terminator) => {
                write!(f, "db \"{}\", {}", escape_string(s), terminator)
            }
            DataDirective::Ascii(s) => {
                write!(f, "db \"{}\"", escape_string(s))
            }
            DataDirective::Resb(size) => write!(f, "resb {}", size),
            DataDirective::Resw(size) => write!(f, "resw {}", size),
            DataDirective::Resd(size) => write!(f, "resd {}", size),
            DataDirective::Resq(size) => write!(f, "resq {}", size),
            DataDirective::Equ(expr) => write!(f, "equ {}", expr),
        }
    }
}


/// Rappresenta un elemento in una sezione assembly
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AssemblyElement {
    /// Etichetta
    Label(String),
    /// Istruzione
    Instruction(Instruction),
    /// Istruzione con commento in linea
    InstructionWithComment(Instruction, String),
    /// Direttiva di dati
    Data(String, DataDirective),
    /// Commento (block-style)
    Comment(String),
    /// Linea vuota
    EmptyLine,
}

impl fmt::Display for AssemblyElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyElement::Label(name) => write!(f, "{}:", name),
            AssemblyElement::Instruction(instr) => write!(f, "    {}", instr),
            AssemblyElement::InstructionWithComment(instr, comment) => write!(f, "    {}    ; {}", instr, comment),
            AssemblyElement::Data(label, directive) => write!(f, "{} {}", label, directive),
            AssemblyElement::Comment(comment) => write!(f, "; {}", comment),
            AssemblyElement::EmptyLine => write!(f, ""),
        }
    }
}

/// Sezione assembly con i suoi elementi
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AssemblySection {
    pub section: Section,
    pub elements: Vec<AssemblyElement>,
}

impl fmt::Display for AssemblySection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.section)?;
        for element in &self.elements {
            writeln!(f, "{}", element)?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
impl AssemblySection {
    pub fn new(section: Section) -> Self {
        Self { section, elements: Vec::new() }
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

    pub fn add_instruction_with_comment(&mut self, instr: Instruction, comment: impl Into<String>) {
        self.elements.push(AssemblyElement::InstructionWithComment(instr, comment.into()));
    }

    pub fn add_empty_line(&mut self) {
        self.elements.push(AssemblyElement::EmptyLine);
    }

    pub fn text_section() -> Self {
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

/// Helper function to escape special characters in strings for assembly output
fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\")
     .replace("\"", "\\\"")
     .replace("\n", "\\n")
     .replace("\t", "\\t")
}
