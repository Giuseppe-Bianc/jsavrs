# Quickstart Guide: Comprehensive Type Promotion Engine Test Suite

**Date**: October 6, 2025  
**Feature**: Comprehensive Type Promotion Engine Test Suite  
**Test File**: `tests/ir_type_promotion_engine_tests.rs`

---

## Quick Reference

```bash
# Run all tests
cargo test ir_type_promotion_engine_tests

# Run specific test group
cargo test ir_type_promotion_engine_tests::test_analyze_binary_promotion

# Run with output
cargo test ir_type_promotion_engine_tests -- --nocapture

# Run in release mode (faster)
cargo test ir_type_promotion_engine_tests --release

# Generate coverage report
cargo llvm-cov --html --package jsavrs --test ir_type_promotion_engine_tests

# Review snapshots (after changes)
cargo insta review

# Accept all snapshots
cargo insta accept
```

---

## Getting Started

### Prerequisites

1. **Install Required Tools**:
   ```bash
   # Install insta for snapshot testing
   cargo install cargo-insta
   
   # Install llvm-cov for coverage
   cargo install cargo-llvm-cov
   
   # Verify installations
   cargo insta --version
   cargo llvm-cov --version
   ```

2. **Add Dependencies** (in `Cargo.toml`):
   ```toml
   [dev-dependencies]
   insta = "1.34"  # Snapshot testing
   ```

---

## Running the Tests

### Basic Test Execution

```bash
# Run all type promotion engine tests
cargo test ir_type_promotion_engine_tests

# Expected output:
# running 120 tests
# test test_analyze_binary_promotion_identity_i32_add ... ok
# test test_analyze_binary_promotion_widening_i8_to_i32 ... ok
# ...
# test result: ok. 120 passed; 0 failed; 0 ignored; 0 measured
```

### Running Specific Test Groups

```bash
# Run only analyze_binary_promotion tests
cargo test test_analyze_binary_promotion

# Run only insert_promotion_casts tests
cargo test test_insert_promotion_casts

# Run only warning generation tests
cargo test test_warning

# Run only edge case tests
cargo test test_edge_case

# Run only corner case tests
cargo test test_corner_case

# Run only concurrent execution tests
cargo test test_concurrent
```

### Running Individual Tests

```bash
# Run single test by exact name
cargo test test_analyze_binary_promotion_i32_f32_add_promotes_to_f32 -- --exact

# Run with output for debugging
cargo test test_analyze_binary_promotion_i32_f32 -- --nocapture
```

---

## Code Coverage

### Generate Coverage Report

```bash
# Generate HTML coverage report
cargo llvm-cov --html --package jsavrs --test ir_type_promotion_engine_tests

# Open report in browser
# Windows
start target/llvm-cov/html/index.html

# macOS
open target/llvm-cov/html/index.html

# Linux
xdg-open target/llvm-cov/html/index.html
```

### Expected Coverage Results

```
File: src/ir/type_promotion_engine.rs
Lines: 100.0% (250/250)
Branches: 100.0% (45/45)
Functions: 100.0% (3/3)

✅ Target: 100% coverage achieved
```

### Coverage by Function

```
analyze_binary_promotion:
  Lines: 100.0% (120/120)
  Branches: 100.0% (30/30)

insert_promotion_casts:
  Lines: 100.0% (80/80)
  Branches: 100.0% (10/10)

insert_cast_instruction (private):
  Lines: 100.0% (50/50)
  Branches: 100.0% (5/5)
```

---

## Snapshot Testing

### Understanding Snapshots

Snapshot tests capture complex output and compare against saved snapshots. When output changes, snapshots can be reviewed and accepted if correct.

### Snapshot Workflow

1. **Run Tests** (generates/compares snapshots):
   ```bash
   cargo test ir_type_promotion_engine_tests
   ```

2. **Review Changed Snapshots**:
   ```bash
   cargo insta review
   ```
   
   **Interactive Review**:
   - `a` - Accept this snapshot
   - `r` - Reject this snapshot
   - `s` - Skip this snapshot
   - `q` - Quit review
   - `d` - Show diff

3. **Accept All Snapshots** (if all changes are correct):
   ```bash
   cargo insta accept
   ```

4. **Reject All Snapshots** (revert to previous):
   ```bash
   # Delete .snap.new files
   find tests/snapshots -name "*.snap.new" -delete
   ```

### Snapshot Locations

```
tests/snapshots/ir_type_promotion_engine_tests/
├── test_analyze_binary_promotion_i32_f32_complex_result.snap
├── test_warning_precision_loss_f64_to_f32_details.snap
├── test_edge_case_float_nan_handling.snap
└── ...
```

### Example Snapshot Review

```bash
$ cargo insta review

Reviewing test_analyze_binary_promotion_i32_f32_add_promotes_to_f32
Snapshot: tests/snapshots/ir_type_promotion_engine_tests/test_analyze_binary_promotion_i32_f32_add_promotes_to_f32.snap

Old:
---
result_type: "I32"  # BUG: Should be F32
left_cast: Some(...)
...

New:
---
result_type: "F32"  # FIXED
left_cast: Some(...)
...

[a]ccept, [r]eject, [s]kip, [q]uit, [d]iff: a
✅ Accepted
```

---

## Test Organization

### Test File Structure

```rust
// tests/ir_type_promotion_engine_tests.rs

// ============================================================================
// Module: Tests for TypePromotionEngine::analyze_binary_promotion
// ============================================================================
// ~40 tests covering type combinations and operations

#[test]
fn test_analyze_binary_promotion_identity_i32_add() { ... }

#[test]
fn test_analyze_binary_promotion_widening_i8_to_i32() { ... }

// ============================================================================
// Module: Tests for TypePromotionEngine::insert_promotion_casts
// ============================================================================
// ~30 tests covering cast insertion scenarios

#[test]
fn test_insert_promotion_casts_no_casts_identity() { ... }

#[test]
fn test_insert_promotion_casts_left_only_i8_to_i32() { ... }

// ... additional groups ...
```

### Test Naming Convention

```
test_<function_or_group>_<scenario>_<expected_outcome>

Examples:
- test_analyze_binary_promotion_i32_f32_promotes_to_f32
- test_insert_promotion_casts_both_i32_u32_to_i64
- test_warning_precision_loss_f64_to_f32_significant_digits
- test_edge_case_float_nan_to_integer
- test_concurrent_10_threads_100_operations
```

---

## Debugging Tests

### Running with Output

```bash
# Show println! and debug output
cargo test test_analyze_binary_promotion_i32_f32 -- --nocapture

# Show test output even on success
cargo test test_analyze_binary_promotion_i32_f32 -- --nocapture --show-output
```

### Debugging Single Test

```rust
#[test]
fn test_analyze_binary_promotion_debug_example() {
    let engine = TypePromotionEngine::new();
    let result = engine.analyze_binary_promotion(
        &IrType::I32,
        &IrType::F32,
        IrBinaryOp::Add,
        SourceSpan::default()
    );
    
    // Debug output
    println!("Result: {:#?}", result);
    println!("Result type: {:?}", result.result_type);
    println!("Left cast: {:?}", result.left_cast);
    println!("Warnings count: {}", result.warnings.len());
    
    // Run with: cargo test test_analyze_binary_promotion_debug_example -- --nocapture
}
```

### Using VS Code Debugger

1. Set breakpoint in test function
2. Run "Debug Test" from test function
3. Inspect variables in Debug panel

---

## Performance Testing

### Measure Test Execution Time

```bash
# Measure total test suite execution time
time cargo test ir_type_promotion_engine_tests --release

# Expected output:
# running 120 tests
# test result: ok. 120 passed; 0 failed; 0 ignored
#
# real    0m2.345s  ✅ Under 10 seconds
# user    0m8.123s
# sys     0m0.456s
```

### Benchmark Individual Tests

```bash
# Run with time output
cargo test test_analyze_binary_promotion_i32_f32 --release -- --nocapture --test-threads=1

# Check that individual tests complete in <100ms
```

### Concurrent Test Performance

```bash
# Concurrent tests should complete in <2 seconds each
cargo test test_concurrent --release -- --nocapture
```

---

## Common Workflows

### Workflow 1: Adding New Tests

```bash
# 1. Write test in tests/ir_type_promotion_engine_tests.rs
#    (follow naming convention and organization)

# 2. Run test to generate initial snapshot (if using snapshots)
cargo test test_new_feature -- --nocapture

# 3. Review and accept snapshot
cargo insta review

# 4. Verify test passes
cargo test test_new_feature

# 5. Check coverage impact
cargo llvm-cov --html

# 6. Commit test and snapshots
git add tests/ir_type_promotion_engine_tests.rs
git add tests/snapshots/ir_type_promotion_engine_tests/*.snap
git commit -m "Add test for new type promotion scenario"
```

### Workflow 2: Fixing Failed Test

```bash
# 1. Run failing test with output
cargo test test_failing -- --nocapture

# 2. Debug test (add println!, set breakpoints)

# 3. Fix implementation in src/ir/type_promotion_engine.rs

# 4. Re-run test
cargo test test_failing

# 5. Update snapshot if output changed
cargo insta review

# 6. Verify all tests still pass
cargo test ir_type_promotion_engine_tests

# 7. Check coverage maintained
cargo llvm-cov --html
```

### Workflow 3: Refactoring TypePromotionEngine

```bash
# 1. Run full test suite before refactoring
cargo test ir_type_promotion_engine_tests
# Ensure all pass: ✅

# 2. Refactor src/ir/type_promotion_engine.rs

# 3. Run tests continuously during refactoring
cargo watch -x "test ir_type_promotion_engine_tests"

# 4. Review any snapshot changes
cargo insta review

# 5. Verify coverage maintained
cargo llvm-cov --html

# 6. Commit refactoring with confidence
git commit -m "Refactor type promotion engine (all tests pass)"
```

---

## Continuous Integration

### CI Configuration Example (GitHub Actions)

```yaml
name: Type Promotion Engine Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install test tools
        run: |
          cargo install cargo-insta
          cargo install cargo-llvm-cov
      
      - name: Run tests
        run: cargo test ir_type_promotion_engine_tests --release
      
      - name: Generate coverage
        run: cargo llvm-cov --html --package jsavrs --test ir_type_promotion_engine_tests
      
      - name: Check coverage
        run: |
          coverage=$(cargo llvm-cov report --package jsavrs | grep 'type_promotion_engine.rs' | awk '{print $4}')
          if [ "${coverage%.*}" -lt 100 ]; then
            echo "❌ Coverage below 100%: $coverage"
            exit 1
          fi
          echo "✅ Coverage: $coverage"
      
      - name: Upload coverage report
        uses: actions/upload-artifact@v3
        with:
          name: coverage-report
          path: target/llvm-cov/html/
```

---

## Troubleshooting

### Problem: Tests Fail with Snapshot Mismatch

**Solution**:
```bash
# Review the differences
cargo insta review

# If changes are correct, accept
cargo insta accept

# If changes are incorrect, fix implementation and re-run
cargo test ir_type_promotion_engine_tests
```

### Problem: Coverage Below 100%

**Solution**:
```bash
# Generate HTML coverage report
cargo llvm-cov --html

# Open report and identify uncovered lines (red highlights)
start target/llvm-cov/html/index.html  # Windows
# Look for red-highlighted lines in src/ir/type_promotion_engine.rs

# Add tests to cover uncovered lines/branches
# Re-run coverage
cargo llvm-cov --html
```

### Problem: Concurrent Tests Hang

**Solution**:
```bash
# Run concurrent tests in isolation
cargo test test_concurrent --release -- --nocapture --test-threads=1

# Check for deadlocks or infinite loops
# Verify Arc usage is correct
# Ensure no mutable state
```

### Problem: Tests Too Slow

**Solution**:
```bash
# Run in release mode for faster execution
cargo test ir_type_promotion_engine_tests --release

# Parallelize tests (default behavior)
cargo test ir_type_promotion_engine_tests -- --test-threads=8

# Profile slow tests
cargo test test_slow_test -- --nocapture --time
```

---

## Best Practices

### Do's ✅

- ✅ Run full test suite before committing (`cargo test ir_type_promotion_engine_tests`)
- ✅ Review snapshots carefully before accepting (`cargo insta review`)
- ✅ Check coverage after adding tests (`cargo llvm-cov --html`)
- ✅ Use descriptive test names following convention
- ✅ Add comments for complex test scenarios
- ✅ Use explicit assertions for critical properties
- ✅ Use snapshots for complex outputs

### Don'ts ❌

- ❌ Don't blindly accept all snapshots (`cargo insta accept` without review)
- ❌ Don't commit `.snap.new` files (review and accept first)
- ❌ Don't skip coverage checks
- ❌ Don't write tests with dependencies on execution order
- ❌ Don't use magic numbers without comments
- ❌ Don't mix test concerns (one test = one scenario)

---

## Quick Reference Commands

```bash
# Essential Commands
cargo test ir_type_promotion_engine_tests                   # Run all tests
cargo test test_analyze_binary_promotion                    # Run test group
cargo llvm-cov --html                                        # Generate coverage
cargo insta review                                           # Review snapshots
cargo insta accept                                           # Accept all snapshots

# Debugging
cargo test <test_name> -- --nocapture                       # Show output
cargo test <test_name> -- --exact                           # Run exact test
cargo test <test_name> -- --nocapture --show-output         # Always show output

# Performance
cargo test ir_type_promotion_engine_tests --release         # Faster execution
time cargo test ir_type_promotion_engine_tests --release    # Measure time

# CI/CD
cargo test ir_type_promotion_engine_tests --release -- --quiet  # Minimal output
cargo llvm-cov report                                        # Coverage summary
```

---

## Success Criteria

### ✅ Test Suite Ready When:

1. All 100-120 tests pass: `cargo test ir_type_promotion_engine_tests`
2. Coverage at 100%: `cargo llvm-cov --html` shows 100% line and branch coverage
3. All snapshots reviewed and accepted: No `.snap.new` files remain
4. Execution time under 10 seconds: `time cargo test ... --release` < 10s
5. No warnings or errors: Clean cargo output
6. All test groups functional: Each group runs independently

---

## Resources

- **Rust Testing Book**: https://doc.rust-lang.org/book/ch11-00-testing.html
- **Insta Documentation**: https://insta.rs/
- **Cargo LLVM Coverage**: https://github.com/taiki-e/cargo-llvm-cov
- **jsavrs Type Promotion Engine**: `src/ir/type_promotion_engine.rs`
- **Existing Tests**: `tests/ir_type_promotion_tests.rs` (reference)

---

**Quickstart Status**: ✅ COMPLETE - Ready for test suite implementation
