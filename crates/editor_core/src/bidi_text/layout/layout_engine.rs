use crate::bidi_text::algorithm::types::Direction;
use crate::bidi_text::algorithm::uax9::analyze_bidi;
use crate::bidi_text::layout::visual_line::{VisualLine, VisualRun};

fn byte_to_char_col(text: &str, byte_idx: usize) -> usize {
    text[..byte_idx.min(text.len())].chars().count()
}

fn run_char_cols(line: &str, run_start: usize, run_end: usize) -> Vec<usize> {
    let mut cols = Vec::new();
    let mut char_indices = line.char_indices().peekable();
    let mut logical_col = 0usize;

    while let Some((byte_start, _)) = char_indices.next() {
        let byte_end = char_indices
            .peek()
            .map(|(next_start, _)| *next_start)
            .unwrap_or(line.len());
        let overlaps_run = byte_end > run_start && byte_start < run_end;
        if overlaps_run {
            cols.push(logical_col);
        }
        logical_col += 1;
    }

    cols
}

fn normalize_visual_map(mut visual_to_logical_cols: Vec<usize>, logical_len: usize) -> Vec<usize> {
    visual_to_logical_cols.retain(|col| *col < logical_len);

    let mut seen = vec![false; logical_len];
    let mut normalized = Vec::with_capacity(logical_len);
    for col in visual_to_logical_cols {
        if !seen[col] {
            normalized.push(col);
            seen[col] = true;
        }
    }

    for (col, already_seen) in seen.into_iter().enumerate() {
        if !already_seen {
            normalized.push(col);
        }
    }

    normalized
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
        let mut run_cols = run_char_cols(line, run.start, run.end);
        if run_cols.is_empty() && end_col > start_col {
            run_cols = (start_col..end_col).collect();
        }
        if matches!(run.direction, Direction::Rtl) {
            run_cols.reverse();
        }
        let run_text: String = run_cols
            .iter()
            .map(|col| logical_chars.get(*col).copied().unwrap_or(' '))
            .collect();
        for col in &run_cols {
            visual_to_logical_cols.push(*col);
        }
        let width = run_cols.len() as f32;
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

    visual_to_logical_cols = normalize_visual_map(visual_to_logical_cols, logical_len);
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
    use std::collections::HashSet;

    #[test]
    fn logical_visual_round_trip() {
        let line = layout_line("abc שלום", Direction::Ltr);
        for logical in 0..line.logical_len {
            let visual = line.logical_to_visual(logical);
            let back = line.visual_to_logical(visual);
            assert!(back <= line.logical_len);
        }
    }

    #[test]
    fn mixed_line_maps_all_logical_columns_once() {
        let line = layout_line("fffr כצחקכגקחכג fmdkf", Direction::Ltr);
        assert_eq!(line.visual_to_logical_cols.len(), line.logical_len);
        let unique: HashSet<usize> = line.visual_to_logical_cols.iter().copied().collect();
        assert_eq!(unique.len(), line.logical_len);
        for col in 0..line.logical_len {
            assert!(unique.contains(&col));
        }
    }
}
