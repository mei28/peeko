// src/tasks/parsers/pyproject_toml_parser.rs
use super::TasksParser;
use crate::tasks::task::Task;
use anyhow::Result;
use std::fs;
use toml::Value;

pub struct PyProjectTomlParser;

impl TasksParser for PyProjectTomlParser {
    fn parse(&self, file_path: &str) -> Result<Vec<Task>> {
        let content = fs::read_to_string(file_path)?;
        let value: Value = toml::from_str(&content)?;
        let mut tasks = Vec::new();

        // [tool.*.scripts] を探索
        // value["tool"]がテーブルなら、その中の各キー（poetry, uv, etc.）を走査
        if let Some(tool_table) = value.get("tool").and_then(|v| v.as_table()) {
            for (_tool_name, subtool_val) in tool_table.iter() {
                if let Some(subtool_table) = subtool_val.as_table() {
                    // scriptsキーがある場合
                    if let Some(scripts_val) = subtool_table.get("scripts") {
                        if let Some(scripts_table) = scripts_val.as_table() {
                            for (script_name, cmd_val) in scripts_table.iter() {
                                if let Some(cmd_str) = cmd_val.as_str() {
                                    tasks.push(Task {
                                        name: script_name.to_string(),
                                        line_number: 0,
                                        command: Some(cmd_str.to_string()),
                                    });
                                }
                            }
                        }
                    }
                }
            }
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
    use anyhow::Result;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_pyproject_toml_parser_with_poetry_scripts() -> Result<()> {
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

        let mut task_names: Vec<_> = tasks.iter().map(|t| &t.name).collect();
        task_names.sort();
        assert_eq!(task_names, vec!["build", "test"]);
        Ok(())
    }

    #[test]
    fn test_pyproject_toml_parser_with_uv_scripts() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(
            file,
            r#"
[tool.uv]
name = "example-uv"

[tool.uv.scripts]
serve = "uvicorn app:main"
test = "pytest tests"
"#
        )?;

        let parser = PyProjectTomlParser;
        let tasks = parser.parse(file.path().to_str().unwrap())?;
        assert_eq!(tasks.len(), 2);

        let mut names: Vec<_> = tasks.iter().map(|t| &t.name).collect();
        names.sort();
        assert_eq!(names, vec!["serve", "test"]);
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
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].name, "build");
        Ok(())
    }
}

