use jsavrs::ir::data_layout::{DataLayout, Endianness, Mangling, ParsedDataLayout, PointerLayout};

// ============================================================================
// BASIC ENUM TESTS
// ============================================================================

#[test]
fn test_endianness_equality() {
    assert_eq!(Endianness::Little, Endianness::Little);
    assert_eq!(Endianness::Big, Endianness::Big);
    assert_ne!(Endianness::Little, Endianness::Big);
}

#[test]
fn test_endianness_debug() {
    let little = format!("{:?}", Endianness::Little);
    let big = format!("{:?}", Endianness::Big);
    assert!(little.contains("Little"));
    assert!(big.contains("Big"));
}

#[test]
fn test_mangling_equality() {
    assert_eq!(Mangling::Elf, Mangling::Elf);
    assert_eq!(Mangling::MachO, Mangling::MachO);
    assert_eq!(Mangling::Coff, Mangling::Coff);
    assert_ne!(Mangling::Elf, Mangling::MachO);
}

#[test]
fn test_mangling_unknown() {
    let unknown1 = Mangling::Unknown("custom".to_string());
    let unknown2 = Mangling::Unknown("custom".to_string());
    let unknown3 = Mangling::Unknown("different".to_string());

    assert_eq!(unknown1, unknown2);
    assert_ne!(unknown1, unknown3);
}

// ============================================================================
// DATA LAYOUT STRING CONSTANTS TESTS
// ============================================================================

#[test]
fn test_data_layout_as_str_linux_x86_64() {
    let layout = DataLayout::LinuxX86_64;
    assert_eq!(layout.as_str(), "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128");
}

#[test]
fn test_data_layout_as_str_linux_aarch64() {
    let layout = DataLayout::LinuxAArch64;
    assert_eq!(layout.as_str(), "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128");
}

#[test]
fn test_data_layout_as_str_windows_x86_64() {
    let layout = DataLayout::WindowsX86_64;
    assert_eq!(layout.as_str(), "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128");
}

#[test]
fn test_data_layout_as_str_macos_x86_64() {
    let layout = DataLayout::MacOSX86_64;
    assert_eq!(layout.as_str(), "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128");
}

#[test]
fn test_data_layout_as_str_bsd_variants() {
    // All BSD variants should use ELF mangling
    assert_eq!(
        DataLayout::FreeBSDX86_64.as_str(),
        "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    );
    assert_eq!(
        DataLayout::NetBSDX86_64.as_str(),
        "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    );
    assert_eq!(
        DataLayout::OpenBSDX86_64.as_str(),
        "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    );
    assert_eq!(
        DataLayout::DragonFlyX86_64.as_str(),
        "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    );
}

#[test]
fn test_data_layout_display() {
    let layout = DataLayout::LinuxX86_64;
    let displayed = format!("{layout}");
    assert_eq!(displayed, layout.as_str());
}

#[test]
fn test_data_layout_equality() {
    assert_eq!(DataLayout::LinuxX86_64, DataLayout::LinuxX86_64);
    assert_ne!(DataLayout::LinuxX86_64, DataLayout::WindowsX86_64);
    assert_ne!(DataLayout::LinuxX86_64, DataLayout::MacOSX86_64);
}

#[test]
fn test_data_layout_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(DataLayout::LinuxX86_64);
    set.insert(DataLayout::WindowsX86_64);
    set.insert(DataLayout::LinuxX86_64); // Duplicate

    assert_eq!(set.len(), 2);
}

// ============================================================================
// PARSING TESTS - ENDIANNESS
// ============================================================================

#[test]
fn test_parse_little_endian() {
    let result = ParsedDataLayout::parse("e").unwrap();
    assert_eq!(result.endianness(), Endianness::Little);
}

#[test]
fn test_parse_big_endian() {
    let result = ParsedDataLayout::parse("E").unwrap();
    assert_eq!(result.endianness(), Endianness::Big);
}

#[test]
fn test_parse_endianness_default() {
    // If no endianness specified, should default to little endian
    let result = ParsedDataLayout::parse("").unwrap();
    assert_eq!(result.endianness(), Endianness::Little);
}

// ============================================================================
// PARSING TESTS - MANGLING
// ============================================================================

#[test]
fn test_parse_mangling_elf() {
    let result = ParsedDataLayout::parse("e-m:e").unwrap();
    assert_eq!(result.mangling(), Some(&Mangling::Elf));
}

#[test]
fn test_parse_mangling_macho() {
    let result = ParsedDataLayout::parse("e-m:o").unwrap();
    assert_eq!(result.mangling(), Some(&Mangling::MachO));
}

#[test]
fn test_parse_mangling_coff() {
    let result = ParsedDataLayout::parse("e-m:w").unwrap();
    assert_eq!(result.mangling(), Some(&Mangling::Coff));
}

#[test]
fn test_parse_mangling_unknown() {
    let result = ParsedDataLayout::parse("e-m:custom").unwrap();
    match result.mangling() {
        Some(Mangling::Unknown(s)) => assert_eq!(s, "custom"),
        _ => panic!("Expected Unknown mangling"),
    }
}

#[test]
fn test_parse_mangling_missing() {
    let result = ParsedDataLayout::parse("e").unwrap();
    assert_eq!(result.mangling(), None);
}

// ============================================================================
// PARSING TESTS - POINTERS
// ============================================================================

#[test]
fn test_parse_pointer_default_address_space() {
    let result = ParsedDataLayout::parse("e-p:64:64").unwrap();
    let ptr = result.pointer_layouts().get(&0).unwrap();

    assert_eq!(ptr.address_space, 0);
    assert_eq!(ptr.size_bits, 64);
    assert_eq!(ptr.abi_align_bits, 64);
    assert_eq!(ptr.pref_align_bits, None);
}

#[test]
fn test_parse_pointer_with_preferred_alignment() {
    let result = ParsedDataLayout::parse("e-p:64:64:128").unwrap();
    let ptr = result.pointer_layouts().get(&0).unwrap();

    assert_eq!(ptr.size_bits, 64);
    assert_eq!(ptr.abi_align_bits, 64);
    assert_eq!(ptr.pref_align_bits, Some(128));
}

#[test]
fn test_parse_pointer_custom_address_space() {
    let result = ParsedDataLayout::parse("e-p270:32:32").unwrap();
    let ptr = result.pointer_layouts().get(&270).unwrap();

    assert_eq!(ptr.address_space, 270);
    assert_eq!(ptr.size_bits, 32);
    assert_eq!(ptr.abi_align_bits, 32);
}

#[test]
fn test_parse_multiple_pointer_address_spaces() {
    let result = ParsedDataLayout::parse("e-p:64:64-p270:32:32-p271:32:32-p272:64:64").unwrap();

    assert_eq!(result.pointer_layouts().len(), 4);
    assert!(result.pointer_layouts().contains_key(&0));
    assert!(result.pointer_layouts().contains_key(&270));
    assert!(result.pointer_layouts().contains_key(&271));
    assert!(result.pointer_layouts().contains_key(&272));
}

#[test]
fn test_parse_pointer_invalid_format() {
    // Missing size and alignment
    let result = ParsedDataLayout::parse("e-p:64");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid pointer layout"));
}

#[test]
fn test_parse_pointer_invalid_number() {
    let result = ParsedDataLayout::parse("e-p:abc:64");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid size"));
}

// ============================================================================
// PARSING TESTS - INTEGERS
// ============================================================================

#[test]
fn test_parse_integer_i8() {
    let result = ParsedDataLayout::parse("e-i8:8").unwrap();
    let int = result.integer_layouts().get(&8).unwrap();

    assert_eq!(int.size_bits, 8);
    assert_eq!(int.abi_align_bits, 8);
    assert_eq!(int.pref_align_bits, None);
}

#[test]
fn test_parse_integer_i64() {
    let result = ParsedDataLayout::parse("e-i64:64").unwrap();
    let int = result.integer_layouts().get(&64).unwrap();

    assert_eq!(int.size_bits, 64);
    assert_eq!(int.abi_align_bits, 64);
}

#[test]
fn test_parse_integer_with_preferred() {
    let result = ParsedDataLayout::parse("e-i32:32:64").unwrap();
    let int = result.integer_layouts().get(&32).unwrap();

    assert_eq!(int.size_bits, 32);
    assert_eq!(int.abi_align_bits, 32);
    assert_eq!(int.pref_align_bits, Some(64));
}

#[test]
fn test_parse_multiple_integers() {
    let result = ParsedDataLayout::parse("e-i8:8-i16:16-i32:32-i64:64").unwrap();

    assert_eq!(result.integer_layouts().len(), 4);
    assert!(result.integer_layouts().contains_key(&8));
    assert!(result.integer_layouts().contains_key(&16));
    assert!(result.integer_layouts().contains_key(&32));
    assert!(result.integer_layouts().contains_key(&64));
}

#[test]
fn test_parse_integer_i128() {
    let result = ParsedDataLayout::parse("e-i128:128").unwrap();
    let int = result.integer_layouts().get(&128).unwrap();

    assert_eq!(int.size_bits, 128);
    assert_eq!(int.abi_align_bits, 128);
}

#[test]
fn test_parse_integer_invalid_format() {
    let result = ParsedDataLayout::parse("e-i64");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid integer layout"));
}

#[test]
fn test_parse_integer_invalid_number() {
    let result = ParsedDataLayout::parse("e-i64:xyz");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid ABI"));
}

// ============================================================================
// PARSING TESTS - FLOATS
// ============================================================================

#[test]
fn test_parse_float_f32() {
    let result = ParsedDataLayout::parse("e-f32:32").unwrap();
    let float = result.float_layouts().get(&32).unwrap();

    assert_eq!(float.size_bits, 32);
    assert_eq!(float.abi_align_bits, 32);
    assert_eq!(float.pref_align_bits, None);
}

#[test]
fn test_parse_float_f64() {
    let result = ParsedDataLayout::parse("e-f64:64").unwrap();
    let float = result.float_layouts().get(&64).unwrap();

    assert_eq!(float.size_bits, 64);
    assert_eq!(float.abi_align_bits, 64);
}

#[test]
fn test_parse_float_f80_with_preferred() {
    let result = ParsedDataLayout::parse("e-f80:128").unwrap();
    let float = result.float_layouts().get(&80).unwrap();

    assert_eq!(float.size_bits, 80);
    assert_eq!(float.abi_align_bits, 128);
}

#[test]
fn test_parse_float_f128() {
    let result = ParsedDataLayout::parse("e-f128:128:256").unwrap();
    let float = result.float_layouts().get(&128).unwrap();

    assert_eq!(float.size_bits, 128);
    assert_eq!(float.abi_align_bits, 128);
    assert_eq!(float.pref_align_bits, Some(256));
}

#[test]
fn test_parse_multiple_floats() {
    let result = ParsedDataLayout::parse("e-f32:32-f64:64-f80:128").unwrap();

    assert_eq!(result.float_layouts().len(), 3);
    assert!(result.float_layouts().contains_key(&32));
    assert!(result.float_layouts().contains_key(&64));
    assert!(result.float_layouts().contains_key(&80));
}

#[test]
fn test_parse_float_invalid_format() {
    let result = ParsedDataLayout::parse("e-f32");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid float layout"));
}

// ============================================================================
// PARSING TESTS - VECTORS
// ============================================================================

#[test]
fn test_parse_vector_v64() {
    let result = ParsedDataLayout::parse("e-v64:64").unwrap();
    let vec = result.vector_layouts().get(&64).unwrap();

    assert_eq!(vec.size_bits, 64);
    assert_eq!(vec.abi_align_bits, 64);
    assert_eq!(vec.pref_align_bits, None);
}

#[test]
fn test_parse_vector_v128() {
    let result = ParsedDataLayout::parse("e-v128:128").unwrap();
    let vec = result.vector_layouts().get(&128).unwrap();

    assert_eq!(vec.size_bits, 128);
    assert_eq!(vec.abi_align_bits, 128);
}

#[test]
fn test_parse_vector_with_preferred() {
    let result = ParsedDataLayout::parse("e-v256:256:512").unwrap();
    let vec = result.vector_layouts().get(&256).unwrap();

    assert_eq!(vec.size_bits, 256);
    assert_eq!(vec.abi_align_bits, 256);
    assert_eq!(vec.pref_align_bits, Some(512));
}

#[test]
fn test_parse_multiple_vectors() {
    let result = ParsedDataLayout::parse("e-v64:64-v128:128-v256:256").unwrap();

    assert_eq!(result.vector_layouts().len(), 3);
    assert!(result.vector_layouts().contains_key(&64));
    assert!(result.vector_layouts().contains_key(&128));
    assert!(result.vector_layouts().contains_key(&256));
}

#[test]
fn test_parse_vector_invalid_format() {
    let result = ParsedDataLayout::parse("e-v128");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid vector layout"));
}

// ============================================================================
// PARSING TESTS - AGGREGATES
// ============================================================================

#[test]
fn test_parse_aggregate_abi_only() {
    let result = ParsedDataLayout::parse("e-a:0").unwrap();
    let agg = result.aggregate_layout().unwrap();

    assert_eq!(agg.abi_align_bits, 0);
    assert_eq!(agg.pref_align_bits, None);
}

#[test]
fn test_parse_aggregate_with_preferred() {
    let result = ParsedDataLayout::parse("e-a:0:64").unwrap();
    let agg = result.aggregate_layout().unwrap();

    assert_eq!(agg.abi_align_bits, 0);
    assert_eq!(agg.pref_align_bits, Some(64));
}

#[test]
fn test_parse_aggregate_nonzero_abi() {
    let result = ParsedDataLayout::parse("e-a:32:64").unwrap();
    let agg = result.aggregate_layout().unwrap();

    assert_eq!(agg.abi_align_bits, 32);
    assert_eq!(agg.pref_align_bits, Some(64));
}

#[test]
fn test_parse_aggregate_missing() {
    let result = ParsedDataLayout::parse("e").unwrap();
    assert!(result.aggregate_layout().is_none());
}

#[test]
fn test_parse_aggregate_invalid_format() {
    let result = ParsedDataLayout::parse("e-a:");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid ABI"));
}

// ============================================================================
// PARSING TESTS - FUNCTION POINTERS
// ============================================================================

#[test]
fn test_parse_function_pointer() {
    let result = ParsedDataLayout::parse("e-Fi8").unwrap();
    let fp = result.function_pointer_layout().unwrap();

    assert_eq!(fp.abi_align_bits, 8);
}

#[test]
fn test_parse_function_pointer_32() {
    let result = ParsedDataLayout::parse("e-Fi32").unwrap();
    let fp = result.function_pointer_layout().unwrap();

    assert_eq!(fp.abi_align_bits, 32);
}

#[test]
fn test_parse_function_pointer_64() {
    let result = ParsedDataLayout::parse("e-Fi64").unwrap();
    let fp = result.function_pointer_layout().unwrap();

    assert_eq!(fp.abi_align_bits, 64);
}

#[test]
fn test_parse_function_pointer_missing() {
    let result = ParsedDataLayout::parse("e").unwrap();
    assert!(result.function_pointer_layout().is_none());
}

#[test]
fn test_parse_function_pointer_invalid() {
    let result = ParsedDataLayout::parse("e-Fi");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid function pointer"));
}

#[test]
fn test_parse_function_pointer_invalid_number() {
    let result = ParsedDataLayout::parse("e-Fixyz");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid function pointer"));
}

// ============================================================================
// PARSING TESTS - NATIVE INTEGER WIDTHS
// ============================================================================

#[test]
fn test_parse_native_widths_single() {
    let result = ParsedDataLayout::parse("e-n32").unwrap();
    let widths = result.native_int_widths().unwrap();

    assert_eq!(widths.widths_bits, vec![32]);
}

#[test]
fn test_parse_native_widths_multiple() {
    let result = ParsedDataLayout::parse("e-n8:16:32:64").unwrap();
    let widths = result.native_int_widths().unwrap();

    assert_eq!(widths.widths_bits, vec![8, 16, 32, 64]);
}

#[test]
fn test_parse_native_widths_128() {
    let result = ParsedDataLayout::parse("e-n32:64:128").unwrap();
    let widths = result.native_int_widths().unwrap();

    assert_eq!(widths.widths_bits, vec![32, 64, 128]);
}

#[test]
fn test_parse_native_widths_missing() {
    let result = ParsedDataLayout::parse("e").unwrap();
    assert!(result.native_int_widths().is_none());
}

#[test]
fn test_parse_native_widths_invalid() {
    let result = ParsedDataLayout::parse("e-n:abc");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid native widths"));
}

// ============================================================================
// PARSING TESTS - STACK ALIGNMENT
// ============================================================================

#[test]
fn test_parse_stack_alignment_128() {
    let result = ParsedDataLayout::parse("e-S128").unwrap();
    assert_eq!(result.stack_align_bits(), Some(128));
}

#[test]
fn test_parse_stack_alignment_256() {
    let result = ParsedDataLayout::parse("e-S256").unwrap();
    assert_eq!(result.stack_align_bits(), Some(256));
}

#[test]
fn test_parse_stack_alignment_missing() {
    let result = ParsedDataLayout::parse("e").unwrap();
    assert_eq!(result.stack_align_bits(), None);
}

#[test]
fn test_parse_stack_alignment_invalid() {
    let result = ParsedDataLayout::parse("e-Sabc");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid stack alignment"));
}

// ============================================================================
// INTEGRATION TESTS - FULL PLATFORM PARSING
// ============================================================================

#[test]
fn test_parse_full_linux_x86_64() {
    let result = DataLayout::LinuxX86_64.parse().unwrap();

    assert_eq!(result.endianness(), Endianness::Little);
    assert_eq!(result.mangling(), Some(&Mangling::Elf));
    assert_eq!(result.pointer_layouts().len(), 3); // p270, p271, p272
    assert_eq!(result.stack_align_bits(), Some(128));

    let widths = result.native_int_widths().unwrap();
    assert_eq!(widths.widths_bits, vec![8, 16, 32, 64]);
}

#[test]
fn test_parse_full_windows_x86_64() {
    let result = DataLayout::WindowsX86_64.parse().unwrap();

    assert_eq!(result.endianness(), Endianness::Little);
    assert_eq!(result.mangling(), Some(&Mangling::Coff));
    assert!(result.pointer_layouts().contains_key(&270));
}

#[test]
fn test_parse_full_macos_x86_64() {
    let result = DataLayout::MacOSX86_64.parse().unwrap();

    assert_eq!(result.endianness(), Endianness::Little);
    assert_eq!(result.mangling(), Some(&Mangling::MachO));
}

#[test]
fn test_parse_full_linux_aarch64() {
    let result = DataLayout::LinuxAArch64.parse().unwrap();

    assert_eq!(result.endianness(), Endianness::Little);
    assert_eq!(result.mangling(), Some(&Mangling::Elf));

    // Check integer layouts specific to AArch64
    assert!(result.integer_layouts().contains_key(&8));
    assert!(result.integer_layouts().contains_key(&16));
    assert!(result.integer_layouts().contains_key(&64));
    assert!(result.integer_layouts().contains_key(&128));
}

#[test]
fn test_parsed_method() {
    // Test that parsed() doesn't panic for built-in layouts
    let _ = DataLayout::LinuxX86_64.parsed();
    let _ = DataLayout::WindowsX86_64.parsed();
    let _ = DataLayout::MacOSX86_64.parsed();
    let _ = DataLayout::LinuxAArch64.parsed();
    let _ = DataLayout::FreeBSDX86_64.parsed();
}

// ============================================================================
// EDGE CASES AND ERROR HANDLING
// ============================================================================

#[test]
fn test_parse_empty_string() {
    let result = ParsedDataLayout::parse("").unwrap();
    assert_eq!(result.endianness(), Endianness::Little);
}

#[test]
fn test_parse_whitespace_only() {
    let result = ParsedDataLayout::parse("   ").unwrap();
    assert_eq!(result.endianness(), Endianness::Little);
}

#[test]
fn test_parse_multiple_separators() {
    let result = ParsedDataLayout::parse("e--m:e--i64:64").unwrap();
    assert_eq!(result.endianness(), Endianness::Little);
    assert_eq!(result.mangling(), Some(&Mangling::Elf));
}

#[test]
fn test_parse_trailing_separator() {
    let result = ParsedDataLayout::parse("e-m:e-").unwrap();
    assert_eq!(result.endianness(), Endianness::Little);
}

#[test]
fn test_parse_leading_separator() {
    let result = ParsedDataLayout::parse("-e-m:e").unwrap();
    assert_eq!(result.endianness(), Endianness::Little);
}

#[test]
fn test_parse_unknown_specifier_ignored() {
    // Unknown specifiers should be silently ignored
    let result = ParsedDataLayout::parse("e-X:something-m:e").unwrap();
    assert_eq!(result.endianness(), Endianness::Little);
    assert_eq!(result.mangling(), Some(&Mangling::Elf));
}

#[test]
fn test_parse_duplicate_endianness() {
    // Last one wins
    let result = ParsedDataLayout::parse("e-E").unwrap();
    assert_eq!(result.endianness(), Endianness::Big);
}

#[test]
fn test_parse_duplicate_mangling() {
    let result = ParsedDataLayout::parse("e-m:e-m:w").unwrap();
    assert_eq!(result.mangling(), Some(&Mangling::Coff));
}

#[test]
fn test_parse_duplicate_pointer_same_address_space() {
    let result = ParsedDataLayout::parse("e-p:32:32-p:64:64").unwrap();
    let ptr = result.pointer_layouts().get(&0).unwrap();

    // Last one wins
    assert_eq!(ptr.size_bits, 64);
    assert_eq!(ptr.abi_align_bits, 64);
}

#[test]
fn test_parse_very_large_alignment() {
    let result = ParsedDataLayout::parse("e-i64:4096").unwrap();
    let int = result.integer_layouts().get(&64).unwrap();
    assert_eq!(int.abi_align_bits, 4096);
}

#[test]
fn test_parse_zero_size_pointer() {
    let result = ParsedDataLayout::parse("e-p:0:8");
    // This should succeed parsing even though it's semantically weird
    assert!(result.is_ok());
}

#[test]
fn test_parse_maximum_u32_values() {
    let max = u32::MAX;
    let layout_str = format!("e-p:{max}:{max}:{max}");
    let result = ParsedDataLayout::parse(&layout_str);

    // Should succeed without overflow
    assert!(result.is_ok());
}

#[test]
fn test_parse_overflow_u32() {
    // Value larger than u32::MAX should fail
    let layout_str = "e-p:4294967296:64"; // u32::MAX + 1
    let result = ParsedDataLayout::parse(layout_str);

    assert!(result.is_err());
}

#[test]
fn test_parse_address_space_zero_explicit() {
    let result = ParsedDataLayout::parse("e-p0:64:64").unwrap();
    let ptr = result.pointer_layouts().get(&0).unwrap();

    assert_eq!(ptr.address_space, 0);
    assert_eq!(ptr.size_bits, 64);
}

#[test]
fn test_parse_integer_unaligned() {
    // Test integer with alignment not matching its size
    let result = ParsedDataLayout::parse("e-i64:32").unwrap();
    let int = result.integer_layouts().get(&64).unwrap();

    assert_eq!(int.size_bits, 64);
    assert_eq!(int.abi_align_bits, 32);
}

#[test]
fn test_parse_preferred_less_than_abi() {
    // Preferred alignment less than ABI (unusual but should parse)
    let result = ParsedDataLayout::parse("e-i64:64:32").unwrap();
    let int = result.integer_layouts().get(&64).unwrap();

    assert_eq!(int.abi_align_bits, 64);
    assert_eq!(int.pref_align_bits, Some(32));
}

#[test]
fn test_parse_negative_number_fails() {
    let result = ParsedDataLayout::parse("e-i64:-32");
    assert!(result.is_err());
}

#[test]
fn test_parse_float_number_fails() {
    let result = ParsedDataLayout::parse("e-i64:32.5");
    assert!(result.is_err());
}

#[test]
fn test_parse_hex_number_fails() {
    let result = ParsedDataLayout::parse("e-i64:0x40");
    assert!(result.is_err());
}

// ============================================================================
// ACCESSOR TESTS
// ============================================================================

#[test]
fn test_accessors_immutability() {
    let parsed = DataLayout::LinuxX86_64.parsed();

    // All accessors should return references
    let _endianness = parsed.endianness();
    let _mangling = parsed.mangling();
    let _pointers = parsed.pointer_layouts();
    let _integers = parsed.integer_layouts();
    let _floats = parsed.float_layouts();
    let _vectors = parsed.vector_layouts();
    let _aggregate = parsed.aggregate_layout();
    let _function_ptr = parsed.function_pointer_layout();
    let _native = parsed.native_int_widths();
    let _stack = parsed.stack_align_bits();
}

#[test]
fn test_pointer_layout_lookup() {
    let parsed = DataLayout::LinuxX86_64.parsed();

    let ptr_270 = parsed.pointer_layouts().get(&270);
    assert!(ptr_270.is_some());

    let ptr_999 = parsed.pointer_layouts().get(&999);
    assert!(ptr_999.is_none());
}

#[test]
fn test_integer_layout_lookup() {
    let parsed = DataLayout::LinuxX86_64.parsed();

    let i64 = parsed.integer_layouts().get(&64);
    assert!(i64.is_some());

    let i7 = parsed.integer_layouts().get(&7);
    assert!(i7.is_none());
}

// ============================================================================
// COMPLEX INTEGRATION TESTS
// ============================================================================

#[test]
fn test_complex_layout_with_all_features() {
    let layout = "E-m:e-p:32:32:64-p270:64:64-i8:8:16-i16:16:32-i32:32-i64:64-f32:32-f64:64:128-f80:128-v64:64-v128:128-a:0:64-Fi32-n8:16:32:64-S256";
    let result = ParsedDataLayout::parse(layout).unwrap();

    assert_eq!(result.endianness(), Endianness::Big);
    assert_eq!(result.mangling(), Some(&Mangling::Elf));
    assert_eq!(result.pointer_layouts().len(), 2);
    assert_eq!(result.integer_layouts().len(), 4);
    assert_eq!(result.float_layouts().len(), 3);
    assert_eq!(result.vector_layouts().len(), 2);
    assert!(result.aggregate_layout().is_some());
    assert!(result.function_pointer_layout().is_some());
    assert!(result.native_int_widths().is_some());
    assert_eq!(result.stack_align_bits(), Some(256));
}

#[test]
fn test_minimal_valid_layout() {
    let result = ParsedDataLayout::parse("e").unwrap();

    assert_eq!(result.endianness(), Endianness::Little);
    assert_eq!(result.pointer_layouts().len(), 0);
}

#[test]
fn test_layout_with_only_pointers() {
    let result = ParsedDataLayout::parse("e-p:64:64-p270:32:32").unwrap();

    assert_eq!(result.pointer_layouts().len(), 2);
    assert_eq!(result.integer_layouts().len(), 0);
    assert_eq!(result.float_layouts().len(), 0);
}

// ============================================================================
// BOUNDARY VALUE TESTS
// ============================================================================

#[test]
fn test_min_alignment_values() {
    let result = ParsedDataLayout::parse("e-i8:1-p:8:1").unwrap();

    let int = result.integer_layouts().get(&8).unwrap();
    assert_eq!(int.abi_align_bits, 1);

    let ptr = result.pointer_layouts().get(&0).unwrap();
    assert_eq!(ptr.abi_align_bits, 1);
}

#[test]
fn test_power_of_two_alignments() {
    let result = ParsedDataLayout::parse("e-i8:1-i16:2-i32:4-i64:8-i128:16").unwrap();

    assert_eq!(result.integer_layouts().get(&8).unwrap().abi_align_bits, 1);
    assert_eq!(result.integer_layouts().get(&16).unwrap().abi_align_bits, 2);
    assert_eq!(result.integer_layouts().get(&32).unwrap().abi_align_bits, 4);
    assert_eq!(result.integer_layouts().get(&64).unwrap().abi_align_bits, 8);
    assert_eq!(result.integer_layouts().get(&128).unwrap().abi_align_bits, 16);
}

#[test]
fn test_non_power_of_two_sizes() {
    // Test unusual sizes that might occur in practice
    let result = ParsedDataLayout::parse("e-i7:8-i13:16-i96:128").unwrap();

    assert!(result.integer_layouts().contains_key(&7));
    assert!(result.integer_layouts().contains_key(&13));
    assert!(result.integer_layouts().contains_key(&96));
}

#[test]
fn test_very_large_address_space_number() {
    let result = ParsedDataLayout::parse("e-p4294967295:64:64").unwrap();
    let ptr = result.pointer_layouts().get(&4_294_967_295).unwrap();

    assert_eq!(ptr.address_space, 4_294_967_295);
}

// ============================================================================
// CLONE AND DEBUG TESTS
// ============================================================================

#[test]
fn test_parsed_data_layout_clone() {
    let original = DataLayout::LinuxX86_64.parsed();
    let cloned = original.clone();

    assert_eq!(cloned.endianness(), original.endianness());
    assert_eq!(cloned.stack_align_bits(), original.stack_align_bits());
}

#[test]
fn test_pointer_layout_clone() {
    let ptr = PointerLayout { address_space: 0, size_bits: 64, abi_align_bits: 64, pref_align_bits: Some(128) };

    let cloned = ptr.clone();
    assert_eq!(cloned.address_space, ptr.address_space);
    assert_eq!(cloned.size_bits, ptr.size_bits);
}

#[test]
fn test_debug_formatting() {
    let layout = DataLayout::LinuxX86_64;
    let debug_str = format!("{layout:?}");
    assert!(debug_str.contains("LinuxX86_64"));
}

// ============================================================================
// CONST CONTEXT TESTS
// ============================================================================

#[test]
fn test_as_str_in_const_context() {
    const LAYOUT_STR: &str = DataLayout::LinuxX86_64.as_str();
    assert!(!LAYOUT_STR.is_empty());
    assert!(LAYOUT_STR.contains("e-m:e"));
}

#[test]
fn test_const_equality() {
    const LINUX: DataLayout = DataLayout::LinuxX86_64;
    const WINDOWS: DataLayout = DataLayout::WindowsX86_64;

    assert_ne!(LINUX, WINDOWS);
}

// ============================================================================
// STRESS TESTS
// ============================================================================

#[test]
fn test_parse_many_specifications() {
    let mut specs = vec!["e", "m:e"];

    // Add 50 integer layouts
    for i in 1..=50 {
        specs.push(Box::leak(format!("i{}:{}", i * 8, i * 8).into_boxed_str()));
    }

    let layout_str = specs.join("-");
    let result = ParsedDataLayout::parse(&layout_str);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().integer_layouts().len(), 50);
}

#[test]
fn test_parse_all_platforms_successfully() {
    let platforms = vec![
        DataLayout::LinuxX86_64,
        DataLayout::LinuxAArch64,
        DataLayout::WindowsX86_64,
        DataLayout::MacOSX86_64,
        DataLayout::FreeBSDX86_64,
        DataLayout::NetBSDX86_64,
        DataLayout::OpenBSDX86_64,
        DataLayout::DragonFlyX86_64,
    ];

    for platform in platforms {
        let result = platform.parse();
        assert!(result.is_ok(), "Failed to parse {platform:?}");
    }
}

// ============================================================================
// UNICODE AND SPECIAL CHARACTERS
// ============================================================================

#[test]
fn test_parse_with_unicode_in_unknown_mangling() {
    let result = ParsedDataLayout::parse("e-m:🦀");
    assert!(result.is_ok());

    match result.unwrap().mangling() {
        Some(Mangling::Unknown(s)) => assert_eq!(s, "🦀"),
        _ => panic!("Expected Unknown mangling"),
    }
}

#[test]
fn test_parse_with_special_chars_in_unknown_spec() {
    // Unknown specs with special characters should be ignored
    let result = ParsedDataLayout::parse("e-???:test-m:e");
    assert!(result.is_ok());
}
