use ratatui::text::{Span, Spans};
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;

/// ファイルの種類を表す列挙型
pub enum FileSyntax {
    Makefile,
    Json,
    Toml,
    Unknown,
}

/// 指定された行をシンタックスハイライトする
pub fn highlight_lines(lines: &[&str], syntax: FileSyntax) -> Vec<Spans<'static>> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax_name = match syntax {
        FileSyntax::Makefile => "Makefile",
        FileSyntax::Json => "JSON",
        FileSyntax::Toml => "TOML",
        _ => "",
    };

    let syntax_ref = ss
        .find_syntax_by_name(syntax_name)
        .unwrap_or(ss.find_syntax_plain_text());

    let theme = &ts.themes["base16-eighties.dark"];

    lines
        .iter()
        .map(|line| {
            let mut h = syntect::easy::HighlightLines::new(syntax_ref, theme);
            let ranges = h.highlight_line(line, &ss).unwrap();

            let spans: Vec<Span> = ranges
                .iter()
                .map(|(style, text)| {
                    let r = style.foreground.r;
                    let g = style.foreground.g;
                    let b = style.foreground.b;
                    Span::styled(
                        text.to_string(),
                        ratatui::style::Style::default().fg(ratatui::style::Color::Rgb(r, g, b)),
                    )
                })
                .collect();

            Spans::from(spans)
        })
        .collect()
}

