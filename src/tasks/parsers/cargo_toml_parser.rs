use super::TasksParser;
use crate::tasks::task::Task;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

pub struct CargoTomlParser;

#[derive(Deserialize, Default)]
struct CargoMeta {
    #[serde(default)]
    package: Package,
}

#[derive(Deserialize, Default)]
struct Package {
    #[serde(default)]
    metadata: Metadata,
}

#[derive(Deserialize, Default)]
struct Metadata {
    #[serde(default)]
    tasks: HashMap<String, String>,
}

impl TasksParser for CargoTomlParser {
    fn parse(&self, file_path: &str) -> Result<Vec<Task>> {
        let content = fs::read_to_string(file_path)?;
        let cargo_meta: CargoMeta = toml::from_str(&content)?;
        let mut tasks = Vec::new();

        for (name, cmd) in cargo_meta.package.metadata.tasks {
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
                command: Some("cargo build".into()),
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
    fn test_cargo_toml_parser_with_tasks() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(
            file,
            r#"
[package]
name = "example"
version = "0.1.0"

[package.metadata.tasks]
build = "cargo build"
test = "cargo test"
"#
        )?;

        let parser = CargoTomlParser;
        let tasks = parser.parse(file.path().to_str().unwrap())?;
        assert_eq!(tasks.len(), 2);

        let mut names: Vec<_> = tasks.iter().map(|t| &t.name).collect();
        names.sort();
        assert_eq!(names, vec!["build", "test"]);

        Ok(())
    }

    #[test]
    fn test_cargo_toml_parser_empty() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        // タスク定義無し
        writeln!(
            file,
            r#"
[package]
name = "no-tasks"
version = "0.1.0"
"#
        )?;

        let parser = CargoTomlParser;
        let tasks = parser.parse(file.path().to_str().unwrap())?;
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].name, "build");
        Ok(())
    }
}

