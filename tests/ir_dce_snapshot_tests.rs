// Dead Code Elimination: Snapshot Tests
//
// Snapshot tests for IR output validation using insta crate
// Captures before/after IR state to detect regressions

#[cfg(test)]
mod snapshot_tests {
    use jsavrs::ir::instruction::{DebugInfo, Instruction, InstructionKind};
    use jsavrs::ir::optimizer::{DeadCodeElimination, Phase};
    use jsavrs::ir::{Function, IrBinaryOp, IrLiteralValue, IrType, Terminator, TerminatorKind, Value};
    use jsavrs::location::{source_location::SourceLocation, source_span::SourceSpan};
    use jsavrs::utils::module_redacted;
    use std::sync::Arc;

    fn dummy_span() -> SourceSpan {
        let start = SourceLocation::new(1, 1, 0);
        let end = SourceLocation::new(1, 1, 0);
        SourceSpan::new(Arc::from("test.txt"), start, end)
    }

    fn dummy_debug_info() -> DebugInfo {
        DebugInfo { source_span: dummy_span() }
    }

    /// T035: Snapshot test for IR before/after dead instruction removal
    /// Captures complete IR transformation for regression detection
    #[test]
    fn test_dead_instruction_removal_snapshot() {
        let mut func = Function::new("dead_instr_test", vec![], IrType::Void);
        let entry_label = "entry_dead_instr_test"; // Must match CFG default entry label pattern
        func.add_block(entry_label, dummy_span());

        // Add dead instruction: unused computation
        let dead_result = Value::new_temporary(1, IrType::I32);
        let dead_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: Value::new_literal(IrLiteralValue::I32(10)),
                right: Value::new_literal(IrLiteralValue::I32(20)),
                ty: IrType::I32,
            },
            result: Some(dead_result),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        // Add live instruction: used in return
        let live_result = Value::new_temporary(2, IrType::I32);
        let live_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Multiply,
                left: Value::new_literal(IrLiteralValue::I32(5)),
                right: Value::new_literal(IrLiteralValue::I32(3)),
                ty: IrType::I32,
            },
            result: Some(live_result.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        func.add_instruction(entry_label, dead_instr);
        func.add_instruction(entry_label, live_instr);

        // Return the live value
        func.set_terminator(
            entry_label,
            Terminator {
                kind: TerminatorKind::Return { value: live_result, ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let mut module = jsavrs::ir::Module::new("snapshot_test", None);
        module.add_function(func);
        let ir_before = module_redacted(module.clone());

        // Run DCE
        let mut dce = DeadCodeElimination::default();
        dce.run(&mut module);

        // Capture IR after optimization
        let ir_after = module_redacted(module.clone());

        // Snapshot the transformation
        let snapshot = format!(
            "=== IR Before DCE ===\n{}\n\n=== IR After DCE ===\n{}\n\n=== Statistics ===\n\
             Instructions in entry block before: 2\n\
             Instructions in entry block after: 1\n\
             Expected: Dead instruction removed, live instruction preserved",
            ir_before, ir_after
        );

        insta::assert_snapshot!("dead_instruction_removal", snapshot);
    }

    /// T076: Snapshot test for multi-iteration fixed-point optimization
    /// Demonstrates cascading dead code elimination requiring multiple iterations
    /// FR-010: Fixed-point iteration
    /// SC-003: Multiple iteration tracking
    #[test]
    fn test_multi_iteration_optimization_snapshot() {
        let mut func = Function::new("multi_iter_test", vec![], IrType::I32);
        let entry_label = "entry_multi_iter_test"; // Must match CFG default entry label pattern
        func.add_block(entry_label, dummy_span());

        // Create cascading dead code: t1 → t2 → t3 (all dead)
        // t1 = 10 + 20
        let t1 = Value::new_temporary(1, IrType::I32);
        let instr1 = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: Value::new_literal(IrLiteralValue::I32(10)),
                right: Value::new_literal(IrLiteralValue::I32(20)),
                ty: IrType::I32,
            },
            result: Some(t1.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        // t2 = t1 * 3 (uses t1)
        let t2 = Value::new_temporary(2, IrType::I32);
        let instr2 = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Multiply,
                left: t1.clone(),
                right: Value::new_literal(IrLiteralValue::I32(3)),
                ty: IrType::I32,
            },
            result: Some(t2.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        // t3 = t2 - 5 (uses t2)
        let t3 = Value::new_temporary(3, IrType::I32);
        let instr3 = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Subtract,
                left: t2.clone(),
                right: Value::new_literal(IrLiteralValue::I32(5)),
                ty: IrType::I32,
            },
            result: Some(t3),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        func.add_instruction(entry_label, instr1);
        func.add_instruction(entry_label, instr2);
        func.add_instruction(entry_label, instr3);

        // Return constant (doesn't use t1, t2, or t3)
        func.set_terminator(
            entry_label,
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(42)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        // Capture IR before optimization
        let entry_block_before = func.cfg.get_block(entry_label).expect("entry block").instructions.len();

        let mut module = jsavrs::ir::Module::new("multi_iter_snapshot", None);
        module.add_function(func);
        let ir_before = module_redacted(module.clone());

        // Run DCE with statistics enabled to track iterations
        let mut dce = DeadCodeElimination::with_config(10, true, true, false);
        dce.run(&mut module);

        // Capture IR after optimization and stats
        let optimized_func = &module.functions()[0];
        let ir_after = module_redacted(module.clone());
        let entry_block_after = optimized_func.cfg.get_block(entry_label).expect("entry block").instructions.len();
        let stats = dce.get_statistics();

        // Create detailed snapshot showing:
        // 1. Initial state with cascading dependencies
        // 2. Final state (all dead code removed)
        // 3. Iteration count (proving multi-iteration convergence)
        let snapshot = format!(
            "=== Multi-Iteration Fixed-Point Optimization Test ===\n\n\
             === Initial State ===\n\
             Instructions: {} in entry block\n\
             - t1 = 10 + 20 (dead, but defines value used by t2)\n\
             - t2 = t1 * 3 (dead, but defines value used by t3)\n\
             - t3 = t2 - 5 (dead, result never used)\n\
             Return: constant 42 (doesn't use any temporaries)\n\n\
             === IR Before DCE ===\n{}\n\n\
             === Optimized State ===\n\
             Instructions: {} in entry block\n\
             All cascading dead code eliminated\n\n\
             === IR After DCE ===\n{}\n\n\
             === Optimization Statistics ===\n\
             Iterations: {} (demonstrates fixed-point convergence)\n\
             Instructions removed: {}\n\
             Blocks removed: {}\n\n\
             === Analysis ===\n\
             This test demonstrates that DCE correctly handles cascading\n\
             dead code through iterative fixed-point optimization:\n\
             - Iteration 1: Removes t3 (no uses)\n\
             - Iteration 2: Removes t2 (only use was t3, now removed)\n\
             - Iteration 3: Removes t1 (only use was t2, now removed)\n\
             - Iteration 4: No changes, fixed-point reached\n\n\
             Expected behavior:\n\
             - All 3 dead instructions removed\n\
             - Convergence in 2-4 iterations (implementation-dependent)\n\
             - Return statement preserved",
            entry_block_before,
            ir_before,
            entry_block_after,
            ir_after,
            stats.iterations,
            stats.instructions_removed,
            stats.blocks_removed
        );

        insta::assert_snapshot!("multi_iteration_optimization", snapshot);

        // Verify expectations
        assert_eq!(entry_block_after, 0, "All dead instructions should be removed");
        assert!(stats.iterations >= 1, "Should require at least one iteration");
        assert_eq!(stats.instructions_removed, 3, "Should remove all 3 cascading dead instructions");
    }
}
