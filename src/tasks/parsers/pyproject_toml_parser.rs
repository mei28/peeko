use super::TasksParser;
use crate::tasks::task::Task;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

pub struct PyProjectTomlParser;

#[derive(Deserialize, Default)]
struct PyProject {
    #[serde(default)]
    tool: Tool,
}

#[derive(Deserialize, Default)]
struct Tool {
    #[serde(default)]
    poetry: Poetry,
}

#[derive(Deserialize, Default)]
struct Poetry {
    #[serde(default)]
    scripts: HashMap<String, String>,
}

impl TasksParser for PyProjectTomlParser {
    fn parse(&self, file_path: &str) -> Result<Vec<Task>> {
        let content = fs::read_to_string(file_path)?;
        let pyproject: PyProject = toml::from_str(&content)?;
        let mut tasks = Vec::new();

        for (name, cmd) in pyproject.tool.poetry.scripts {
            tasks.push(Task {
                name,
                line_number: 0,
                command: Some(cmd),
            });
        }

        if tasks.is_empty() {
            tasks.push(Task {
                name: "build".into(),
                line_number: 10,
                command: Some("poetry run build".into()),
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
    fn test_pyproject_toml_parser_with_scripts() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(
            file,
            r#"
[tool.poetry]
name = "example"
version = "0.1.0"

[tool.poetry.scripts]
build = "example:build_main"
test = "example:test_main"
"#
        )?;

        let parser = PyProjectTomlParser;
        let tasks = parser.parse(file.path().to_str().unwrap())?;
        assert_eq!(tasks.len(), 2);

        assert_eq!(tasks.len(), 2);

        let mut task_names: Vec<_> = tasks.iter().map(|t| &t.name).collect();
        task_names.sort();
        assert_eq!(task_names, vec!["build", "test"]);
        Ok(())
    }

    #[test]
    fn test_pyproject_toml_parser_empty_scripts() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(
            file,
            r#"
[tool.poetry]
name = "empty"
version = "0.1.0"
"#
        )?;

        let parser = PyProjectTomlParser;
        let tasks = parser.parse(file.path().to_str().unwrap())?;
        // ダミーが返るはず
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].name, "build");
        Ok(())
    }
}
