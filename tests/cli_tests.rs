// tests/cli_tests.rs
use assert_cmd::Command;
use clap::Parser;
use clap::error::ErrorKind;
use jsavrs::cli::Args;
use predicates::prelude::*;
use std::path::PathBuf;

#[test]
fn help_displays_correctly() {
    Command::cargo_bin("jsavrs")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"))
        .stdout(predicate::str::contains("-i, --input <FILE>"));
}

#[test]
fn help_short_flag_works() {
    Command::cargo_bin("jsavrs")
        .unwrap()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"))
        .stdout(predicate::str::contains("-i, --input <FILE>"));
}

#[test]
fn version_displays_correctly() {
    let version = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Command::cargo_bin("jsavrs").unwrap().arg("--version").assert().success().stdout(predicate::str::contains(version));
}

#[test]
fn version_short_flag_works() {
    let version = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Command::cargo_bin("jsavrs").unwrap().arg("-V").assert().success().stdout(predicate::str::contains(version));
}

#[test]
fn missing_input_argument() {
    Command::cargo_bin("jsavrs")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("required arguments were not provided"));
}

#[test]
fn invalid_file_extension() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("invalid_file.txt");

    // Create temporary test file with wrong extension
    std::fs::write(&path, "test content").unwrap();

    cmd.arg("-i").arg(&path).assert().failure().stderr(predicate::str::contains("expected a path to a .vn file"));

    // Cleanup
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn invalid_file_path() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("non_existent_file.vn");

    cmd.arg("-i").arg(path).assert().failure().stderr(predicate::str::contains("I/O"));
}

#[test]
fn valid_file_with_short_flags() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test.vn");

    // Create temporary test file
    std::fs::write(&path, "fun a(num1: i8, num2: i8): i8 { return 0i8 }").unwrap();

    cmd.arg("-i").arg(&path).assert().success();

    // Cleanup
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn verbose_flag_works() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test_verbose_unique.vn");

    // Create temporary test file
    std::fs::write(&path, "fun a(num1: i8, num2: i8): i8 { return 0i8 }").unwrap();

    cmd.arg("-i").arg(&path).arg("-v").assert().success();

    // Cleanup
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn test_relative_path_input() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let path = PathBuf::from("test_relative.vn");

    // Create temporary test file
    std::fs::write(&path, "fun a(num1: i8, num2: i8): i8 { return 0i8 }").unwrap();

    cmd.arg("-i").arg(&path).assert().success();

    // Cleanup
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn long_flag_works() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test2.vn");

    // Create temporary test file
    std::fs::write(&path, "fun a(num1: i8, num2: i8): i8 { return 0i8 }").unwrap();

    cmd.arg("--input").arg(&path).arg("-v").assert().success();

    // Cleanup
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn long_verbose_flag_works() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test3.vn");

    // Create temporary test file
    std::fs::write(&path, "fun a(num1: i8, num2: i8): i8 { return 0i8 }").unwrap();

    cmd.arg("--input").arg(&path).arg("--verbose").assert().success();

    // Cleanup
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn multiple_combinations_of_flags() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test4.vn");

    // Create temporary test file
    std::fs::write(&path, "fun a(num1: i8, num2: i8): i8 { return 0i8 }").unwrap();

    // Test short input with long verbose
    cmd.arg("-i").arg(&path).arg("--verbose").assert().success();

    // Cleanup
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn test_parse_short_options() {
    let args = Args::try_parse_from(["jsavrs", "-i", "test.vn", "-v"]).unwrap();
    assert_eq!(args.input, PathBuf::from("test.vn"));
    assert!(args.verbose);
}

#[test]
fn test_parse_long_options() {
    let args = Args::try_parse_from(["jsavrs", "--input", "test.vn", "--verbose"]).unwrap();
    assert_eq!(args.input, PathBuf::from("test.vn"));
    assert!(args.verbose);
}

#[test]
fn test_parse_mixed_options() {
    let args = Args::try_parse_from(["jsavrs", "-i", "test.vn", "--verbose"]).unwrap();
    assert_eq!(args.input, PathBuf::from("test.vn"));
    assert!(args.verbose);

    let args2 = Args::try_parse_from(["jsavrs", "--input", "test.vn", "-v"]).unwrap();
    assert_eq!(args2.input, PathBuf::from("test.vn"));
    assert!(args2.verbose);
}

#[test]
fn test_missing_input() {
    let result = Args::try_parse_from(["jsavrs"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn test_unknown_argument() {
    let result = Args::try_parse_from(["jsavrs", "-i", "test.vn", "--unknown"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnknownArgument);
}

#[test]
fn test_verbose_flag_absence() {
    let args = Args::try_parse_from(["jsavrs", "-i", "test.vn"]).unwrap();
    assert_eq!(args.input, PathBuf::from("test.vn"));
    assert!(!args.verbose);
}

#[test]
fn test_case_insensitive_extension() {
    // Test that .VN (uppercase) works
    let args = Args::try_parse_from(["jsavrs", "-i", "test.VN"]).unwrap();
    assert_eq!(args.input, PathBuf::from("test.VN"));

    // Test that .Vn (mixed case) works
    let args2 = Args::try_parse_from(["jsavrs", "-i", "test.Vn"]).unwrap();
    assert_eq!(args2.input, PathBuf::from("test.Vn"));

    // Test that .vN (mixed case) works
    let args3 = Args::try_parse_from(["jsavrs", "-i", "test.vN"]).unwrap();
    assert_eq!(args3.input, PathBuf::from("test.vN"));
}

#[test]
fn test_file_extension_validation() {
    // Valid .vn extension
    let result = Args::try_parse_from(["jsavrs", "-i", "valid_file.vn"]);
    assert!(result.is_ok());

    // Invalid extensions should fail
    let result_txt = Args::try_parse_from(["jsavrs", "-i", "invalid_file.txt"]);
    assert!(result_txt.is_err());

    let result_no_ext = Args::try_parse_from(["jsavrs", "-i", "no_extension"]);
    assert!(result_no_ext.is_err());

    let result_wrong_case_ext = Args::try_parse_from(["jsavrs", "-i", "wrong.VNT"]); // Wrong extension
    assert!(result_wrong_case_ext.is_err());
}

#[test]
fn test_help_content_completeness() {
    Command::cargo_bin("jsavrs")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Input file for compilation"))
        .stdout(predicate::str::contains("Show verbose output"))
        .stdout(predicate::str::contains("Print help"))
        .stdout(predicate::str::contains("Print version"));
}

#[test]
fn test_empty_input_file() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("empty_test.vn");

    // Create empty temporary test file
    std::fs::write(&path, "").unwrap();

    cmd.arg("-i").arg(&path).assert().success();

    // Cleanup
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn test_file_with_spaces_in_name() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test file with spaces.vn");

    // Create temporary test file
    std::fs::write(&path, "fun a(num1: i8, num2: i8): i8 { return 0i8 }").unwrap();

    cmd.arg("-i").arg(&path).assert().success();

    // Cleanup
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn test_absolute_path_input() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test_absolute.vn");

    // Create temporary test file
    std::fs::write(&path, "fun a(num1: i8, num2: i8): i8 { return 0i8 }").unwrap();

    cmd.arg("-i").arg(&path).assert().success();

    // Cleanup
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn test_all_flag_combinations() {
    // Test -i -v combination
    let args1 = Args::try_parse_from(["jsavrs", "-i", "test.vn", "-v"]).unwrap();
    assert_eq!(args1.input, PathBuf::from("test.vn"));
    assert!(args1.verbose);

    // Test -v -i combination
    let args2 = Args::try_parse_from(["jsavrs", "-v", "-i", "test.vn"]).unwrap();
    assert_eq!(args2.input, PathBuf::from("test.vn"));
    assert!(args2.verbose);

    // Test --input -v combination
    let args3 = Args::try_parse_from(["jsavrs", "--input", "test.vn", "-v"]).unwrap();
    assert_eq!(args3.input, PathBuf::from("test.vn"));
    assert!(args3.verbose);

    // Test -i --verbose combination
    let args4 = Args::try_parse_from(["jsavrs", "-i", "test.vn", "--verbose"]).unwrap();
    assert_eq!(args4.input, PathBuf::from("test.vn"));
    assert!(args4.verbose);
}
