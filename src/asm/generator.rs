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

impl AssemblyElement {
    /// Check if this element is a section
    pub fn is_section(&self) -> bool {
        matches!(self, AssemblyElement::Section(_))
    }
    
    /// Check if this element is a label
    pub fn is_label(&self) -> bool {
        matches!(self, AssemblyElement::Label(_))
    }
    
    /// Check if this element is an instruction
    pub fn is_instruction(&self) -> bool {
        matches!(self, AssemblyElement::Instruction(_))
    }
    
    /// Check if this element is a directive
    pub fn is_directive(&self) -> bool {
        matches!(self, AssemblyElement::Directive(_))
    }
    
    /// Check if this element is a comment
    pub fn is_comment(&self) -> bool {
        matches!(self, AssemblyElement::Comment(_))
    }
    
    /// Check if this element is an empty line
    pub fn is_empty_line(&self) -> bool {
        matches!(self, AssemblyElement::EmptyLine)
    }
    
    /// Check if this element is a global directive
    pub fn is_global(&self) -> bool {
        matches!(self, AssemblyElement::Global(_))
    }
    
    /// Check if this element is an extern directive
    pub fn is_extern(&self) -> bool {
        matches!(self, AssemblyElement::Extern(_))
    }
    
    /// Check if this element is a data definition
    pub fn is_data_definition(&self) -> bool {
        matches!(self, AssemblyElement::DataDefinition(_, _, _))
    }
    
    /// Get the section if this element is a section
    pub fn as_section(&self) -> Option<&Section> {
        match self {
            AssemblyElement::Section(section) => Some(section),
            _ => None,
        }
    }
    
    /// Get the label name if this element is a label
    pub fn as_label(&self) -> Option<&str> {
        match self {
            AssemblyElement::Label(name) => Some(name),
            _ => None,
        }
    }
    
    /// Get the instruction if this element is an instruction
    pub fn as_instruction(&self) -> Option<&Instruction> {
        match self {
            AssemblyElement::Instruction(instruction) => Some(instruction),
            _ => None,
        }
    }
    
    /// Get the directive content if this element is a directive
    pub fn as_directive(&self) -> Option<&str> {
        match self {
            AssemblyElement::Directive(dir) => Some(dir),
            _ => None,
        }
    }
    
    /// Get the comment text if this element is a comment
    pub fn as_comment(&self) -> Option<&str> {
        match self {
            AssemblyElement::Comment(text) => Some(text),
            _ => None,
        }
    }
    
    /// Get the global symbol name if this element is a global directive
    pub fn as_global(&self) -> Option<&str> {
        match self {
            AssemblyElement::Global(name) => Some(name),
            _ => None,
        }
    }
    
    /// Get the extern symbol name if this element is an extern directive
    pub fn as_extern(&self) -> Option<&str> {
        match self {
            AssemblyElement::Extern(name) => Some(name),
            _ => None,
        }
    }
    
    /// Get the data definition components if this element is a data definition
    pub fn as_data_definition(&self) -> Option<(&str, &str, &str)> {
        match self {
            AssemblyElement::DataDefinition(label, dtype, value) => Some((label, dtype, value)),
            _ => None,
        }
    }
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

impl TargetOS {
    /// Get the appropriate calling convention register for the nth parameter
    pub fn param_register(&self, n: usize) -> Option<Register> {
        match self {
            TargetOS::Windows => Register::windows_param_register(n),
            TargetOS::Linux | TargetOS::MacOS => Register::systemv_param_register(n),
        }
    }

    /// Check if a register is a parameter register for this OS
    pub fn is_param_register(&self, reg: &Register) -> bool {
        match self {
            TargetOS::Windows => reg.is_windows_param_register(),
            TargetOS::Linux | TargetOS::MacOS => reg.is_systemv_param_register(),
        }
    }

    /// Check if a register is caller-saved for this OS
    pub fn is_caller_saved(&self, reg: &Register) -> bool {
        match self {
            TargetOS::Windows => reg.is_windows_caller_saved(),
            TargetOS::Linux | TargetOS::MacOS => reg.is_systemv_caller_saved(),
        }
    }

    /// Check if a register is callee-saved for this OS
    pub fn is_callee_saved(&self, reg: &Register) -> bool {
        match self {
            TargetOS::Windows => reg.is_windows_callee_saved(),
            TargetOS::Linux | TargetOS::MacOS => reg.is_systemv_callee_saved(),
        }
    }

    /// Get all callee-saved registers for this OS
    pub fn callee_saved_registers(&self) -> Vec<Register> {
        match self {
            TargetOS::Windows => vec![
                Register::RBX, Register::RBP, Register::RDI, Register::RSI,
                Register::R12, Register::R13, Register::R14, Register::R15
            ],
            TargetOS::Linux | TargetOS::MacOS => vec![
                Register::RBX, Register::RBP, Register::R12, Register::R13, Register::R14, Register::R15
            ],
        }
    }
    
    /// Get the name of the OS as a string
    pub fn name(&self) -> &'static str {
        match self {
            TargetOS::Linux => "Linux",
            TargetOS::Windows => "Windows",
            TargetOS::MacOS => "MacOS",
        }
    }
    
    /// Check if this is a Windows target
    pub fn is_windows(&self) -> bool {
        matches!(self, TargetOS::Windows)
    }
    
    /// Check if this is a Unix-like target (Linux or MacOS)
    pub fn is_unix(&self) -> bool {
        matches!(self, TargetOS::Linux | TargetOS::MacOS)
    }
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

    /// Get the target OS
    pub fn target_os(&self) -> &TargetOS {
        &self.target_os
    }

    /// Get a reference to the elements vector
    pub fn elements(&self) -> &Vec<AssemblyElement> {
        &self.elements
    }

    /// Get a mutable reference to the elements vector
    pub fn elements_mut(&mut self) -> &mut Vec<AssemblyElement> {
        &mut self.elements
    }

    /// Add an element to the assembly code
    pub fn add_element(&mut self, element: AssemblyElement) {
        self.elements.push(element);
    }

    /// Add multiple elements to the assembly code
    pub fn add_elements(&mut self, elements: Vec<AssemblyElement>) {
        self.elements.extend(elements);
    }

    /// Add a section
    pub fn add_section(&mut self, section: Section) {
        self.elements.push(AssemblyElement::Section(section));
    }

    /// Add an instruction
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.elements.push(AssemblyElement::Instruction(instruction));
    }

    /// Add multiple instructions
    pub fn add_instructions(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            self.elements.push(AssemblyElement::Instruction(instruction));
        }
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

    /// Add multiple comments
    pub fn add_comments(&mut self, comments: Vec<&str>) {
        for comment in comments {
            self.elements.push(AssemblyElement::Comment(comment.to_string()));
        }
    }

    /// Add an empty line
    pub fn add_empty_line(&mut self) {
        self.elements.push(AssemblyElement::EmptyLine);
    }

    /// Add multiple empty lines
    pub fn add_empty_lines(&mut self, count: usize) {
        for _ in 0..count {
            self.elements.push(AssemblyElement::EmptyLine);
        }
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
        self.add_comment(&format!("Target OS: {:?}", self.target_os));
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

    /// Create a simple "Hello, World!" program for Windows
    pub fn create_hello_world_windows(&mut self) {
        self.add_standard_prelude();
        
        // Data section
        self.add_section(Section::Data);
        self.add_data("msg", "db", "'Hello, World!', 0xA, 0");
        self.add_data("msg_len", "equ", "14");
        self.add_empty_line();
        
        // Text section
        self.add_section(Section::Text);
        self.add_global("main");
        self.add_extern("ExitProcess");
        self.add_extern("WriteConsoleA");
        self.add_extern("GetStdHandle");
        self.add_empty_line();
        
        self.add_label("main");
        self.add_comment("Get stdout handle");
        self.add_instruction(Instruction::Push(Operand::imm(-11))); // STD_OUTPUT_HANDLE
        self.add_instruction(Instruction::Call("GetStdHandle".to_string()));
        self.add_instruction(Instruction::Add(
            Operand::reg(Register::RSP),
            Operand::imm(8)
        ));
        self.add_instruction(Instruction::Mov(
            Operand::reg(Register::R12),
            Operand::reg(Register::RAX)
        ));
        
        self.add_empty_line();
        self.add_comment("Write message");
        self.add_instruction(Instruction::Sub(
            Operand::reg(Register::RSP),
            Operand::imm(32)
        )); // Shadow space
        self.add_instruction(Instruction::Push(Operand::imm(0))); // Reserved
        self.add_instruction(Instruction::Push(Operand::imm(0))); // Reserved
        self.add_instruction(Instruction::Push(Operand::label("msg_len"))); // Length
        self.add_instruction(Instruction::Push(Operand::label("msg"))); // Message
        self.add_instruction(Instruction::Push(Operand::reg(Register::R12))); // Handle
        self.add_instruction(Instruction::Call("WriteConsoleA".to_string()));
        self.add_instruction(Instruction::Add(
            Operand::reg(Register::RSP),
            Operand::imm(48)
        )); // Clean up stack (32 + 5*8)
        
        self.add_empty_line();
        self.add_comment("Exit process");
        self.add_instruction(Instruction::Sub(
            Operand::reg(Register::RSP),
            Operand::imm(32)
        )); // Shadow space
        self.add_instruction(Instruction::Push(Operand::imm(0))); // Exit code
        self.add_instruction(Instruction::Call("ExitProcess".to_string()));
        // No need to clean up stack as process exits
    }

    /// Create a function prologue based on the target OS
    pub fn add_function_prologue(&mut self) {
        // Save callee-saved registers
        for reg in self.target_os.callee_saved_registers() {
            self.add_instruction(Instruction::Push(Operand::reg(reg)));
        }
        
        // Set up stack frame
        self.add_instruction(Instruction::Mov(
            Operand::reg(Register::RBP),
            Operand::reg(Register::RSP)
        ));
    }

    /// Create a function epilogue based on the target OS
    pub fn add_function_epilogue(&mut self) {
        // Restore stack frame
        self.add_instruction(Instruction::Mov(
            Operand::reg(Register::RSP),
            Operand::reg(Register::RBP)
        ));
        
        // Restore callee-saved registers in reverse order
        let callee_saved = self.target_os.callee_saved_registers();
        for reg in callee_saved.iter().rev() {
            self.add_instruction(Instruction::Pop(Operand::reg(*reg)));
        }
        
        // Return
        self.add_instruction(Instruction::Ret);
    }

    /// Create a factorial function
    pub fn create_factorial_function(&mut self) {
        self.add_comment("Function to calculate factorial of n");
        match self.target_os {
            TargetOS::Windows => {
                self.add_comment("Input: rcx = n (Windows x64 ABI)");
                self.add_comment("Output: rax = n!");
            },
            TargetOS::Linux | TargetOS::MacOS => {
                self.add_comment("Input: rdi = n (System V ABI)");
                self.add_comment("Output: rax = n!");
            }
        }
        self.add_label("factorial");
        
        // Function prologue
        self.add_function_prologue();
        
        // Get parameter register based on target OS
        let param_reg = match self.target_os {
            TargetOS::Windows => Register::RCX,
            TargetOS::Linux | TargetOS::MacOS => Register::RDI,
        };
        
        // Base case: if n <= 1, return 1
        self.add_instruction(Instruction::Cmp(
            Operand::reg(param_reg),
            Operand::imm(1)
        ));
        let base_case = self.generate_label("factorial_base");
        self.add_instruction(Instruction::Jle(base_case.clone()));
        
        // Recursive case: n * factorial(n-1)
        self.add_instruction(Instruction::Push(Operand::reg(param_reg)));
        self.add_instruction(Instruction::Dec(Operand::reg(param_reg)));
        self.add_instruction(Instruction::Call("factorial".to_string()));
        self.add_instruction(Instruction::Pop(Operand::reg(param_reg)));
        self.add_instruction(Instruction::Imul(
            Operand::reg(Register::RAX), 
            Some(Operand::reg(param_reg)), 
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
        self.add_function_epilogue();
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
    
    /// Clear all elements
    pub fn clear(&mut self) {
        self.elements.clear();
        self.label_counter = 0;
    }
    
    /// Get the number of elements
    pub fn len(&self) -> usize {
        self.elements.len()
    }
    
    /// Check if there are no elements
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl Default for NasmGenerator {
    fn default() -> Self {
        Self::new(TargetOS::Linux)
    }
}