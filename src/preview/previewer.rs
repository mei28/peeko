pub fn get_preview_lines(
    lines: &[String],
    center_line: usize,
    preview_height: usize,
) -> (usize, Vec<&str>) {
    let half = preview_height / 2;
    let start = if center_line > half {
        center_line - half
    } else {
        0
    };
    let end = std::cmp::min(start + preview_height, lines.len());
    let snippet = &lines[start..end];
    (start, snippet.iter().map(|s| &**s).collect())
}
