use crate::tasks::parsers::{MakefileParser, PackageJsonParser, PyProjectTomlParser, TasksParser};
use anyhow::Result;
use std::path::Path;

pub enum FileType {
    Makefile,
    PackageJson,
    PyProjectToml,
    Unknown,
}

pub fn detect_file_type(file_path: &str) -> FileType {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    if file_path.ends_with("Makefile") {
        FileType::Makefile
    } else if ext == "json" {
        FileType::PackageJson
    } else if ext == "toml" {
        FileType::PyProjectToml
    } else {
        FileType::Unknown
    }
}

pub fn get_parser(file_type: FileType) -> Box<dyn TasksParser> {
    match file_type {
        FileType::Makefile => Box::new(MakefileParser),
        FileType::PackageJson => Box::new(PackageJsonParser),
        FileType::PyProjectToml => Box::new(PyProjectTomlParser),
        FileType::Unknown => {
            // ダミーのパーサを返してもよい
            Box::new(MakefileParser)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_file_type_makefile() {
        assert!(matches!(detect_file_type("SomeMakefile"), FileType::Makefile));
    }

    #[test]
    fn test_detect_file_type_json() {
        assert!(matches!(detect_file_type("package.json"), FileType::PackageJson));
    }

    #[test]
    fn test_detect_file_type_toml() {
        assert!(matches!(detect_file_type("pyproject.toml"), FileType::PyProjectToml));
    }

    #[test]
    fn test_detect_file_type_unknown() {
        assert!(matches!(detect_file_type("something.txt"), FileType::Unknown));
    }
}
