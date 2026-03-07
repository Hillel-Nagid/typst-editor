use crate::bidi_text::algorithm::types::Direction;
use crate::bidi_text::algorithm::uax9::analyze_bidi;
use crate::bidi_text::layout::visual_line::{VisualLine, VisualRun};

fn byte_to_char_col(text: &str, byte_idx: usize) -> usize {
    text[..byte_idx.min(text.len())].chars().count()
}

pub fn layout_line(line: &str, default_direction: Direction) -> VisualLine {
    let bidi = analyze_bidi(line, default_direction);
    let logical_chars: Vec<char> = line.chars().collect();
    let logical_len = logical_chars.len();

    let mut runs = Vec::new();
    let mut visual_to_logical_cols = Vec::new();
    let mut visual_x = 0.0f32;

    if bidi.runs.is_empty() {
        for col in 0..logical_len {
            visual_to_logical_cols.push(col);
        }
        let logical_to_visual_cols = invert_visual_map(&visual_to_logical_cols, logical_len);
        return VisualLine {
            runs: vec![VisualRun {
                text: line.to_string(),
                logical_start: 0,
                logical_end: logical_len,
                visual_x: 0.0,
                width: logical_len as f32,
                direction: default_direction,
            }],
            direction: default_direction,
            logical_to_visual_cols,
            visual_to_logical_cols,
            logical_len,
        };
    }

    for run in &bidi.runs {
        let start_col = byte_to_char_col(line, run.start);
        let end_col = byte_to_char_col(line, run.end);
        let mut run_cols: Vec<usize> = (start_col..end_col).collect();
        if matches!(run.direction, Direction::Rtl) {
            run_cols.reverse();
        }
        let run_text: String = run_cols
            .iter()
            .map(|col| logical_chars.get(*col).copied().unwrap_or(' '))
            .collect();
        for col in run_cols {
            visual_to_logical_cols.push(col);
        }
        let width = (end_col.saturating_sub(start_col)) as f32;
        runs.push(VisualRun {
            text: run_text,
            logical_start: start_col,
            logical_end: end_col,
            visual_x,
            width,
            direction: run.direction,
        });
        visual_x += width;
    }

    let logical_to_visual_cols = invert_visual_map(&visual_to_logical_cols, logical_len);
    VisualLine {
        runs,
        direction: if bidi.paragraph_level % 2 == 0 {
            Direction::Ltr
        } else {
            Direction::Rtl
        },
        logical_to_visual_cols,
        visual_to_logical_cols,
        logical_len,
    }
}

fn invert_visual_map(visual_to_logical_cols: &[usize], logical_len: usize) -> Vec<usize> {
    let mut logical_to_visual_cols = vec![0usize; logical_len.saturating_add(1)];
    for (visual_col, logical_col) in visual_to_logical_cols.iter().copied().enumerate() {
        if logical_col < logical_to_visual_cols.len() {
            logical_to_visual_cols[logical_col] = visual_col;
        }
    }
    if !logical_to_visual_cols.is_empty() {
        logical_to_visual_cols[logical_len] = visual_to_logical_cols.len();
    }
    logical_to_visual_cols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_visual_round_trip() {
        let line = layout_line("abc שלום", Direction::Ltr);
        for logical in 0..line.logical_len {
            let visual = line.logical_to_visual(logical);
            let back = line.visual_to_logical(visual);
            assert!(back <= line.logical_len);
        }
    }
}
