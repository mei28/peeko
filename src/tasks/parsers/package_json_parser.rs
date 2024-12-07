use super::TasksParser;
use crate::tasks::task::Task;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

pub struct PackageJsonParser;

#[derive(Deserialize)]
struct PackageJson {
    scripts: Option<HashMap<String, String>>,
}

impl TasksParser for PackageJsonParser {
    fn parse(&self, file_path: &str) -> Result<Vec<Task>> {
        let content = fs::read_to_string(file_path)?;
        let package: PackageJson = serde_json::from_str(&content)?;
        let mut tasks = Vec::new();
        if let Some(scripts) = package.scripts {
            // scriptsのキーをタスク名、値をコマンドとする
            for (name, cmd) in scripts {
                // 行番号は不明なため0で仮指定
                tasks.push(Task {
                    name,
                    line_number: 0,
                    command: Some(cmd),
                });
            }
        }

        if tasks.is_empty() {
            // スクリプトがなければダミー
            tasks.push(Task {
                name: "build".into(),
                line_number: 10,
                command: Some("npm run build".into()),
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
    fn test_package_json_parser_with_scripts() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(
            file,
            "{}",
            r####"{ "scripts": { "build": "echo build", "test": "echo test" } }"####
        )?;
        let parser = PackageJsonParser;
        let tasks = parser.parse(file.path().to_str().unwrap())?;
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].name, "build");
        assert_eq!(tasks[1].name, "test");
        Ok(())
    }

    #[test]
    fn test_package_json_parser_empty_scripts() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "{{}}")?; // {}をエスケープする方法

        let parser = PackageJsonParser;
        let tasks = parser.parse(file.path().to_str().unwrap())?;
        // ダミーが返るはず
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].name, "build");
        Ok(())
    }
}
