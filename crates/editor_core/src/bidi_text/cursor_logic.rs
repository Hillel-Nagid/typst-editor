use crate::bidi_text::layout::visual_line::VisualLine;

pub fn move_visual_horizontal(line: &VisualLine, logical_col: usize, delta: isize) -> usize {
    if line.visual_to_logical_cols.is_empty() {
        return logical_col;
    }

    let current_visual = line
        .logical_to_visual_cols
        .get(logical_col.min(line.logical_len))
        .copied()
        .unwrap_or(0);

    let next_visual = if delta < 0 {
        current_visual.saturating_sub(delta.unsigned_abs())
    } else {
        (current_visual + delta as usize).min(line.visual_to_logical_cols.len().saturating_sub(1))
    };

    line.visual_to_logical_cols[next_visual]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bidi_text::Direction;
    use crate::bidi_text::layout_line;

    #[test]
    fn moves_across_mixed_direction_visually() {
        let line = layout_line("abc שלום", Direction::Ltr);
        let moved = move_visual_horizontal(&line, 3, 1);
        assert!(moved <= line.logical_len);
    }
}
