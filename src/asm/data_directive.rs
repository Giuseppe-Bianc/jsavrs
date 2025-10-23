/// Module for representing assembly data directives and assembly structure elements.
///
/// This module provides types for working with assembly language constructs including
/// data directives (db, dw, dd, dq), string declarations, space reservations, and
/// complete assembly sections with their elements.
use super::{Instruction, Section};
use std::fmt;

/// Represents an expression used in EQU directives.
///
/// EQU expressions define constant values or calculations that can be used
/// throughout the assembly program. These are evaluated at assembly time.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum EquExpression {
    /// A constant integer value.
    ///
    /// Represents a simple numeric constant that can be positive or negative.
    Constant(i64),

    /// Length calculation expression: `$ - label`.
    ///
    /// Calculates the distance from a specified label to the current position ($).
    /// Commonly used to determine the length of data sections (e.g., string length).
    /// The string contains the name of the label to calculate from.
    LengthOf(String),

    /// Generic expression for complex calculations.
    ///
    /// Represents any other type of expression that doesn't fit the predefined
    /// categories. The string contains the raw expression text to be included
    /// in the assembly output.
    Generic(String),
}

/// Represents a data directive in assembly language.
///
/// Data directives are used to declare and initialize data in assembly programs.
/// They specify how data should be stored in memory, including integers of various
/// sizes, strings, and reserved space.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DataDirective {
    /// Byte (8-bit) data directive.
    ///
    /// Stores one or more 8-bit values. Equivalent to the `db` (define byte)
    /// directive in NASM/YASM assembly.
    Db(Vec<u8>),

    /// Word (16-bit) data directive.
    ///
    /// Stores one or more 16-bit values. Equivalent to the `dw` (define word)
    /// directive in NASM/YASM assembly.
    Dw(Vec<u16>),

    /// Double word (32-bit) data directive.
    ///
    /// Stores one or more 32-bit values. Equivalent to the `dd` (define double word)
    /// directive in NASM/YASM assembly.
    Dd(Vec<u32>),

    /// Quad word (64-bit) data directive.
    ///
    /// Stores one or more 64-bit values. Equivalent to the `dq` (define quad word)
    /// directive in NASM/YASM assembly.
    Dq(Vec<u64>),

    /// String with a terminator byte.
    ///
    /// Stores a string followed by a specified terminator byte. The first field
    /// contains the string content, and the second field specifies the terminator
    /// (commonly 0 for null-terminated strings). This is equivalent to ASCIZ or
    /// null-terminated string declarations in assembly.
    Asciz(String, u8),

    /// String without a terminator.
    ///
    /// Stores a string without any terminator byte. Equivalent to the ASCII
    /// directive that defines raw string data.
    Ascii(String),

    /// Reserved space in bytes.
    ///
    /// Reserves a specified number of bytes in memory without initialization.
    /// Equivalent to the `resb` (reserve bytes) directive in NASM/YASM assembly.
    /// The value specifies the number of bytes to reserve.
    Resb(usize),

    /// Reserved space in words.
    ///
    /// Reserves a specified number of words (16-bit values) in memory without
    /// initialization. Equivalent to the `resw` (reserve words) directive.
    /// The value specifies the number of words to reserve.
    Resw(usize),

    /// Reserved space in double words.
    ///
    /// Reserves a specified number of double words (32-bit values) in memory
    /// without initialization. Equivalent to the `resd` (reserve double words)
    /// directive. The value specifies the number of double words to reserve.
    Resd(usize),

    /// Reserved space in quad words.
    ///
    /// Reserves a specified number of quad words (64-bit values) in memory
    /// without initialization. Equivalent to the `resq` (reserve quad words)
    /// directive. The value specifies the number of quad words to reserve.
    Resq(usize),

    /// EQU directive - defines a constant or calculated value.
    ///
    /// Used to assign symbolic names to constant values or expressions.
    /// Common uses include defining buffer lengths (e.g., `len equ $ - msg`)
    /// or numeric constants. The expression can be a simple constant or a
    /// calculation involving labels and the current position marker ($).
    Equ(EquExpression),
}

impl DataDirective {
    /// Creates a new null-terminated string directive (ASCIZ).
    ///
    /// # Arguments
    ///
    /// * `s` - The string content to store. Can be any type that converts into a String.
    ///
    /// # Returns
    ///
    /// A `DataDirective::Asciz` variant with a null terminator (0x00).
    ///
    /// # Examples
    ///
    /// ```
    /// use jsavrs::asm::DataDirective;
    /// let directive = DataDirective::new_asciz("Hello, World!");
    /// // Results in: db "Hello, World!", 0
    /// ```
    pub fn new_asciz(s: impl Into<String>) -> Self {
        DataDirective::Asciz(s.into(), 0)
    }

    /// Creates a new string directive with a custom terminator byte.
    ///
    /// # Arguments
    ///
    /// * `s` - The string content to store. Can be any type that converts into a String.
    /// * `terminator` - The byte value to append after the string.
    ///
    /// # Returns
    ///
    /// A `DataDirective::Asciz` variant with the specified terminator.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsavrs::asm::DataDirective;
    /// let directive = DataDirective::new_asciiz_with_terminator("data", 0x0A);
    /// // Results in: db "data", 10  (terminated with newline)
    /// ```
    pub fn new_asciiz_with_terminator(s: impl Into<String>, terminator: u8) -> Self {
        DataDirective::Asciz(s.into(), terminator)
    }

    /// Creates a new EQU directive with a constant value.
    ///
    /// # Arguments
    ///
    /// * `value` - The constant integer value to assign.
    ///
    /// # Returns
    ///
    /// A `DataDirective::Equ` variant containing a constant expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsavrs::asm::DataDirective;
    /// let directive = DataDirective::new_equ_constant(42);
    /// // Results in: equ 42
    /// ```
    pub fn new_equ_constant(value: i64) -> Self {
        DataDirective::Equ(EquExpression::Constant(value))
    }

    /// Creates a new EQU directive that calculates length from a label.
    ///
    /// # Arguments
    ///
    /// * `label` - The name of the label to calculate from. Can be any type that
    ///   converts into a String.
    ///
    /// # Returns
    ///
    /// A `DataDirective::Equ` variant containing a length-of expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsavrs::asm::DataDirective;
    /// let directive = DataDirective::new_equ_length_of("msg");
    /// // Results in: equ $ - msg
    /// ```
    pub fn new_equ_length_of(label: impl Into<String>) -> Self {
        DataDirective::Equ(EquExpression::LengthOf(label.into()))
    }

    /// Creates a new EQU directive with a generic expression.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression string to use. Can be any type that converts
    ///   into a String.
    ///
    /// # Returns
    ///
    /// A `DataDirective::Equ` variant containing a generic expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use jsavrs::asm::DataDirective;
    /// let directive = DataDirective::new_equ_generic("BUFFER_SIZE * 2");
    /// // Results in: equ BUFFER_SIZE * 2
    /// ```
    pub fn new_equ_generic(expr: impl Into<String>) -> Self {
        DataDirective::Equ(EquExpression::Generic(expr.into()))
    }
}

impl fmt::Display for EquExpression {
    /// Formats the EQU expression for assembly output.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation.
    ///
    /// # Format
    ///
    /// * `Constant(n)` outputs the numeric value directly
    /// * `LengthOf(label)` outputs "$ - label"
    /// * `Generic(expr)` outputs the expression as-is
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EquExpression::Constant(value) => write!(f, "{}", value),
            EquExpression::LengthOf(label) => write!(f, "$ - {}", label),
            EquExpression::Generic(expr) => write!(f, "{}", expr),
        }
    }
}

impl fmt::Display for DataDirective {
    /// Formats the data directive for NASM/YASM assembly output.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation.
    ///
    /// # Format
    ///
    /// Each directive type produces valid NASM/YASM syntax:
    /// * Numeric directives output values in hexadecimal with appropriate padding
    /// * String directives properly escape special characters
    /// * Reserve directives output the size to reserve
    /// * EQU directives delegate to the EquExpression formatter
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

/// Represents a single element within an assembly section.
///
/// Assembly sections are composed of various elements including labels,
/// instructions, data declarations, comments, and formatting (empty lines).
/// This enum captures all possible element types that can appear in an
/// assembly program.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AssemblyElement {
    /// A label definition.
    ///
    /// Labels mark positions in the code or data and can be referenced by
    /// instructions or other directives. The string contains the label name
    /// without the trailing colon.
    Label(String),

    /// An instruction without an inline comment.
    ///
    /// Represents a single assembly instruction (e.g., mov, add, call).
    Instruction(Instruction),

    /// An instruction with an inline comment.
    ///
    /// Combines an instruction with explanatory text. The first field contains
    /// the instruction, and the second field contains the comment text (without
    /// the semicolon prefix).
    InstructionWithComment(Instruction, String),

    /// A data directive with its label.
    ///
    /// Represents a labeled data declaration. The first field contains the label
    /// name, and the second field contains the data directive specification.
    Data(String, DataDirective),

    /// A standalone comment line.
    ///
    /// Represents a full-line comment (not inline). The string contains the
    /// comment text without the semicolon prefix.
    Comment(String),

    /// An empty line for formatting.
    ///
    /// Used to add visual separation between logical sections of assembly code
    /// for improved readability.
    EmptyLine,
}

impl fmt::Display for AssemblyElement {
    /// Formats the assembly element for NASM/YASM assembly output.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation.
    ///
    /// # Format
    ///
    /// * Labels are formatted with a trailing colon at column 0
    /// * Instructions are indented with 4 spaces
    /// * Comments use semicolon syntax
    /// * Data directives combine label and directive on one line
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

/// Represents a complete assembly section with all its elements.
///
/// An assembly section combines a section type (text, data, bss, rodata) with
/// a collection of elements (labels, instructions, data, comments) that belong
/// to that section. This structure allows for organized generation of complete
/// assembly programs.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AssemblySection {
    /// The type of section (text, data, bss, or rodata).
    pub section: Section,

    /// The ordered list of elements within this section.
    ///
    /// Elements are stored in the order they should appear in the final
    /// assembly output.
    pub elements: Vec<AssemblyElement>,
}

impl fmt::Display for AssemblySection {
    /// Formats the complete assembly section for NASM/YASM assembly output.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the formatting operation.
    ///
    /// # Format
    ///
    /// Outputs the section directive (e.g., "section .text") followed by all
    /// elements in order, each on its own line.
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
    /// Creates a new empty assembly section of the specified type.
    ///
    /// # Arguments
    ///
    /// * `section` - The section type (text, data, bss, or rodata).
    ///
    /// # Returns
    ///
    /// A new `AssemblySection` with no elements.
    pub fn new(section: Section) -> Self {
        Self { section, elements: Vec::new() }
    }

    /// Adds a label to this section.
    ///
    /// # Arguments
    ///
    /// * `label` - The label name. Can be any type that converts into a String.
    ///
    /// # Side Effects
    ///
    /// Appends a new label element to the section's element list.
    pub fn add_label(&mut self, label: impl Into<String>) {
        self.elements.push(AssemblyElement::Label(label.into()));
    }

    /// Adds an instruction to this section.
    ///
    /// # Arguments
    ///
    /// * `instr` - The instruction to add.
    ///
    /// # Side Effects
    ///
    /// Appends a new instruction element to the section's element list.
    pub fn add_instruction(&mut self, instr: Instruction) {
        self.elements.push(AssemblyElement::Instruction(instr));
    }

    /// Adds a labeled data directive to this section.
    ///
    /// # Arguments
    ///
    /// * `label` - The label name for the data. Can be any type that converts
    ///   into a String.
    /// * `directive` - The data directive specification.
    ///
    /// # Side Effects
    ///
    /// Appends a new data element to the section's element list.
    pub fn add_data(&mut self, label: impl Into<String>, directive: DataDirective) {
        self.elements.push(AssemblyElement::Data(label.into(), directive));
    }

    /// Adds a comment line to this section.
    ///
    /// # Arguments
    ///
    /// * `comment` - The comment text. Can be any type that converts into a String.
    ///
    /// # Side Effects
    ///
    /// Appends a new comment element to the section's element list.
    pub fn add_comment(&mut self, comment: impl Into<String>) {
        self.elements.push(AssemblyElement::Comment(comment.into()));
    }

    /// Adds an instruction with an inline comment to this section.
    ///
    /// # Arguments
    ///
    /// * `instr` - The instruction to add.
    /// * `comment` - The inline comment text. Can be any type that converts
    ///   into a String.
    ///
    /// # Side Effects
    ///
    /// Appends a new instruction-with-comment element to the section's element list.
    pub fn add_instruction_with_comment(&mut self, instr: Instruction, comment: impl Into<String>) {
        self.elements.push(AssemblyElement::InstructionWithComment(instr, comment.into()));
    }

    /// Adds an empty line to this section for formatting purposes.
    ///
    /// # Side Effects
    ///
    /// Appends an empty line element to the section's element list, which will
    /// appear as a blank line in the formatted output.
    pub fn add_empty_line(&mut self) {
        self.elements.push(AssemblyElement::EmptyLine);
    }

    /// Creates a new .text section (executable code).
    ///
    /// # Returns
    ///
    /// A new empty `AssemblySection` of type `Section::Text`.
    ///
    /// # Usage
    ///
    /// The .text section typically contains executable instructions and is
    /// marked as read-only and executable in the final binary.
    pub fn text_section() -> Self {
        Self::new(Section::Text)
    }

    /// Creates a new .data section (initialized data).
    ///
    /// # Returns
    ///
    /// A new empty `AssemblySection` of type `Section::Data`.
    ///
    /// # Usage
    ///
    /// The .data section contains initialized data that can be read and written
    /// at runtime.
    pub fn data_section() -> Self {
        Self::new(Section::Data)
    }

    /// Creates a new .bss section (uninitialized data).
    ///
    /// # Returns
    ///
    /// A new empty `AssemblySection` of type `Section::Bss`.
    ///
    /// # Usage
    ///
    /// The .bss section reserves space for uninitialized data. It doesn't take
    /// space in the binary file but is allocated at runtime.
    pub fn bss_section() -> Self {
        Self::new(Section::Bss)
    }

    /// Creates a new .rodata section (read-only data).
    ///
    /// # Returns
    ///
    /// A new empty `AssemblySection` of type `Section::Rodata`.
    ///
    /// # Usage
    ///
    /// The .rodata section contains constant data that cannot be modified at
    /// runtime. It's typically placed in read-only memory pages.
    pub fn rodata_section() -> Self {
        Self::new(Section::Rodata)
    }
}

/// Escapes special characters in strings for assembly output.
///
/// # Arguments
///
/// * `s` - The string to escape.
///
/// # Returns
///
/// A new `String` with special characters properly escaped for use in assembly
/// string literals.
///
/// # Escaped Characters
///
/// * `\` becomes `\\` (backslash)
/// * `"` becomes `\"` (double quote)
/// * `\n` becomes `\\n` (newline)
/// * `\t` becomes `\\t` (tab)
fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\").replace("\"", "\\\"").replace("\n", "\\\\n").replace("\t", "\\\\t")
}
