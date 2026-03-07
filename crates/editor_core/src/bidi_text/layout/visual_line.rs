use crate::bidi_text::algorithm::types::Direction;

#[derive(Debug, Clone)]
pub struct VisualRun {
    pub text: String,
    pub logical_start: usize,
    pub logical_end: usize,
    pub visual_x: f32,
    pub width: f32,
    pub direction: Direction,
}

#[derive(Debug, Clone)]
pub struct VisualLine {
    pub runs: Vec<VisualRun>,
    pub direction: Direction,
    pub logical_to_visual_cols: Vec<usize>,
    pub visual_to_logical_cols: Vec<usize>,
    pub logical_len: usize,
}

impl VisualLine {
    pub fn logical_to_visual(&self, logical_offset: usize) -> f32 {
        let logical = logical_offset.min(self.logical_len);
        let visual_col = self
            .logical_to_visual_cols
            .get(logical)
            .copied()
            .unwrap_or(self.visual_to_logical_cols.len());
        visual_col as f32
    }

    pub fn visual_to_logical(&self, visual_x: f32) -> usize {
        if self.visual_to_logical_cols.is_empty() {
            return 0;
        }
        let col = visual_x.max(0.0).round() as usize;
        let clamped = col.min(self.visual_to_logical_cols.len().saturating_sub(1));
        self.visual_to_logical_cols[clamped]
    }

    pub fn selection_ranges(&self, start: usize, end: usize) -> Vec<(f32, f32)> {
        if start >= end {
            return Vec::new();
        }
        let mut ranges = Vec::new();
        let mut run_start: Option<usize> = None;
        for visual_col in 0..self.visual_to_logical_cols.len() {
            let logical = self.visual_to_logical_cols[visual_col];
            let selected = logical >= start && logical < end;
            match (run_start, selected) {
                (None, true) => run_start = Some(visual_col),
                (Some(s), false) => {
                    ranges.push((s as f32, visual_col as f32));
                    run_start = None;
                }
                _ => {}
            }
        }
        if let Some(s) = run_start {
            ranges.push((s as f32, self.visual_to_logical_cols.len() as f32));
        }
        ranges
    }
}
