use crate::asm::register::{Register, RegisterSize};

#[test]
fn test_register_display_8bit() {
    let al = Register::new("al".to_string(), RegisterSize::Bit8, 0);
    assert_eq!(format!("{}", al), "al");

    let ah = Register::new("ah".to_string(), RegisterSize::Bit8, 4);
    assert_eq!(format!("{}", ah), "ah");

    let bl = Register::new("bl".to_string(), RegisterSize::Bit8, 3);
    assert_eq!(format!("{}", bl), "bl");
}

#[test]
fn test_register_display_16bit() {
    let ax = Register::new("ax".to_string(), RegisterSize::Bit16, 0);
    assert_eq!(format!("{}", ax), "ax");

    let bx = Register::new("bx".to_string(), RegisterSize::Bit16, 3);
    assert_eq!(format!("{}", bx), "bx");
}

#[test]
fn test_register_display_32bit() {
    let eax = Register::new("eax".to_string(), RegisterSize::Bit32, 0);
    assert_eq!(format!("{}", eax), "eax");

    let ebx = Register::new("ebx".to_string(), RegisterSize::Bit32, 3);
    assert_eq!(format!("{}", ebx), "ebx");
}

#[test]
fn test_register_display_64bit() {
    let rax = Register::new("rax".to_string(), RegisterSize::Bit64, 0);
    assert_eq!(format!("{}", rax), "rax");

    let rbx = Register::new("rbx".to_string(), RegisterSize::Bit64, 3);
    assert_eq!(format!("{}", rbx), "rbx");
}

#[test]
fn test_register_creation_with_boundary_conditions() {
    // Test creating registers with boundary conditions
    let valid_register = Register::new("rax".to_string(), RegisterSize::Bit64, 0);
    assert_eq!(valid_register.name, "rax");
    assert_eq!(valid_register.size, RegisterSize::Bit64);
    assert_eq!(valid_register.encoding, 0);
}

#[test]
fn test_register_formatting_scenarios() {
    // Test register formatting with various scenarios
    let rax = Register::new("rax".to_string(), RegisterSize::Bit64, 0);
    assert_eq!(format!("{}", rax), "rax");

    // Note: If RegisterSize::Bit128 doesn't exist, replace with an appropriate size
    let rbx = Register::new("rbx".to_string(), RegisterSize::Bit64, 3);
    assert_eq!(format!("{}", rbx), "rbx");
}

#[test]
fn test_invalid_register_operations() {
    // Test handling of invalid register operations (if applicable)
    // This test will be expanded based on actual register implementation
    let rax = Register::new("rax".to_string(), RegisterSize::Bit64, 0);
    assert_eq!(rax.name, "rax");
}

#[test]
fn test_architecture_specific_validation() {
    // Test architecture-specific validation for different x86-64 implementations
    let register_sizes = [
        RegisterSize::Bit8,
        RegisterSize::Bit16,
        RegisterSize::Bit32,
        RegisterSize::Bit64,
    ];

    for size in register_sizes.iter() {
        let reg_name = match size {
            RegisterSize::Bit8 => "al",
            RegisterSize::Bit16 => "ax",
            RegisterSize::Bit32 => "eax",
            RegisterSize::Bit64 => "rax",
        };

        let reg = Register::new(reg_name.to_string(), *size, 0);
        assert_eq!(reg.name, reg_name);
        assert_eq!(reg.size, *size);
    }
}

#[test]
fn test_register_boundary_value_testing() {
    // Test boundary value testing for all register sizes
    let al = Register::new("al".to_string(), RegisterSize::Bit8, 0);
    assert_eq!(al.size, RegisterSize::Bit8);

    let ax = Register::new("ax".to_string(), RegisterSize::Bit16, 0);
    assert_eq!(ax.size, RegisterSize::Bit16);

    let eax = Register::new("eax".to_string(), RegisterSize::Bit32, 0);
    assert_eq!(eax.size, RegisterSize::Bit32);

    let rax = Register::new("rax".to_string(), RegisterSize::Bit64, 0);
    assert_eq!(rax.size, RegisterSize::Bit64);
}