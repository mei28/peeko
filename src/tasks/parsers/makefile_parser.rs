use super::TasksParser;
use crate::tasks::task::Task;
use anyhow::{bail, Result};
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
            bail!("No tasks found in Makefile");
        }

        Ok(tasks)
    }
}

