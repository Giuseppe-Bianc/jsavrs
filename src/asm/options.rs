//! Options for controlling assembly code generation
//!
//! Defines configuration options for the assembly generator including
//! optimization levels, debug information, and instruction set flags.

/// Options for controlling assembly code generation
#[derive(Debug, Clone)]
pub struct CodeGenOptions {
    /// Optimization level (0 = debug, 1 = basic, 2 = aggressive)
    pub optimization_level: u8,
    
    /// Include debug information in assembly
    pub debug_info: bool,
    
    /// Generate human-readable comments
    pub include_comments: bool,
    
    /// Symbol naming prefix
    pub symbol_prefix: Option<String>,
    
    /// Maximum stack frame size (bytes)
    pub max_stack_frame_size: u32,
    
    /// Enable/disable specific instruction sets
    pub instruction_sets: InstructionSetFlags,
}

impl Default for CodeGenOptions {
    fn default() -> Self {
        Self {
            optimization_level: 1,
            debug_info: false,
            include_comments: true,
            symbol_prefix: None,
            max_stack_frame_size: 1024 * 1024, // 1MB
            instruction_sets: InstructionSetFlags {
                sse: true,
                sse2: true,
                avx: false,
                avx2: false,
            },
        }
    }
}

/// Bit flags for enabling instruction set extensions
#[derive(Debug, Clone, Copy)]
pub struct InstructionSetFlags {
    pub sse: bool,
    pub sse2: bool,
    pub avx: bool,
    pub avx2: bool,
}

impl InstructionSetFlags {
    pub fn new() -> Self {
        InstructionSetFlags {
            sse: false,
            sse2: false,
            avx: false,
            avx2: false,
        }
    }
    
    pub fn with_sse(mut self) -> Self {
        self.sse = true;
        self
    }
    
    pub fn with_sse2(mut self) -> Self {
        self.sse2 = true;
        self
    }
    
    pub fn with_avx(mut self) -> Self {
        self.avx = true;
        self
    }
    
    pub fn with_avx2(mut self) -> Self {
        self.avx2 = true;
        self
    }
}