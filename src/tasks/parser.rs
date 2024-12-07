use crate::tasks::detector::{detect_file_type, get_parser};
use crate::tasks::task::Task;
use anyhow::Result;

pub fn parse_tasks(file_path: &str) -> Result<Vec<Task>> {
    let file_type = detect_file_type(file_path);
    let parser = get_parser(file_type);
    parser.parse(file_path)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_tasks_unknown_file() {
        let tmp = tempdir().unwrap();
        let file_path = tmp.path().join("unknown_file");
        File::create(&file_path).unwrap(); // 空ファイル

        let tasks = parse_tasks(file_path.to_str().unwrap()).unwrap();
        assert!(!tasks.is_empty()); // ダミーが返るはず
    }
}

