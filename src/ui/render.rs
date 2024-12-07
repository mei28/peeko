use crate::tasks::task::Task;
use ratatui::backend::Backend;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::AppState;

fn format_task_list(tasks: &[Task], selected_idx: usize) -> Vec<String> {
    tasks
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let prefix = if i == selected_idx { "> " } else { "  " };
            format!("{}{}", prefix, t.name)
        })
        .collect()
}
pub fn draw_ui<B: Backend>(f: &mut Frame<B>, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let preview_block = Block::default().title("Preview").borders(Borders::ALL);
    f.render_widget(preview_block, chunks[0]);

    let items: Vec<ListItem> = format_task_list(&state.tasks, state.selected_index)
        .into_iter()
        .map(ListItem::new)
        .collect();

    let tasks_list = List::new(items).block(Block::default().title("Tasks").borders(Borders::ALL));
    f.render_widget(tasks_list, chunks[1]);
}
