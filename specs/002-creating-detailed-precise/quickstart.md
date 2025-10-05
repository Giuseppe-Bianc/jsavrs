# Quickstart Guide: Type Promotion Test Suite

**Date**: 2025-10-05  
**Feature**: Comprehensive Test Suite for Type Promotion Module  
**Purpose**: Fast validation workflow for test execution, coverage, and verification

---

## Prerequisites

**Required Tools**:
- Rust toolchain (stable): `rustc --version` (tested with 1.70+)
- Cargo: `cargo --version`
- cargo-llvm-cov: `cargo install cargo-llvm-cov` (for coverage reports)
- Insta (via Cargo.toml): Already in project dependencies

**Environment Setup**:
```powershell
# Verify Rust installation
rustc --version
cargo --version

# Install coverage tool (if not already installed)
cargo install cargo-llvm-cov

# Clone repository (if not already)
git clone <repository-url>
cd jsavrs
```

---

## Quick Validation (30 seconds)

**Goal**: Run all type promotion tests and verify they pass.

```powershell
# Run type promotion test suite only
cargo test --test ir_type_promotion_tests

# Expected output:
# running 40-60 tests
# test test_promotion_matrix_new ... ok
# test test_i32_to_f64_direct ... ok
# ...
# test result: ok. 40-60 passed; 0 failed; 0 ignored; 0 measured
```

**Success Criteria**:
- ✅ All tests pass (0 failed)
- ✅ Execution time <5 seconds
- ✅ No compiler warnings

---

## Coverage Validation (60 seconds)

**Goal**: Verify 100% line coverage for `src/ir/type_promotion.rs`.

### Step 1: Generate Coverage Report

```powershell
# Generate coverage with HTML report
cargo llvm-cov --package jsavrs --lib --html --open

# This will:
# 1. Compile with instrumentation
# 2. Run all tests
# 3. Generate HTML report in target/llvm-cov/html/
# 4. Open report in default browser
```

### Step 2: Verify Coverage Target

**In Terminal**:
```powershell
# Get text summary of coverage
cargo llvm-cov --package jsavrs --lib --text | grep "src/ir/type_promotion.rs"

# Expected output:
# src/ir/type_promotion.rs             428      428   100.00%
#                                     ^^^      ^^^   ^^^^^^^
#                                    total   covered  percentage
```

**In Browser** (HTML report):
1. Navigate to `src/ir/type_promotion.rs` in report
2. Verify all lines are green (covered)
3. Check for any red lines (uncovered) → MUST BE ZERO

**Success Criteria**:
- ✅ Line coverage: 100.00%
- ✅ Function coverage: 100.00%
- ✅ No red lines in HTML report

---

## Snapshot Validation (if applicable)

**Goal**: Review and accept snapshot test outputs.

### First-Time Snapshot Review

```powershell
# Run snapshot tests (generates .snap files)
cargo test --test ir_type_promotion_tests

# Review snapshots interactively
cargo insta review

# Insta will show:
# - New snapshots (green): Accept with 'a'
# - Changed snapshots (yellow): Accept with 'a' or skip with 's'
# - Unchanged snapshots: No action needed

# Accept all new snapshots
# (Press 'a' for each, or 'A' to accept all)
```

### Subsequent Runs

```powershell
# Run tests (should auto-match snapshots)
cargo test --test ir_type_promotion_tests

# Expected: No snapshot prompts (all match)
```

**Success Criteria**:
- ✅ All snapshots reviewed and accepted
- ✅ Snapshots committed to version control (.snapshots/ directory)
- ✅ No unexpected snapshot changes

---

## Individual Test Execution

**Goal**: Run specific tests for debugging or focused validation.

### Run Single Test

```powershell
# Run one specific test by name
cargo test --test ir_type_promotion_tests test_promotion_matrix_new

# Expected output:
# running 1 test
# test test_promotion_matrix_new ... ok
```

### Run Tests by Pattern

```powershell
# Run all edge case tests (assuming naming convention)
cargo test --test ir_type_promotion_tests edge_case

# Run all panic tests
cargo test --test ir_type_promotion_tests panics
```

### Run with Output

```powershell
# Show println! output even for passing tests
cargo test --test ir_type_promotion_tests -- --nocapture

# Show test execution time
cargo test --test ir_type_promotion_tests -- --show-output
```

**Success Criteria**:
- ✅ Specific test passes
- ✅ Output matches expectations (if using --nocapture)

---

## Performance Validation

**Goal**: Verify test suite executes within performance targets.

### Measure Total Execution Time

```powershell
# Measure total time (PowerShell)
Measure-Command { cargo test --test ir_type_promotion_tests }

# Expected output:
# TotalSeconds : 3.2  # ✅ <5 seconds target
```

### Measure Per-Test Time

```powershell
# Use test harness timing (nightly Rust required)
cargo +nightly test --test ir_type_promotion_tests -- -Z unstable-options --report-time

# Expected output:
# test test_promotion_matrix_new ... ok <10ms>
# test test_i32_to_f64_direct ... ok <5ms>
# ...
```

**Success Criteria**:
- ✅ Full suite: <5 seconds
- ✅ Individual tests: <100ms each (no I/O or heavy computation)

---

## CI/CD Integration (GitHub Actions Example)

**Goal**: Automate test + coverage validation in CI pipeline.

**Workflow File**: `.github/workflows/test-type-promotion.yml`

```yaml
name: Type Promotion Tests

on:
  push:
    paths:
      - 'src/ir/type_promotion.rs'
      - 'tests/ir_type_promotion_tests.rs'
  pull_request:
    paths:
      - 'src/ir/type_promotion.rs'
      - 'tests/ir_type_promotion_tests.rs'

jobs:
  test-and-coverage:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v3

      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true

      - name: Install cargo-llvm-cov
        run: cargo install cargo-llvm-cov

      - name: Run tests
        run: cargo test --test ir_type_promotion_tests

      - name: Generate coverage
        run: cargo llvm-cov --package jsavrs --lib --text > coverage.txt

      - name: Verify 100% coverage
        run: |
          COVERAGE=$(grep "src/ir/type_promotion.rs" coverage.txt | awk '{print $4}')
          if [ "$COVERAGE" != "100.00%" ]; then
            echo "❌ Coverage is $COVERAGE, expected 100.00%"
            exit 1
          else
            echo "✅ Coverage: $COVERAGE"
          fi

      - name: Upload coverage report
        uses: actions/upload-artifact@v3
        with:
          name: coverage-report
          path: target/llvm-cov/html
```

**Success Criteria**:
- ✅ CI pipeline passes
- ✅ 100% coverage gate enforced
- ✅ Coverage report artifact uploaded

---

## Troubleshooting

### Issue: Tests Fail with "No such file or directory"

**Symptom**:
```
error: could not compile `jsavrs` due to previous error
error: test failed, to rerun pass '--test ir_type_promotion_tests'
```

**Solution**:
```powershell
# Clean build artifacts
cargo clean

# Rebuild and test
cargo test --test ir_type_promotion_tests
```

---

### Issue: Coverage Not Reaching 100%

**Symptom**:
```
src/ir/type_promotion.rs             428      420   98.13%
                                                ^^^   ^^^^^
                                             8 lines uncovered!
```

**Solution**:
1. **Open HTML report**: `cargo llvm-cov --html --open`
2. **Identify red lines**: Navigate to `src/ir/type_promotion.rs`
3. **Write tests for uncovered lines**:
   - Red lines often indicate:
     - Unused helper functions (mark `#[cfg(test)]` if test-only)
     - Unreachable error paths (add error handling tests)
     - Edge cases not tested (add boundary value tests)

**Example**:
```rust
// If line 142 is uncovered (unreachable! macro):
unreachable!("Invalid promotion path"); // Line 142

// Add test to cover this:
#[test]
#[should_panic(expected = "Invalid promotion path")]
fn test_invalid_promotion_path_panics() {
    // Trigger unreachable! path
}
```

---

### Issue: Snapshot Test Mismatch

**Symptom**:
```
test test_promotion_result_snapshot ... FAILED

Snapshot mismatch:
- expected: PromotionResult { result_type: F64, ... }
+ actual:   PromotionResult { result_type: F32, ... }
```

**Solution**:
```powershell
# Review snapshot diff
cargo insta review

# Options:
# 1. Accept new snapshot (if intentional change): Press 'a'
# 2. Reject snapshot (fix code instead): Press 'r'

# Update snapshots permanently
cargo insta accept
```

---

### Issue: Tests Pass Locally but Fail in CI

**Symptom**:
- ✅ Local: `cargo test` → all pass
- ❌ CI: `cargo test` → failures

**Common Causes**:
1. **Platform differences**: Tests assume Windows paths (use `/` not `\`)
2. **Non-deterministic tests**: Tests depend on timing, randomness, or order
3. **Missing dependencies**: CI missing `cargo-llvm-cov` or other tools

**Solution**:
```rust
// ❌ Bad: Platform-specific path
let path = "C:\\dev\\jsavrs\\src"; 

// ✅ Good: Cross-platform path
let path = std::env::current_dir().unwrap().join("src");

// ❌ Bad: Non-deterministic
let result = compute_with_timeout(Duration::from_millis(10)); // Flaky!

// ✅ Good: Deterministic
let result = compute_promotion(&IrType::I32, &IrType::F64);
```

---

## Validation Checklist

Use this checklist to verify test suite health:

### Pre-Commit Checklist
- [ ] All tests pass: `cargo test --test ir_type_promotion_tests`
- [ ] 100% coverage: `cargo llvm-cov --lib --text | grep "100.00%"`
- [ ] No compiler warnings: `cargo test 2>&1 | Select-String "warning"`
- [ ] Snapshots reviewed: `cargo insta review` (no pending changes)
- [ ] Execution time <5s: `Measure-Command { cargo test }`

### Pre-Push Checklist
- [ ] CI pipeline passes (GitHub Actions green)
- [ ] Coverage report uploaded to artifacts
- [ ] All snapshot files committed: `git status .snapshots/`
- [ ] No new panic tests without documentation

### Release Checklist
- [ ] All functional requirements validated (FR-001 to FR-010)
- [ ] Edge cases tested (4 categories: type boundaries, numeric boundaries, circular deps, resource exhaustion)
- [ ] Performance targets met (<5s suite, <100ms per test)
- [ ] Documentation up-to-date (rustdoc comments, quickstart.md)

---

## Quick Reference Commands

```powershell
# Essential Commands
cargo test --test ir_type_promotion_tests       # Run all tests
cargo llvm-cov --lib --html --open              # Coverage report
cargo insta review                              # Review snapshots

# Debugging Commands
cargo test <test_name> -- --nocapture           # Show output
cargo test --test ir_type_promotion_tests -- --show-output  # Timings
cargo clean && cargo test                        # Fresh build

# Validation Commands
cargo llvm-cov --lib --text | grep "type_promotion"  # Check coverage %
Measure-Command { cargo test }                   # Measure time
cargo test 2>&1 | Select-String "warning"       # Check warnings
```

---

## Next Steps

After quickstart validation:
1. **Extend tests**: Add more edge cases if coverage <100%
2. **Refactor**: Improve test naming, documentation, or organization
3. **Automate**: Add CI/CD workflow (see GitHub Actions example above)
4. **Monitor**: Track coverage trends over time (codecov.io, coveralls)

**Documentation Updates**:
- Update `README.md` with link to quickstart.md
- Add coverage badge to repository (shields.io or codecov)
- Document new test patterns in `research.md`

---

## Summary

This quickstart guide provides:
- ✅ **30-second validation**: Run tests, verify pass
- ✅ **60-second coverage**: Generate report, check 100%
- ✅ **Snapshot workflow**: Review and accept snapshots
- ✅ **Performance validation**: Measure execution time
- ✅ **CI/CD integration**: GitHub Actions example
- ✅ **Troubleshooting**: Common issues and solutions
- ✅ **Checklists**: Pre-commit, pre-push, release validation

**Estimated Time to Full Validation**: 2-3 minutes (tests + coverage + review)

**Contact**: For issues or questions, see `QWEN.md` or project maintainers.
