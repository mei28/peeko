use peeko::app::{App, AppState};
use peeko::tasks::task::Task;

#[test]
fn test_app_initialization() {
    let tasks = vec![
        Task {
            name: "build".to_string(),
            line_number: 10,
            command: Some("cargo build".to_string()),
        },
        Task {
            name: "test".to_string(),
            line_number: 20,
            command: Some("cargo test".to_string()),
        },
    ];

    let app = App::new(tasks.clone(), "dummy_file".to_string()).unwrap();
    assert_eq!(app.state.tasks.len(), tasks.len());
    assert_eq!(app.state.selected_index, 0);
    assert!(app.is_running());
}

#[test]
fn test_app_quit() {
    let tasks = vec![];
    let mut app = App::new(tasks, "dummy".to_string()).unwrap();
    app.quit();
    assert!(!app.is_running());
}

#[test]
fn test_app_navigation() {
    let tasks = vec![
        Task {
            name: "build".to_string(),
            line_number: 10,
            command: Some("cargo build".to_string()),
        },
        Task {
            name: "test".to_string(),
            line_number: 20,
            command: Some("cargo test".to_string()),
        },
    ];

    let mut app = App::new(tasks, "dummy".to_string()).unwrap();
    app.next_task();
    assert_eq!(app.state.selected_index, 1);

    app.prev_task();
    assert_eq!(app.state.selected_index, 0);
}
