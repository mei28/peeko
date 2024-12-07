use ratatui::text::Span;

/// Span の一部をスライスする関数
pub fn slice_spans(original: &[Span<'static>], start: usize, end: usize) -> Vec<Span<'static>> {
    let mut result = Vec::new();
    let mut current_pos = 0;

    for span in original {
        let span_len = span.content.len();
        if span_len == 0 {
            continue;
        }

        let span_start = current_pos;
        let span_end = current_pos + span_len;

        if span_end <= start {
            current_pos += span_len;
            continue;
        }

        if span_start >= end {
            break;
        }

        let sub_start = start.saturating_sub(span_start);
        let sub_end = (end - span_start).min(span_len);

        if sub_start < sub_end {
            let sub_str = &span.content.as_ref()[sub_start..sub_end];
            let new_span = Span::styled(sub_str.to_string(), span.style.clone());
            result.push(new_span);
        }

        current_pos += span_len;
    }
    result
}

