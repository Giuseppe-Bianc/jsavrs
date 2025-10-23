use jsavrs::asm::Platform;

#[test]
fn test_platform_display() {
    assert_eq!(format!("{}", Platform::Windows), "Windows");
    assert_eq!(format!("{}", Platform::Linux), "Linux");
    assert_eq!(format!("{}", Platform::MacOS), "macOS");
}

#[test]
fn test_platform_equality() {
    assert_eq!(Platform::Windows, Platform::Windows);
    assert_eq!(Platform::Linux, Platform::Linux);
    assert_eq!(Platform::MacOS, Platform::MacOS);

    assert_ne!(Platform::Windows, Platform::Linux);
    assert_ne!(Platform::Windows, Platform::MacOS);
    assert_ne!(Platform::Linux, Platform::MacOS);
}

#[test]
fn test_platform_clone() {
    let platform = Platform::Linux;
    let cloned_platform = platform.clone();
    assert_eq!(platform, cloned_platform);
}

#[test]
fn test_platform_debug() {
    let platform = Platform::Windows;
    let debug_str = format!("{:?}", platform);
    assert!(debug_str.contains("Windows"));
}

#[test]
fn test_all_platform_variants() {
    let platforms = [Platform::Windows, Platform::Linux, Platform::MacOS];
    assert_eq!(platforms.len(), 3);

    // Ensure all platforms can be formatted
    for platform in platforms.iter() {
        let formatted = format!("{}", platform);
        assert!(!formatted.is_empty());
    }
}
