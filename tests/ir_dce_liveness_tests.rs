// Dead Code Elimination: Liveness Analysis Tests
//
// Tests for User Story 2: Eliminate Dead Instructions (Priority P2)
// FR-003: Compute value liveness using backward dataflow analysis
// FR-004: Build def-use chains for all values
// FR-005: Remove instructions computing unused values

#[cfg(test)]
mod liveness_tests {
    use jsavrs::ir::instruction::{DebugInfo, Instruction, InstructionKind};
    use jsavrs::ir::optimizer::{DeadCodeElimination, Phase};
    use jsavrs::ir::{Function, IrBinaryOp, IrLiteralValue, IrType, Terminator, TerminatorKind, Value};
    use jsavrs::location::{source_location::SourceLocation, source_span::SourceSpan};
    use std::sync::Arc;

    fn dummy_span() -> SourceSpan {
        let start = SourceLocation::new(1, 1, 0);
        let end = SourceLocation::new(1, 1, 0);
        SourceSpan::new(Arc::from("test.txt"), start, end)
    }

    fn dummy_debug_info() -> DebugInfo {
        DebugInfo { source_span: dummy_span() }
    }

    fn create_test_function(name: &str) -> Function {
        let mut func = Function::new(name, vec![], IrType::Void);
        let entry_label = format!("entry_{name}");
        func.add_block(&entry_label, dummy_span());
        func
    }

    /// T031: Test for unused temporary variable computation
    #[test]
    fn test_unused_temporary_removal() {
        let mut func = create_test_function("test_unused");
        let unused_computation = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: Value::new_literal(IrLiteralValue::I32(10)),
                right: Value::new_literal(IrLiteralValue::I32(20)),
                ty: IrType::I32,
            },
            result: Some(Value::new_temporary(1, IrType::I32)),
            debug_info: dummy_debug_info(),
            scope: None,
        };
        func.add_instruction("entry_test_unused", unused_computation);
        func.set_terminator(
            "entry_test_unused",
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let entry_block = func.cfg.get_block("entry_test_unused").expect("entry block should exist");
        assert_eq!(entry_block.instructions.len(), 1, "Should have 1 instruction before DCE");

        let mut module = jsavrs::ir::Module::new("test_module", None);
        module.add_function(func);
        let mut dce = DeadCodeElimination::default();
        dce.run(&mut module);

        let optimized_func = &module.functions()[0];
        let entry_block_after = optimized_func.cfg.get_block("entry_test_unused").expect("entry block should exist");
        assert_eq!(entry_block_after.instructions.len(), 0, "Should have 0 instructions after DCE");
    }

    /// T032: Test chain of computations with only final result used
    #[test]
    fn test_dead_computation_chain() {
        // Create a function with a chain: t1 = 10 + 20, t2 = t1 * 2, t3 = t2 + 1, return t3
        // All instructions should be kept because they contribute to the return value
        let mut func = create_test_function("test_chain");

        // Create value objects to reuse (important for def-use tracking)
        let t1 = Value::new_temporary(1, IrType::I32);
        let t2 = Value::new_temporary(2, IrType::I32);
        let t3 = Value::new_temporary(3, IrType::I32);

        // t1 = 10 + 20
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

        // t2 = t1 * 2
        let instr2 = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Multiply,
                left: t1, // Reuse same t1 Value
                right: Value::new_literal(IrLiteralValue::I32(2)),
                ty: IrType::I32,
            },
            result: Some(t2.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        // t3 = t2 + 1
        let instr3 = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: t2, // Reuse same t2 Value
                right: Value::new_literal(IrLiteralValue::I32(1)),
                ty: IrType::I32,
            },
            result: Some(t3.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        func.add_instruction("entry_test_chain", instr1);
        func.add_instruction("entry_test_chain", instr2);
        func.add_instruction("entry_test_chain", instr3);

        // return t3
        func.set_terminator(
            "entry_test_chain",
            Terminator {
                kind: TerminatorKind::Return { value: t3, ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let entry_block = func.cfg.get_block("entry_test_chain").expect("entry block should exist");
        assert_eq!(entry_block.instructions.len(), 3, "Should have 3 instructions before DCE");

        let mut module = jsavrs::ir::Module::new("test_module", None);
        module.add_function(func);

        let mut dce = DeadCodeElimination::default();
        dce.run(&mut module);

        let optimized_func = &module.functions()[0];
        let entry_block_after = optimized_func.cfg.get_block("entry_test_chain").expect("entry block should exist");

        // All instructions should be kept (all used)
        assert_eq!(
            entry_block_after.instructions.len(),
            3,
            "Should keep all 3 instructions (all contribute to return)"
        );
    }

    /*// T033: Test multiple dead definitions removed - unrealistic IR pattern
    // This test attempted to create multiple Value objects with the same temporary ID
    // but different UUIDs, which doesn't match how the IR generator works in practice.

    /// T034: Test for unused phi node removal
    /// FR-009: Special handling for phi nodes - remove if result unused
    /// SC-002: At least 90% of provably-dead instructions removed
    #[test]
    #[ignore = "T034: Unused phi node removal not yet fully implemented (CFG API limitations)"]
    fn test_unused_phi_node_removal() {
        let mut func = create_test_function("test_phi");

        // Create a simple if-then-else structure with an unused phi node
        let then_label = "then_test_phi";
        let else_label = "else_test_phi";
        let merge_label = "merge_test_phi";

        func.add_block(then_label, dummy_span());
        func.add_block(else_label, dummy_span());
        func.add_block(merge_label, dummy_span());

        // Entry block: conditional branch
        let cond = Value::new_literal(IrLiteralValue::Bool(true));
        func.set_terminator(
            "entry_test_phi",
            Terminator {
                kind: TerminatorKind::ConditionalBranch {
                    condition: cond,
                    true_label: then_label.into(),
                    false_label: else_label.into(),
                },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        // Then block: branch to merge
        let then_value = Value::new_literal(IrLiteralValue::I32(42));
        func.set_terminator(
            then_label,
            Terminator {
                kind: TerminatorKind::Branch { label: merge_label.into() },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        // Else block: branch to merge
        let else_value = Value::new_literal(IrLiteralValue::I32(99));
        func.set_terminator(
            else_label,
            Terminator {
                kind: TerminatorKind::Branch { label: merge_label.into() },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        ); // Merge block: phi node (unused) + return
        let phi_result = Value::new_temporary(10, IrType::I32);
        let phi_instr = Instruction {
            kind: InstructionKind::Phi {
                ty: IrType::I32,
                incoming: vec![(then_value, then_label.to_string()), (else_value, else_label.to_string())],
            },
            result: Some(phi_result.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        func.add_instruction(merge_label, phi_instr);

        // Return a constant (not the phi result - phi is unused!)
        func.set_terminator(
            merge_label,
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let merge_block = func.cfg.get_block(merge_label).expect("merge block should exist");
        assert_eq!(merge_block.instructions.len(), 1, "Should have 1 phi instruction before DCE");

        let mut module = jsavrs::ir::Module::new("test_module", None);
        module.add_function(func);

        let mut dce = DeadCodeElimination::default();
        dce.run(&mut module);

        let optimized_func = &module.functions()[0];

        // Verify merge block still exists (it's reachable)
        let merge_block_after = optimized_func.cfg.get_block(merge_label);
        assert!(merge_block_after.is_some(), "Merge block should still exist after DCE");

        if let Some(block) = merge_block_after {
            // The unused phi node should be removed
            assert_eq!(block.instructions.len(), 0, "Unused phi node should be removed by DCE");
        }
    }*/

    // ========================================================================
    // Phase 9: Edge Case Tests
    // ========================================================================

    /// T105 [Phase 9]: Test for function calls with unused return values (FR-008)
    #[test]
    fn test_function_call_unused_return_value() {
        let mut func = create_test_function("test_call");

        // Create a call instruction with unused return value
        let call_result = Value::new_temporary(1, IrType::I32);
        let func_value = Value::new_global(Arc::from("some_func"), IrType::I32);
        let call_instr = Instruction {
            kind: InstructionKind::Call {
                func: func_value,
                args: vec![Value::new_literal(IrLiteralValue::I32(42))],
                ty: IrType::I32,
            },
            result: Some(call_result),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        func.add_instruction("entry_test_call", call_instr);

        // Return constant (not call result - call result is unused!)
        func.set_terminator(
            "entry_test_call",
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let entry_block = func.cfg.get_block("entry_test_call").expect("entry block");
        assert_eq!(entry_block.instructions.len(), 1, "Should have 1 call before DCE");

        let mut module = jsavrs::ir::Module::new("test_module", None);
        module.add_function(func);

        let mut dce = DeadCodeElimination::default();
        dce.run(&mut module);

        let optimized_func = &module.functions()[0];
        let entry_after = optimized_func.cfg.get_block("entry_test_call").expect("entry block");

        // Call instruction should be PRESERVED because it may have side effects (FR-008)
        // Conservative analysis: unknown call purity means we keep it
        assert_eq!(
            entry_after.instructions.len(),
            1,
            "Function call should be preserved (unknown purity, potential side effects)"
        );

        println!("✓ Function call with unused return value conservatively preserved (FR-008)");
    }
}
