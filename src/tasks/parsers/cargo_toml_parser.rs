use super::TasksParser;
use crate::tasks::task::Task;
use anyhow::{bail, Result};
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
            bail!("No tasks found in Cargo.toml metadata");
        }

        Ok(tasks)
    }
}

