pub struct Task {
    pub name: String,
    pub line_number: usize,
    pub command: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Task {
        // テスト用メソッド例
        pub fn is_build_task(&self) -> bool {
            self.name.contains("build")
        }
    }

    #[test]
    fn test_task_creation() {
        let t = Task {
            name: "build".into(),
            line_number: 42,
            command: Some("make build".into()),
        };
        assert_eq!(t.name, "build");
        assert_eq!(t.line_number, 42);
        assert_eq!(t.command.as_deref(), Some("make build"));
    }

    #[test]
    fn test_is_build_task() {
        let t = Task {
            name: "build-test".into(),
            line_number: 10,
            command: None,
        };
        assert!(t.is_build_task());
    }
}

