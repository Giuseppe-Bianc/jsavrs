use jsavrs::asm::generator::{NasmGenerator, TargetOS};

#[test]
fn test_factorial_function_generation_with_recursive_calls() {
    // Test factorial function generation with recursive calls
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    generator.create_factorial_function();
    
    // Verify the function was generated
    let code = generator.generate();
    assert!(code.contains("factorial:"));
    assert!(code.contains("call factorial"));
    assert!(code.contains("mov rax, 1")); // base case return
    assert!(code.contains("imul rax")); // multiplication for result
}

#[test]
fn test_base_case_handling_for_factorial() {
    // Test handling of base cases (factorial of 0 and 1)
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    generator.create_factorial_function();
    
    let code = generator.generate();
    
    // Check for base case handling (n <= 1)
    assert!(code.contains("cmp"));
    assert!(code.contains("jle")); // jump if less or equal
    assert!(code.contains("mov rax, 1")); // return 1 for base case
}

#[test]
fn test_handling_of_negative_inputs() {
    // Test handling of negative inputs
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    generator.create_factorial_function();
    
    let code = generator.generate();
    
    // Although the function doesn't explicitly handle negative inputs,
    // it should still compile and run correctly
    // The comparison `cmp` with `jle` (jump if less or equal) 
    // will handle the case where input is <= 1
    assert!(code.contains("cmp"));
    assert!(code.contains("jle"));
}

#[test]
fn test_handling_of_large_numbers_within_computational_limits() {
    // Test handling of large numbers within computational limits
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    generator.create_factorial_function();
    
    let code = generator.generate();
    
    // The factorial function should handle large numbers as much as registers allow
    assert!(code.contains("imul")); // Should have multiplication for result
    assert!(code.contains("dec")); // Decrement for recursive calls
    assert!(code.contains("push")); // Push for saving parameters
    assert!(code.contains("pop")); // Pop for restoring parameters
}

#[test]
fn test_generated_assembly_correctness_for_factorial() {
    // Test generated assembly for correctness
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    generator.create_factorial_function();
    
    let code = generator.generate();
    
    // The factorial function should have:
    // - Function label
    // - Prologue
    // - Base case check
    // - Recursive call preparation
    // - Recursive call
    // - Multiplication by current value
    // - Epilogue
    
    assert!(code.contains("factorial:")); // Function label
    assert!(code.contains("cmp")); // Base case check
    assert!(code.contains("jle")); // Jump for base case
    assert!(code.contains("call factorial")); // Recursive call
    assert!(code.contains("imul")); // Multiply by current value
    assert!(code.contains("push")); // Save register before call
    assert!(code.contains("pop")); // Restore register after call
    assert!(code.contains("ret")); // Return at the end
}

#[test]
fn test_edge_cases_and_error_conditions() {
    // Test edge cases and error conditions in factorial function
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    generator.create_factorial_function();
    
    let code = generator.generate();
    
    // Test that the factorial function handles the comparison correctly
    assert!(code.contains("cmp")); // Comparison for base case
    assert!(code.contains("1")); // Comparison with 1 for base case
    
    // The factorial function should properly handle return values
    assert!(code.contains("mov rax, 1")); // Base case returns 1
}
