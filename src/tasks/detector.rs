use crate::tasks::parsers::{
    CargoTomlParser, MakefileParser, PackageJsonParser, PyProjectTomlParser, TasksParser,
};
use std::path::Path;

pub enum FileType {
    Makefile,
    PackageJson,
    PyProjectToml,
    CargoToml,
    Unknown,
}

pub fn detect_file_type(file_path: &str) -> FileType {
    let path = Path::new(file_path);
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    if file_name == "Makefile" {
        FileType::Makefile
    } else if file_name == "package.json" {
        FileType::PackageJson
    } else if file_name == "pyproject.toml" {
        FileType::PyProjectToml
    } else if file_name == "Cargo.toml" {
        FileType::CargoToml
    } else {
        match ext {
            "json" => FileType::PackageJson,
            "toml" => {
                // tomlだけど、pyprojectやCargoじゃない場合はUnknown扱いでも可
                // とりあえずUnknown
                FileType::Unknown
            }
            _ => FileType::Unknown,
        }
    }
}

pub fn get_parser(file_type: FileType) -> Box<dyn TasksParser> {
    match file_type {
        FileType::Makefile => Box::new(MakefileParser),
        FileType::PackageJson => Box::new(PackageJsonParser),
        FileType::PyProjectToml => Box::new(PyProjectTomlParser),
        FileType::CargoToml => Box::new(CargoTomlParser),
        FileType::Unknown => {
            // ダミーとしてMakefileParserを返す
            Box::new(MakefileParser)
        }
    }
}

