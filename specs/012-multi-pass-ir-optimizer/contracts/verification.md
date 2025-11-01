# Contract: Verification Interface

**Purpose**: Defines verification functions and rollback mechanisms ensuring optimizer preserves IR correctness invariants (SSA form, CFG consistency, type correctness).

## Verification Functions

### 1. SSA Form Verification

```rust
/// Verifies SSA form invariants
/// 
/// Checks:
/// - Each temporary is defined exactly once
/// - All uses are dominated by their definitions
/// - Phi nodes have correct predecessor counts
/// - No uses of undefined values
/// 
/// # Parameters
/// 
/// - `function`: Function to verify
/// 
/// # Returns
/// 
/// `Ok(())` if SSA form is valid, `Err(VerificationError)` otherwise
/// 
/// # Example
/// 
/// ```rust
/// match verify_ssa_form(function) {
///     Ok(()) => println!("SSA form valid"),
///     Err(VerificationError::DuplicateDefinition { value_id, block }) => {
///         eprintln!("Duplicate definition of {} in block {}", value_id, block);
///     }
///     Err(e) => eprintln!("SSA verification failed: {:?}", e),
/// }
/// ```
pub fn verify_ssa_form(function: &Function) -> Result<(), VerificationError>;
```

**Detailed Checks**:

1. **Unique Definitions**:
   ```rust
   let mut defined_temps = HashSet::new();
   for instruction in all_instructions {
       if let Some(result) = instruction.result_value() {
           if !defined_temps.insert(result.id) {
               return Err(VerificationError::DuplicateDefinition { ... });
           }
       }
   }
   ```

2. **Dominance**:
   ```rust
   for instruction in all_instructions {
       for operand in instruction.operands() {
           let def_block = find_definition_block(operand);
           let use_block = current_block;
           if !dominance_info.dominates(def_block, use_block) {
               return Err(VerificationError::UseNotDominatedByDef { ... });
           }
       }
   }
   ```

3. **Phi Node Consistency**:
   ```rust
   for phi in phi_nodes {
       let predecessors = cfg.predecessors(current_block);
       if phi.incoming.len() != predecessors.len() {
           return Err(VerificationError::PhiPredecessorMismatch { ... });
       }
       for (_, pred_label) in phi.incoming {
           if !predecessors.contains(pred_label) {
               return Err(VerificationError::InvalidPhiPredecessor { ... });
           }
       }
   }
   ```

---

### 2. CFG Consistency Verification

```rust
/// Verifies control flow graph consistency
/// 
/// Checks:
/// - Entry block has no predecessors
/// - All blocks reachable from entry
/// - All terminator targets are valid blocks
/// - No unreachable terminators (should be removed by DCE)
/// 
/// # Parameters
/// 
/// - `function`: Function to verify
/// 
/// # Returns
/// 
/// `Ok(())` if CFG is consistent, `Err(VerificationError)` otherwise
/// 
/// # Example
/// 
/// ```rust
/// if let Err(e) = verify_cfg_consistency(function) {
///     eprintln!("CFG verification failed: {:?}", e);
///     // Rollback will be triggered
/// }
/// ```
pub fn verify_cfg_consistency(function: &Function) -> Result<(), VerificationError>;
```

**Detailed Checks**:

1. **Entry Block**:
   ```rust
   let entry = cfg.entry_block();
   if !cfg.predecessors(entry).is_empty() {
       return Err(VerificationError::EntryBlockHasPredecessors);
   }
   ```

2. **Reachability**:
   ```rust
   let reachable = cfg.reachable_from_entry();  // DFS from entry
   if reachable.len() != cfg.blocks.len() {
       return Err(VerificationError::UnreachableBlocks {
           count: cfg.blocks.len() - reachable.len(),
       });
   }
   ```

3. **Valid Targets**:
   ```rust
   for (block_label, block) in &cfg.blocks {
       for target in block.terminator.targets() {
           if !cfg.blocks.contains_key(&target) {
               return Err(VerificationError::InvalidTerminatorTarget {
                   block: block_label.clone(),
                   target,
               });
           }
       }
   }
   ```

4. **No Unreachable Terminators**:
   ```rust
   for (block_label, block) in &cfg.blocks {
       if matches!(block.terminator.kind, TerminatorKind::Unreachable) {
           return Err(VerificationError::UnreachableTerminator {
               block: block_label.clone(),
           });
       }
   }
   ```

---

### 3. Type Consistency Verification

```rust
/// Verifies type consistency across instructions
/// 
/// Checks:
/// - Binary operation operands have matching types
/// - Phi incoming values have identical types
/// - Load operations have pointer-typed operands
/// - Cast operations have valid type conversions
/// 
/// # Parameters
/// 
/// - `function`: Function to verify
/// 
/// # Returns
/// 
/// `Ok(())` if types are consistent, `Err(VerificationError)` otherwise
/// 
/// # Example
/// 
/// ```rust
/// verify_type_consistency(function)?;
/// ```
pub fn verify_type_consistency(function: &Function) -> Result<(), VerificationError>;
```

**Detailed Checks**:

1. **Binary Operations**:
   ```rust
   match &instruction.kind {
       InstructionKind::Binary { lhs, rhs, ty, .. } => {
           if lhs.ty != rhs.ty {
               return Err(VerificationError::BinaryOperandTypeMismatch { ... });
           }
           if lhs.ty != *ty {
               return Err(VerificationError::BinaryResultTypeMismatch { ... });
           }
       }
       // ...
   }
   ```

2. **Phi Nodes**:
   ```rust
   match &instruction.kind {
       InstructionKind::Phi { incoming, ty } => {
           for (value, _) in incoming {
               if value.ty != *ty {
                   return Err(VerificationError::PhiTypeMismatch { ... });
               }
           }
       }
       // ...
   }
   ```

3. **Memory Operations**:
   ```rust
   match &instruction.kind {
       InstructionKind::Load { ptr, .. } => {
           if !matches!(ptr.ty, IrType::Pointer(_)) {
               return Err(VerificationError::LoadFromNonPointer { ... });
           }
       }
       // ...
   }
   ```

---

### 4. Composite Verification

```rust
/// Runs all verification checks on function
/// 
/// Convenience function combining SSA, CFG, and type verification.
/// 
/// # Parameters
/// 
/// - `function`: Function to verify
/// 
/// # Returns
/// 
/// `Ok(())` if all checks pass, `Err(VerificationError)` on first failure
/// 
/// # Example
/// 
/// ```rust
/// fn run_pass_with_verification(
///     pass: &dyn OptimizationPass,
///     function: &mut Function,
///     analysis_mgr: &AnalysisManager,
///     use_def_mgr: &mut UseDefManager,
/// ) -> Result<PassResult, OptimizerError> {
///     let snapshot = FunctionSnapshot::capture(function, use_def_mgr);
///     let result = pass.run(function, analysis_mgr);
///     
///     if result.changed {
///         if let Err(err) = verify_function(function) {
///             snapshot.restore(function, use_def_mgr);
///             return Err(OptimizerError::VerificationFailed {
///                 pass: pass.name().to_string(),
///                 message: format!("{:?}", err),
///             });
///         }
///     }
///     
///     Ok(result)
/// }
/// ```
pub fn verify_function(function: &Function) -> Result<(), VerificationError> {
    verify_ssa_form(function)?;
    verify_cfg_consistency(function)?;
    verify_type_consistency(function)?;
    Ok(())
}
```

---

## Rollback Mechanism

### FunctionSnapshot

```rust
/// Captures function state for rollback on verification failure
/// 
/// Stores complete function state (blocks, edges, use-def chains) to enable
/// exact restoration after failed optimization pass.
/// 
/// # Example
/// 
/// ```rust
/// let snapshot = FunctionSnapshot::capture(function, use_def_mgr);
/// 
/// // ... pass modifies function ...
/// 
/// if verification_fails {
///     snapshot.restore(function, use_def_mgr);
///     // Function now in exact pre-pass state
/// }
/// ```
pub struct FunctionSnapshot {
    /// Cloned basic blocks (instruction sequences)
    blocks: Vec<BasicBlock>,
    
    /// CFG edges (src_label, dst_label)
    edges: Vec<(String, String)>,
    
    /// Use-def chains snapshot
    use_def_chains: HashMap<ValueId, InstructionRef>,
    
    /// Def-use chains snapshot
    def_use_chains: HashMap<ValueId, Vec<InstructionRef>>,
}

impl FunctionSnapshot {
    /// Captures current function state
    /// 
    /// # Parameters
    /// 
    /// - `function`: Function to capture
    /// - `use_def_mgr`: Use-def manager to capture
    /// 
    /// # Returns
    /// 
    /// Immutable snapshot enabling exact restoration
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let snapshot = FunctionSnapshot::capture(function, use_def_mgr);
    /// assert_eq!(snapshot.blocks.len(), function.cfg.blocks.len());
    /// ```
    pub fn capture(function: &Function, use_def_mgr: &UseDefManager) -> Self {
        FunctionSnapshot {
            blocks: function.cfg.blocks.values().cloned().collect(),
            edges: function.cfg.edges()
                .map(|(s, d)| (s.clone(), d.clone()))
                .collect(),
            use_def_chains: use_def_mgr.use_def.clone(),
            def_use_chains: use_def_mgr.def_use.clone(),
        }
    }
    
    /// Restores function to captured state
    /// 
    /// # Parameters
    /// 
    /// - `function`: Function to restore (mutable)
    /// - `use_def_mgr`: Use-def manager to restore (mutable)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// snapshot.restore(function, use_def_mgr);
    /// // function.cfg.blocks == snapshot.blocks
    /// // use_def_mgr.use_def == snapshot.use_def_chains
    /// ```
    pub fn restore(&self, function: &mut Function, use_def_mgr: &mut UseDefManager) {
        // Clear existing state
        function.cfg.clear();
        
        // Restore blocks
        for block in &self.blocks {
            function.cfg.insert_block(block.clone());
        }
        
        // Restore edges
        for (src, dst) in &self.edges {
            function.cfg.connect_blocks(src, dst);
        }
        
        // Restore use-def chains
        use_def_mgr.use_def = self.use_def_chains.clone();
        use_def_mgr.def_use = self.def_use_chains.clone();
    }
}
```

---

## Error Types

### VerificationError

```rust
/// Errors detected during verification
/// 
/// Represents violations of SSA, CFG, or type invariants.
#[derive(Debug, Clone)]
pub enum VerificationError {
    /// SSA: Temporary defined multiple times
    DuplicateDefinition {
        value_id: ValueId,
        block: String,
    },
    
    /// SSA: Value used before definition
    UseBeforeDefinition {
        value_id: ValueId,
        block: String,
    },
    
    /// SSA: Use not dominated by definition
    UseNotDominatedByDef {
        value_id: ValueId,
        def_block: String,
        use_block: String,
    },
    
    /// SSA: Phi node has wrong number of incoming values
    PhiPredecessorMismatch {
        block: String,
        expected: usize,
        actual: usize,
    },
    
    /// SSA: Phi node references non-predecessor
    InvalidPhiPredecessor {
        block: String,
        pred: String,
    },
    
    /// CFG: Entry block has predecessors
    EntryBlockHasPredecessors,
    
    /// CFG: Unreachable blocks exist
    UnreachableBlocks {
        count: usize,
    },
    
    /// CFG: Terminator targets invalid block
    InvalidTerminatorTarget {
        block: String,
        target: String,
    },
    
    /// CFG: Unreachable terminator present (should be removed by DCE)
    UnreachableTerminator {
        block: String,
    },
    
    /// Type: Binary operands have mismatched types
    BinaryOperandTypeMismatch {
        block: String,
        lhs_ty: IrType,
        rhs_ty: IrType,
    },
    
    /// Type: Binary result type doesn't match operands
    BinaryResultTypeMismatch {
        block: String,
        operand_ty: IrType,
        result_ty: IrType,
    },
    
    /// Type: Load from non-pointer value
    LoadFromNonPointer {
        block: String,
        ty: IrType,
    },
    
    /// Type: Phi incoming values have mismatched types
    PhiTypeMismatch {
        block: String,
        expected: IrType,
        actual: IrType,
    },
}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DuplicateDefinition { value_id, block } => {
                write!(f, "Duplicate definition of {} in block {}", value_id, block)
            }
            // ... (format all variants)
        }
    }
}

impl std::error::Error for VerificationError {}
```

---

## Contract Guarantees

### Verification Guarantees

1. **Soundness**:
   - Verification failure indicates actual IR inconsistency
   - No false positives (correct IR never fails verification)

2. **Completeness**:
   - All critical invariants checked (SSA, CFG, types)
   - No silent corruption (all violations detected)

3. **Performance**:
   - Verification completes in O(n) to O(n²) time
   - Snapshot capture/restore in O(n) time

### Rollback Guarantees

1. **Exactness**:
   - Restore produces exact pre-pass state
   - No data loss during capture/restore cycle

2. **Atomicity**:
   - Either pass succeeds fully or function unchanged
   - No partial modifications left after rollback

3. **Consistency**:
   - Restored function passes verification
   - Use-def chains consistent with instructions

---

## Usage Guidelines

### When to Verify

- **Always**: After pass reports `changed = true`
- **Never**: After pass reports `changed = false` (no modifications to verify)
- **Optional**: Before pass execution (defensive programming, catches malformed input)

### Performance Considerations

- Verification is O(n) to O(n²), acceptable for large functions (<1s for 10k instructions)
- Snapshot is O(n), dominated by block cloning
- Optimize by skipping verification when `config.skip_verification = true` (unsafe mode for performance testing)

---

## Version History

- **1.0** (2025-11-01): Initial contract definition
