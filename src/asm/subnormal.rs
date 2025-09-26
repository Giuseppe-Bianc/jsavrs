//! Subnormal number handling for IEEE 754 floating-point operations
//! 
//! This module provides support for handling subnormal (denormal) numbers
//! including Flush-To-Zero (FTZ) and Denormals-Are-Zero (DAZ) modes.

use std::f64;

/// Subnormal number handling modes for performance optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubnormalHandlingMode {
    /// Standard IEEE 754 behavior with full subnormal number support
    Standard,
    /// Flush-To-Zero: treat subnormal results as zero 
    FlushToZero,
    /// Denormals-Are-Zero: treat subnormal inputs as zero
    DenormalsAreZero,
    /// Both FTZ and DAZ enabled for maximum performance
    Both,
}

impl SubnormalHandlingMode {
    /// Check if Flush-To-Zero mode is enabled
    pub fn is_flush_to_zero_enabled(&self) -> bool {
        matches!(self, SubnormalHandlingMode::FlushToZero | SubnormalHandlingMode::Both)
    }

    /// Check if Denormals-Are-Zero mode is enabled
    pub fn is_denormals_are_zero_enabled(&self) -> bool {
        matches!(self, SubnormalHandlingMode::DenormalsAreZero | SubnormalHandlingMode::Both)
    }

    /// Check if standard subnormal handling is enabled
    pub fn is_standard_enabled(&self) -> bool {
        matches!(self, SubnormalHandlingMode::Standard)
    }
}

/// Subnormal number detection and handling utilities
pub struct SubnormalHandler {
    mode: SubnormalHandlingMode,
}

impl SubnormalHandler {
    /// Create a new handler with the specified mode
    pub fn new(mode: SubnormalHandlingMode) -> Self {
        Self { mode }
    }

    /// Create a handler with standard subnormal handling
    pub fn standard() -> Self {
        Self::new(SubnormalHandlingMode::Standard)
    }

    /// Create a handler with Flush-To-Zero mode
    pub fn flush_to_zero() -> Self {
        Self::new(SubnormalHandlingMode::FlushToZero)
    }

    /// Create a handler with Denormals-Are-Zero mode
    pub fn denormals_are_zero() -> Self {
        Self::new(SubnormalHandlingMode::DenormalsAreZero)
    }

    /// Create a handler with both FTZ and DAZ enabled
    pub fn performance_optimized() -> Self {
        Self::new(SubnormalHandlingMode::Both)
    }

    /// Check if a f64 value is subnormal (non-zero but smaller than the minimum normal number)
    pub fn is_subnormal_f64(value: f64) -> bool {
        if value == 0.0 || value.is_infinite() || value.is_nan() {
            return false;
        }
        value.abs() < f64::MIN_POSITIVE
    }

    /// Check if a f32 value is subnormal (non-zero but smaller than the minimum normal number)
    pub fn is_subnormal_f32(value: f32) -> bool {
        if value == 0.0 || value.is_infinite() || value.is_nan() {
            return false;
        }
        value.abs() < f32::MIN_POSITIVE
    }

    /// Process a f64 value according to the current subnormal handling mode
    pub fn process_f64(&self, value: f64) -> f64 {
        match self.mode {
            SubnormalHandlingMode::Standard => value, // Standard: keep subnormals as is
            SubnormalHandlingMode::FlushToZero => {
                // FTZ: treat subnormal results as zero
                if Self::is_subnormal_f64(value) {
                    if value.is_sign_negative() {
                        -0.0
                    } else {
                        0.0
                    }
                } else {
                    value
                }
            },
            SubnormalHandlingMode::DenormalsAreZero => {
                // DAZ: This would typically be applied to inputs before operations,
                // but we'll include it here for consistency
                value
            },
            SubnormalHandlingMode::Both => {
                // Both FTZ and DAZ: treat subnormal results as zero
                if Self::is_subnormal_f64(value) {
                    if value.is_sign_negative() {
                        -0.0
                    } else {
                        0.0
                    }
                } else {
                    value
                }
            }
        }
    }

    /// Process a f32 value according to the current subnormal handling mode
    pub fn process_f32(&self, value: f32) -> f32 {
        match self.mode {
            SubnormalHandlingMode::Standard => value, // Standard: keep subnormals as is
            SubnormalHandlingMode::FlushToZero => {
                // FTZ: treat subnormal results as zero
                if Self::is_subnormal_f32(value) {
                    if value.is_sign_negative() {
                        -0.0
                    } else {
                        0.0
                    }
                } else {
                    value
                }
            },
            SubnormalHandlingMode::DenormalsAreZero => {
                // DAZ: This would typically be applied to inputs before operations,
                // but we'll include it here for consistency
                value
            },
            SubnormalHandlingMode::Both => {
                // Both FTZ and DAZ: treat subnormal results as zero
                if Self::is_subnormal_f32(value) {
                    if value.is_sign_negative() {
                        -0.0
                    } else {
                        0.0
                    }
                } else {
                    value
                }
            }
        }
    }

    /// Process f64 inputs before arithmetic operations according to DAZ mode
    pub fn process_input_f64(&self, value: f64) -> f64 {
        match self.mode {
            SubnormalHandlingMode::DenormalsAreZero | SubnormalHandlingMode::Both => {
                // DAZ: treat subnormal inputs as zero
                if Self::is_subnormal_f64(value) {
                    if value.is_sign_negative() {
                        -0.0
                    } else {
                        0.0
                    }
                } else {
                    value
                }
            },
            _ => value, // Standard, FTZ: don't modify inputs
        }
    }

    /// Process f32 inputs before arithmetic operations according to DAZ mode
    pub fn process_input_f32(&self, value: f32) -> f32 {
        match self.mode {
            SubnormalHandlingMode::DenormalsAreZero | SubnormalHandlingMode::Both => {
                // DAZ: treat subnormal inputs as zero
                if Self::is_subnormal_f32(value) {
                    if value.is_sign_negative() {
                        -0.0
                    } else {
                        0.0
                    }
                } else {
                    value
                }
            },
            _ => value, // Standard, FTZ: don't modify inputs
        }
    }

    /// Set the subnormal handling mode
    pub fn set_mode(&mut self, mode: SubnormalHandlingMode) {
        self.mode = mode;
    }

    /// Get the current subnormal handling mode
    pub fn get_mode(&self) -> SubnormalHandlingMode {
        self.mode
    }

    /// Get the x86-64 MXCSR bits for FTZ and DAZ flags
    /// FTZ is bit 15, DAZ is bit 6 in the MXCSR register
    pub fn get_mxcsr_bits(&self) -> (u32, u32) {
        let ftz_bit = if self.mode.is_flush_to_zero_enabled() { 1u32 << 15 } else { 0 };
        let daz_bit = if self.mode.is_denormals_are_zero_enabled() { 1u32 << 6 } else { 0 };
        (ftz_bit, daz_bit)
    }

    /// Apply the subnormal handling settings to an MXCSR value
    pub fn apply_to_mxcsr(&self, mxcsr: u32) -> u32 {
        let (ftz_bit, daz_bit) = self.get_mxcsr_bits();
        // Clear existing FTZ and DAZ bits
        let mxcsr = mxcsr & !( (1u32 << 15) | (1u32 << 6) );
        // Set the new bits
        mxcsr | ftz_bit | daz_bit
    }
}

impl Default for SubnormalHandler {
    fn default() -> Self {
        Self::standard()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subnormal() {
        assert!(SubnormalHandler::is_subnormal_f64(1e-308)); // Subnormal
        assert!(!SubnormalHandler::is_subnormal_f64(1e-307)); // Normal
        assert!(!SubnormalHandler::is_subnormal_f64(0.0)); // Zero
        assert!(!SubnormalHandler::is_subnormal_f64(f64::INFINITY)); // Infinity
    }

    #[test]
    fn test_flush_to_zero() {
        let handler = SubnormalHandler::flush_to_zero();
        assert_eq!(handler.process_f64(1e-308), 0.0); // Subnormal becomes zero
        assert_eq!(handler.process_f64(1e-307), 1e-307); // Normal stays normal
    }

    #[test]
    fn test_process_input_daz() {
        let handler = SubnormalHandler::denormals_are_zero();
        assert_eq!(handler.process_input_f64(1e-308), 0.0); // Subnormal input becomes zero
        assert_eq!(handler.process_input_f64(1e-307), 1e-307); // Normal input stays normal
    }
}