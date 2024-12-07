use peeko::tasks::detector::{detect_file_type, FileType}; // FileType を適切にインポート
use peeko::tasks::parser::parse_tasks;
use peeko::tasks::task::Task;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

#[test]
fn test_detect_file_type_for_cargo_toml() {
    assert_eq!(
        detect_file_type("../example/Cargo.toml"),
        FileType::CargoToml
    );
}

#[test]
fn test_parse_tasks_from_makefile() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "build:\n\t@echo building").unwrap();
    writeln!(file, "test:\n\t@echo testing").unwrap();

    let tasks = parse_tasks(file.path().to_str().unwrap()).unwrap();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].name, "build");
    assert_eq!(tasks[1].name, "test");
}

#[test]
fn test_parse_tasks_from_cargo_toml() {
    // `example/Cargo.toml` を指定
    let file_path = "example/Cargo.toml";

    // ファイルが存在することを確認
    assert!(
        Path::new(file_path).exists(),
        "example/Cargo.toml does not exist"
    );

    // `parse_tasks` を呼び出してタスクを取得
    let tasks = parse_tasks(file_path).unwrap();

    // タスク名をソートして比較
    let mut sorted_tasks: Vec<_> = tasks.iter().map(|t| t.name.clone()).collect();
    sorted_tasks.sort();

    // 期待されるタスク名
    let expected = vec!["build".to_string(), "test".to_string()];
    assert_eq!(sorted_tasks, expected);
}

#[test]
fn test_parse_tasks_no_tasks() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(
        file,
        r#"
[package]
name = "example"
version = "0.1.0"
"#
    )
    .unwrap();

    let result = parse_tasks(file.path().to_str().unwrap());
    assert!(result.is_err());
}
