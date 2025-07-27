// tests/cli_tests.rs
use assert_cmd::Command;
use clap::error::ErrorKind;
use clap::Parser;
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
        .stdout(predicate::str::contains("-i, --input <INPUT>"));
}

#[test]
fn version_displays_correctly() {
    let version = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    Command::cargo_bin("jsavrs")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(version));
}

#[test]
fn missing_input_argument() {
    Command::cargo_bin("jsavrs")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "required arguments were not provided",
        ));
}

#[test]
fn invalid_file_path() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("non_existent_file.vn");

    cmd.arg("-i")
        .arg(path)
        .assert()
        .failure()
        .stderr(predicate::str::contains("I/O"));
}

#[test]
fn verbose_flag_works() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test.vn");

    // Create temporary test file
    std::fs::write(&path, "fun a(num1: i8, num2: i8): i8 {}").unwrap();

    cmd.arg("-i").arg(&path).arg("-v").assert().success();

    // Cleanup
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn long_flag_works() {
    let mut cmd = Command::cargo_bin("jsavrs").unwrap();
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test2.vn");

    // Create temporary test file
    std::fs::write(&path, "fun a(num1: i8, num2: i8): i8 {}").unwrap();

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
    std::fs::write(&path, "fun a(num1: i8, num2: i8): i8 {}").unwrap();

    cmd.arg("--input")
        .arg(&path)
        .arg("--verbose")
        .assert()
        .success();

    // Cleanup
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn test_parse_short_options() {
    let args = Args::try_parse_from(&["jsavrs", "-i", "test.vn", "-v"]).unwrap();
    assert_eq!(args.input, PathBuf::from("test.vn"));
    assert!(args.verbose);
}

#[test]
fn test_parse_long_options() {
    let args = Args::try_parse_from(&["jsavrs", "--input", "test.vn", "--verbose"]).unwrap();
    assert_eq!(args.input, PathBuf::from("test.vn"));
    assert!(args.verbose);
}

#[test]
fn test_missing_input() {
    let result = Args::try_parse_from(&["jsavrs"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
}

#[test]
fn test_unknown_argument() {
    let result = Args::try_parse_from(&["jsavrs", "-i", "test.vn", "--unknown"]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::UnknownArgument);
}

#[test]
fn test_verbose_flag_absence() {
    let args = Args::try_parse_from(&["jsavrs", "-i", "test.vn"]).unwrap();
    assert_eq!(args.input, PathBuf::from("test.vn"));
    assert!(!args.verbose);
}
