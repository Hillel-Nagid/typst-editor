use editor_core::VisualLine;

pub fn visual_order_chars(line: &str, visual: &VisualLine) -> Vec<char> {
    let logical_chars: Vec<char> = line.chars().collect();
    let reordered: Vec<char> = visual
        .visual_to_logical_cols
        .iter()
        .map(|logical_col| logical_chars.get(*logical_col).copied().unwrap_or(' '))
        .collect();

    // Guard against invalid mapping fallback that would hide script glyphs.
    if reordered.iter().all(|ch| ch.is_whitespace())
        && logical_chars.iter().any(|ch| !ch.is_whitespace())
    {
        return logical_chars;
    }

    reordered
}
