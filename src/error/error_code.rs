//! # Error Code System for jsavrs Compiler
//!
//! This module provides a comprehensive, standardized error code management system
//! for the jsavrs compiler. Each error has a unique identifier (e.g., E0001) that
//! enables quick reference, documentation lookup, and IDE integration.
//!
//! ## Error Code Ranges
//!
//! | Range | Phase | Description |
//! |-------|-------|-------------|
//! | E0001-E0999 | Lexical Analysis | Token recognition, literals, comments |
//! | E1001-E1999 | Parsing | Syntax structure, grammar violations |
//! | E2001-E2999 | Semantic Analysis | Types, scopes, declarations |
//! | E3001-E3999 | IR Generation | CFG, SSA, control flow |
//! | E4001-E4999 | Code Generation | Assembly, ABI, registers |
//! | E5001-E5999 | I/O & System | File operations, CLI |
//!
//! ## Usage Example
//!
//! ```rust
//! use jsavrs::error::error_code::{ErrorCode, Severity, CompilerPhase};
//!
//! let code = ErrorCode::E2023;
//! println!("Error {}: {}", code.code(), code.message());
//! println!("Severity: {:?}", code.severity());
//! println!("Phase: {:?}", code.phase());
//! ```

use std::fmt;

/// Represents the severity level of a compiler error.
///
/// Severity levels help distinguish between fatal errors that prevent compilation,
/// errors that may allow partial recovery, and informational warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Severity {
    /// Informational note, does not affect compilation
    Note = 0,
    /// Warning that might indicate a problem
    Warning = 1,
    /// Error that prevents successful compilation
    Error = 2,
    /// Fatal error that stops compilation immediately
    Fatal = 3,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Note => write!(f, "note"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
            Self::Fatal => write!(f, "fatal"),
        }
    }
}

/// Represents the compiler phase where an error occurred.
///
/// This helps categorize errors and provides context about which part
/// of the compilation pipeline detected the issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CompilerPhase {
    /// Lexical analysis phase (tokenization)
    Lexer = 0,
    /// Parsing phase (AST construction)
    Parser = 1,
    /// Semantic analysis phase (type checking, scoping)
    Semantic = 2,
    /// Intermediate representation generation
    IrGeneration = 3,
    /// Assembly code generation
    CodeGeneration = 4,
    /// I/O and system operations
    System = 5,
}

impl fmt::Display for CompilerPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lexer => write!(f, "lexer"),
            Self::Parser => write!(f, "parser"),
            Self::Semantic => write!(f, "semantic"),
            Self::IrGeneration => write!(f, "ir-gen"),
            Self::CodeGeneration => write!(f, "codegen"),
            Self::System => write!(f, "system"),
        }
    }
}

/// Unified error code system for the jsavrs compiler.
///
/// Each error code follows the pattern `E{NNNN}_{Name}` where:
/// - `NNNN` is a 4-digit numeric code
/// - `Name` is a descriptive `PascalCase` identifier
///
/// Error codes are organized by compiler phase with reserved numeric ranges.
/// This design ensures:
/// - Unique identification for each error type
/// - Easy documentation lookup via error code
/// - Stable references for tooling and IDE integration
/// - Room for future expansion within each category
///
/// # Examples
///
/// ```rust
/// use jsavrs::error::error_code::ErrorCode;
///
/// let code = ErrorCode::E0001;
/// assert_eq!(code.code(), "E0001");
/// assert_eq!(code.numeric_code(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum ErrorCode {
    // =========================================================================
    // LEXICAL ANALYSIS ERRORS (E0001-E0999)
    // =========================================================================
    /// Error E0001: Invalid or unrecognized token
    ///
    /// This error occurs when the lexer encounters a character sequence
    /// that doesn't match any valid token pattern.
    ///
    /// # Example
    /// ```compile_fail
    /// var x: i32 = @invalid
    /// ```
    ///
    /// # Solution
    /// Check for typos or unsupported characters. Valid tokens include
    /// identifiers, keywords, operators, and literals.
    E0001,

    /// Error E0002: Malformed binary number literal
    ///
    /// Binary literals must have at least one binary digit (0 or 1) after `#b`.
    ///
    /// # Example
    /// ```compile_fail
    /// var x: i32 = #b   // Missing digits
    /// var y: i32 = #b2  // Invalid digit '2' in binary
    /// ```
    ///
    /// # Solution
    /// Provide valid binary digits: `#b1010` for decimal 10.
    E0002,

    /// Error E0003: Malformed octal number literal
    ///
    /// Octal literals must have at least one octal digit (0-7) after `#o`.
    ///
    /// # Example
    /// ```compile_fail
    /// var x: i32 = #o   // Missing digits
    /// var y: i32 = #o8  // Invalid digit '8' in octal
    /// ```
    ///
    /// # Solution
    /// Provide valid octal digits: `#o755` for decimal 493.
    E0003,

    /// Error E0004: Malformed hexadecimal number literal
    ///
    /// Hexadecimal literals must have at least one hex digit (0-9, a-f, A-F) after `#x`.
    ///
    /// # Example
    /// ```compile_fail
    /// var x: i32 = #x     // Missing digits
    /// var y: i32 = #xGH   // Invalid hex digits
    /// ```
    ///
    /// # Solution
    /// Provide valid hexadecimal digits: `#xDEAD` for decimal 57005.
    E0004,

    /// Error E0005: Unterminated string literal
    ///
    /// String literals must be closed with a matching double quote.
    ///
    /// # Example
    /// ```compile_fail
    /// var s: string = "hello world
    /// ```
    ///
    /// # Solution
    /// Close the string with `"`: `var s: string = "hello world"`
    E0005,

    /// Error E0006: Unterminated character literal
    ///
    /// Character literals must be closed with a matching single quote.
    ///
    /// # Example
    /// ```compile_fail
    /// var c: char = 'x
    /// ```
    ///
    /// # Solution
    /// Close the character with `'`: `var c: char = 'x'`
    E0006,

    /// Error E0007: Invalid escape sequence
    ///
    /// The escape sequence is not recognized. Valid escapes include:
    /// `\n`, `\r`, `\t`, `\\`, `\'`, `\"`, `\0`, `\u{XXXX}`.
    ///
    /// # Example
    /// ```compile_fail
    /// var s: string = "hello\q"  // \q is invalid
    /// ```
    ///
    /// # Solution
    /// Use a valid escape sequence or escape the backslash: `"hello\\q"`.
    E0007,

    /// Error E0008: Unterminated multi-line comment
    ///
    /// Multi-line comments opened with `/*` must be closed with `*/`.
    ///
    /// # Example
    /// ```compile_fail
    /// /* This comment never ends
    /// var x: i32 = 1
    /// ```
    ///
    /// # Solution
    /// Close the comment with `*/`.
    E0008,

    /// Error E0009: Invalid number suffix
    ///
    /// The suffix on the numeric literal is not recognized.
    /// Valid suffixes: `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `f32`, `f64`.
    ///
    /// # Example
    /// ```compile_fail
    /// var x = 42i128  // i128 not supported
    /// ```
    ///
    /// # Solution
    /// Use a supported type suffix.
    E0009,

    /// Error E0010: Number literal overflow
    ///
    /// The numeric value exceeds the range of the target type.
    ///
    /// # Example
    /// ```compile_fail
    /// var x: i8 = 256i8  // i8 max is 127
    /// ```
    ///
    /// # Solution
    /// Use a larger type or reduce the value.
    E0010,

    // =========================================================================
    // PARSING ERRORS (E1001-E1999)
    // =========================================================================
    /// Error E1001: Maximum recursion depth exceeded
    ///
    /// The parser has exceeded its recursion limit, usually due to deeply
    /// nested expressions or pathological input.
    ///
    /// # Example
    /// ```compile_fail
    /// var x = ((((((((((((((((((((... // too deep
    /// ```
    ///
    /// # Solution
    /// Simplify the expression or break it into smaller parts.
    E1001,

    /// Error E1002: Invalid type specification
    ///
    /// Expected a valid type but found something else.
    /// Valid types: `i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`,
    /// `f32`, `f64`, `char`, `string`, `bool`, or custom identifiers.
    ///
    /// # Example
    /// ```compile_fail
    /// var x: 123 = 0  // 123 is not a type
    /// ```
    ///
    /// # Solution
    /// Use a valid type name.
    E1002,

    /// Error E1003: Invalid assignment target
    ///
    /// Only variables and array elements can be assigned to.
    ///
    /// # Example
    /// ```compile_fail
    /// 5 = x       // Cannot assign to literal
    /// foo() = 1   // Cannot assign to function call
    /// ```
    ///
    /// # Solution
    /// Assign to a variable or array element: `x = 5` or `arr[0] = 1`.
    E1003,

    /// Error E1004: Unexpected token
    ///
    /// The parser encountered a token it didn't expect in this context.
    ///
    /// # Example
    /// ```compile_fail
    /// fun foo( { }  // Expected ')' not '{'
    /// ```
    ///
    /// # Solution
    /// Check syntax and add missing tokens.
    E1004,

    /// Error E1005: Invalid binary operator
    ///
    /// The token cannot be used as a binary operator.
    ///
    /// # Solution
    /// Use a valid binary operator: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, etc.
    E1005,

    /// Error E1006: Expected expression
    ///
    /// An expression was expected but not found.
    ///
    /// # Example
    /// ```compile_fail
    /// var x: i32 =   // Missing initializer expression
    /// ```
    ///
    /// # Solution
    /// Provide an expression.
    E1006,

    /// Error E1007: Expected statement
    ///
    /// A statement was expected but not found.
    E1007,

    /// Error E1008: Expected identifier
    ///
    /// An identifier (variable or function name) was expected.
    ///
    /// # Example
    /// ```compile_fail
    /// var 123: i32 = 0  // 123 is not an identifier
    /// ```
    ///
    /// # Solution
    /// Use a valid identifier starting with a letter or underscore.
    E1008,

    /// Error E1009: Expected type annotation
    ///
    /// A type annotation was expected after `:`.
    ///
    /// # Example
    /// ```compile_fail
    /// var x: = 0  // Missing type after ':'
    /// ```
    ///
    /// # Solution
    /// Provide a type: `var x: i32 = 0`.
    E1009,

    /// Error E1010: Unmatched parenthesis
    ///
    /// Opening `(` without matching `)` or vice versa.
    ///
    /// # Solution
    /// Balance parentheses.
    E1010,

    /// Error E1011: Unmatched brace
    ///
    /// Opening `{` without matching `}` or vice versa.
    ///
    /// # Solution
    /// Balance braces.
    E1011,

    /// Error E1012: Unmatched bracket
    ///
    /// Opening `[` without matching `]` or vice versa.
    ///
    /// # Solution
    /// Balance brackets.
    E1012,

    /// Error E1013: Missing semicolon (informational)
    ///
    /// A semicolon may be missing. This is currently a warning
    /// as semicolons are optional in many contexts.
    E1013,

    /// Error E1014: Invalid function signature
    ///
    /// The function declaration has an invalid structure.
    E1014,

    /// Error E1015: Invalid parameter list
    ///
    /// The function parameter list is malformed.
    E1015,

    // =========================================================================
    // SEMANTIC / TYPE ERRORS (E2001-E2999)
    // =========================================================================
    /// Error E2001: Initializer count mismatch
    ///
    /// The number of initializers doesn't match the number of variables.
    ///
    /// # Example
    /// ```compile_fail
    /// var a, b: i32 = 1  // 2 vars but 1 initializer
    /// ```
    ///
    /// # Solution
    /// Provide one initializer per variable: `var a, b: i32 = 1, 2`.
    E2001,

    /// Error E2002: Type mismatch in assignment
    ///
    /// Cannot assign a value of one type to a variable of an incompatible type.
    ///
    /// # Example
    /// ```compile_fail
    /// var x: i32 = "hello"  // string not assignable to i32
    /// ```
    ///
    /// # Solution
    /// Use compatible types or explicit conversion.
    E2002,

    /// Error E2003: Missing return path
    ///
    /// A function with a non-void return type may not return a value
    /// in all code paths.
    ///
    /// # Example
    /// ```compile_fail
    /// fun foo(): i32 {
    ///     if (true) { return 1 }
    ///     // Missing return in else path
    /// }
    /// ```
    ///
    /// # Solution
    /// Ensure all code paths return a value.
    E2003,

    /// Error E2004: Non-boolean condition
    ///
    /// Conditions in `if`, `while`, and `for` must be boolean expressions.
    ///
    /// # Example
    /// ```compile_fail
    /// if (42) { }  // i64 is not bool
    /// ```
    ///
    /// # Solution
    /// Use a boolean expression: `if (x > 0) { }`.
    E2004,

    /// Error E2005: Return outside function
    ///
    /// A `return` statement was found outside of a function body.
    ///
    /// # Solution
    /// Move the return statement inside a function.
    E2005,

    /// Error E2006: Return value in void function
    ///
    /// A void function cannot return a value.
    ///
    /// # Example
    /// ```compile_fail
    /// fun foo() {  // implicit void
    ///     return 42  // Error: cannot return value
    /// }
    /// ```
    ///
    /// # Solution
    /// Remove the return value or change the function's return type.
    E2006,

    /// Error E2007: Return type mismatch
    ///
    /// The returned value's type doesn't match the function's declared return type.
    ///
    /// # Example
    /// ```compile_fail
    /// fun foo(): i32 {
    ///     return "hello"  // string != i32
    /// }
    /// ```
    ///
    /// # Solution
    /// Return a value of the correct type.
    E2007,

    /// Error E2008: Missing return value
    ///
    /// A non-void function returned without a value.
    ///
    /// # Example
    /// ```compile_fail
    /// fun foo(): i32 {
    ///     return  // Missing value
    /// }
    /// ```
    ///
    /// # Solution
    /// Provide a return value: `return 0`.
    E2008,

    /// Error E2009: Break outside loop
    ///
    /// A `break` statement was found outside of a loop.
    ///
    /// # Solution
    /// Use `break` only inside `while` or `for` loops.
    E2009,

    /// Error E2010: Continue outside loop
    ///
    /// A `continue` statement was found outside of a loop.
    ///
    /// # Solution
    /// Use `continue` only inside `while` or `for` loops.
    E2010,

    /// Error E2011: Bitwise operator on non-integer
    ///
    /// Bitwise operators (`&`, `|`, `^`, `<<`, `>>`) require integer operands.
    ///
    /// # Example
    /// ```compile_fail
    /// var x = 1.5 & 2.5  // f64 not allowed
    /// ```
    ///
    /// # Solution
    /// Use integer operands.
    E2011,

    /// Error E2012: Logical operator on non-boolean
    ///
    /// Logical operators (`&&`, `||`) require boolean operands.
    ///
    /// # Example
    /// ```compile_fail
    /// var x = 1 && 2  // i64 not allowed
    /// ```
    ///
    /// # Solution
    /// Use boolean expressions: `x > 0 && y > 0`.
    E2012,

    /// Error E2013: Arithmetic operator on non-numeric
    ///
    /// Arithmetic operators (`+`, `-`, `*`, `/`, `%`) require numeric operands.
    ///
    /// # Example
    /// ```compile_fail
    /// var x = "a" + "b"  // string + not supported
    /// ```
    ///
    /// # Solution
    /// Use numeric operands.
    E2013,

    /// Error E2014: Incompatible comparison types
    ///
    /// Comparison operators require compatible types.
    ///
    /// # Example
    /// ```compile_fail
    /// var x = 1 < "hello"  // i64 vs string
    /// ```
    ///
    /// # Solution
    /// Compare values of compatible types.
    E2014,

    /// Error E2015: Binary operation type mismatch
    ///
    /// The types in a binary expression are incompatible.
    ///
    /// # Solution
    /// Ensure both operands have compatible types.
    E2015,

    /// Error E2016: Unsupported arithmetic operation
    ///
    /// The arithmetic operation is not supported for this type.
    E2016,

    /// Error E2017: Logical operation requires bool
    ///
    /// Logical operations require boolean operands.
    E2017,

    /// Error E2018: Negation on non-numeric type
    ///
    /// Unary negation (`-`) requires a numeric operand.
    ///
    /// # Example
    /// ```compile_fail
    /// var x = -true  // bool cannot be negated
    /// ```
    ///
    /// # Solution
    /// Use a numeric operand.
    E2018,

    /// Error E2019: Logical NOT on non-boolean
    ///
    /// Logical NOT (`!`) requires a boolean operand.
    ///
    /// # Example
    /// ```compile_fail
    /// var x = !42  // i64 cannot be NOTted
    /// ```
    ///
    /// # Solution
    /// Use a boolean operand: `!flag`.
    E2019,

    /// Error E2020: Empty array literal
    ///
    /// Array literals must have at least one element for type inference.
    ///
    /// # Example
    /// ```compile_fail
    /// var arr: i32[] = {}  // Empty not allowed
    /// ```
    ///
    /// # Solution
    /// Provide at least one element: `{0}`.
    E2020,

    /// Error E2021: Mixed array element types
    ///
    /// All elements in an array literal must have the same type.
    ///
    /// # Example
    /// ```compile_fail
    /// var arr = {1, "hello", 3}  // mixed i64 and string
    /// ```
    ///
    /// # Solution
    /// Use elements of the same type.
    E2021,

    /// Error E2022: Function used as variable
    ///
    /// A function name cannot be used where a variable is expected.
    ///
    /// # Example
    /// ```compile_fail
    /// fun foo() { }
    /// var x = foo  // foo is a function, not a value
    /// ```
    ///
    /// # Solution
    /// Call the function: `var x = foo()`.
    E2022,

    /// Error E2023: Undefined variable
    ///
    /// The variable has not been declared in the current scope or any outer scope.
    ///
    /// # Example
    /// ```compile_fail
    /// var x = y  // y is not defined
    /// ```
    ///
    /// # Solution
    /// Declare the variable before use.
    E2023,

    /// Error E2024: Assignment to immutable variable
    ///
    /// Cannot assign to a variable declared with `const`.
    ///
    /// # Example
    /// ```compile_fail
    /// const x: i32 = 1
    /// x = 2  // Error: x is immutable
    /// ```
    ///
    /// # Solution
    /// Use `var` for mutable variables or don't reassign.
    E2024,

    /// Error E2025: Undefined variable in assignment
    ///
    /// Attempting to assign to an undefined variable.
    E2025,

    /// Error E2026: Invalid callee
    ///
    /// The expression being called is not a function.
    ///
    /// # Example
    /// ```compile_fail
    /// var x = 42
    /// x()  // x is not callable
    /// ```
    ///
    /// # Solution
    /// Call a function name.
    E2026,

    /// Error E2027: Undefined function
    ///
    /// The function has not been declared.
    ///
    /// # Example
    /// ```compile_fail
    /// foo()  // foo is not defined
    /// ```
    ///
    /// # Solution
    /// Define the function before calling it.
    E2027,

    /// Error E2028: Argument count mismatch
    ///
    /// The number of arguments doesn't match the function's parameter count.
    ///
    /// # Example
    /// ```compile_fail
    /// fun add(a: i32, b: i32): i32 { return a + b }
    /// add(1)  // Expected 2 args, got 1
    /// ```
    ///
    /// # Solution
    /// Provide the correct number of arguments.
    E2028,

    /// Error E2029: Argument type mismatch
    ///
    /// An argument's type doesn't match the expected parameter type.
    ///
    /// # Example
    /// ```compile_fail
    /// fun foo(x: i32) { }
    /// foo("hello")  // string != i32
    /// ```
    ///
    /// # Solution
    /// Provide an argument of the correct type.
    E2029,

    /// Error E2030: Non-integer array index
    ///
    /// Array indices must be integer types.
    ///
    /// # Example
    /// ```compile_fail
    /// var arr: i32[5] = {1,2,3,4,5}
    /// var x = arr[1.5]  // f64 index not allowed
    /// ```
    ///
    /// # Solution
    /// Use an integer index.
    E2030,

    /// Error E2031: Indexing non-array type
    ///
    /// Cannot use array indexing on a non-array type.
    ///
    /// # Example
    /// ```compile_fail
    /// var x: i32 = 42
    /// var y = x[0]  // i32 is not indexable
    /// ```
    ///
    /// # Solution
    /// Only index into array types.
    E2031,

    /// Error E2032: Duplicate declaration
    ///
    /// An identifier with this name already exists in the current scope.
    ///
    /// # Example
    /// ```compile_fail
    /// var x: i32 = 1
    /// var x: i32 = 2  // x already declared
    /// ```
    ///
    /// # Solution
    /// Use a different name or remove the duplicate.
    E2032,

    // =========================================================================
    // IR GENERATION ERRORS (E3001-E3999)
    // =========================================================================
    /// Error E3001: Break outside loop in IR generation
    ///
    /// Internal error: break control flow encountered outside loop context.
    E3001,

    /// Error E3002: Continue outside loop in IR generation
    ///
    /// Internal error: continue control flow encountered outside loop context.
    E3002,

    /// Error E3003: Invalid IR instruction
    ///
    /// The IR generator attempted to create an invalid instruction.
    E3003,

    /// Error E3004: Undefined IR variable
    ///
    /// Reference to an undefined variable during IR generation.
    E3004,

    /// Error E3005: Invalid basic block
    ///
    /// Malformed basic block in the control flow graph.
    E3005,

    /// Error E3006: Invalid terminator
    ///
    /// Basic block has an invalid or missing terminator.
    E3006,

    /// Error E3007: SSA transformation error
    ///
    /// Error during Static Single Assignment transformation.
    E3007,

    /// Error E3008: CFG construction error
    ///
    /// Error during Control Flow Graph construction.
    E3008,

    // =========================================================================
    // CODE GENERATION ERRORS (E4001-E4999)
    // =========================================================================
    /// Error E4001: Invalid assembly instruction
    ///
    /// Cannot generate valid assembly for this operation.
    E4001,

    /// Error E4002: Register allocation failed
    ///
    /// Unable to allocate registers for the operation.
    E4002,

    /// Error E4003: Stack overflow
    ///
    /// Stack frame exceeds maximum size.
    E4003,

    /// Error E4004: Unsupported platform
    ///
    /// Target platform is not supported.
    E4004,

    /// Error E4005: ABI violation
    ///
    /// Generated code violates the target ABI.
    E4005,

    // =========================================================================
    // I/O AND SYSTEM ERRORS (E5001-E5999)
    // =========================================================================
    /// Error E5001: File not found
    ///
    /// The specified source file could not be found.
    E5001,

    /// Error E5002: Permission denied
    ///
    /// Insufficient permissions to access the file.
    E5002,

    /// Error E5003: Invalid file extension
    ///
    /// Source files must have the `.vn` extension.
    E5003,

    /// Error E5004: Write error
    ///
    /// Error writing output file.
    E5004,

    /// Error E5005: Read error
    ///
    /// Error reading input file.
    E5005,
}

impl ErrorCode {
    /// Returns the error code as a string (e.g., "E0001").
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jsavrs::error::error_code::ErrorCode;
    ///
    /// assert_eq!(ErrorCode::E0001.code(), "E0001");
    /// assert_eq!(ErrorCode::E2023.code(), "E2023");
    /// ```
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            // Lexical errors
            Self::E0001 => "E0001",
            Self::E0002 => "E0002",
            Self::E0003 => "E0003",
            Self::E0004 => "E0004",
            Self::E0005 => "E0005",
            Self::E0006 => "E0006",
            Self::E0007 => "E0007",
            Self::E0008 => "E0008",
            Self::E0009 => "E0009",
            Self::E0010 => "E0010",

            // Parser errors
            Self::E1001 => "E1001",
            Self::E1002 => "E1002",
            Self::E1003 => "E1003",
            Self::E1004 => "E1004",
            Self::E1005 => "E1005",
            Self::E1006 => "E1006",
            Self::E1007 => "E1007",
            Self::E1008 => "E1008",
            Self::E1009 => "E1009",
            Self::E1010 => "E1010",
            Self::E1011 => "E1011",
            Self::E1012 => "E1012",
            Self::E1013 => "E1013",
            Self::E1014 => "E1014",
            Self::E1015 => "E1015",

            // Type errors
            Self::E2001 => "E2001",
            Self::E2002 => "E2002",
            Self::E2003 => "E2003",
            Self::E2004 => "E2004",
            Self::E2005 => "E2005",
            Self::E2006 => "E2006",
            Self::E2007 => "E2007",
            Self::E2008 => "E2008",
            Self::E2009 => "E2009",
            Self::E2010 => "E2010",
            Self::E2011 => "E2011",
            Self::E2012 => "E2012",
            Self::E2013 => "E2013",
            Self::E2014 => "E2014",
            Self::E2015 => "E2015",
            Self::E2016 => "E2016",
            Self::E2017 => "E2017",
            Self::E2018 => "E2018",
            Self::E2019 => "E2019",
            Self::E2020 => "E2020",
            Self::E2021 => "E2021",
            Self::E2022 => "E2022",
            Self::E2023 => "E2023",
            Self::E2024 => "E2024",
            Self::E2025 => "E2025",
            Self::E2026 => "E2026",
            Self::E2027 => "E2027",
            Self::E2028 => "E2028",
            Self::E2029 => "E2029",
            Self::E2030 => "E2030",
            Self::E2031 => "E2031",
            Self::E2032 => "E2032",

            // IR errors
            Self::E3001 => "E3001",
            Self::E3002 => "E3002",
            Self::E3003 => "E3003",
            Self::E3004 => "E3004",
            Self::E3005 => "E3005",
            Self::E3006 => "E3006",
            Self::E3007 => "E3007",
            Self::E3008 => "E3008",

            // Code gen errors
            Self::E4001 => "E4001",
            Self::E4002 => "E4002",
            Self::E4003 => "E4003",
            Self::E4004 => "E4004",
            Self::E4005 => "E4005",

            // I/O errors
            Self::E5001 => "E5001",
            Self::E5002 => "E5002",
            Self::E5003 => "E5003",
            Self::E5004 => "E5004",
            Self::E5005 => "E5005",
        }
    }

    /// Returns the numeric portion of the error code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jsavrs::error::error_code::ErrorCode;
    ///
    /// assert_eq!(ErrorCode::E0001.numeric_code(), 1);
    /// assert_eq!(ErrorCode::E2023.numeric_code(), 2023);
    /// ```
    #[must_use]
    pub const fn numeric_code(&self) -> u16 {
        match self {
            // Lexical errors (0001-0999)
            Self::E0001 => 1,
            Self::E0002 => 2,
            Self::E0003 => 3,
            Self::E0004 => 4,
            Self::E0005 => 5,
            Self::E0006 => 6,
            Self::E0007 => 7,
            Self::E0008 => 8,
            Self::E0009 => 9,
            Self::E0010 => 10,

            // Parser errors (1001-1999)
            Self::E1001 => 1001,
            Self::E1002 => 1002,
            Self::E1003 => 1003,
            Self::E1004 => 1004,
            Self::E1005 => 1005,
            Self::E1006 => 1006,
            Self::E1007 => 1007,
            Self::E1008 => 1008,
            Self::E1009 => 1009,
            Self::E1010 => 1010,
            Self::E1011 => 1011,
            Self::E1012 => 1012,
            Self::E1013 => 1013,
            Self::E1014 => 1014,
            Self::E1015 => 1015,

            // Type errors (2001-2999)
            Self::E2001 => 2001,
            Self::E2002 => 2002,
            Self::E2003 => 2003,
            Self::E2004 => 2004,
            Self::E2005 => 2005,
            Self::E2006 => 2006,
            Self::E2007 => 2007,
            Self::E2008 => 2008,
            Self::E2009 => 2009,
            Self::E2010 => 2010,
            Self::E2011 => 2011,
            Self::E2012 => 2012,
            Self::E2013 => 2013,
            Self::E2014 => 2014,
            Self::E2015 => 2015,
            Self::E2016 => 2016,
            Self::E2017 => 2017,
            Self::E2018 => 2018,
            Self::E2019 => 2019,
            Self::E2020 => 2020,
            Self::E2021 => 2021,
            Self::E2022 => 2022,
            Self::E2023 => 2023,
            Self::E2024 => 2024,
            Self::E2025 => 2025,
            Self::E2026 => 2026,
            Self::E2027 => 2027,
            Self::E2028 => 2028,
            Self::E2029 => 2029,
            Self::E2030 => 2030,
            Self::E2031 => 2031,
            Self::E2032 => 2032,

            // IR errors (3001-3999)
            Self::E3001 => 3001,
            Self::E3002 => 3002,
            Self::E3003 => 3003,
            Self::E3004 => 3004,
            Self::E3005 => 3005,
            Self::E3006 => 3006,
            Self::E3007 => 3007,
            Self::E3008 => 3008,

            // Code gen errors (4001-4999)
            Self::E4001 => 4001,
            Self::E4002 => 4002,
            Self::E4003 => 4003,
            Self::E4004 => 4004,
            Self::E4005 => 4005,

            // I/O errors (5001-5999)
            Self::E5001 => 5001,
            Self::E5002 => 5002,
            Self::E5003 => 5003,
            Self::E5004 => 5004,
            Self::E5005 => 5005,
        }
    }

    /// Returns the severity level of this error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jsavrs::error::error_code::{ErrorCode, Severity};
    ///
    /// assert_eq!(ErrorCode::E0001.severity(), Severity::Error);
    /// assert_eq!(ErrorCode::E1013.severity(), Severity::Warning);
    /// ```
    #[must_use]
    pub const fn severity(&self) -> Severity {
        match self {
            // Warnings
            Self::E1013 => Severity::Warning,

            // All others are errors
            _ => Severity::Error,
        }
    }

    /// Returns the compiler phase where this error occurs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jsavrs::error::error_code::{ErrorCode, CompilerPhase};
    ///
    /// assert_eq!(ErrorCode::E0001.phase(), CompilerPhase::Lexer);
    /// assert_eq!(ErrorCode::E1001.phase(), CompilerPhase::Parser);
    /// assert_eq!(ErrorCode::E2023.phase(), CompilerPhase::Semantic);
    /// ```
    #[must_use]
    pub const fn phase(&self) -> CompilerPhase {
        match self.numeric_code() {
            1..=999 => CompilerPhase::Lexer,
            1001..=1999 => CompilerPhase::Parser,
            2001..=2999 => CompilerPhase::Semantic,
            3001..=3999 => CompilerPhase::IrGeneration,
            4001..=4999 => CompilerPhase::CodeGeneration,
            // 5001..=5999 and any unknown codes default to System
            _ => CompilerPhase::System,
        }
    }

    /// Returns a brief message describing this error.
    ///
    /// For detailed explanations, use [`explanation()`](Self::explanation).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jsavrs::error::error_code::ErrorCode;
    ///
    /// assert_eq!(
    ///     ErrorCode::E0001.message(),
    ///     "invalid or unrecognized token"
    /// );
    /// ```
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            // Lexical errors
            Self::E0001 => "invalid or unrecognized token",
            Self::E0002 => "malformed binary number literal",
            Self::E0003 => "malformed octal number literal",
            Self::E0004 => "malformed hexadecimal number literal",
            Self::E0005 => "unterminated string literal",
            Self::E0006 => "unterminated character literal",
            Self::E0007 => "invalid escape sequence",
            Self::E0008 => "unterminated multi-line comment",
            Self::E0009 => "invalid number suffix",
            Self::E0010 => "number literal overflow",

            // Parser errors
            Self::E1001 => "maximum recursion depth exceeded",
            Self::E1002 => "invalid type specification",
            Self::E1003 => "invalid assignment target",
            Self::E1004 => "unexpected token",
            Self::E1005 => "invalid binary operator",
            Self::E1006 => "expected expression",
            Self::E1007 => "expected statement",
            Self::E1008 => "expected identifier",
            Self::E1009 => "expected type annotation",
            Self::E1010 => "unmatched parenthesis",
            Self::E1011 => "unmatched brace",
            Self::E1012 => "unmatched bracket",
            Self::E1013 => "missing semicolon",
            Self::E1014 => "invalid function signature",
            Self::E1015 => "invalid parameter list",

            // Type errors
            Self::E2001 => "initializer count mismatch",
            Self::E2002 => "type mismatch in assignment",
            Self::E2003 => "missing return in some code paths",
            Self::E2004 => "condition must be boolean",
            Self::E2005 => "return outside function",
            Self::E2006 => "cannot return value from void function",
            Self::E2007 => "return type mismatch",
            Self::E2008 => "missing return value",
            Self::E2009 => "break outside loop",
            Self::E2010 => "continue outside loop",
            Self::E2011 => "bitwise operator requires integer operands",
            Self::E2012 => "logical operator requires boolean operands",
            Self::E2013 => "arithmetic operator requires numeric operands",
            Self::E2014 => "incompatible types in comparison",
            Self::E2015 => "type mismatch in binary operation",
            Self::E2016 => "unsupported arithmetic operation",
            Self::E2017 => "logical operation requires boolean",
            Self::E2018 => "negation requires numeric type",
            Self::E2019 => "logical NOT requires boolean type",
            Self::E2020 => "empty array literal",
            Self::E2021 => "mixed types in array literal",
            Self::E2022 => "function cannot be used as variable",
            Self::E2023 => "undefined variable",
            Self::E2024 => "cannot assign to immutable variable",
            Self::E2025 => "undefined variable in assignment",
            Self::E2026 => "callee must be a function",
            Self::E2027 => "undefined function",
            Self::E2028 => "wrong number of arguments",
            Self::E2029 => "argument type mismatch",
            Self::E2030 => "array index must be integer",
            Self::E2031 => "cannot index non-array type",
            Self::E2032 => "duplicate declaration",

            // IR errors
            Self::E3001 => "break outside loop in IR",
            Self::E3002 => "continue outside loop in IR",
            Self::E3003 => "invalid IR instruction",
            Self::E3004 => "undefined variable in IR",
            Self::E3005 => "invalid basic block",
            Self::E3006 => "invalid block terminator",
            Self::E3007 => "SSA transformation error",
            Self::E3008 => "CFG construction error",

            // Code gen errors
            Self::E4001 => "invalid assembly instruction",
            Self::E4002 => "register allocation failed",
            Self::E4003 => "stack frame overflow",
            Self::E4004 => "unsupported target platform",
            Self::E4005 => "ABI violation",

            // I/O errors
            Self::E5001 => "file not found",
            Self::E5002 => "permission denied",
            Self::E5003 => "invalid file extension",
            Self::E5004 => "write error",
            Self::E5005 => "read error",
        }
    }

    /// Returns a detailed explanation of this error with examples and solutions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use jsavrs::error::error_code::ErrorCode;
    ///
    /// let explanation = ErrorCode::E2023.explanation();
    /// assert!(explanation.contains("declare"));
    /// ```
    #[must_use]
    pub const fn explanation(&self) -> &'static str {
        match self {
            Self::E0001 => {
                "The lexer encountered a character sequence that doesn't match any valid token pattern.\n\
                Check for typos or unsupported characters. Valid tokens include identifiers,\n\
                keywords, operators, and literals."
            }
            Self::E0002 => {
                "Binary literals must have at least one binary digit (0 or 1) after `#b`.\n\
                Example: `#b1010` for decimal 10."
            }
            Self::E0003 => {
                "Octal literals must have at least one octal digit (0-7) after `#o`.\n\
                Example: `#o755` for decimal 493."
            }
            Self::E0004 => {
                "Hexadecimal literals must have at least one hex digit (0-9, a-f, A-F) after `#x`.\n\
                Example: `#xDEAD` for decimal 57005."
            }
            Self::E0005 => {
                "String literals must be closed with a matching double quote before the end of the line.\n\
                Example: `\"hello world\"` instead of `\"hello world`."
            }
            Self::E0006 => {
                "Character literals must be closed with a matching single quote.\n\
                Example: `'x'` instead of `'x`."
            }
            Self::E0007 => {
                "The escape sequence is not recognized. Valid escape sequences include:\n\
                \\n (newline), \\r (carriage return), \\t (tab), \\\\ (backslash),\n\
                \\' (single quote), \\\" (double quote), \\0 (null), \\u{XXXX} (unicode)."
            }
            Self::E0008 => {
                "Multi-line comments opened with `/*` must be closed with `*/`.\n\
                Check for missing closing markers or accidental nesting."
            }
            Self::E0009 => {
                "The suffix on the numeric literal is not recognized.\n\
                Valid suffixes: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64."
            }
            Self::E0010 => {
                "The numeric value exceeds the range of the target type.\n\
                Use a larger type or reduce the value."
            }
            Self::E1001 => {
                "The parser has exceeded its recursion limit due to deeply nested expressions.\n\
                Simplify the expression or break it into smaller parts."
            }
            Self::E1002 => {
                "Expected a valid type but found something else.\n\
                Valid types: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, char, string, bool,\n\
                or custom type identifiers."
            }
            Self::E1003 => {
                "Only variables and array elements can be assigned to.\n\
                Examples: `x = 5` or `arr[0] = 1`."
            }
            Self::E2023 => {
                "The variable has not been declared in the current scope or any outer scope.\n\
                Declare the variable with `var` or `const` before using it."
            }
            Self::E2024 => {
                "Variables declared with `const` cannot be reassigned.\n\
                Use `var` for mutable variables or remove the reassignment."
            }
            Self::E2027 => {
                "The function has not been declared.\n\
                Define the function with `fun` before calling it."
            }
            Self::E2028 => {
                "The number of arguments provided doesn't match the function's parameter count.\n\
                Check the function definition and provide the correct number of arguments."
            }
            // Default explanation for other errors
            _ => "See the error message for details.",
        }
    }

    /// Returns suggested fixes for this error, if available.
    ///
    /// Returns an empty slice if no specific suggestions are available.
    #[must_use]
    pub const fn suggestions(&self) -> &'static [&'static str] {
        match self {
            Self::E0002 => &["Add binary digits after #b: #b1010", "Check for invalid digits (only 0 and 1 allowed)"],
            Self::E0003 => &["Add octal digits after #o: #o755", "Check for invalid digits (only 0-7 allowed)"],
            Self::E0004 => {
                &["Add hexadecimal digits after #x: #xDEAD", "Check for invalid digits (only 0-9, a-f, A-F allowed)"]
            }
            Self::E0005 => &[
                "Add a closing double quote: \"hello\"",
                "Use escape sequence for embedded quotes: \"say \\\"hello\\\"\"",
            ],
            Self::E2023 => &[
                "Declare the variable: var x: i32 = 0",
                "Check for typos in the variable name",
                "Ensure the variable is in scope",
            ],
            Self::E2024 => &["Use 'var' instead of 'const' for mutable variables", "Remove the reassignment"],
            Self::E2009 | Self::E2010 => {
                &["Move the statement inside a while or for loop", "Use return to exit a function instead"]
            }
            _ => &[],
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code(), self.message())
    }
}

impl std::error::Error for ErrorCode {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_format() {
        assert_eq!(ErrorCode::E0001.code(), "E0001");
        assert_eq!(ErrorCode::E1001.code(), "E1001");
        assert_eq!(ErrorCode::E2023.code(), "E2023");
        assert_eq!(ErrorCode::E3001.code(), "E3001");
        assert_eq!(ErrorCode::E4001.code(), "E4001");
        assert_eq!(ErrorCode::E5001.code(), "E5001");
    }

    #[test]
    fn test_numeric_code() {
        assert_eq!(ErrorCode::E0001.numeric_code(), 1);
        assert_eq!(ErrorCode::E2023.numeric_code(), 2023);
    }

    #[test]
    fn test_phase_detection() {
        assert_eq!(ErrorCode::E0001.phase(), CompilerPhase::Lexer);
        assert_eq!(ErrorCode::E1001.phase(), CompilerPhase::Parser);
        assert_eq!(ErrorCode::E2023.phase(), CompilerPhase::Semantic);
        assert_eq!(ErrorCode::E3001.phase(), CompilerPhase::IrGeneration);
        assert_eq!(ErrorCode::E4001.phase(), CompilerPhase::CodeGeneration);
        assert_eq!(ErrorCode::E5001.phase(), CompilerPhase::System);
    }

    #[test]
    fn test_severity() {
        assert_eq!(ErrorCode::E0001.severity(), Severity::Error);
        assert_eq!(ErrorCode::E1013.severity(), Severity::Warning);
    }

    #[test]
    fn test_display() {
        let code = ErrorCode::E2023;
        let display = format!("{code}");
        assert!(display.contains("E2023"));
        assert!(display.contains("undefined variable"));
    }

    #[test]
    fn test_suggestions_not_empty() {
        let suggestions = ErrorCode::E2023.suggestions();
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_explanation_not_empty() {
        let explanation = ErrorCode::E2023.explanation();
        assert!(!explanation.is_empty());
        assert!(explanation.contains("declare"));
    }
}
