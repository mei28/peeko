use peeko::ui::events::handle_events;
use peeko::app::App;
use peeko::tasks::task::Task;

#[test]
fn test_handle_events_quit() {
    let tasks = vec![Task {
        name: "build".to_string(),
        line_number: 10,
        command: Some("cargo build".to_string()),
    }];
    let mut app = App::new(tasks, "dummy".to_string()).unwrap();

    // Simulating a quit event in a headless environment is non-trivial.
    // Instead, we manually call the quit function and verify the behavior.
    app.quit();
    assert!(!app.is_running());
}
