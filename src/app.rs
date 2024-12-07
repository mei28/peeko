use crate::tasks::task::Task;
use anyhow::Result;
use std::fs;

pub struct AppState {
    pub tasks: Vec<Task>,
    pub selected_index: usize,
    pub file_path: String,
    pub lines: Vec<String>,
    pub preview_offset: usize,
    pub preview_height: usize,
}

pub struct App {
    state: AppState,
    running: bool,
}

impl App {
    pub fn new(tasks: Vec<Task>, file_path: String) -> Result<Self> {
        let content = fs::read_to_string(&file_path).unwrap_or_default();
        let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

        Ok(Self {
            state: AppState {
                tasks,
                selected_index: 0,
                file_path,
                lines,
                preview_offset: 0,
                preview_height: 20, // 仮固定値
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
            self.update_preview_offset();
        }
    }

    pub fn prev_task(&mut self) {
        if self.state.selected_index > 0 {
            self.state.selected_index -= 1;
            self.update_preview_offset();
        }
    }

    fn update_preview_offset(&mut self) {
        let line_num = self.state.tasks[self.state.selected_index].line_number;
        let half = self.state.preview_height / 2;
        let offset = if line_num > half { line_num - half } else { 0 };
        self.state.preview_offset = offset;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::task::Task;

    #[test]
    fn test_app_initialization() {
        let tasks = vec![
            Task {
                name: "build".into(),
                line_number: 10,
                command: None,
            },
            Task {
                name: "test".into(),
                line_number: 20,
                command: None,
            },
        ];
        let app = App::new(tasks, "dummy_file".to_string()).unwrap();
        assert_eq!(app.state().tasks.len(), 2);
        assert_eq!(app.state().selected_index, 0);
        assert!(app.is_running());
    }

    #[test]
    fn test_app_task_navigation() {
        let tasks = vec![
            Task {
                name: "build".into(),
                line_number: 10,
                command: None,
            },
            Task {
                name: "test".into(),
                line_number: 20,
                command: None,
            },
        ];
        let mut app = App::new(tasks, "dummy_file".into()).unwrap();
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

    #[test]
    fn test_update_preview_offset() {
        let tasks = vec![
            Task {
                name: "build".into(),
                line_number: 10,
                command: None,
            },
            Task {
                name: "test".into(),
                line_number: 20,
                command: None,
            },
        ];
        let mut app = App::new(tasks, "dummy".into()).unwrap();

        app.state_mut().preview_height = 10;
        // selected_index=0, line_number=10
        app.prev_task(); // 実質変わらないが呼んでみる
        app.update_preview_offset();
        assert_eq!(app.state().preview_offset, 5); // line_number=10, half=5

        app.next_task();
        // selected_index=1, line_number=20
        // half=5, offset=20-5=15
        assert_eq!(app.state().preview_offset, 15);
    }
}

