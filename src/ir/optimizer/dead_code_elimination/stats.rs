//! Statistics and diagnostics for Dead Code Elimination.
//!
//! Provides metrics about optimization effectiveness and warnings about
//! conservative decisions that prevented code removal.

use std::fmt;
use std::fmt::Write;
use std::sync::Arc;

/// Statistics collected during Dead Code Elimination optimization.
///
/// Provides metrics about the optimization's effectiveness including
/// the number of instructions and blocks removed, iterations required
/// to reach fixed-point, and any conservative decisions that prevented
/// more aggressive optimization.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OptimizationStats {
    /// Total number of instructions removed across all iterations.
    pub instructions_removed: usize,

    /// Total number of basic blocks removed across all iterations.
    pub blocks_removed: usize,

    /// Number of fixed-point iterations performed.
    /// 1 means no changes were made (optimization was no-op).
    /// Higher values indicate cascading dead code removal.
    pub iterations: usize,

    /// Warnings about conservative decisions that prevented removal.
    /// Empty if `verbose_warnings` is disabled on the optimizer.
    pub conservative_warnings: Vec<ConservativeWarning>,
}

impl OptimizationStats {
    /// Creates empty statistics (no removals).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if any code was removed.
    #[must_use]
    pub const fn had_effect(&self) -> bool {
        self.instructions_removed > 0 || self.blocks_removed > 0
    }

    /// Formats statistics for human-readable display.
    #[allow(clippy::unwrap_used)]
    #[must_use]
    pub fn format_report(&self, function_name: &str) -> String {
        let mut output = String::with_capacity(256);

        writeln!(output, "📊 DCE Statistics for '{function_name}':").unwrap();
        writeln!(output, "✂️  Instructions removed: {}", self.instructions_removed).unwrap();
        writeln!(output, "🗑️  Blocks removed: {}", self.blocks_removed).unwrap();
        writeln!(output, "🔄 Iterations: {}", self.iterations).unwrap();
        writeln!(output, "⚠️  Conservative warnings: {}", self.conservative_warnings.len()).unwrap();

        output
    }
}

impl fmt::Display for OptimizationStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OptimizationStats {{ instructions: {}, blocks: {}, iterations: {}, warnings: {} }}",
            self.instructions_removed,
            self.blocks_removed,
            self.iterations,
            self.conservative_warnings.len()
        )
    }
}

/// Warning about a conservative decision that prevented code removal.
///
/// Used for diagnostics and debugging to understand why certain
/// instructions or blocks were not removed despite appearing unused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConservativeWarning {
    /// Human-readable description of the instruction.
    pub instruction_debug: Arc<str>,

    /// The specific reason this instruction was conservatively kept.
    pub reason: ConservativeReason,

    /// Optional: The basic block label where this instruction appears.
    pub block_label: Option<Arc<str>>,
}

impl ConservativeWarning {
    /// Creates a new warning.
    #[must_use]
    pub const fn new(instruction_debug: Arc<str>, reason: ConservativeReason, block_label: Option<Arc<str>>) -> Self {
        Self { instruction_debug, reason, block_label }
    }
}

impl fmt::Display for ConservativeWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref label) = self.block_label {
            write!(f, "⚠️  Conservative: {} in block '{}' (reason: {})", self.instruction_debug, label, self.reason)
        } else {
            write!(f, "⚠️  Conservative: {} (reason: {})", self.instruction_debug, self.reason)
        }
    }
}

/// Reasons why an instruction was conservatively preserved.
///
/// Each variant represents a specific limitation in the analysis
/// that prevented proving the instruction was safe to remove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConservativeReason {
    /// The instruction may alias with other memory locations.
    MayAlias,

    /// The function being called has unknown purity.
    UnknownCallPurity,

    /// The pointer operand escapes the current function.
    EscapedPointer,

    /// The instruction may have other side effects.
    PotentialSideEffect,
}

impl ConservativeReason {
    /// Returns a human-readable explanation of this reason.
    #[inline]
    #[must_use]
    pub const fn explanation(&self) -> &'static str {
        match self {
            Self::MayAlias => "instruction may alias with other memory locations",
            Self::UnknownCallPurity => "function call has unknown purity (may have side effects)",
            Self::EscapedPointer => "pointer operand escapes the current function",
            Self::PotentialSideEffect => "instruction may have other observable side effects",
        }
    }
}

impl fmt::Display for ConservativeReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.explanation())
    }
}
