use anyhow::Result;
use peeko::app::App;
use peeko::tasks::parser;

#[test]
fn test_app_with_parsed_tasks() -> Result<()> {
    let tasks = parser::parse_tasks("dummy_file")?;
    let mut app = App::new(tasks, "dummy_file".into())?;
    // 適当なアサーション(ファイルに応じて変化)
    // ダミーの場合は2タスクあるはず
    assert_eq!(app.state().tasks.len(), 2);
    // 初期選択は0
    assert_eq!(app.state().selected_index, 0);
    Ok(())
}

