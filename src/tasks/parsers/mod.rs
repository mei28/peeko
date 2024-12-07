use crate::tasks::task::Task;
use anyhow::Result;

pub trait TasksParser {
    fn parse(&self, file_path: &str) -> Result<Vec<Task>>;
}

mod makefile_parser;
mod package_json_parser;
mod pyproject_toml_parser;

pub use makefile_parser::MakefileParser;
pub use package_json_parser::PackageJsonParser;
pub use pyproject_toml_parser::PyProjectTomlParser;

