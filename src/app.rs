use crate::tasks::task::Task;

pub struct AppState {
    pub tasks: Vec<Task>,
    pub selected_index: usize,
    pub file_path: String,
}

pub struct App {
    state: AppState,
    running: bool,
}

impl App {
    pub fn new(tasks: Vec<Task>, file_path: String) -> anyhow::Result<Self> {
        Ok(Self {
            state: AppState {
                tasks,
                selected_index: 0, // 最初は0番目のタスクを選択中とする
                file_path,
            },
            running: true,
        })
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn next_task(&mut self) {
        if self.state.selected_index + 1 < self.state.tasks.len() {
            self.state.selected_index += 1;
        }
    }

    pub fn prev_task(&mut self) {
        if self.state.selected_index > 0 {
            self.state.selected_index -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::task::Task;

    #[test]
    fn test_app_initialization() {
        let tasks = vec![
            Task { name: "build".into(), line_number: 10, command: None },
            Task { name: "test".into(), line_number: 20, command: None },
        ];
        let app = App::new(tasks, "Makefile".to_string()).unwrap();
        assert_eq!(app.state().file_path, "Makefile");
        assert_eq!(app.state().selected_index, 0);
        assert!(app.is_running());
    }

    #[test]
    fn test_app_task_navigation() {
        let tasks = vec![
            Task { name: "build".into(), line_number: 10, command: None },
            Task { name: "test".into(), line_number: 20, command: None },
        ];
        let mut app = App::new(tasks, "Makefile".into()).unwrap();
        app.next_task();
        assert_eq!(app.state().selected_index, 1);
        app.prev_task();
        assert_eq!(app.state().selected_index, 0);
    }

    #[test]
    fn test_app_quit() {
        let tasks = vec![];
        let mut app = App::new(tasks, "dummy".into()).unwrap();
        app.quit();
        assert!(!app.is_running());
    }
}
