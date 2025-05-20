use jsavrs::location::source_location::SourceLocation;

#[test]
fn source_location_new_and_fields() {
    let loc = SourceLocation::new(3, 5, 42);
    assert_eq!(loc.line, 3);
    assert_eq!(loc.column, 5);
    assert_eq!(loc.absolute_pos, 42);
}

#[test]
fn test_source_location_default() {
    let loc = SourceLocation::default();
    assert_eq!(loc.line, 0);
    assert_eq!(loc.column, 0);
    assert_eq!(loc.absolute_pos, 0);
}