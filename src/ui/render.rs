use crate::app::AppState;
use crate::preview::highlighter::{highlight_lines, FileSyntax};
use crate::preview::previewer::get_preview_lines;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::borrow::Cow;
use std::path::Path;

fn file_syntax_from_path(path: &str) -> FileSyntax {
    let ext = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    if path.ends_with("Makefile") {
        FileSyntax::Makefile
    } else if ext == "json" {
        FileSyntax::Json
    } else if ext == "toml" {
        FileSyntax::Toml
    } else {
        FileSyntax::Unknown
    }
}

/// `slice_spans`は、original行スパンから[start..end]の部分文字列を抜き出して新たなSpanを返すヘルパー
fn slice_spans(original: &[Span<'static>], start: usize, end: usize) -> Vec<Span<'static>> {
    let mut result = Vec::new();
    let mut current_pos = 0;
    for span in original {
        let span_len = span.content.len();
        if span_len == 0 {
            continue;
        }

        let full_span_start = current_pos;
        let full_span_end = current_pos + span_len;

        if full_span_end <= start {
            current_pos += span_len;
            continue;
        }

        if full_span_start >= end {
            break;
        }

        let sub_start = start.saturating_sub(full_span_start);
        let sub_end = (end - full_span_start).min(span_len);
        if sub_start < sub_end {
            let sub_str = &span.content.as_ref()[sub_start..sub_end];
            let new_content = Cow::Owned(sub_str.to_string());
            let mut new_span = span.clone();
            new_span.content = new_content;
            result.push(new_span);
        }

        current_pos += span_len;
    }
    result
}

/// 選択中のタスク行で、タスク名やコマンドに該当する部分だけをハイライトする処理
fn highlight_task_line(
    original_spans: &[Span<'static>],
    full_line: &str,
    task_name: &str,
    task_cmd: Option<&str>,
) -> Vec<Span<'static>> {
    let mut highlight_targets = vec![task_name];
    if let Some(cmd) = task_cmd {
        highlight_targets.push(cmd);
    }

    let mut matches = Vec::new();
    for target in &highlight_targets {
        let mut search_start = 0;
        while let Some(pos) = full_line[search_start..].find(target) {
            let absolute_pos = search_start + pos;
            matches.push((absolute_pos, absolute_pos + target.len()));
            search_start = absolute_pos + target.len();
        }
    }
    matches.sort_by_key(|m| m.0);

    let highlight_style = Style::default()
        .fg(Color::Rgb(30, 30, 30))
        .bg(Color::Rgb(195, 232, 141))
        .add_modifier(Modifier::BOLD | Modifier::ITALIC);

    let line_len = full_line.len();
    let mut new_spans = Vec::new();
    let mut current_pos = 0;

    for (m_start, m_end) in matches {
        if m_start > current_pos {
            let normal_spans = slice_spans(original_spans, current_pos, m_start);
            new_spans.extend(normal_spans);
        }
        let highlight_sub = slice_spans(original_spans, m_start, m_end);
        let mut highlighted_sub: Vec<Span> = highlight_sub
            .into_iter()
            .map(|mut s| {
                s.style = s.style.patch(highlight_style);
                s
            })
            .collect();
        new_spans.append(&mut highlighted_sub);
        current_pos = m_end;
    }

    if current_pos < line_len {
        let normal_spans = slice_spans(original_spans, current_pos, line_len);
        new_spans.extend(normal_spans);
    }

    new_spans
}

/// 指定したファイルパスと行テキストsnippetに対してハイライト済みSpansを取得する
fn get_highlighted_spans(snippet: &[&str], file_path: &str) -> Vec<Spans<'static>> {
    let file_syntax = file_syntax_from_path(file_path);
    highlight_lines(snippet, file_syntax)
}

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, state: &AppState) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(size);

    let selected_task = &state.tasks[state.selected_index];
    let center_line = selected_task.line_number;
    let (start_line, snippet) = get_preview_lines(&state.lines, center_line, state.preview_height);

    let mut highlighted = get_highlighted_spans(&snippet, &state.file_path);

    // 選択行の箇所的ハイライト
    let selected_line_in_preview = center_line.saturating_sub(start_line);
    if selected_line_in_preview < highlighted.len() {
        // line_spansをクローン: Spans<'static>→Vec<Span<'static>>
        let line_spans_cloned: Vec<Span<'static>> = highlighted[selected_line_in_preview]
            .0
            .iter()
            .cloned()
            .collect();
        let full_line: String = line_spans_cloned
            .iter()
            .map(|s| s.content.as_ref())
            .collect();

        let new_spans = highlight_task_line(
            &line_spans_cloned,
            &full_line,
            &selected_task.name,
            selected_task.command.as_deref(),
        );
        highlighted[selected_line_in_preview] = Spans::from(new_spans);
    }

    // プレビュー
    let preview_block = Block::default()
        .title("📜 Preview")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    let preview_paragraph = Paragraph::new(highlighted).block(preview_block);
    f.render_widget(preview_paragraph, chunks[0]);

    // タスクリスト
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Modifier;

    #[test]
    fn test_file_syntax_from_path() {
        assert_eq!(file_syntax_from_path("Makefile"), FileSyntax::Makefile);
        assert_eq!(file_syntax_from_path("project.json"), FileSyntax::Json);
        assert_eq!(file_syntax_from_path("config.toml"), FileSyntax::Toml);
        assert_eq!(file_syntax_from_path("unknown.txt"), FileSyntax::Unknown);
    }

    #[test]
    fn test_highlight_task_line() {
        let original_spans: Vec<Span<'static>> = vec![
            Span::raw("This is a build task: "),
            Span::raw("build"),
            Span::raw(" and we run it."),
        ];
        let full_line = "This is a build task: build and we run it.";

        let result = highlight_task_line(&original_spans, full_line, "build", None);

        // "build"がハイライトされているはず
        let mut build_found = false;
        for span in &result {
            if span.content.as_ref() == "build" {
                build_found = true;
                // スタイルがBOLD|ITALICついてるか簡易チェック
                assert!(span.style.add_modifier.contains(Modifier::BOLD));
                assert!(span.style.add_modifier.contains(Modifier::ITALIC));
            }
        }
        assert!(build_found, "build substring should be highlighted");
    }
}

