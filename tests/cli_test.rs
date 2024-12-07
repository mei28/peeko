use assert_cmd::Command;

#[test]
fn test_cli_with_dummy_file() {
    let mut cmd = Command::cargo_bin("peeko").unwrap();
    // --no-tuiでタスクリスト表示のみ
    cmd.arg("Makefile")
       .arg("--no-tui")
       .assert()
       .success()
       .stdout(predicates::str::contains("Found task: build at line 10"));
}

