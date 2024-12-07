use super::TasksParser;
use crate::tasks::task::Task;
use anyhow::{bail, Result};
use std::fs;
use toml::Value;

pub struct PyProjectTomlParser;

impl TasksParser for PyProjectTomlParser {
    fn parse(&self, file_path: &str) -> Result<Vec<Task>> {
        let content = fs::read_to_string(file_path)?;
        let value: Value = toml::from_str(&content)?;
        let mut tasks = Vec::new();

        if let Some(tool_table) = value.get("tool").and_then(|v| v.as_table()) {
            for (_, subtool_val) in tool_table.iter() {
                if let Some(subtool_table) = subtool_val.as_table() {
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
            bail!("No tasks found in pyproject.toml scripts");
        }

        Ok(tasks)
    }
}

