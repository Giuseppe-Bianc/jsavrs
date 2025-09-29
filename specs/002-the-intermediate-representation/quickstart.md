# Quickstart Guide: IR Type Promotion System

**Feature**: IR Type Promotion System Correction  
**Date**: 29 settembre 2025  
**Status**: Phase 1 Design Complete

## Quick Setup

### Prerequisites
- Rust 1.90.0 or later
- jsavrs development environment set up
- Working knowledge of IR concepts

### 5-Minute Test Drive

**Step 1: Verify Current Behavior (Broken)**
```bash
cd /path/to/jsavrs
echo 'main() { 
    var x: i32 = 10;
    var y: f32 = 3.14;
    var result = x + y;  // Should be f32, but currently returns i32
}' > test_promotion.vn

cargo run -- compile test_promotion.vn --emit-ir
```

**Expected Current Output** (demonstrating the bug):
```ir
%t1 = add i32 %x, %y    ; ❌ Wrong: result is i32, should be f32
```

**Step 2: After Implementation Test**
```bash
# Same input file
cargo run -- compile test_promotion.vn --emit-ir --enable-type-promotion
```

**Expected Fixed Output**:
```ir
%t0 = cast i32 %x to f32    ; ✅ Correct: explicit cast inserted
%t1 = add f32 %t0, %y       ; ✅ Correct: result is f32
```

### Integration Test Scenarios

**Scenario 1: Integer + Float Promotion**

*Input*:
```javascript
main() {
    var a: i32 = 42;
    var b: f32 = 3.14;
    var sum = a + b;
    var product = b * a;
}
```

*Expected IR Output*:
```ir
; sum calculation
%t0 = cast i32 %a to f32    ; i32 -> f32 cast
%t1 = add f32 %t0, %b       ; f32 result

; product calculation  
%t2 = cast i32 %a to f32    ; i32 -> f32 cast
%t3 = mul f32 %b, %t2       ; f32 result
```

*Validation*:
- ✅ Both operations result in f32 type
- ✅ Explicit casts inserted for i32 operands
- ✅ No precision loss warnings (i32 fits in f32)

**Scenario 2: Signed/Unsigned Integer Mixing**

*Input*:
```javascript
main() {
    var signed: i32 = -100;
    var unsigned: u32 = 200;
    var result = signed + unsigned;
}
```

*Expected IR Output*:
```ir
%t0 = cast i32 %signed to i64      ; i32 -> i64 cast
%t1 = cast u32 %unsigned to i64    ; u32 -> i64 cast  
%t2 = add i64 %t0, %t1             ; i64 result
```

*Expected Warnings*:
```
warning: mixing signed and unsigned integers of same width
  --> test.vn:4:18
   |
4  |     var result = signed + unsigned;
   |                  ^^^^^^ ^^^^^^^^ promoted to i64 to prevent overflow
```

*Validation*:
- ✅ Result type is i64 (next larger signed type)
- ✅ Both operands cast to i64
- ✅ Warning generated for potential confusion

**Scenario 3: Complex Expression with Multiple Promotions**

*Input*:
```javascript
main() {
    var a: i16 = 10;
    var b: u32 = 20;
    var c: f64 = 3.14159;
    var result = (a * b) + c;
}
```

*Expected IR Output*:
```ir
; First: a * b (i16 * u32 -> i64)
%t0 = cast i16 %a to i64      ; i16 -> i64
%t1 = cast u32 %b to i64      ; u32 -> i64
%t2 = mul i64 %t0, %t1        ; i64 intermediate result

; Second: (i64) + f64 -> f64
%t3 = cast i64 %t2 to f64     ; i64 -> f64  
%t4 = add f64 %t3, %c         ; f64 final result
```

*Validation*:
- ✅ Correct promotion precedence (float > integer)
- ✅ Signed/unsigned handling in intermediate calculation
- ✅ No precision loss in i64 -> f64 conversion

**Scenario 4: Special Float Values**

*Input*:
```javascript
main() {
    var nan_val: f32 = 0.0 / 0.0;
    var int_val: i32 = 42;
    var result = nan_val + int_val;
}
```

*Expected IR Output*:
```ir
%t0 = div f32 0.0, 0.0        ; NaN generation
%t1 = cast i32 %int_val to f32 ; i32 -> f32
%t2 = add f32 %t0, %t1        ; NaN + 42.0 = NaN
```

*Expected Warnings*:
```
warning: operation may produce NaN or infinity
  --> test.vn:4:18
   |
4  |     var result = nan_val + int_val;
   |                  ^^^^^^^ special float value detected
```

*Validation*:
- ✅ NaN handling preserved through promotion
- ✅ Warning for special float values
- ✅ IEEE 754 compliance maintained

### Performance Validation

**Test Large Expression Trees**:
```bash
# Generate test with 1000 mixed-type operations
python3 scripts/generate_promotion_test.py --size 1000 > large_test.vn

# Measure compilation time
time cargo run --release -- compile large_test.vn --emit-ir

# Should complete in reasonable time (< 5 seconds)
```

### Regression Test Validation

**Test Historical Compatibility**:
```bash
# Run existing test suite to ensure no regressions
cargo test

# Run specific promotion tests
cargo test type_promotion

# Run snapshot tests to verify IR output
cargo test --test ir_promotion_snapshots
```

### Error Handling Validation

**Test Invalid Promotions**:

*Input*:
```javascript
main() {
    var str_val: string = "hello";
    var int_val: i32 = 42;
    var result = str_val + int_val;  // Should error
}
```

*Expected Error*:
```
error: cannot promote incompatible types
  --> test.vn:4:18
   |
4  |     var result = str_val + int_val;
   |                  ^^^^^^^ ^^^^^^^ string and i32 are not compatible for arithmetic
   |
help: consider using explicit conversion functions
```

*Validation*:
- ✅ Clear error message with location
- ✅ Helpful suggestion for resolution
- ✅ Compilation fails gracefully

### Manual Testing Checklist

**Phase 1: Basic Functionality**
- [ ] Integer + Float operations promote to float
- [ ] Signed + Unsigned operations promote to larger signed
- [ ] Cast instructions appear in generated IR
- [ ] Warnings generated for precision loss

**Phase 2: Edge Cases**
- [ ] NaN and infinity handling
- [ ] Overflow scenarios
- [ ] Complex expression trees
- [ ] Error recovery for invalid operations

**Phase 3: Integration**
- [ ] Backend accepts promoted IR
- [ ] Assembly generation works correctly
- [ ] Debug information preserved
- [ ] Cross-platform consistency

**Phase 4: Performance**
- [ ] Compilation time acceptable (< 10% increase)
- [ ] Generated code quality maintained
- [ ] Memory usage reasonable
- [ ] No performance regressions in existing code

### Troubleshooting

**Common Issues**:

1. **Cast Instructions Missing**
   - Check promotion matrix configuration
   - Verify `analyze_binary_promotion` is called
   - Ensure cast insertion logic is active

2. **Wrong Result Types**
   - Review type lattice ordering
   - Check `compute_common_type` implementation  
   - Verify promotion rules are correct

3. **Compilation Errors**
   - Check error handling in promotion analysis
   - Verify IR instruction generation
   - Review integration with existing generator

4. **Performance Issues**
   - Profile promotion matrix lookups
   - Check for unnecessary cast generations
   - Optimize common promotion paths

### Next Steps

After completing quickstart validation:

1. **Run Full Test Suite**: `cargo test`
2. **Performance Benchmarking**: `cargo bench`
3. **Documentation Review**: Verify all changes documented
4. **Integration Testing**: Test with real-world programs

This quickstart guide provides a comprehensive validation path for the type promotion system, ensuring all requirements are met and the implementation works correctly in both common and edge case scenarios.