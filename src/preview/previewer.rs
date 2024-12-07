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

#[cfg(test)]
mod tests {
    use super::get_preview_lines;

    #[test]
    fn test_get_preview_lines_basic() {
        let lines = vec![
            "line0".to_string(),
            "line1".to_string(),
            "line2".to_string(),
            "line3".to_string(),
            "line4".to_string(),
        ];
        // center_line=2, preview_height=3 => half=1
        // start=2-1=1, end=1+3=4
        // snippet: line1,line2,line3
        let (start, snippet) = get_preview_lines(&lines, 2, 3);
        assert_eq!(start, 1);
        assert_eq!(snippet, vec!["line1", "line2", "line3"]);

        // center_line=0で先頭行
        let (start, snippet) = get_preview_lines(&lines, 0, 3);
        // half=1, line_num=0なので start=0
        // snippet: line0,line1,line2
        assert_eq!(start, 0);
        assert_eq!(snippet, vec!["line0", "line1", "line2"]);

        // center_lineが末尾近く(4)
        let (start, snippet) = get_preview_lines(&lines, 4, 3);
        // half=1, line_num=4 => start=4-1=3
        // snippet: line3,line4
        assert_eq!(start, 3);
        assert_eq!(snippet, vec!["line3", "line4"]);
    }

    #[test]
    fn test_get_preview_lines_empty() {
        let lines: Vec<String> = vec![];
        let (start, snippet) = get_preview_lines(&lines, 0, 5);
        assert_eq!(start, 0);
        assert!(snippet.is_empty());
    }
}
