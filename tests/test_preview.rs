use peeko::preview::previewer::get_preview_lines;

#[test]
fn test_get_preview_lines() {
    let lines: Vec<String> = (1..=100).map(|i| format!("Line {}", i)).collect();
    let center_line = 50;
    let preview_height = 10;

    let (start, snippet) = get_preview_lines(&lines, center_line, preview_height);
    assert_eq!(start, 45); // center_line - preview_height / 2
    assert_eq!(snippet.len(), preview_height);
    assert_eq!(snippet[0], "Line 46");
    assert_eq!(snippet[9], "Line 55");
}
