use crate::app::state::AppState;
use crate::tasks::task::Task;
use anyhow::Result;
use std::fs;

/// アプリケーションロジック
pub struct App {
    pub state: AppState,
    running: bool,
}

impl App {
    /// App インスタンスの初期化
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
                preview_height: 20,
                task_offset: 0,
                max_visible_tasks: 10, // 表示可能なタスク数
            },
            running: true,
        })
    }

    /// 実行中かどうかを確認
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// アプリケーションを終了
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// 次のタスクに移動
    pub fn next_task(&mut self) {
        if self.state.selected_index + 1 < self.state.tasks.len() {
            self.state.selected_index += 1;
            self.state.update_task_offset(); // スクロールオフセットを更新
        }
    }

    /// 前のタスクに移動
    pub fn prev_task(&mut self) {
        if self.state.selected_index > 0 {
            self.state.selected_index -= 1;
            self.state.update_task_offset(); // スクロールオフセットを更新
        }
    }
}

