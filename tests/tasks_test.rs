use peeko::tasks::parser;

#[test]
fn test_parse_tasks() {
    let tasks = parser::parse_tasks("dummy_file").unwrap();
    assert!(!tasks.is_empty());
    assert_eq!(tasks[0].name, "build");
}
