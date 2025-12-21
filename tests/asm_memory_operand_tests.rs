//! Comprehensive test suite for the `MemoryOperand` type in the `asm::instruction` module.
//!
//! This module tests all functionality related to memory addressing operands
//! including construction, builder pattern methods, display formatting with
//! various addressing modes, and edge cases with different sizes and displacements.

use jsavrs::asm::{GPRegister64, MemoryOperand};

#[test]
fn test_new_with_base_register() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax));

    assert_eq!(mem.base, Some(GPRegister64::Rax));
    assert_eq!(mem.index, None);
    assert_eq!(mem.scale, 1);
    assert_eq!(mem.displacement, 0);
    assert_eq!(mem.size, 8);
}

#[test]
fn test_new_with_none_base() {
    let mem = MemoryOperand::new(None);

    assert_eq!(mem.base, None);
    assert_eq!(mem.index, None);
    assert_eq!(mem.scale, 1);
    assert_eq!(mem.displacement, 0);
    assert_eq!(mem.size, 8);
}

#[test]
fn test_new_with_various_base_registers() {
    let registers = [
        GPRegister64::Rax,
        GPRegister64::Rbx,
        GPRegister64::Rcx,
        GPRegister64::Rdx,
        GPRegister64::Rsi,
        GPRegister64::Rdi,
        GPRegister64::Rbp,
        GPRegister64::Rsp,
        GPRegister64::R8,
        GPRegister64::R9,
        GPRegister64::R10,
        GPRegister64::R11,
        GPRegister64::R12,
        GPRegister64::R13,
        GPRegister64::R14,
        GPRegister64::R15,
    ];

    for reg in registers {
        let mem = MemoryOperand::new(Some(reg));
        assert_eq!(mem.base, Some(reg));
    }
}

#[test]
fn test_with_positive_displacement() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_displacement(16);

    assert_eq!(mem.displacement, 16);
    assert_eq!(mem.base, Some(GPRegister64::Rax));
}

#[test]
fn test_with_negative_displacement() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rbx)).with_displacement(-24);

    assert_eq!(mem.displacement, -24);
}

#[test]
fn test_with_zero_displacement() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rcx)).with_displacement(0);

    assert_eq!(mem.displacement, 0);
}

#[test]
fn test_with_max_i32_displacement() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rdx)).with_displacement(i32::MAX);

    assert_eq!(mem.displacement, i32::MAX);
}

#[test]
fn test_with_min_i32_displacement() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rsi)).with_displacement(i32::MIN);

    assert_eq!(mem.displacement, i32::MIN);
}

#[test]
fn test_with_displacement_chaining() {
    // Second call should override the first
    let mem = MemoryOperand::new(Some(GPRegister64::Rdi)).with_displacement(100).with_displacement(200);

    assert_eq!(mem.displacement, 200);
}

#[test]
fn test_with_index_scale_1() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_index(GPRegister64::Rcx, 1);

    assert_eq!(mem.index, Some(GPRegister64::Rcx));
    assert_eq!(mem.scale, 1);
}

#[test]
fn test_with_index_scale_2() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rbx)).with_index(GPRegister64::Rdx, 2);

    assert_eq!(mem.index, Some(GPRegister64::Rdx));
    assert_eq!(mem.scale, 2);
}

#[test]
fn test_with_index_scale_4() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rcx)).with_index(GPRegister64::Rsi, 4);

    assert_eq!(mem.index, Some(GPRegister64::Rsi));
    assert_eq!(mem.scale, 4);
}

#[test]
fn test_with_index_scale_8() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rdx)).with_index(GPRegister64::Rdi, 8);

    assert_eq!(mem.index, Some(GPRegister64::Rdi));
    assert_eq!(mem.scale, 8);
}

#[test]
fn test_with_index_preserves_base() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rbp)).with_index(GPRegister64::R8, 4);

    assert_eq!(mem.base, Some(GPRegister64::Rbp));
    assert_eq!(mem.index, Some(GPRegister64::R8));
}

#[test]
fn test_with_index_on_no_base() {
    let mem = MemoryOperand::new(None).with_index(GPRegister64::R9, 2);

    assert_eq!(mem.base, None);
    assert_eq!(mem.index, Some(GPRegister64::R9));
    assert_eq!(mem.scale, 2);
}

#[test]
fn test_with_size_1_byte() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(1);
    assert_eq!(mem.size, 1);
}

#[test]
fn test_with_size_2_bytes() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rbx)).with_size(2);
    assert_eq!(mem.size, 2);
}

#[test]
fn test_with_size_4_bytes() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rcx)).with_size(4);
    assert_eq!(mem.size, 4);
}

#[test]
fn test_with_size_8_bytes() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rdx)).with_size(8);
    assert_eq!(mem.size, 8);
}

#[test]
fn test_with_size_16_bytes_xmmword() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rsi)).with_size(16);
    assert_eq!(mem.size, 16);
}

#[test]
fn test_with_size_32_bytes_ymmword() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rdi)).with_size(32);
    assert_eq!(mem.size, 32);
}

#[test]
fn test_with_size_chaining_overrides() {
    let mem = MemoryOperand::new(Some(GPRegister64::R10)).with_size(4).with_size(8);
    assert_eq!(mem.size, 8);
}

#[test]
fn test_full_builder_chain() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax))
        .with_index(GPRegister64::Rcx, 4)
        .with_displacement(128)
        .with_size(4);

    assert_eq!(mem.base, Some(GPRegister64::Rax));
    assert_eq!(mem.index, Some(GPRegister64::Rcx));
    assert_eq!(mem.scale, 4);
    assert_eq!(mem.displacement, 128);
    assert_eq!(mem.size, 4);
}

#[test]
fn test_builder_chain_order_independence() {
    let mem1 =
        MemoryOperand::new(Some(GPRegister64::Rbx)).with_size(2).with_displacement(64).with_index(GPRegister64::Rdx, 2);

    let mem2 =
        MemoryOperand::new(Some(GPRegister64::Rbx)).with_index(GPRegister64::Rdx, 2).with_size(2).with_displacement(64);

    assert_eq!(mem1, mem2);
}

#[test]
fn test_complex_addressing_mode() {
    // [rbp + r12*8 - 256] with QWORD size
    let mem = MemoryOperand::new(Some(GPRegister64::Rbp))
        .with_index(GPRegister64::R12, 8)
        .with_displacement(-256)
        .with_size(8);

    assert_eq!(mem.base, Some(GPRegister64::Rbp));
    assert_eq!(mem.index, Some(GPRegister64::R12));
    assert_eq!(mem.scale, 8);
    assert_eq!(mem.displacement, -256);
    assert_eq!(mem.size, 8);
}

#[test]
fn test_display_byte_ptr_prefix() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(1);
    let output = format!("{mem}");
    assert!(output.starts_with("BYTE PTR "));
}

#[test]
fn test_display_word_ptr_prefix() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(2);
    let output = format!("{mem}");
    assert!(output.starts_with("WORD PTR "));
}

#[test]
fn test_display_dword_ptr_prefix() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(4);
    let output = format!("{mem}");
    assert!(output.starts_with("DWORD PTR "));
}

#[test]
fn test_display_qword_ptr_prefix() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(8);
    let output = format!("{mem}");
    assert!(output.starts_with("QWORD PTR "));
}

#[test]
fn test_display_xmmword_ptr_prefix() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(16);
    let output = format!("{mem}");
    assert!(output.starts_with("XMMWORD PTR "));
}

#[test]
fn test_display_ymmword_ptr_prefix() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(32);
    let output = format!("{mem}");
    assert!(output.starts_with("YMMWORD PTR "));
}

#[test]
fn test_display_unknown_size_no_prefix() {
    // Size 3 is not a standard size, should have no prefix
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(3);
    let output = format!("{mem}");
    assert!(output.starts_with('['));
}

#[test]
fn test_display_size_0_no_prefix() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(0);
    let output = format!("{mem}");
    assert!(output.starts_with('['));
}

#[test]
fn test_display_base_only() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(8);
    let output = format!("{mem}");
    assert_eq!(output, "QWORD PTR [rax]");
}

#[test]
fn test_display_base_with_positive_displacement() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rbx)).with_displacement(16).with_size(4);
    let output = format!("{mem}");
    assert_eq!(output, "DWORD PTR [rbx + 16]");
}

#[test]
fn test_display_base_with_negative_displacement() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rcx)).with_displacement(-8).with_size(4);
    let output = format!("{mem}");
    assert_eq!(output, "DWORD PTR [rcx - 8]");
}

#[test]
fn test_display_base_plus_index_scale_1() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_index(GPRegister64::Rcx, 1).with_size(8);
    let output = format!("{mem}");
    assert_eq!(output, "QWORD PTR [rax + rcx]");
}

#[test]
fn test_display_base_plus_index_with_scale() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_index(GPRegister64::Rcx, 4).with_size(8);
    let output = format!("{mem}");
    assert_eq!(output, "QWORD PTR [rax + rcx*4]");
}

#[test]
fn test_display_base_plus_index_scale_8_plus_displacement() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rbp))
        .with_index(GPRegister64::Rdi, 8)
        .with_displacement(128)
        .with_size(8);
    let output = format!("{mem}");
    assert_eq!(output, "QWORD PTR [rbp + rdi*8 + 128]");
}

#[test]
fn test_display_base_plus_index_with_negative_displacement() {
    let mem =
        MemoryOperand::new(Some(GPRegister64::Rsp)).with_index(GPRegister64::R8, 2).with_displacement(-32).with_size(4);
    let output = format!("{mem}");
    assert_eq!(output, "DWORD PTR [rsp + r8*2 - 32]");
}

#[test]
fn test_display_index_only_with_scale() {
    let mem = MemoryOperand::new(None).with_index(GPRegister64::R9, 4).with_size(8);
    let output = format!("{mem}");
    assert_eq!(output, "QWORD PTR [r9*4]");
}

#[test]
fn test_display_index_only_scale_1() {
    let mem = MemoryOperand::new(None).with_index(GPRegister64::R10, 1).with_size(8);
    let output = format!("{mem}");
    assert_eq!(output, "QWORD PTR [r10]");
}

#[test]
fn test_display_displacement_only() {
    let mem = MemoryOperand::new(None).with_displacement(0x1000).with_size(4);
    let output = format!("{mem}");
    assert_eq!(output, "DWORD PTR [4096]");
}

#[test]
fn test_display_negative_displacement_only() {
    let mem = MemoryOperand::new(None).with_displacement(-100).with_size(1);
    let output = format!("{mem}");
    assert_eq!(output, "BYTE PTR [-100]");
}

#[test]
fn test_display_empty_addressing_shows_zero() {
    let mem = MemoryOperand::new(None).with_size(8);
    let output = format!("{mem}");
    assert_eq!(output, "QWORD PTR [0]");
}

#[test]
fn test_display_with_r8_register() {
    let mem = MemoryOperand::new(Some(GPRegister64::R8)).with_size(8);
    let output = format!("{mem}");
    assert!(output.contains("r8"));
}

#[test]
fn test_display_with_r15_register() {
    let mem = MemoryOperand::new(Some(GPRegister64::R15)).with_size(8);
    let output = format!("{mem}");
    assert!(output.contains("r15"));
}

#[test]
fn test_display_with_rsp_base() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rsp)).with_displacement(8).with_size(8);
    let output = format!("{mem}");
    assert!(output.contains("rsp"));
}

#[test]
fn test_display_with_rbp_base_typical_stack_access() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rbp)).with_displacement(-16).with_size(8);
    let output = format!("{mem}");
    assert_eq!(output, "QWORD PTR [rbp - 16]");
}

#[test]
fn test_memory_operand_clone() {
    let mem1 = MemoryOperand::new(Some(GPRegister64::Rax))
        .with_index(GPRegister64::Rcx, 4)
        .with_displacement(100)
        .with_size(4);
    let mem2 = mem1.clone();

    assert_eq!(mem1, mem2);
}

#[test]
fn test_memory_operand_eq_identical() {
    let mem1 = MemoryOperand::new(Some(GPRegister64::Rbx)).with_displacement(50);
    let mem2 = MemoryOperand::new(Some(GPRegister64::Rbx)).with_displacement(50);

    assert_eq!(mem1, mem2);
}

#[test]
fn test_memory_operand_neq_different_base() {
    let mem1 = MemoryOperand::new(Some(GPRegister64::Rax));
    let mem2 = MemoryOperand::new(Some(GPRegister64::Rbx));

    assert_ne!(mem1, mem2);
}

#[test]
fn test_memory_operand_neq_different_displacement() {
    let mem1 = MemoryOperand::new(Some(GPRegister64::Rax)).with_displacement(10);
    let mem2 = MemoryOperand::new(Some(GPRegister64::Rax)).with_displacement(20);

    assert_ne!(mem1, mem2);
}

#[test]
fn test_memory_operand_neq_different_index() {
    let mem1 = MemoryOperand::new(Some(GPRegister64::Rax)).with_index(GPRegister64::Rcx, 4);
    let mem2 = MemoryOperand::new(Some(GPRegister64::Rax)).with_index(GPRegister64::Rdx, 4);

    assert_ne!(mem1, mem2);
}

#[test]
fn test_memory_operand_neq_different_scale() {
    let mem1 = MemoryOperand::new(Some(GPRegister64::Rax)).with_index(GPRegister64::Rcx, 2);
    let mem2 = MemoryOperand::new(Some(GPRegister64::Rax)).with_index(GPRegister64::Rcx, 4);

    assert_ne!(mem1, mem2);
}

#[test]
fn test_memory_operand_neq_different_size() {
    let mem1 = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(4);
    let mem2 = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(8);

    assert_ne!(mem1, mem2);
}

#[test]
fn test_memory_operand_debug_contains_base() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax));
    let debug_str = format!("{mem:?}");

    assert!(debug_str.contains("MemoryOperand"));
    assert!(debug_str.contains("base"));
    assert!(debug_str.contains("Rax"));
}

#[test]
fn test_memory_operand_debug_contains_fields() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rbx))
        .with_index(GPRegister64::Rcx, 4)
        .with_displacement(128)
        .with_size(4);
    let debug_str = format!("{mem:?}");

    assert!(debug_str.contains("index"));
    assert!(debug_str.contains("scale"));
    assert!(debug_str.contains("displacement"));
    assert!(debug_str.contains("size"));
}

#[test]
fn test_max_displacement_display() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_displacement(i32::MAX).with_size(8);
    let output = format!("{mem}");

    assert!(output.contains(&i32::MAX.to_string()));
}

#[test]
fn test_min_displacement_display() {
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_displacement(i32::MIN).with_size(8);
    let output = format!("{mem}");

    // i32::MIN is -2147483648, displayed as subtraction: " - 2147483648"
    // Actually, the code checks `displacement > 0` for positive, else shows negative
    assert!(output.contains("2147483648"));
}

#[test]
fn test_all_scale_values() {
    for scale in [1u8, 2, 4, 8] {
        let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_index(GPRegister64::Rcx, scale);
        let output = format!("{mem}");

        if scale == 1 {
            // Scale 1 should not show "*1"
            assert!(!output.contains("*1"));
        } else {
            assert!(output.contains(&format!("*{scale}")));
        }
    }
}

#[test]
fn test_unusual_scale_value() {
    // Scale value 3 is unusual but should still work
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_index(GPRegister64::Rcx, 3);
    let output = format!("{mem}");

    assert!(output.contains("*3"));
}

#[test]
fn test_index_with_displacement_no_base() {
    let mem = MemoryOperand::new(None).with_index(GPRegister64::R11, 8).with_displacement(1024).with_size(8);
    let output = format!("{mem}");

    assert_eq!(output, "QWORD PTR [r11*8 + 1024]");
}

#[test]
fn test_large_size_value() {
    // Size 64 (512-bit ZMM) - no specific prefix
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(64);
    let output = format!("{mem}");

    // Should have no prefix for size 64
    assert!(output.starts_with('['));
}

#[test]
fn test_repeated_builder_calls_use_last_value() {
    let mem =
        MemoryOperand::new(Some(GPRegister64::Rax)).with_displacement(10).with_displacement(20).with_displacement(30);

    assert_eq!(mem.displacement, 30);
}

#[test]
fn test_stack_local_variable_access() {
    // Typical: mov eax, [rbp - 4]
    let mem = MemoryOperand::new(Some(GPRegister64::Rbp)).with_displacement(-4).with_size(4);
    let output = format!("{mem}");

    assert_eq!(output, "DWORD PTR [rbp - 4]");
}

#[test]
fn test_array_access_pattern() {
    // Typical: mov rax, [rdi + rsi*8]
    let mem = MemoryOperand::new(Some(GPRegister64::Rdi)).with_index(GPRegister64::Rsi, 8).with_size(8);
    let output = format!("{mem}");

    assert_eq!(output, "QWORD PTR [rdi + rsi*8]");
}

#[test]
fn test_struct_member_access() {
    // Access struct member at offset 16: mov eax, [rdi + 16]
    let mem = MemoryOperand::new(Some(GPRegister64::Rdi)).with_displacement(16).with_size(4);
    let output = format!("{mem}");

    assert_eq!(output, "DWORD PTR [rdi + 16]");
}

#[test]
fn test_array_of_structs_access() {
    // Access: array[index].field where struct size is 24, field offset is 8
    // Address: base + index*24 + 8 (but x86 only supports 1,2,4,8 scales)
    // Simplified pattern: [rax + rcx*8 + 16]
    let mem =
        MemoryOperand::new(Some(GPRegister64::Rax)).with_index(GPRegister64::Rcx, 8).with_displacement(16).with_size(4);
    let output = format!("{mem}");

    assert_eq!(output, "DWORD PTR [rax + rcx*8 + 16]");
}

#[test]
#[allow(clippy::unreadable_literal)]
fn test_global_variable_access() {
    // Absolute address access: mov rax, [0x401000]
    let mem = MemoryOperand::new(None).with_displacement(0x401000).with_size(8);
    let output = format!("{mem}");

    assert_eq!(output, "QWORD PTR [4198400]");
}

#[test]
fn test_byte_array_element() {
    // Access byte at array[i]: mov al, [rdi + rsi]
    let mem = MemoryOperand::new(Some(GPRegister64::Rdi)).with_index(GPRegister64::Rsi, 1).with_size(1);
    let output = format!("{mem}");

    assert_eq!(output, "BYTE PTR [rdi + rsi]");
}

#[test]
fn test_simd_aligned_access() {
    // Aligned 16-byte load: movaps xmm0, [rax]
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(16);
    let output = format!("{mem}");

    assert_eq!(output, "XMMWORD PTR [rax]");
}

#[test]
fn test_avx_aligned_access() {
    // Aligned 32-byte load: vmovaps ymm0, [rax]
    let mem = MemoryOperand::new(Some(GPRegister64::Rax)).with_size(32);
    let output = format!("{mem}");

    assert_eq!(output, "YMMWORD PTR [rax]");
}
