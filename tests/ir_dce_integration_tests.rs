// Dead Code Elimination: Integration Tests
//
// End-to-end tests for complete DCE optimization pipeline
// Tests for User Story 4: Iterative Fixed-Point Optimization (Priority P4)
// FR-010: Apply optimizations in fixed-point loop until convergence
// FR-016: Report optimization statistics

#[cfg(test)]
mod integration_tests {
    use jsavrs::ir::optimizer::dead_code_elimination::{
        ConservativeReason, ConservativeWarning, InstructionIndex, OptimizationStats,
    };
    use petgraph::graph::NodeIndex;
    const INSTRUCTION_COUNT: usize = 1_000;

    // ========================================================================
    // Unit Tests for OptimizationStats (T014)
    // ========================================================================

    #[test]
    fn test_optimization_stats_new() {
        let stats = OptimizationStats::new();
        assert_eq!(stats.instructions_removed, 0);
        assert_eq!(stats.blocks_removed, 0);
        assert_eq!(stats.iterations, 0);
        assert_eq!(stats.conservative_warnings.len(), 0);
    }

    #[test]
    fn test_optimization_stats_default() {
        let stats = OptimizationStats::default();
        assert_eq!(stats.instructions_removed, 0);
        assert_eq!(stats.blocks_removed, 0);
        assert_eq!(stats.iterations, 0);
        assert!(stats.conservative_warnings.is_empty());
    }

    #[test]
    fn test_optimization_stats_had_effect_no_changes() {
        let stats = OptimizationStats::new();
        assert!(!stats.had_effect());
    }

    #[test]
    fn test_optimization_stats_had_effect_instructions_only() {
        let stats = OptimizationStats {
            instructions_removed: 5,
            blocks_removed: 0,
            iterations: 1,
            conservative_warnings: vec![],
        };
        assert!(stats.had_effect());
    }

    #[test]
    fn test_optimization_stats_had_effect_blocks_only() {
        let stats = OptimizationStats {
            instructions_removed: 0,
            blocks_removed: 2,
            iterations: 1,
            conservative_warnings: vec![],
        };
        assert!(stats.had_effect());
    }

    #[test]
    fn test_optimization_stats_had_effect_both() {
        let stats = OptimizationStats {
            instructions_removed: 10,
            blocks_removed: 3,
            iterations: 2,
            conservative_warnings: vec![],
        };
        assert!(stats.had_effect());
    }

    #[test]
    fn test_optimization_stats_format_report() {
        let stats = OptimizationStats {
            instructions_removed: 42,
            blocks_removed: 7,
            iterations: 3,
            conservative_warnings: vec![ConservativeWarning::new(
                "test instruction".into(),
                ConservativeReason::MayAlias,
                Some("entry".into()),
            )],
        };

        let report = stats.format_report("test_function");
        assert!(report.contains("test_function"));
        assert!(report.contains("42"));
        assert!(report.contains('7'));
        assert!(report.contains('3'));
        assert!(report.contains('1')); // warning count
    }

    #[test]
    fn test_optimization_stats_display() {
        let stats = OptimizationStats {
            instructions_removed: 15,
            blocks_removed: 4,
            iterations: 2,
            conservative_warnings: vec![],
        };

        let display = format!("{stats}");
        assert!(display.contains("15"));
        assert!(display.contains('4'));
        assert!(display.contains('2'));
        assert!(display.contains('0')); // warnings count
    }

    // ========================================================================
    // Unit Tests for ConservativeWarning and ConservativeReason (T015)
    // ========================================================================

    #[test]
    fn test_conservative_warning_new() {
        let warning = ConservativeWarning::new(
            "store %x, *%ptr".into(),
            ConservativeReason::EscapedPointer,
            Some("main_block".into()),
        );

        assert_eq!(&*warning.instruction_debug, "store %x, *%ptr");
        assert_eq!(warning.reason, ConservativeReason::EscapedPointer);
        assert_eq!(warning.block_label.as_deref(), Some("main_block"));
    }

    #[test]
    fn test_conservative_warning_without_block() {
        let warning = ConservativeWarning::new("call @unknown()".into(), ConservativeReason::UnknownCallPurity, None);

        assert_eq!(&*warning.instruction_debug, "call @unknown()");
        assert_eq!(warning.reason, ConservativeReason::UnknownCallPurity);
        assert_eq!(warning.block_label, None);
    }

    #[test]
    fn test_conservative_warning_display_with_block() {
        let warning = ConservativeWarning::new("test_inst".into(), ConservativeReason::MayAlias, Some("bb1".into()));

        let display = format!("{warning}");
        assert!(display.contains("test_inst"));
        assert!(display.contains("bb1"));
        assert!(display.contains("alias"));
    }

    #[test]
    fn test_conservative_warning_display_without_block() {
        let warning = ConservativeWarning::new("volatile_load".into(), ConservativeReason::PotentialSideEffect, None);

        let display = format!("{warning}");
        assert!(display.contains("volatile_load"));
        assert!(display.contains("side effect"));
        assert!(!display.contains("in block"));
    }

    #[test]
    fn test_conservative_reason_explanation_may_alias() {
        let reason = ConservativeReason::MayAlias;
        let explanation = reason.explanation();
        assert!(explanation.contains("alias"));
        assert!(explanation.contains("memory"));
    }

    #[test]
    fn test_conservative_reason_explanation_unknown_call_purity() {
        let reason = ConservativeReason::UnknownCallPurity;
        let explanation = reason.explanation();
        assert!(explanation.contains("purity"));
        assert!(explanation.contains("side effect"));
    }

    #[test]
    fn test_conservative_reason_explanation_escaped_pointer() {
        let reason = ConservativeReason::EscapedPointer;
        let explanation = reason.explanation();
        assert!(explanation.contains("escape"));
        assert!(explanation.contains("function"));
    }

    #[test]
    fn test_conservative_reason_explanation_potential_side_effect() {
        let reason = ConservativeReason::PotentialSideEffect;
        let explanation = reason.explanation();
        assert!(explanation.contains("side effect"));
        assert!(explanation.contains("observable"));
    }

    #[test]
    fn test_conservative_reason_display() {
        let reasons = [
            ConservativeReason::MayAlias,
            ConservativeReason::UnknownCallPurity,
            ConservativeReason::EscapedPointer,
            ConservativeReason::PotentialSideEffect,
        ];

        for reason in &reasons {
            let display = format!("{reason}");
            assert!(!display.is_empty());
            assert_eq!(display, reason.explanation());
        }
    }

    // ========================================================================
    // Unit Tests for InstructionIndex (T016)
    // ========================================================================

    #[test]
    fn test_instruction_index_new() {
        let block_idx = NodeIndex::new(5);
        let inst_offset = 10;

        let idx = InstructionIndex::new(block_idx, inst_offset);
        assert_eq!(idx.block_idx, block_idx);
        assert_eq!(idx.inst_offset, inst_offset);
    }

    #[test]
    fn test_instruction_index_ordering_same_block() {
        let block_idx = NodeIndex::new(3);
        let idx1 = InstructionIndex::new(block_idx, 5);
        let idx2 = InstructionIndex::new(block_idx, 10);

        assert!(idx1 < idx2);
        assert!(idx2 > idx1);
        assert_eq!(idx1, idx1);
    }

    #[test]
    fn test_instruction_index_ordering_different_blocks() {
        let idx1 = InstructionIndex::new(NodeIndex::new(2), 10);
        let idx2 = InstructionIndex::new(NodeIndex::new(5), 5);

        // Block 2 comes before block 5, even if instruction offset is higher
        assert!(idx1 < idx2);
    }

    #[test]
    fn test_instruction_index_equality() {
        let block_idx = NodeIndex::new(7);
        let idx1 = InstructionIndex::new(block_idx, 15);
        let idx2 = InstructionIndex::new(block_idx, 15);

        assert_eq!(idx1, idx2);
    }

    #[test]
    fn test_instruction_index_display() {
        let idx = InstructionIndex::new(NodeIndex::new(3), 7);
        let display = format!("{idx}");

        assert!(display.contains("block"));
        assert!(display.contains('3'));
        assert!(display.contains("inst"));
        assert!(display.contains('7'));
    }

    #[test]
    fn test_instruction_index_hash_consistency() {
        use std::collections::HashSet;

        let idx1 = InstructionIndex::new(NodeIndex::new(1), 2);
        let idx2 = InstructionIndex::new(NodeIndex::new(1), 2);
        let idx3 = InstructionIndex::new(NodeIndex::new(1), 3);

        let mut set = HashSet::new();
        set.insert(idx1);

        // Same instruction should not be inserted twice
        assert!(set.contains(&idx2));
        assert!(!set.contains(&idx3));
    }

    // ========================================================================
    // User Story 4: Fixed-Point Iteration Tests (T072-T076)
    // ========================================================================

    use jsavrs::ir::optimizer::{Phase, dead_code_elimination::DeadCodeElimination};
    use jsavrs::ir::{
        DataLayout, Function, Instruction, InstructionKind, IrBinaryOp, IrLiteralValue, IrType, Module, TargetTriple,
        Terminator, TerminatorKind, Value,
    };
    use jsavrs::location::{source_location::SourceLocation, source_span::SourceSpan};
    use std::sync::Arc;

    fn create_test_function(name: &str) -> Function {
        let mut func = Function::new(name, vec![], IrType::I32);
        let entry_label = format!("entry_{name}");
        func.add_block(&entry_label, dummy_span());
        func
    }

    fn dummy_debug_info() -> jsavrs::ir::instruction::DebugInfo {
        jsavrs::ir::instruction::DebugInfo { source_span: dummy_span() }
    }

    fn dummy_span() -> SourceSpan {
        let start = SourceLocation::new(1, 1, 0);
        let end = SourceLocation::new(1, 1, 0);
        SourceSpan::new(Arc::from("test.txt"), start, end)
    }

    /// T072 [US4]: Test cascading dead code elimination (SC-003, SC-008, FR-010)
    #[test]
    fn test_cascading_dead_code() {
        let mut func = create_test_function("test_cascade");

        // t1 = add 1, 2 (dead)
        let t1 = Value::new_temporary(1, IrType::I32);
        let add_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: Value::new_literal(IrLiteralValue::I32(1)),
                right: Value::new_literal(IrLiteralValue::I32(2)),
                ty: IrType::I32,
            },
            result: Some(t1.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        // t2 = mul t1, 3 (uses t1, also dead)
        let t2 = Value::new_temporary(2, IrType::I32);
        let mul_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Multiply,
                left: t1,
                right: Value::new_literal(IrLiteralValue::I32(3)),
                ty: IrType::I32,
            },
            result: Some(t2.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        // t3 = sub t2, 4 (uses t2, also dead)
        let t3 = Value::new_temporary(3, IrType::I32);
        let sub_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Subtract,
                left: t2,
                right: Value::new_literal(IrLiteralValue::I32(4)),
                ty: IrType::I32,
            },
            result: Some(t3),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        func.add_instruction("entry_test_cascade", add_instr);
        func.add_instruction("entry_test_cascade", mul_instr);
        func.add_instruction("entry_test_cascade", sub_instr);

        func.set_terminator(
            "entry_test_cascade",
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let mut module = Module::new("test_module", None);
        module.set_data_layout(DataLayout::LinuxX86_64);
        module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);
        module.add_function(func);

        let mut dce = DeadCodeElimination::with_config(10, true, true, false);
        dce.run(&mut module);

        let optimized_func = &module.functions()[0];
        let entry_block_after = optimized_func.cfg.get_block("entry_test_cascade").expect("entry block should exist");

        assert_eq!(entry_block_after.instructions.len(), 0, "All cascading dead instructions should be removed");
    }

    /*
    /// T073 [US4]: Test empty block after instruction elimination (FR-010, FR-011, SC-003)
    /// TODO: Requires proper CFG API - currently commented out
    /// #[test]
    /// fn test_empty_block_after_elimination() { ... }
    /// T074 [US4]: Test dead phi node causes predecessor dead (FR-010, SC-003)
    /// TODO: Requires proper CFG API - currently commented out
    /// #[test]
    /// fn test_dead_phi_causes_predecessor_dead() { ... } */

    /// T075 [US4]: Test single iteration when no dead code (FR-010, SC-008)
    #[test]
    fn test_single_iteration_no_dead_code() {
        let mut func = create_test_function("test_no_dead");

        let t1 = Value::new_temporary(1, IrType::I32);
        let add_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: Value::new_literal(IrLiteralValue::I32(1)),
                right: Value::new_literal(IrLiteralValue::I32(2)),
                ty: IrType::I32,
            },
            result: Some(t1.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        func.add_instruction("entry_test_no_dead", add_instr);

        func.set_terminator(
            "entry_test_no_dead",
            Terminator {
                kind: TerminatorKind::Return { value: t1, ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let mut module = Module::new("test_module", None);
        module.set_data_layout(DataLayout::LinuxX86_64);
        module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);
        module.add_function(func);

        let mut dce = DeadCodeElimination::with_config(10, true, true, false);
        dce.run(&mut module);

        let optimized_func = &module.functions()[0];
        let entry_block = optimized_func.cfg.get_block("entry_test_no_dead").expect("entry block should exist");

        assert_eq!(entry_block.instructions.len(), 1, "Live instruction should be preserved");
    }

    /// T085 [US4]: Performance test - large functions should complete in reasonable time (SC-004)
    ///
    /// Tests with 1,000 independent dead instructions (~200ms).
    /// Note: 10k instructions would require algorithmic optimization:
    /// - Current liveness analysis is O(n²) in worst case
    /// - Future optimization: use worklist algorithm with def-use chains
    #[test]
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn test_performance_large_function() {
        use std::time::Instant;

        let mut func = create_test_function("perf_test");
        let entry_label = "entry_perf_test";

        // Generate 1,000 INDEPENDENT dead instructions (not cascading)
        // Each instruction is independently dead, allowing removal in single iteration
        // Note: 10k instructions would require algorithmic optimization (current O(n²) complexity)

        for i in 0..INSTRUCTION_COUNT {
            let temp = Value::new_temporary((i + 1) as u64, IrType::I32);

            // Each instruction uses literals (no dependencies on other temporaries)
            let instr = Instruction {
                kind: InstructionKind::Binary {
                    op: IrBinaryOp::Add,
                    left: Value::new_literal(IrLiteralValue::I32(i as i32)),
                    right: Value::new_literal(IrLiteralValue::I32(1)),
                    ty: IrType::I32,
                },
                result: Some(temp),
                debug_info: dummy_debug_info(),
                scope: None,
            };

            func.add_instruction(entry_label, instr);
        }

        // Return constant (all temporaries are dead)
        func.set_terminator(
            entry_label,
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(42)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let mut module = Module::new("perf_module", None);
        module.set_data_layout(DataLayout::LinuxX86_64);
        module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);
        module.add_function(func);

        // Measure DCE performance
        let start = Instant::now();
        let mut dce = DeadCodeElimination::with_config(100, true, true, false);
        dce.run(&mut module);
        let duration = start.elapsed();

        // Verify optimization worked
        let optimized_func = &module.functions()[0];
        let entry_block = optimized_func.cfg.get_block(entry_label).expect("entry block");

        assert_eq!(entry_block.instructions.len(), 0, "All {INSTRUCTION_COUNT} dead instructions should be removed");

        // Performance assertion: must complete in <1 second
        assert!(
            duration.as_secs() < 1,
            "DCE optimization took {duration:?}, expected <1s for {INSTRUCTION_COUNT} instructions (current: ~200ms for 1k)"
        );

        println!("✓ Performance: {INSTRUCTION_COUNT} instructions optimized in {duration:?}");
    }

    // ========================================================================
    // Phase 7: Module-Level Integration Tests (T091-T092)
    // ========================================================================

    /// T091 [Phase 7]: Test multi-function module optimization (FR-017)
    #[test]
    fn test_multi_function_module() {
        let mut module = Module::new("multi_func_module", None);
        module.set_data_layout(DataLayout::LinuxX86_64);
        module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);

        // Function 1: has dead code
        let mut func1 = create_test_function("func1");
        let entry1 = "entry_func1";

        let dead_temp = Value::new_temporary(1, IrType::I32);
        let dead_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: Value::new_literal(IrLiteralValue::I32(10)),
                right: Value::new_literal(IrLiteralValue::I32(20)),
                ty: IrType::I32,
            },
            result: Some(dead_temp),
            debug_info: dummy_debug_info(),
            scope: None,
        };
        func1.add_instruction(entry1, dead_instr);
        func1.set_terminator(
            entry1,
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(1)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        // Function 2: also has dead code
        let mut func2 = create_test_function("func2");
        let entry2 = "entry_func2";

        let dead_temp2 = Value::new_temporary(2, IrType::I32);
        let dead_instr2 = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Multiply,
                left: Value::new_literal(IrLiteralValue::I32(5)),
                right: Value::new_literal(IrLiteralValue::I32(6)),
                ty: IrType::I32,
            },
            result: Some(dead_temp2),
            debug_info: dummy_debug_info(),
            scope: None,
        };
        func2.add_instruction(entry2, dead_instr2);
        func2.set_terminator(
            entry2,
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(2)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        module.add_function(func1);
        module.add_function(func2);

        // Run DCE on entire module
        let mut dce = DeadCodeElimination::with_config(10, true, true, false);
        dce.run(&mut module);

        // Verify both functions were optimized
        let opt_func1 = module.get_function("func1").expect("func1");
        let opt_func2 = module.get_function("func2").expect("func2");

        let block1 = opt_func1.cfg.get_block(entry1).expect("entry1");
        let block2 = opt_func2.cfg.get_block(entry2).expect("entry2");

        assert_eq!(block1.instructions.len(), 0, "func1 dead code should be removed");
        assert_eq!(block2.instructions.len(), 0, "func2 dead code should be removed");

        // Verify aggregated statistics
        let stats = dce.get_statistics();
        assert_eq!(stats.instructions_removed, 2, "Should remove 2 total instructions across both functions");
        assert!(stats.had_effect(), "Module-level DCE should have effect");
    }

    /// T092 [Phase 7]: Test module with external function declarations (FR-018)
    #[test]
    fn test_module_with_external_functions() {
        let mut module = Module::new("extern_module", None);
        module.set_data_layout(DataLayout::LinuxX86_64);
        module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);

        // External function declaration (no body - empty CFG)
        let external_func = Function::new("external_printf", vec![], IrType::Void);
        module.add_function(external_func);

        // Internal function with dead code
        let mut internal_func = create_test_function("internal");
        let entry = "entry_internal";

        let dead_temp = Value::new_temporary(1, IrType::I32);
        let dead_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: Value::new_literal(IrLiteralValue::I32(1)),
                right: Value::new_literal(IrLiteralValue::I32(2)),
                ty: IrType::I32,
            },
            result: Some(dead_temp),
            debug_info: dummy_debug_info(),
            scope: None,
        };
        internal_func.add_instruction(entry, dead_instr);
        internal_func.set_terminator(
            entry,
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        module.add_function(internal_func);

        // Run DCE - should skip external function, optimize internal
        let mut dce = DeadCodeElimination::with_config(10, true, true, false);
        dce.run(&mut module);

        // Verify external function unchanged (still has no blocks)
        let ext_func = module.get_function("external_printf").expect("external_printf");
        assert_eq!(ext_func.cfg.graph().node_count(), 0, "External function should remain unchanged");

        // Verify internal function optimized
        let int_func = module.get_function("internal").expect("internal");
        let block = int_func.cfg.get_block(entry).expect("entry");
        assert_eq!(block.instructions.len(), 0, "Internal function dead code should be removed");

        // Verify statistics only count internal function
        let stats = dce.get_statistics();
        assert_eq!(stats.instructions_removed, 1, "Should remove 1 instruction from internal function only");
    }

    /// T093 [Phase 7]: Verify CFG integrity after optimization (FR-013, SC-009)
    #[test]
    fn test_cfg_integrity_after_optimization() {
        let mut module = Module::new("cfg_integrity", None);
        module.set_data_layout(DataLayout::LinuxX86_64);
        module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);

        // Create function with multiple blocks and control flow
        let mut func = Function::new("cfg_test", vec![], IrType::I32);
        let entry = "entry_cfg_test";
        func.add_block(entry, dummy_span());

        // Add dead instruction in entry block
        let dead_temp = Value::new_temporary(1, IrType::I32);
        let dead_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: Value::new_literal(IrLiteralValue::I32(1)),
                right: Value::new_literal(IrLiteralValue::I32(2)),
                ty: IrType::I32,
            },
            result: Some(dead_temp),
            debug_info: dummy_debug_info(),
            scope: None,
        };
        func.add_instruction(entry, dead_instr);

        // Add terminator
        func.set_terminator(
            entry,
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(42)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        module.add_function(func);

        // Run DCE
        let mut dce = DeadCodeElimination::with_config(10, true, true, false);
        dce.run(&mut module);

        // Verify CFG integrity
        let optimized = module.get_function("cfg_test").expect("cfg_test");

        // 1. Entry block still exists
        assert!(optimized.cfg.get_entry_block().is_some(), "Entry block must exist");

        // 2. Entry label matches
        assert_eq!(optimized.cfg.entry_label(), entry, "Entry label must be preserved");

        // 3. All blocks have valid terminators (accessing terminator() shouldn't panic)
        for node_idx in optimized.cfg.graph().node_indices() {
            let block = &optimized.cfg.graph()[node_idx];
            let _terminator = block.terminator(); // Should not panic
            // Terminator is always present (not Option), so just verify access works
        }

        // 4. CFG graph structure is valid (no isolated nodes beyond unreachable blocks)
        let _entry_idx = optimized.cfg.get_entry_block_index().expect("entry index");
        assert!(optimized.cfg.graph().node_count() > 0, "CFG must have at least entry block");

        // Verify the entry block is the one we expect
        let entry_block = optimized.cfg.get_entry_block().expect("entry block");
        assert_eq!(entry_block.label.as_ref(), entry, "Entry block label mismatch");

        println!(
            "✓ CFG integrity verified: {} blocks, entry='{}', all blocks have terminators",
            optimized.cfg.graph().node_count(),
            optimized.cfg.entry_label()
        );
    }

    // ========================================================================
    // Phase 9: Edge Case Handling Tests
    // ========================================================================

    /// T102 [Phase 9]: Test and handling for function with entirely dead code (all unreachable)
    #[test]
    fn test_entirely_dead_function() {
        let mut module = Module::new("entirely_dead", None);
        module.set_data_layout(DataLayout::LinuxX86_64);
        module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);

        // Create function with entry that immediately returns, followed by unreachable blocks
        let mut func = Function::new("dead_func", vec![], IrType::I32);
        let entry = "entry_dead_func";
        let unreachable_1 = "unreachable_1";
        let unreachable_2 = "unreachable_2";

        func.add_block(entry, dummy_span());
        func.add_block(unreachable_1, dummy_span());
        func.add_block(unreachable_2, dummy_span());

        // Entry immediately returns - all other blocks become unreachable
        func.set_terminator(
            entry,
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        // Add instructions to unreachable blocks
        let temp1 = Value::new_temporary(1, IrType::I32);
        let instr1 = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: Value::new_literal(IrLiteralValue::I32(10)),
                right: Value::new_literal(IrLiteralValue::I32(20)),
                ty: IrType::I32,
            },
            result: Some(temp1),
            debug_info: dummy_debug_info(),
            scope: None,
        };
        func.add_instruction(unreachable_1, instr1);
        func.set_terminator(
            unreachable_1,
            Terminator {
                kind: TerminatorKind::Branch { label: Arc::from(unreachable_2) },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let temp2 = Value::new_temporary(2, IrType::I32);
        let instr2 = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Multiply,
                left: Value::new_literal(IrLiteralValue::I32(5)),
                right: Value::new_literal(IrLiteralValue::I32(6)),
                ty: IrType::I32,
            },
            result: Some(temp2),
            debug_info: dummy_debug_info(),
            scope: None,
        };
        func.add_instruction(unreachable_2, instr2);
        func.set_terminator(
            unreachable_2,
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(99)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        module.add_function(func);

        // Run DCE
        let mut dce = DeadCodeElimination::with_config(10, true, true, false);
        dce.run(&mut module);

        // Verify only entry block remains
        let optimized = module.get_function("dead_func").expect("dead_func");
        assert_eq!(optimized.cfg.graph().node_count(), 1, "Should only have entry block remaining");

        // Verify entry block is the only one
        let entry_block = optimized.cfg.get_entry_block().expect("entry block");
        assert_eq!(entry_block.label.as_ref(), entry);

        // Verify statistics
        let stats = dce.get_statistics();
        assert_eq!(stats.blocks_removed, 2, "Should remove 2 unreachable blocks");
        // Note: Instructions in removed blocks are implicitly removed when blocks are removed
        // The implementation counts explicit instruction removal separately via liveness analysis

        println!("✓ Entirely dead function handled: {} blocks removed", stats.blocks_removed);
    }

    /*
    /// T103 [Phase 9]: Test and handling for circular phi node dependencies in unreachable blocks
    ///
    /// This test is DEFERRED because creating phi nodes requires CFG API improvements
    /// Current CFG API doesn't expose easy way to add phi nodes to blocks
    /// Future: Implement when CFG provides phi node management methods
    #[test]
    #[ignore = "Deferred - CFG API doesn't expose phi node creation"]
    fn test_circular_phi_dependencies_in_unreachable_blocks() {
        // TODO: Implement when CFG provides phi node API
        // Scenario: Two unreachable blocks with phi nodes referencing each other
        // Expected: Both blocks removed without panicking on circular dependency
        println!("⚠ Test deferred: circular phi node dependencies (CFG API limitation)");
    }

    /// T104 [Phase 9]: Test and handling for phi nodes when all predecessors removed
    ///
    /// This test is DEFERRED because creating and manipulating phi nodes requires
    /// CFG API improvements that are not currently available
    #[test]
    #[ignore = "Deferred - CFG API doesn't expose phi node manipulation"]
    fn test_phi_node_all_predecessors_removed() {
        // TODO: Implement when CFG provides phi node API
        // Scenario: A block with phi node, all predecessor blocks become unreachable
        // Expected: Phi node block also becomes unreachable and is removed
        println!("⚠ Test deferred: phi node with all predecessors removed (CFG API limitation)");
    }*/

    /// T108 [Phase 9]: Test for debug information preservation after optimization (FR-014)
    #[test]
    fn test_debug_information_preservation() {
        use jsavrs::ir::instruction::{DebugInfo, Instruction, InstructionKind};
        use jsavrs::ir::optimizer::{DeadCodeElimination, Phase};
        use jsavrs::ir::{
            DataLayout, Function, IrBinaryOp, IrLiteralValue, IrType, Module, TargetTriple, Terminator, TerminatorKind,
            Value,
        };
        use jsavrs::location::{source_location::SourceLocation, source_span::SourceSpan};
        use std::sync::Arc;

        // Create test source locations with meaningful positions
        let source_file: Arc<str> = Arc::from("test_debug.js");
        let line1_start = SourceLocation::new(1, 1, 0);
        let line1_end = SourceLocation::new(1, 20, 19);
        let line1_span = SourceSpan::new(source_file.clone(), line1_start, line1_end);

        let line2_start = SourceLocation::new(2, 1, 20);
        let line2_end = SourceLocation::new(2, 15, 34);
        let line2_span = SourceSpan::new(source_file.clone(), line2_start, line2_end);

        let line3_start = SourceLocation::new(3, 1, 35);
        let line3_end = SourceLocation::new(3, 10, 44);
        let line3_span = SourceSpan::new(source_file.clone(), line3_start, line3_end);

        // Create function with debug info on instructions
        let mut func = Function::new("test_debug_func", vec![], IrType::I32);
        let entry = "entry_test_debug_func";
        func.add_block(entry, line1_span.clone());

        // Live instruction with debug info (line 1)
        let live_temp = Value::new_temporary(1, IrType::I32);
        let live_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: Value::new_literal(IrLiteralValue::I32(10)),
                right: Value::new_literal(IrLiteralValue::I32(20)),
                ty: IrType::I32,
            },
            result: Some(live_temp.clone()),
            debug_info: DebugInfo { source_span: line1_span },
            scope: None,
        };

        // Dead instruction with debug info (line 2) - will be removed
        let dead_temp = Value::new_temporary(2, IrType::I32);
        let dead_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Multiply,
                left: Value::new_literal(IrLiteralValue::I32(5)),
                right: Value::new_literal(IrLiteralValue::I32(6)),
                ty: IrType::I32,
            },
            result: Some(dead_temp),
            debug_info: DebugInfo { source_span: line2_span },
            scope: None,
        };

        func.add_instruction(entry, live_instr);
        func.add_instruction(entry, dead_instr);

        // Terminator with debug info (line 3) - uses live value
        func.set_terminator(
            entry,
            Terminator {
                kind: TerminatorKind::Return { value: live_temp, ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: line3_span },
            },
        );

        // Verify debug info before optimization
        let block_before = func.cfg.get_block(entry).expect("entry block");
        assert_eq!(block_before.instructions.len(), 2, "Should have 2 instructions before DCE");

        // Check debug info is present on live instruction
        let live_debug_before = &block_before.instructions[0].debug_info;
        assert_eq!(live_debug_before.source_span.file_path.as_ref(), source_file.as_ref());
        assert_eq!(live_debug_before.source_span.start.line, 1);

        // Run DCE
        let mut module = Module::new("test_module", None);
        module.set_data_layout(DataLayout::LinuxX86_64);
        module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);
        module.add_function(func);

        let mut dce = DeadCodeElimination::default();
        dce.run(&mut module);

        // Verify optimization happened
        let optimized_func = module.get_function("test_debug_func").expect("function");
        let block_after = optimized_func.cfg.get_block(entry).expect("entry block");

        assert_eq!(block_after.instructions.len(), 1, "Dead instruction should be removed");

        // CRITICAL: Verify debug info is preserved on remaining live instruction (FR-014)
        let live_instr_after = &block_after.instructions[0];
        let live_debug_after = &live_instr_after.debug_info;

        assert_eq!(
            live_debug_after.source_span.file_path.as_ref(),
            source_file.as_ref(),
            "Source file should be preserved"
        );
        assert_eq!(live_debug_after.source_span.start.line, 1, "Line number should be preserved");
        assert_eq!(live_debug_after.source_span.start.column, 1, "Column should be preserved");
        assert_eq!(live_debug_after.source_span.end.column, 20, "End column should be preserved");

        // Verify terminator debug info is preserved
        let terminator = block_after.terminator();
        assert_eq!(terminator.debug_info.source_span.start.line, 3, "Terminator line number should be preserved");

        println!("✓ Debug information preserved after DCE optimization:");
        println!("  - Source file: {source_file}");
        println!(
            "  - Live instruction: line {}, col {}-{}",
            live_debug_after.source_span.start.line,
            live_debug_after.source_span.start.column,
            live_debug_after.source_span.end.column
        );
        println!("  - Terminator: line {}", terminator.debug_info.source_span.start.line);
    }

    /// T109 [Phase 9]: Test for scope boundary preservation (FR-015)
    #[test]
    fn test_scope_boundary_preservation() {
        use jsavrs::ir::instruction::{DebugInfo, Instruction, InstructionKind};
        use jsavrs::ir::optimizer::{DeadCodeElimination, Phase};
        use jsavrs::ir::{
            DataLayout, Function, IrBinaryOp, IrLiteralValue, IrType, Module, ScopeId, TargetTriple, Terminator,
            TerminatorKind, Value,
        };
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

        // Create function with nested scopes
        let mut func = Function::new("test_scope_func", vec![], IrType::I32);
        let entry = "entry_test_scope_func";
        func.add_block(entry, dummy_span());

        // Create scope IDs for nested scopes
        let outer_scope = ScopeId::new();
        let inner_scope = ScopeId::new();

        // Outer scope: Live instruction
        let outer_live = Value::new_temporary(1, IrType::I32);
        let outer_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: Value::new_literal(IrLiteralValue::I32(10)),
                right: Value::new_literal(IrLiteralValue::I32(20)),
                ty: IrType::I32,
            },
            result: Some(outer_live.clone()),
            debug_info: dummy_debug_info(),
            scope: Some(outer_scope),
        };

        // Inner scope: Dead instruction (unused)
        let inner_dead = Value::new_temporary(2, IrType::I32);
        let inner_instr = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Multiply,
                left: Value::new_literal(IrLiteralValue::I32(5)),
                right: Value::new_literal(IrLiteralValue::I32(6)),
                ty: IrType::I32,
            },
            result: Some(inner_dead),
            debug_info: dummy_debug_info(),
            scope: Some(inner_scope),
        };

        // Another outer scope instruction (live)
        let outer_live2 = Value::new_temporary(3, IrType::I32);
        let outer_instr2 = Instruction {
            kind: InstructionKind::Binary {
                op: IrBinaryOp::Add,
                left: outer_live,
                right: Value::new_literal(IrLiteralValue::I32(1)),
                ty: IrType::I32,
            },
            result: Some(outer_live2.clone()),
            debug_info: dummy_debug_info(),
            scope: Some(outer_scope),
        };

        func.add_instruction(entry, outer_instr);
        func.add_instruction(entry, inner_instr);
        func.add_instruction(entry, outer_instr2);

        // Return value from outer scope
        func.set_terminator(
            entry,
            Terminator {
                kind: TerminatorKind::Return { value: outer_live2, ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        // Note: We expect 3 instructions before DCE (will verify after by counting removed)
        let initial_instruction_count = 3;

        // Run DCE
        let mut module = Module::new("test_module", None);
        module.set_data_layout(DataLayout::LinuxX86_64);
        module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);
        module.add_function(func);

        let mut dce = DeadCodeElimination::default();
        dce.run(&mut module);

        // Verify optimization
        let optimized_func = module.get_function("test_scope_func").expect("function");
        let block_after = optimized_func.cfg.get_block(entry).expect("entry block");

        // Dead instruction in inner scope should be removed
        assert_eq!(block_after.instructions.len(), 2, "Dead instruction in inner scope should be removed");

        // Verify remaining instructions belong to outer scope
        for instr in &block_after.instructions {
            assert_eq!(instr.scope, Some(outer_scope), "Remaining instructions should belong to outer scope");
        }

        println!("✓ Scope boundary preservation verified:");
        println!(
            "  - Dead instruction in inner scope removed: {}",
            initial_instruction_count - block_after.instructions.len()
        );
        println!("  - Live instructions in outer scope preserved: {}", block_after.instructions.len());
        println!("  - Scope integrity maintained (FR-015)");
    }
}
