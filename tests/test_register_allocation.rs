//! Unit tests for register allocation algorithms
//! Based on: T038 [P] Unit tests for register allocation algorithms in tests/test_register_allocation.rs

use jsavrs::asm::register::{RegisterAllocator, Register, GPRegister, /*XMMRegister,*/ RegisterInfo};

#[test]
fn test_register_allocator_creation() {
    let allocator = RegisterAllocator::new();
    
    // Check that allocator has initial registers available
    assert!(!allocator.available_registers.is_empty());
    assert!(allocator.allocated_map.is_empty());
    assert_eq!(allocator.spill_location, 0);
}

#[test]
fn test_register_allocation() {
    let mut allocator = RegisterAllocator::new();
    
    // Try to allocate a register
    let allocated = allocator.allocate_register("test_var");
    assert!(allocated.is_some());
    
    let reg = allocated.unwrap();
    assert!(allocator.allocated_map.contains_key("test_var"));
    assert_eq!(allocator.allocated_map.get("test_var"), Some(&reg));
}

#[test]
fn test_register_freeing() {
    let mut allocator = RegisterAllocator::new();
    
    // Allocate and then free a register
    let reg = allocator.allocate_register("test_var").unwrap();
    assert!(allocator.allocated_map.contains_key("test_var"));
    
    // Freeing a register should make it available again
    allocator.free_register(reg);
    
    // The register should now be in the available list
    assert!(allocator.available_registers.contains(&reg));
}

#[test]
fn test_register_spilling() {
    let mut allocator = RegisterAllocator::new();
    
    // Allocate a register
    let _reg = allocator.allocate_register("test_var").unwrap();
    assert!(allocator.allocated_map.contains_key("test_var"));
    
    // Spill the register
    let spill_location = allocator.spill_to_stack("test_var");
    assert_eq!(spill_location, 0);
    assert!(!allocator.allocated_map.contains_key("test_var"));
    assert_eq!(allocator.spill_location, 1);
}

#[test]
fn test_register_info_traits() {
    let allocator = RegisterAllocator::new();
    
    // Test that allocator implements RegisterInfo trait
    assert!(!allocator.available_gp_registers().is_empty());
    assert!(!allocator.available_xmm_registers().is_empty());
    
    // Test caller-saved register detection
    assert!(allocator.is_caller_saved(Register::GP(GPRegister::RAX)));
    assert!(allocator.is_caller_saved(Register::GP(GPRegister::RCX)));
    
    // Test callee-saved register detection
    assert!(allocator.is_callee_saved(Register::GP(GPRegister::RBX)));
    assert!(allocator.is_callee_saved(Register::GP(GPRegister::RBP)));
}

#[test]
fn test_register_allocation_statistics() {
    let mut allocator = RegisterAllocator::new();
    
    // Initially no registers used
    assert_eq!(allocator.get_stats().registers_used, 0);
    
    // After allocation
    let _reg = allocator.allocate_register("test_var").unwrap();
    assert_eq!(allocator.get_stats().registers_used, 1);
}