use super::task::Task;
use anyhow::Result;

pub fn parse_tasks(_file_path: &str) -> Result<Vec<Task>> {
    Ok(vec![
        Task {
            name: "build".into(),
            line_number: 10,
            command: Some("make build".into()),
        },
        Task {
            name: "test".into(),
            line_number: 20,
            command: Some("make test".into()),
        },
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_parse_tasks_dummy() -> Result<()> {
        let tasks = parse_tasks("dummy_file")?;
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].name, "build");
        Ok(())
    }

    // 将来的には、実在するテスト用ファイルを`tests/fixtures/`等に置いて、
    // そこをparse_tasksに渡して、実際に正しいパースができるか確認するテストも可能
}
