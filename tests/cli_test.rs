// tests/cli_test.rs
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_with_dummy_file() {
    let mut cmd = Command::cargo_bin("peeko").unwrap();
    cmd.arg("Makefile")
        .arg("--no-tui") // no-tuiオプションを付ける
        .assert()
        .success()
        .stdout(predicates::str::contains("Found task: build at line 10"));
}

