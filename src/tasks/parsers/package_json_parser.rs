use super::TasksParser;
use crate::tasks::task::Task;
use anyhow::{bail, Result};
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
            for (name, cmd) in scripts {
                tasks.push(Task {
                    name,
                    line_number: 0,
                    command: Some(cmd),
                });
            }
        }

        if tasks.is_empty() {
            bail!("No tasks found in package.json scripts");
        }

        Ok(tasks)
    }
}

