//! IEEE 754 exception handling for floating-point operations

/// The five standard IEEE 754 exception types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IEEE754ExceptionType {
    /// Invalid operation (e.g., sqrt of negative number, 0/0, inf-inf)
    InvalidOperation,
    /// Division by zero (finite/0)
    DivisionByZero,
    /// Overflow (result too large for target format)
    Overflow,
    /// Underflow (result too small for target format)
    Underflow,
    /// Inexact (result cannot be represented exactly)
    Inexact,
}

impl std::fmt::Display for IEEE754ExceptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidOperation => write!(f, "invalid_operation"),
            Self::DivisionByZero => write!(f, "division_by_zero"),
            Self::Overflow => write!(f, "overflow"),
            Self::Underflow => write!(f, "underflow"),
            Self::Inexact => write!(f, "inexact"),
        }
    }
}

/// IEEE 754 exception handling and management
pub struct IEEE754ExceptionHandler {
    /// Mask for each exception type (true = masked, false = unmasked)
    exception_masks: [bool; 5],
    /// Status flags for each exception type (true = occurred)
    exception_flags: [bool; 5],
}

impl IEEE754ExceptionHandler {
    /// Create a new exception handler with all exceptions masked by default
    pub fn new() -> Self {
        Self {
            exception_masks: [true; 5],  // All masked by default
            exception_flags: [false; 5], // No exceptions initially
        }
    }

    /// Create a strict exception handler with all exceptions unmasked
    pub fn strict() -> Self {
        Self {
            exception_masks: [false; 5], // All unmasked
            exception_flags: [false; 5], // No exceptions initially
        }
    }

    /// Create a permissive exception handler with all exceptions masked
    pub fn permissive() -> Self {
        Self {
            exception_masks: [true; 5],  // All masked
            exception_flags: [false; 5], // No exceptions initially
        }
    }

    /// Raise an exception (set its flag)
    pub fn raise_exception(&mut self, exception_type: IEEE754ExceptionType) {
        let index = self.exception_type_to_index(exception_type);
        self.exception_flags[index] = true;
    }

    /// Check if a specific exception has been raised
    pub fn has_exception(&self, exception_type: IEEE754ExceptionType) -> bool {
        let index = self.exception_type_to_index(exception_type);
        self.exception_flags[index]
    }

    /// Check if a specific exception is masked
    pub fn is_exception_masked(&self, exception_type: IEEE754ExceptionType) -> bool {
        let index = self.exception_type_to_index(exception_type);
        self.exception_masks[index]
    }

    /// Set the mask for a specific exception type
    pub fn set_exception_mask(&mut self, exception_type: IEEE754ExceptionType, masked: bool) {
        let index = self.exception_type_to_index(exception_type);
        self.exception_masks[index] = masked;
    }

    /// Get all exceptions that have occurred and are unmasked (not suppressed)
    pub fn get_unmasked_raised_exceptions(&self) -> Vec<IEEE754ExceptionType> {
        let mut exceptions = Vec::new();
        for (i, (&flag, &masked)) in self.exception_flags.iter().zip(self.exception_masks.iter()).enumerate() {
            if flag && !masked {
                exceptions.push(self.index_to_exception_type(i));
            }
        }
        exceptions
    }

    /// Get all exceptions that have occurred (regardless of masking)
    pub fn get_all_raised_exceptions(&self) -> Vec<IEEE754ExceptionType> {
        let mut exceptions = Vec::new();
        for (i, &flag) in self.exception_flags.iter().enumerate() {
            if flag {
                exceptions.push(self.index_to_exception_type(i));
            }
        }
        exceptions
    }

    /// Clear the flag for a specific exception
    pub fn clear_exception(&mut self, exception_type: IEEE754ExceptionType) {
        let index = self.exception_type_to_index(exception_type);
        self.exception_flags[index] = false;
    }

    /// Clear all exception flags
    pub fn clear_all_exceptions(&mut self) {
        self.exception_flags = [false; 5];
    }

    /// Check if any unmasked exceptions have occurred
    pub fn has_any_unmasked_exception(&self) -> bool {
        self.get_unmasked_raised_exceptions().is_empty()
    }

    /// Check if any exceptions have occurred (regardless of masking)
    pub fn has_any_exception(&self) -> bool {
        self.exception_flags.iter().any(|&flag| flag)
    }

    /// Get the x86-64 exception flag bit position for this exception type
    pub fn get_x86_exception_bit(&self, exception_type: IEEE754ExceptionType) -> u8 {
        // In x86-64 MXCSR, the bits are arranged as follows:
        // Bit 0: Invalid Operation (IE)
        // Bit 2: Denormal (DE) - not in our enum
        // Bit 3: Divide-by-zero (ZE)
        // Bit 4: Overflow (OE)
        // Bit 5: Underflow (UE)
        // Bit 6: Precision (PE)
        match exception_type {
            IEEE754ExceptionType::InvalidOperation => 0, // IE bit
            IEEE754ExceptionType::DivisionByZero => 3,   // ZE bit
            IEEE754ExceptionType::Overflow => 4,         // OE bit
            IEEE754ExceptionType::Underflow => 5,        // UE bit
            IEEE754ExceptionType::Inexact => 6,          // PE bit
        }
    }

    /// Get the exception type from the x86-64 bit position
    pub fn get_exception_from_x86_bit(bit: u8) -> Option<IEEE754ExceptionType> {
        match bit {
            0 => Some(IEEE754ExceptionType::InvalidOperation),
            3 => Some(IEEE754ExceptionType::DivisionByZero),
            4 => Some(IEEE754ExceptionType::Overflow),
            5 => Some(IEEE754ExceptionType::Underflow),
            6 => Some(IEEE754ExceptionType::Inexact),
            _ => None, // Other bits in MXCSR are not our exceptions
        }
    }

    /// Convert exception type to internal array index
    fn exception_type_to_index(&self, exception_type: IEEE754ExceptionType) -> usize {
        match exception_type {
            IEEE754ExceptionType::InvalidOperation => 0,
            IEEE754ExceptionType::DivisionByZero => 1,
            IEEE754ExceptionType::Overflow => 2,
            IEEE754ExceptionType::Underflow => 3,
            IEEE754ExceptionType::Inexact => 4,
        }
    }

    /// Convert internal array index to exception type
    fn index_to_exception_type(&self, index: usize) -> IEEE754ExceptionType {
        match index {
            0 => IEEE754ExceptionType::InvalidOperation,
            1 => IEEE754ExceptionType::DivisionByZero,
            2 => IEEE754ExceptionType::Overflow,
            3 => IEEE754ExceptionType::Underflow,
            4 => IEEE754ExceptionType::Inexact,
            _ => panic!("Invalid exception index: {}", index),
        }
    }
}

impl Default for IEEE754ExceptionHandler {
    fn default() -> Self {
        Self::new()
    }
}
