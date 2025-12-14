use jsavrs::asm::Section;

#[test]
fn test_section_name() {
    assert_eq!(Section::Text.name(), ".text");
    assert_eq!(Section::Data.name(), ".data");
    assert_eq!(Section::Bss.name(), ".bss");
    assert_eq!(Section::Rodata.name(), ".rodata");
}

#[test]
fn test_section_type_checking() {
    assert!(Section::Text.is_text());
    assert!(!Section::Text.is_data());
    assert!(!Section::Text.is_bss());
    assert!(!Section::Text.is_rodata());

    assert!(!Section::Data.is_text());
    assert!(Section::Data.is_data());
    assert!(!Section::Data.is_bss());
    assert!(!Section::Data.is_rodata());

    assert!(!Section::Bss.is_text());
    assert!(!Section::Bss.is_data());
    assert!(Section::Bss.is_bss());
    assert!(!Section::Bss.is_rodata());

    assert!(!Section::Rodata.is_text());
    assert!(!Section::Rodata.is_data());
    assert!(!Section::Rodata.is_bss());
    assert!(Section::Rodata.is_rodata());
}

#[test]
fn test_section_display() {
    assert_eq!(format!("{}", Section::Text), "section .text");
    assert_eq!(format!("{}", Section::Data), "section .data");
    assert_eq!(format!("{}", Section::Bss), "section .bss");
    assert_eq!(format!("{}", Section::Rodata), "section .rodata");
}

#[test]
fn test_section_equality() {
    assert_eq!(Section::Text, Section::Text);
    assert_eq!(Section::Data, Section::Data);
    assert_eq!(Section::Bss, Section::Bss);
    assert_eq!(Section::Rodata, Section::Rodata);

    assert_ne!(Section::Text, Section::Data);
    assert_ne!(Section::Text, Section::Bss);
    assert_ne!(Section::Text, Section::Rodata);
    assert_ne!(Section::Data, Section::Bss);
    assert_ne!(Section::Data, Section::Rodata);
    assert_ne!(Section::Bss, Section::Rodata);
}

#[test]
fn test_section_clone() {
    let section = Section::Text;
    let cloned_section = section.clone();
    assert_eq!(section, cloned_section);
}

#[test]
fn test_section_debug() {
    let section = Section::Rodata;
    let debug_str = format!("{section:?}");
    assert!(debug_str.contains("Rodata"));
}

#[test]
fn test_section_hash() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(Section::Text, "code");
    map.insert(Section::Data, "data");
    map.insert(Section::Bss, "uninitialized");
    map.insert(Section::Rodata, "constants");

    assert_eq!(map.get(&Section::Text), Some(&"code"));
    assert_eq!(map.get(&Section::Data), Some(&"data"));
    assert_eq!(map.get(&Section::Bss), Some(&"uninitialized"));
    assert_eq!(map.get(&Section::Rodata), Some(&"constants"));
}

#[test]
fn test_all_section_variants() {
    let sections = [Section::Text, Section::Data, Section::Bss, Section::Rodata];
    assert_eq!(sections.len(), 4);

    for section in &sections {
        let name = section.name();
        assert!(!name.is_empty());
        assert!(name.starts_with('.'));

        let display = format!("{section}");
        assert!(display.contains("section"));
        assert!(display.contains(name));
    }
}

#[test]
fn test_section_equality_properties() {
    // Reflexivity
    assert_eq!(Section::Text, Section::Text);

    // Symmetry
    let a = Section::Data;
    let b = Section::Data;
    assert_eq!(a, b);
    assert_eq!(b, a);

    // Transitivity
    let c = Section::Data;
    assert_eq!(a, b);
    assert_eq!(b, c);
    assert_eq!(a, c);
}
