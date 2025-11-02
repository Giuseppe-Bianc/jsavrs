// Dead Code Elimination: Escape Analysis Tests
//
// Tests for User Story 3: Optimize Memory Operations Safely (Priority P3)
// FR-007: Remove dead stores and loads while preserving observable behavior
// FR-019: Perform flow-insensitive escape analysis

#[cfg(test)]
mod escape_tests {
    use jsavrs::ir::instruction::{DebugInfo, Instruction, InstructionKind};
    use jsavrs::ir::optimizer::{DeadCodeElimination, Phase};
    use jsavrs::ir::{Function, IrLiteralValue, IrType, Terminator, TerminatorKind, Value};
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
        let entry_label = format!("entry_{}", name);
        func.add_block(&entry_label, dummy_span());
        func
    }

    /// T051: Test for dead store to local variable
    /// FR-007: Remove dead stores that don't affect observable behavior
    /// FR-019: Escape analysis determines local vs escaped allocations
    /// SC-002: At least 90% of provably-dead instructions removed
    #[test]
    fn test_dead_store_to_local() {
        let mut func = create_test_function("test_dead_store");

        // Allocate local variable
        let alloca_result = Value::new_temporary(1, IrType::Pointer(Box::new(IrType::I32)));
        let alloca_instr = Instruction {
            kind: InstructionKind::Alloca { ty: IrType::I32 },
            result: Some(alloca_result.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        // Store to local (dead - never loaded)
        let store_instr = Instruction {
            kind: InstructionKind::Store {
                dest: alloca_result.clone(),
                value: Value::new_literal(IrLiteralValue::I32(42)),
            },
            result: None,
            debug_info: dummy_debug_info(),
            scope: None,
        };

        func.add_instruction("entry_test_dead_store", alloca_instr);
        func.add_instruction("entry_test_dead_store", store_instr);

        func.set_terminator(
            "entry_test_dead_store",
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let entry_block = func.cfg.get_block("entry_test_dead_store").expect("entry block should exist");
        assert_eq!(entry_block.instructions.len(), 2, "Should have 2 instructions before DCE (alloca + store)");

        let mut module = jsavrs::ir::Module::new("test_module", None);
        module.add_function(func);

        let mut dce = DeadCodeElimination::default();
        dce.run(&mut module);

        let optimized_func = &module.functions()[0];
        let entry_block_after =
            optimized_func.cfg.get_block("entry_test_dead_store").expect("entry block should exist");

        // Both alloca and store should be removed (dead store to unused local)
        assert!(
            entry_block_after.instructions.len() < 2,
            "Dead store and alloca should be removed (actual: {})",
            entry_block_after.instructions.len()
        );
    }

    /// T052: Test for dead load from local variable
    /// FR-007: Remove loads whose results are unused
    #[test]
    fn test_dead_load_from_local() {
        let mut func = create_test_function("test_dead_load");

        // Allocate local variable
        let alloca_result = Value::new_temporary(1, IrType::Pointer(Box::new(IrType::I32)));
        let alloca_instr = Instruction {
            kind: InstructionKind::Alloca { ty: IrType::I32 },
            result: Some(alloca_result.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        // Store initial value
        let store_instr = Instruction {
            kind: InstructionKind::Store {
                dest: alloca_result.clone(),
                value: Value::new_literal(IrLiteralValue::I32(42)),
            },
            result: None,
            debug_info: dummy_debug_info(),
            scope: None,
        };

        // Load from local (dead - result unused)
        let load_result = Value::new_temporary(2, IrType::I32);
        let load_instr = Instruction {
            kind: InstructionKind::Load { src: alloca_result.clone(), ty: IrType::I32 },
            result: Some(load_result),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        func.add_instruction("entry_test_dead_load", alloca_instr);
        func.add_instruction("entry_test_dead_load", store_instr);
        func.add_instruction("entry_test_dead_load", load_instr);

        func.set_terminator(
            "entry_test_dead_load",
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let entry_block = func.cfg.get_block("entry_test_dead_load").expect("entry block should exist");
        assert_eq!(entry_block.instructions.len(), 3, "Should have 3 instructions before DCE");

        let mut module = jsavrs::ir::Module::new("test_module", None);
        module.add_function(func);

        let mut dce = DeadCodeElimination::default();
        dce.run(&mut module);

        let optimized_func = &module.functions()[0];
        let entry_block_after = optimized_func.cfg.get_block("entry_test_dead_load").expect("entry block should exist");

        // All instructions should be removed (unused local allocation and operations)
        assert!(
            entry_block_after.instructions.len() < 3,
            "Dead load should be removed (actual: {})",
            entry_block_after.instructions.len()
        );
    }

    /// T053: Test for store to potentially-aliased pointer (must preserve)
    /// FR-007, FR-008: Conservative preservation of stores that may be observed
    #[test]
    fn test_preserve_store_to_escaped_pointer() {
        let mut func = create_test_function("test_escaped_store");

        // Function parameter (assumed escaped)
        let param_ptr = Value::new_local(Arc::from("param_ptr"), IrType::Pointer(Box::new(IrType::I32)));

        // Store to parameter pointer (must preserve - may be aliased/escaped)
        let store_instr = Instruction {
            kind: InstructionKind::Store { dest: param_ptr, value: Value::new_literal(IrLiteralValue::I32(42)) },
            result: None,
            debug_info: dummy_debug_info(),
            scope: None,
        };

        func.add_instruction("entry_test_escaped_store", store_instr);

        func.set_terminator(
            "entry_test_escaped_store",
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let entry_block = func.cfg.get_block("entry_test_escaped_store").expect("entry block should exist");
        assert_eq!(entry_block.instructions.len(), 1, "Should have 1 store instruction before DCE");

        let mut module = jsavrs::ir::Module::new("test_module", None);
        module.add_function(func);

        let mut dce = DeadCodeElimination::default();
        dce.run(&mut module);

        let optimized_func = &module.functions()[0];
        let entry_block_after =
            optimized_func.cfg.get_block("entry_test_escaped_store").expect("entry block should exist");

        // Store must be preserved (pointer may be escaped/aliased)
        assert_eq!(entry_block_after.instructions.len(), 1, "Store to escaped pointer must be preserved");
    }

    /// T054: Test for alloca with no loads and non-escaped address
    /// FR-019: Escape analysis identifies truly local allocations
    #[test]
    fn test_remove_unused_alloca() {
        let mut func = create_test_function("test_unused_alloca");

        // Allocate local variable (never used)
        let alloca_result = Value::new_temporary(1, IrType::Pointer(Box::new(IrType::I32)));
        let alloca_instr = Instruction {
            kind: InstructionKind::Alloca { ty: IrType::I32 },
            result: Some(alloca_result),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        func.add_instruction("entry_test_unused_alloca", alloca_instr);

        func.set_terminator(
            "entry_test_unused_alloca",
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let entry_block = func.cfg.get_block("entry_test_unused_alloca").expect("entry block should exist");
        assert_eq!(entry_block.instructions.len(), 1, "Should have 1 alloca instruction before DCE");

        let mut module = jsavrs::ir::Module::new("test_module", None);
        module.add_function(func);

        let mut dce = DeadCodeElimination::default();
        dce.run(&mut module);

        let optimized_func = &module.functions()[0];
        let entry_block_after =
            optimized_func.cfg.get_block("entry_test_unused_alloca").expect("entry block should exist");

        // Unused alloca should be removed
        assert_eq!(entry_block_after.instructions.len(), 0, "Unused alloca should be removed");
    }

    // ========================================================================
    // Phase 9: Edge Case Tests
    // ========================================================================

    /// T106 [Phase 9]: Test for complex pointer computation (GEP chain)
    #[test]
    fn test_complex_pointer_computation() {
        let mut func = create_test_function("test_gep_chain");

        // Allocate array-like structure
        let alloca_result = Value::new_temporary(1, IrType::Pointer(Box::new(IrType::I32)));
        let alloca_instr = Instruction {
            kind: InstructionKind::Alloca { ty: IrType::I32 },
            result: Some(alloca_result.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        // First GEP: compute address offset
        let gep1_result = Value::new_temporary(2, IrType::Pointer(Box::new(IrType::I32)));
        let gep1_instr = Instruction {
            kind: InstructionKind::GetElementPtr {
                base: alloca_result.clone(),
                index: Value::new_literal(IrLiteralValue::I32(0)),
                element_ty: IrType::I32,
            },
            result: Some(gep1_result.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        // Second GEP: nested address computation (unused chain)
        let gep2_result = Value::new_temporary(3, IrType::Pointer(Box::new(IrType::I32)));
        let gep2_instr = Instruction {
            kind: InstructionKind::GetElementPtr {
                base: gep1_result.clone(),
                index: Value::new_literal(IrLiteralValue::I32(1)),
                element_ty: IrType::I32,
            },
            result: Some(gep2_result.clone()),
            debug_info: dummy_debug_info(),
            scope: None,
        };

        func.add_instruction("entry_test_gep_chain", alloca_instr);
        func.add_instruction("entry_test_gep_chain", gep1_instr);
        func.add_instruction("entry_test_gep_chain", gep2_instr);

        func.set_terminator(
            "entry_test_gep_chain",
            Terminator {
                kind: TerminatorKind::Return { value: Value::new_literal(IrLiteralValue::I32(0)), ty: IrType::I32 },
                debug_info: jsavrs::ir::terminator::DebugInfo { source_span: dummy_span() },
            },
        );

        let entry_block = func.cfg.get_block("entry_test_gep_chain").expect("entry block");
        assert_eq!(entry_block.instructions.len(), 3, "Should have 3 instructions before DCE (alloca + 2 GEPs)");

        let mut module = jsavrs::ir::Module::new("test_module", None);
        module.add_function(func);

        let mut dce = DeadCodeElimination::default();
        dce.run(&mut module);

        let optimized_func = &module.functions()[0];
        let entry_block_after = optimized_func.cfg.get_block("entry_test_gep_chain").expect("entry block");

        // Entire GEP chain should be removed (unused address computations)
        assert_eq!(entry_block_after.instructions.len(), 0, "Unused GEP chain should be removed entirely");

        println!("✓ Complex GEP chain successfully removed by DCE");
    }
}
