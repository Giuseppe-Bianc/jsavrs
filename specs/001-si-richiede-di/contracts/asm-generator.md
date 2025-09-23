# Contract: ASM Generator

## Overview
This document specifies the interface and responsibilities of the ASM Generator component within the JSAVRS compiler. It defines the public API, details the expected behavior under standard and exceptional conditions, and describes the recommended usage patterns for transforming intermediate representations into assembly code. The guidelines herein aim to ensure consistency, reliability, and maintainability of assembly code generation across different stages of compilation.

## Public API

### ASMGenerator
```rust
pub struct ASMGenerator {
    sections: HashMap<String, Section>,
    current_section: Option<String>,
    labels: HashMap<String, usize>,
}

impl ASMGenerator {
    pub fn new() -> Self
    pub fn add_section(&mut self, name: &str) -> Result<(), ASMError>
    pub fn switch_section(&mut self, name: &str) -> Result<(), ASMError>
    pub fn add_instruction(&mut self, inst: Instruction) -> Result<(), ASMError>
    pub fn add_label(&mut self, name: &str) -> Result<(), ASMError>
    pub fn generate(&self) -> String
}
```

## API Behavior

### new()
- **Description**: Instantiates a new `ASMGenerator` object, initializing it with default settings. This function prepares the object for subsequent configuration and section definition.
- **Preconditions**: None.
- **Postconditions**: Upon execution, the following conditions are established:
  - A new `ASMGenerator` object is created and initialized.
  - The object contains no predefined sections.
  - No section is set as active or currently selected.
- **Errors**: None


### add_section(name: &str)
- **Description**: Registers a new section within the assembly, updating the internal section collection accordingly. This function ensures that the assembly’s structure reflects the newly added section and maintains the current active section when necessary.
- **Preconditions**: 
  - the `name` parameter must conform to the assembly’s identifier naming rules and cannot be empty or contain invalid characters.
- **Postconditions**:
  - A new section with the specified name is added to the assembly’s section collection.
  - If no section is currently active, the newly added section is designated as the active section.
- **Errors**:
  - Returns `ASMError::InvalidSectionName` if the name is invalid
  - Returns `ASMError::DuplicateSection` if a section with this name already exists in the assembly.

### switch_section(name: &str)
- **Description**: Changes the active section of the program to the section identified by the specified name. This function ensures that subsequent operations are performed within the context of the newly selected section.
- **Preconditions**: 
  - A section with the specified name must exist prior to invoking this function.
- **Postconditions**:
  - Upon successful execution, the current section is updated to reference the section corresponding to the specified name.
- **Errors**:
  - Returns `ASMError::SectionNotFound` if a section with the specified name cannot be located, indicating that the requested section does not exist.


### add_instruction(inst: Instruction)
- **Description**: Appends the specified instruction to the list of instructions associated with the currently active section.
- **Preconditions**:
  - A section must be actively selected prior to invoking this function.
- **Postconditions**:
  - The specified instruction is appended to the instruction list of the currently active section, updating the section's state.
- **Errors**:
  - Returns `ASMError::NoCurrentSection` if no section is currently selected, indicating that the operation could not be performed.

### add_label(name: &str)
- **Description**: Adds a label at the current instruction within the active section. The label serves as a reference point for jumps or other control-flow instructions in the assembly code.
- **Parameters**:
  - `name` (`&str`): The name of the label to be added. This string slice must be unique within the active section.
- **Preconditions**:
  - An active section must be selected; otherwise, the function cannot determine where to place the label.
  - No label with the specified name exists in the current section, ensuring uniqueness of label identifiers.
- **Postconditions**:
  - The label is stored in the section's labels collection, mapped to the current instruction index or position within the section.
  - The labels collection can later be referenced for branching or code analysis.
- **Errors**:
  - Returns `ASMError::NoCurrentSection` if no active section is selected.
  - Returns `ASMError::DuplicateLabel` if a label with the specified name already exists in the current section.


### generate()
- **Description**: This function generates the complete assembly code representation for the program or module under compilation. The output is formatted according to standard assembly syntax conventions, ensuring it is ready for use by an assembler.
- **Preconditions**: No preconditions are required. The function operates independently of any prior setup or external input state.
- **Postconditions**: Returns a string containing the fully formatted assembly code. The string adheres to the established assembly code syntax and formatting standards.

- **Errors**: None

## Usage Examples

### Basic Usage
```rust
let mut generator = ASMGenerator::new();
generator.add_section(".text")?;
generator.switch_section(".text")?;
generator.add_label("main")?;

let mut mov_inst = Instruction::new("mov");
mov_inst.add_operand(Operand::Register(Register::new("rax", 64, 0)));
mov_inst.add_operand(Operand::Immediate(42));
generator.add_instruction(mov_inst)?;

let assembly = generator.generate();
```

## Error Types
```rust
pub enum ASMError {
    InvalidSectionName(String),
    DuplicateSection(String),
    SectionNotFound(String),
    NoCurrentSection,
    DuplicateLabel(String),
}
```

## Implementation Constraints
1. **Assembly Code Compatibility**  
   All assembly code produced by the system must be fully compatible with the Netwide Assembler (NASM). This ensures that generated code can be assembled and executed without errors in NASM.
2. **Section Naming Conventions**  
   All section names must adhere to NASM conventions, including the use of standard sections such as `.text` for code, `.data` for initialized data, and `.bss` for uninitialized data.
3. **Label Uniqueness**  
   All label identifiers must be unique throughout the entire assembly program to prevent naming conflicts and ensure proper control flow.
4. **Instruction Validation**  
   All assembly instructions must undergo thorough validation to ensure syntactic correctness and proper operand usage. This validation helps prevent runtime errors and ensures logical consistency in the program.