//! NASM x86-64 assembly code generator
use std::fmt;

use super::register::Register;
use super::operand::{Operand};
use super::instruction::Instruction;

/// Assembly sections
#[derive(Debug, Clone)]
pub enum Section {
    Text,
    Data,
    Bss,
    Rodata,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Section::Text => write!(f, "section .text"),
            Section::Data => write!(f, "section .data"),
            Section::Bss => write!(f, "section .bss"),
            Section::Rodata => write!(f, "section .rodata"),
        }
    }
}

/// Assembly program elements
#[derive(Debug, Clone)]
pub enum AssemblyElement {
    Section(Section),
    Label(String),
    Instruction(Instruction),
    Directive(String),
    Comment(String),
    EmptyLine,
    Global(String),
    Extern(String),
    DataDefinition(String, String, String), // label, type, value
}

impl fmt::Display for AssemblyElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssemblyElement::Section(sec) => write!(f, "{}", sec),
            AssemblyElement::Label(name) => write!(f, "{}:", name),
            AssemblyElement::Instruction(inst) => write!(f, "{}", inst),
            AssemblyElement::Directive(dir) => write!(f, "{}", dir),
            AssemblyElement::Comment(text) => write!(f, "; {}", text),
            AssemblyElement::EmptyLine => write!(f, ""),
            AssemblyElement::Global(name) => write!(f, "global {}", name),
            AssemblyElement::Extern(name) => write!(f, "extern {}", name),
            AssemblyElement::DataDefinition(label, dtype, value) => {
                write!(f, "{}: {} {}", label, dtype, value)
            }
        }
    }
}

/// Target operating system for code generation
#[derive(Debug, Clone)]
pub enum TargetOS {
    Linux,
    Windows,
    MacOS,
}

/// NASM x86-64 assembly code generator
pub struct NasmGenerator {
    elements: Vec<AssemblyElement>,
    label_counter: u32,
    #[allow(dead_code)]
    target_os: TargetOS,
}

impl NasmGenerator {
    /// Create a new generator for the specified target OS
    pub fn new(target_os: TargetOS) -> Self {
        Self {
            elements: Vec::new(),
            label_counter: 0,
            target_os,
        }
    }

    /// Add an element to the assembly code
    pub fn add_element(&mut self, element: AssemblyElement) {
        self.elements.push(element);
    }

    /// Add a section
    pub fn add_section(&mut self, section: Section) {
        self.elements.push(AssemblyElement::Section(section));
    }

    /// Add an instruction
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.elements.push(AssemblyElement::Instruction(instruction));
    }

    /// Add a label
    pub fn add_label(&mut self, name: &str) {
        self.elements.push(AssemblyElement::Label(name.to_string()));
    }

    /// Generate a unique label with the given prefix
    pub fn generate_label(&mut self, prefix: &str) -> String {
        self.label_counter += 1;
        format!("{}_{}", prefix, self.label_counter)
    }

    /// Add a comment
    pub fn add_comment(&mut self, text: &str) {
        self.elements.push(AssemblyElement::Comment(text.to_string()));
    }

    /// Add an empty line
    pub fn add_empty_line(&mut self) {
        self.elements.push(AssemblyElement::EmptyLine);
    }

    /// Add a global directive
    pub fn add_global(&mut self, name: &str) {
        self.elements.push(AssemblyElement::Global(name.to_string()));
    }

    /// Add an extern directive
    pub fn add_extern(&mut self, name: &str) {
        self.elements.push(AssemblyElement::Extern(name.to_string()));
    }

    /// Add a data definition
    pub fn add_data(&mut self, label: &str, data_type: &str, value: &str) {
        self.elements.push(AssemblyElement::DataDefinition(
            label.to_string(),
            data_type.to_string(),
            value.to_string(),
        ));
    }

    /// Generate the standard prelude for a NASM program
    pub fn add_standard_prelude(&mut self) {
        self.add_comment("Generated automatically with Rust NASM Generator");
        self.add_comment("Architecture: x64");
        self.add_comment("Format: NASM");
        self.add_empty_line();
        self.elements.push(AssemblyElement::Directive("bits 64".to_string()));
        self.add_empty_line();
    }

    /// Create a simple "Hello, World!" program for Linux
    pub fn create_hello_world_linux(&mut self) {
        self.add_standard_prelude();
        
        // Data section
        self.add_section(Section::Data);
        self.add_data("msg", "db", "'Hello, World!', 0xA, 0");
        self.add_data("msg_len", "equ", "$ - msg - 1");
        self.add_empty_line();
        
        // Text section
        self.add_section(Section::Text);
        self.add_global("_start");
        self.add_empty_line();
        
        self.add_label("_start");
        self.add_comment("sys_write");
        self.add_instruction(Instruction::Mov(
            Operand::reg(Register::RAX),
            Operand::imm(1)
        ));
        self.add_comment("stdout");
        self.add_instruction(Instruction::Mov(
            Operand::reg(Register::RDI),
            Operand::imm(1)
        ));
        self.add_comment("message");
        self.add_instruction(Instruction::Mov(
            Operand::reg(Register::RSI),
            Operand::label("msg")
        ));
        self.add_comment("length");
        self.add_instruction(Instruction::Mov(
            Operand::reg(Register::RDX),
            Operand::label("msg_len")
        ));
        self.add_instruction(Instruction::Syscall);
        self.add_empty_line();
        
        self.add_comment("sys_exit");
        self.add_instruction(Instruction::Mov(
            Operand::reg(Register::RAX),
            Operand::imm(60)
        ));
        self.add_comment("exit code");
        self.add_instruction(Instruction::Mov(
            Operand::reg(Register::RDI),
            Operand::imm(0)
        ));
        self.add_instruction(Instruction::Syscall);
    }

    /// Create a factorial function
    pub fn create_factorial_function(&mut self) {
        self.add_comment("Function to calculate factorial of n");
        self.add_comment("Input: rdi = n");
        self.add_comment("Output: rax = n!");
        self.add_label("factorial");
        
        // Function prologue
        self.add_instruction(Instruction::Push(Operand::reg(Register::RBP)));
        self.add_instruction(Instruction::Mov(
            Operand::reg(Register::RBP),
            Operand::reg(Register::RSP)
        ));
        
        // Base case: if n <= 1, return 1
        self.add_instruction(Instruction::Cmp(
            Operand::reg(Register::RDI),
            Operand::imm(1)
        ));
        let base_case = self.generate_label("factorial_base");
        self.add_instruction(Instruction::Jle(base_case.clone()));
        
        // Recursive case: n * factorial(n-1)
        self.add_instruction(Instruction::Push(Operand::reg(Register::RDI)));
        self.add_instruction(Instruction::Dec(Operand::reg(Register::RDI)));
        self.add_instruction(Instruction::Call("factorial".to_string()));
        self.add_instruction(Instruction::Pop(Operand::reg(Register::RDI)));
        self.add_instruction(Instruction::Imul(
            Operand::reg(Register::RAX), 
            Some(Operand::reg(Register::RDI)), 
            None
        ));
        
        let end_label = self.generate_label("factorial_end");
        self.add_instruction(Instruction::Jmp(end_label.clone()));
        
        // Base case
        self.add_label(&base_case);
        self.add_instruction(Instruction::Mov(
            Operand::reg(Register::RAX),
            Operand::imm(1)
        ));
        
        // Function epilogue
        self.add_label(&end_label);
        self.add_instruction(Instruction::Pop(Operand::reg(Register::RBP)));
        self.add_instruction(Instruction::Ret);
        self.add_empty_line();
    }

    /// Generate the complete assembly code
    pub fn generate(&self) -> String {
        self.elements
            .iter()
            .map(|element| element.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Save the generated code to a file
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        use std::fs;
        fs::write(filename, self.generate())
    }
}

impl Default for NasmGenerator {
    fn default() -> Self {
        Self::new(TargetOS::Linux)
    }
}