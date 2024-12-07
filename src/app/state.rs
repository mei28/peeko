use crate::tasks::task::Task;

/// アプリケーション状態を表す構造体
pub struct AppState {
    pub tasks: Vec<Task>,
    pub selected_index: usize,
    pub file_path: String,
    pub lines: Vec<String>,
    pub preview_offset: usize,
    pub preview_height: usize,
}

impl AppState {
    /// プレビューオフセットを更新
    pub fn update_preview_offset(&mut self) {
        let line_num = self.tasks[self.selected_index].line_number;
        let half = self.preview_height / 2;
        self.preview_offset = if line_num > half { line_num - half } else { 0 };
    }
}
