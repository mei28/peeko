use crate::tasks::task::Task;

pub struct AppState {
    pub tasks: Vec<Task>,
    pub selected_index: usize,
    pub file_path: String,
    pub lines: Vec<String>,
    pub preview_offset: usize,
    pub preview_height: usize,
    pub task_offset: usize,       // タスクリストのスクロールオフセット
    pub max_visible_tasks: usize, // ウィンドウ内で表示可能なタスク数
}

impl AppState {
    pub fn new(
        tasks: Vec<Task>,
        file_path: String,
        lines: Vec<String>,
        preview_height: usize,
    ) -> Self {
        let max_visible_tasks = 10; // 表示可能なタスク数
        AppState {
            tasks,
            selected_index: 0,
            file_path,
            lines,
            preview_offset: 0,
            preview_height,
            task_offset: 0,
            max_visible_tasks,
        }
    }

    /// タスクリストのスクロールオフセットを更新
    pub fn update_task_offset(&mut self) {
        if self.selected_index < self.task_offset {
            self.task_offset = self.selected_index;
        } else if self.selected_index >= self.task_offset + self.max_visible_tasks {
            self.task_offset = self.selected_index + 1 - self.max_visible_tasks;
        }
    }
}

