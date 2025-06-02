// tests/cli_tests.rs
use assert_cmd::Command;
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
        .stderr(predicate::str::contains("required arguments were not provided"));
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
    std::fs::write(&path, "print \"test\";").unwrap();
    
    cmd.arg("-i")
        .arg(&path)
        .arg("-v")
        .assert()
        .success();
    
    // Cleanup
    std::fs::remove_file(&path).unwrap();
}