pub struct Task {
    pub name: String,
    pub line_number: usize,
    pub command: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let t = Task {
            name: "build".to_string(),
            line_number: 42,
            command: Some("make build".to_string()),
        };
        assert_eq!(t.name, "build");
        assert_eq!(t.line_number, 42);
        assert_eq!(t.command.as_deref(), Some("make build"));
    }

    #[test]
    fn test_is_build_task() {
        let t = Task {
            name: "build-test".to_string(),
            line_number: 10,
            command: None,
        };
        // 仮にis_build_taskがあると想定
        // assert!(t.is_build_task());
    }
}
