//! IEEE 754 rounding mode control for floating-point operations

/// The four standard IEEE 754 rounding modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoundingMode {
    /// Round to the nearest representable value (ties to even)
    ToNearest,
    /// Round toward positive infinity
    TowardPositiveInfinity,
    /// Round toward negative infinity
    TowardNegativeInfinity,
    /// Round toward zero (truncate)
    TowardZero,
}

impl std::fmt::Display for RoundingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ToNearest => write!(f, "to_nearest"),
            Self::TowardPositiveInfinity => write!(f, "toward_positive_infinity"),
            Self::TowardNegativeInfinity => write!(f, "toward_negative_infinity"),
            Self::TowardZero => write!(f, "toward_zero"),
        }
    }
}

impl Default for RoundingMode {
    fn default() -> Self {
        Self::ToNearest // IEEE 754 default
    }
}

impl RoundingMode {
    /// Get the x86-64 rounding mode bits for the MXCSR register.
    /// In x86-64, rounding mode is encoded in bits 13-14 of MXCSR.
    pub fn get_x86_mxcsr_bits(&self) -> u8 {
        match self {
            Self::ToNearest => 0b00,                // 00: Round to nearest (even)
            Self::TowardPositiveInfinity => 0b01,   // 01: Round up (toward +inf)
            Self::TowardNegativeInfinity => 0b10,   // 10: Round down (toward -inf)
            Self::TowardZero => 0b11,               // 11: Round toward zero (truncate)
        }
    }

    /// Create a RoundingMode from x86-64 MXCSR bits (bits 13-14)
    pub fn from_x86_mxcsr_bits(bits: u8) -> Option<Self> {
        match bits & 0b11 {  // Only look at the last 2 bits
            0b00 => Some(Self::ToNearest),
            0b01 => Some(Self::TowardPositiveInfinity),
            0b10 => Some(Self::TowardNegativeInfinity),
            0b11 => Some(Self::TowardZero),
            _ => None, // Should not happen
        }
    }

    /// Get a description of the rounding mode
    pub fn description(&self) -> &'static str {
        match self {
            Self::ToNearest => "Round to nearest, ties to even",
            Self::TowardPositiveInfinity => "Round toward positive infinity (ceiling)",
            Self::TowardNegativeInfinity => "Round toward negative infinity (floor)",
            Self::TowardZero => "Round toward zero (truncation)",
        }
    }

    /// Get the abbreviation for the rounding mode
    pub fn abbreviation(&self) -> &'static str {
        match self {
            Self::ToNearest => "rne",  // Round to Nearest Even
            Self::TowardPositiveInfinity => "rup",   // Round Up
            Self::TowardNegativeInfinity => "rdo",   // Round Down
            Self::TowardZero => "rtz",   // Round Toward Zero
        }
    }
}

/// Rounding mode control system
pub struct RoundingModeController {
    current_mode: RoundingMode,
}

impl RoundingModeController {
    /// Create a new controller with the default rounding mode
    pub fn new() -> Self {
        Self {
            current_mode: RoundingMode::default(),
        }
    }

    /// Create a controller with a specific rounding mode
    pub fn with_mode(mode: RoundingMode) -> Self {
        Self {
            current_mode: mode,
        }
    }

    /// Set the rounding mode
    pub fn set_rounding_mode(&mut self, mode: RoundingMode) {
        self.current_mode = mode;
    }

    /// Get the current rounding mode
    pub fn get_rounding_mode(&self) -> RoundingMode {
        self.current_mode
    }

    /// Apply the rounding mode to an x86-64 MXCSR register value
    pub fn apply_to_mxcsr(&self, mxcsr: u32) -> u32 {
        // Clear the rounding mode bits (bits 13-14)
        let mxcsr = mxcsr & !(0b11 << 13);
        // Set the new rounding mode bits
        mxcsr | ((self.current_mode.get_x86_mxcsr_bits() as u32) << 13)
    }

    /// Get the rounding mode from an x86-64 MXCSR register value
    pub fn from_mxcsr(mxcsr: u32) -> Option<RoundingMode> {
        let bits = ((mxcsr >> 13) & 0b11) as u8;
        RoundingMode::from_x86_mxcsr_bits(bits)
    }

    /// Create a rounding mode controller from MXCSR value
    pub fn from_mxcsr_value(mxcsr: u32) -> Self {
        let mode = Self::from_mxcsr(mxcsr).unwrap_or_default();
        Self { current_mode: mode }
    }
}

impl Default for RoundingModeController {
    fn default() -> Self {
        Self::new()
    }
}
