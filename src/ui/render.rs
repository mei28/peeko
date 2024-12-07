use crate::app::AppState;
use crate::preview::highlighter::{highlight_lines, FileSyntax};
use crate::preview::previewer::get_preview_lines;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Spans,
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

    // 選択行ハイライト
    let selected_line_in_preview = center_line.saturating_sub(start_line);
    if selected_line_in_preview < highlighted.len() {
        let mut spans = highlighted[selected_line_in_preview].0.clone();
        for span in &mut spans {
            span.style = span
                .style
                .fg(Color::Rgb(30, 30, 30)) // ダークグレー文字
                .bg(Color::Rgb(195, 232, 141)) // パステルグリーン背景
                .add_modifier(Modifier::BOLD | Modifier::ITALIC);
        }
        highlighted[selected_line_in_preview] = Spans::from(spans);
    }

    // プレビュー枠
    let preview_block = Block::default()
        .title("📜 Preview")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    let preview_paragraph = Paragraph::new(highlighted).block(preview_block);
    f.render_widget(preview_paragraph, chunks[0]);

    // タスクリスト
    // 選択中: "❯ "、非選択: "  "
    let items: Vec<ListItem> = state
        .tasks
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let prefix = if i == state.selected_index {
                "❯ "
            } else {
                "· "
            };
            ListItem::new(format!("{}{}", prefix, t.name))
        })
        .collect();

    let tasks_block = Block::default()
        .title("⚙ Tasks")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    let tasks_list = List::new(items).block(tasks_block);
    f.render_widget(tasks_list, chunks[1]);
}

