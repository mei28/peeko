use crate::app::AppState;
use crate::preview::highlighter::{highlight_lines, FileSyntax};
use crate::preview::previewer::get_preview_lines;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::path::Path;

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, state: &AppState) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(size);

    let selected_task = &state.tasks[state.selected_index];
    let center_line = selected_task.line_number;
    let (start_line, snippet) = get_preview_lines(&state.lines, center_line, state.preview_height);

    let ext = Path::new(&state.file_path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let file_syntax = match ext {
        "" if state.file_path.ends_with("Makefile") => FileSyntax::Makefile,
        "json" => FileSyntax::Json,
        "toml" => FileSyntax::Toml,
        _ => FileSyntax::Unknown,
    };

    let mut highlighted = highlight_lines(&snippet, file_syntax);

    // 選択行のハイライト強化
    let selected_line_in_preview = center_line.saturating_sub(start_line);
    if selected_line_in_preview < highlighted.len() {
        let mut spans = highlighted[selected_line_in_preview].0.clone();
        for span in &mut spans {
            // 背景をCyan、前景をBlack、太字を付けて強調表示
            span.style = span
                .style
                .fg(ratatui::style::Color::Black)
                .bg(ratatui::style::Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD);
        }
        highlighted[selected_line_in_preview] = ratatui::text::Spans::from(spans);
    }

    // プレビュー描画
    let preview_block = Block::default().title("Preview").borders(Borders::ALL);
    let preview_paragraph = Paragraph::new(highlighted).block(preview_block);
    f.render_widget(preview_paragraph, chunks[0]);

    // タスクリスト描画
    let items: Vec<ListItem> = state
        .tasks
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let prefix = if i == state.selected_index {
                "> "
            } else {
                "  "
            };
            ListItem::new(format!("{}{}", prefix, t.name))
        })
        .collect();

    let tasks_block = Block::default().title("Tasks").borders(Borders::ALL);
    let tasks_list = List::new(items).block(tasks_block);
    f.render_widget(tasks_list, chunks[1]);
}

