//! Warning generation utilities for type promotions.
//!
//! This module provides functions to generate warnings about potentially unsafe
//! or lossy type conversions.

use crate::ir::{CastKind, IrType};

use super::types::{PromotionRule, PromotionWarning};

/// Generate precision loss warning for a type conversion (T019)
pub fn generate_precision_loss_warning(
    from_type: &IrType, to_type: &IrType, rule: &PromotionRule,
) -> Option<PromotionWarning> {
    if let PromotionRule::Direct { may_lose_precision, precision_loss_estimate, .. } = rule
        && *may_lose_precision
        && let Some(estimate) = precision_loss_estimate
    {
        return Some(PromotionWarning::PrecisionLoss {
            from_type: from_type.clone(),
            to_type: to_type.clone(),
            estimated_loss: *estimate,
        });
    }
    None
}

/// Generate signedness change warning for a type conversion (T020)
pub fn generate_signedness_change_warning(
    from_type: &IrType, to_type: &IrType, rule: &PromotionRule,
) -> Option<PromotionWarning> {
    if let PromotionRule::Direct { cast_kind, .. } = rule
        && *cast_kind == CastKind::IntBitcast
    {
        let from_signed = from_type.is_signed_integer();
        let to_signed = to_type.is_signed_integer();
        // Check if exactly one is signed (XOR logic)
        if from_signed != to_signed {
            return Some(PromotionWarning::SignednessChange { from_signed, to_signed, may_affect_comparisons: true });
        }
    }
    None
}

/// Generate Unicode validation warning for integer→char conversions (T030)
pub fn generate_unicode_validation_warning(value: u32, to_type: &IrType) -> Option<PromotionWarning> {
    // Only generate warnings for char target type
    if *to_type != IrType::Char {
        return None;
    }

    // Validate Unicode scalar value
    if !is_valid_unicode_scalar(value) {
        let reason = if (0xD800..=0xDFFF).contains(&value) {
            "surrogate code point (reserved for UTF-16)".to_string()
        } else if value > 0x10FFFF {
            "value exceeds maximum Unicode code point U+10FFFF".to_string()
        } else {
            "invalid Unicode scalar value".to_string()
        };

        return Some(PromotionWarning::InvalidUnicodeCodePoint { value, reason });
    }

    None
}

/// Check if a u32 value is a valid Unicode scalar value
///
/// Valid range: U+0000 to U+10FFFF, excluding surrogate pairs U+D800 to U+DFFF
fn is_valid_unicode_scalar(value: u32) -> bool {
    value <= 0x10FFFF && !(0xD800..=0xDFFF).contains(&value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_unicode_scalars() {
        assert!(is_valid_unicode_scalar(0x0000)); // NULL
        assert!(is_valid_unicode_scalar(0x0041)); // 'A'
        assert!(is_valid_unicode_scalar(0xD7FF)); // Before surrogates
        assert!(is_valid_unicode_scalar(0xE000)); // After surrogates
        assert!(is_valid_unicode_scalar(0x10FFFF)); // Max valid
    }

    #[test]
    fn test_invalid_unicode_scalars() {
        assert!(!is_valid_unicode_scalar(0xD800)); // Start of surrogates
        assert!(!is_valid_unicode_scalar(0xDFFF)); // End of surrogates
        assert!(!is_valid_unicode_scalar(0x110000)); // Beyond max
        assert!(!is_valid_unicode_scalar(0xFFFFFFFF)); // Way beyond max
    }
}
