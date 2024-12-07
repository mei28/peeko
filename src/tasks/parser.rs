use crate::tasks::detector::{detect_file_type, get_parser};
use crate::tasks::task::Task;
use anyhow::Result;

/// タスクをパース
pub fn parse_tasks(file_path: &str) -> Result<Vec<Task>> {
    let file_type = detect_file_type(file_path);
    let parser = get_parser(file_type);
    parser.parse(file_path)
}

