use anyhow::Result;
use peeko::app::App;
use peeko::tasks::parser;

#[test]
fn test_app_with_parsed_tasks() -> Result<()> {
    let tasks = parser::parse_tasks("dummy_file")?;
    let mut app = App::new(tasks, "dummy_file".into())?;
    // 仮にappにrender_like_output()的な関数があったと仮定:
    // let output = app.render_like_output();
    // assert!(output.contains("build"));
    // assert!(output.contains("test"));

    // 現実にはui::run_tuiはターミナルに直接描画するため、
    // このような直接的な検証は困難。
    // 代わりにApp内部ロジック（tasksや選択状態など）を検証する。
    assert_eq!(app.state().tasks.len(), 2);
    Ok(())
}
