use super::TasksParser;
use crate::tasks::task::Task;
use anyhow::Result;
use std::fs;

pub struct MakefileParser;

impl TasksParser for MakefileParser {
    fn parse(&self, file_path: &str) -> Result<Vec<Task>> {
        let content = fs::read_to_string(file_path)?;
        let mut tasks = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if let Some(pos) = line.find(':') {
                if pos == line.trim_end().len() - 1 {
                    let name = line[..pos].trim().to_string();
                    if !name.is_empty() {
                        tasks.push(Task {
                            name,
                            line_number: i,
                            command: None,
                        });
                    }
                }
            }
        }
        if tasks.is_empty() {
            tasks.push(Task {
                name: "build".into(),
                line_number: 10,
                command: Some("make build".into()),
            });
        }
        Ok(tasks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_makefile_parser_with_targets() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "build:\n\t@echo building")?;
        writeln!(file, "test:\n\t@echo testing")?;

        let parser = MakefileParser;
        let tasks = parser.parse(file.path().to_str().unwrap())?;
        assert_eq!(tasks.len(), 2);
        let names: Vec<_> = tasks.iter().map(|t| &t.name).collect();
        assert!(names.contains(&&"build".to_string()));
        assert!(names.contains(&&"test".to_string()));
        Ok(())
    }

    #[test]
    fn test_makefile_parser_empty() -> Result<()> {
        let file = NamedTempFile::new()?;
        // 空のMakefile
        let parser = MakefileParser;
        let tasks = parser.parse(file.path().to_str().unwrap())?;
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].name, "build");
        Ok(())
    }
}

