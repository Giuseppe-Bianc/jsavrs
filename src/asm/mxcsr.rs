//! x86-64 MXCSR register management for floating-point control and status

use super::exception::IEEE754ExceptionType;
use super::rounding::RoundingMode;

/// Special floating-point handling modes for performance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushToZeroMode {
    /// Standard IEEE 754 behavior with subnormal numbers
    Standard,
    /// Flush-To-Zero: treat subnormals as zero for performance
    FlushToZero,
    /// Denormals-Are-Zero: treat subnormal inputs as zero
    DenormalsAreZero,
    /// Both FTZ and DAZ enabled
    Both,
}

/// x86-64 MXCSR register representation for SSE/AVX floating-point operations
#[derive(Debug, Clone, PartialEq)]
pub struct MXCSRRegister {
    /// Exception mask bits (true = masked, false = unmasked)
    pub exception_masks: [bool; 5], // Order: IE, ZE, OE, UE, PE (Invalid, Zero-divide, Overflow, Underflow, Precision)
    /// Exception status flags (true = exception occurred)
    pub exception_flags: [bool; 5], // Same order as masks
    /// Current rounding mode
    pub rounding_mode: RoundingMode,
    /// Flush-to-zero and denormals-are-zero modes
    pub ftz_daz_mode: FlushToZeroMode,
}

impl Default for MXCSRRegister {
    fn default() -> Self {
        Self {
            // Default: all exceptions masked (true)
            exception_masks: [true; 5],
            // No exceptions initially
            exception_flags: [false; 5],
            // Default rounding mode
            rounding_mode: RoundingMode::default(),
            // Standard mode
            ftz_daz_mode: FlushToZeroMode::Standard,
        }
    }
}

impl MXCSRRegister {
    /// Create a new MXCSR with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set exception mask for a specific exception type
    pub fn set_exception_mask(&mut self, exception: IEEE754ExceptionType, masked: bool) {
        let index = exception_type_to_index(exception);
        self.exception_masks[index] = masked;
    }

    /// Get exception mask for a specific exception type
    pub fn get_exception_mask(&self, exception: IEEE754ExceptionType) -> bool {
        let index = exception_type_to_index(exception);
        self.exception_masks[index]
    }

    /// Set exception flag for a specific exception type
    pub fn set_exception_flag(&mut self, exception: IEEE754ExceptionType, flag: bool) {
        let index = exception_type_to_index(exception);
        self.exception_flags[index] = flag;
    }

    /// Get exception flag for a specific exception type
    pub fn get_exception_flag(&self, exception: IEEE754ExceptionType) -> bool {
        let index = exception_type_to_index(exception);
        self.exception_flags[index]
    }

    /// Clear all exception flags
    pub fn clear_exception_flags(&mut self) {
        self.exception_flags = [false; 5];
    }

    /// Check if any exception has occurred (any flag is set)
    pub fn has_any_exception(&self) -> bool {
        self.exception_flags.iter().any(|&flag| flag)
    }

    /// Get all exception flags that are currently set
    pub fn get_set_exception_flags(&self) -> Vec<IEEE754ExceptionType> {
        let mut exceptions = Vec::new();
        for (i, &flag) in self.exception_flags.iter().enumerate() {
            if flag {
                match i {
                    0 => exceptions.push(IEEE754ExceptionType::InvalidOperation),
                    1 => exceptions.push(IEEE754ExceptionType::DivisionByZero),
                    2 => exceptions.push(IEEE754ExceptionType::Overflow),
                    3 => exceptions.push(IEEE754ExceptionType::Underflow),
                    4 => exceptions.push(IEEE754ExceptionType::Inexact),
                    _ => {} // Should not happen
                }
            }
        }
        exceptions
    }

    /// Set the rounding mode
    pub fn set_rounding_mode(&mut self, mode: RoundingMode) {
        self.rounding_mode = mode;
    }

    /// Get the current rounding mode
    pub fn get_rounding_mode(&self) -> RoundingMode {
        self.rounding_mode
    }

    /// Set the FTZ/DAZ mode
    pub fn set_ftz_daz_mode(&mut self, mode: FlushToZeroMode) {
        self.ftz_daz_mode = mode;
    }

    /// Get the current FTZ/DAZ mode
    pub fn get_ftz_daz_mode(&self) -> FlushToZeroMode {
        self.ftz_daz_mode
    }

    /// Check if denormals are flushed to zero
    pub fn is_flush_to_zero_enabled(&self) -> bool {
        matches!(self.ftz_daz_mode, FlushToZeroMode::FlushToZero | FlushToZeroMode::Both)
    }

    /// Check if denormal inputs are treated as zero
    pub fn is_denormals_are_zero_enabled(&self) -> bool {
        matches!(self.ftz_daz_mode, FlushToZeroMode::DenormalsAreZero | FlushToZeroMode::Both)
    }

    /// Check if a specific exception is masked
    pub fn is_exception_masked(&self, exception: IEEE754ExceptionType) -> bool {
        self.get_exception_mask(exception)
    }

    /// Check if a specific exception has occurred and is unmasked
    pub fn has_unmasked_exception(&self, exception: IEEE754ExceptionType) -> bool {
        !self.is_exception_masked(exception) && self.get_exception_flag(exception)
    }

    /// Generate x86-64 assembly code to load this MXCSR value
    pub fn generate_stmxcsr_code(&self) -> String {
        // For now, return a placeholder as we don't have instruction generation here
        "    ; MXCSR register state management".to_string()
    }

    /// Generate x86-64 assembly code to store this MXCSR value
    pub fn generate_ldmxcsr_code(&self) -> String {
        // For now, return a placeholder as we don't have instruction generation here
        "    ; MXCSR register state setting".to_string()
    }

    /// Create an MXCSR register with all exceptions unmasked (strict IEEE 754 mode)
    pub fn strict_ieee754() -> Self {
        Self {
            exception_masks: [false; 5], // Unmask all exceptions
            exception_flags: [false; 5], // No exceptions initially
            rounding_mode: RoundingMode::ToNearest, // Default IEEE 754 rounding
            ftz_daz_mode: FlushToZeroMode::Standard, // Standard behavior, no FTZ/DAZ
        }
    }

    /// Create an MXCSR register with performance-optimized settings (FTZ/DAZ enabled)
    pub fn performance_optimized() -> Self {
        Self {
            exception_masks: [true; 5], // Mask all exceptions for performance
            exception_flags: [false; 5], // No exceptions initially
            rounding_mode: RoundingMode::ToNearest, // Default IEEE 754 rounding
            ftz_daz_mode: FlushToZeroMode::Both, // Enable both FTZ and DAZ for performance
        }
    }
}

/// Convert IEEE754ExceptionType to array index
fn exception_type_to_index(exception: IEEE754ExceptionType) -> usize {
    match exception {
        IEEE754ExceptionType::InvalidOperation => 0, // IE (Invalid)
        IEEE754ExceptionType::DivisionByZero => 1,   // ZE (Zero-divide) - simplified mapping
        IEEE754ExceptionType::Overflow => 2,         // OE (Overflow)
        IEEE754ExceptionType::Underflow => 3,        // UE (Underflow)
        IEEE754ExceptionType::Inexact => 4,          // PE (Precision)
    }
}
