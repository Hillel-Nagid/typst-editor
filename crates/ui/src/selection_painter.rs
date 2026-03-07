use editor_core::VisualLine;
use std::ops::Range;

pub fn visual_selection_ranges(
    visual_line: &VisualLine,
    logical_ranges: &[Range<usize>],
) -> Vec<Range<usize>> {
    let mut out = Vec::new();
    for range in logical_ranges {
        for (start, end) in visual_line.selection_ranges(range.start, range.end) {
            let s = start.round() as usize;
            let e = end.round() as usize;
            if e > s {
                out.push(s..e);
            }
        }
    }
    if out.is_empty() {
        return out;
    }

    out.sort_by_key(|r| r.start);
    let mut merged: Vec<Range<usize>> = Vec::with_capacity(out.len());
    for range in out {
        if let Some(last) = merged.last_mut() {
            if range.start <= last.end {
                last.end = last.end.max(range.end);
                continue;
            }
        }
        merged.push(range);
    }
    merged
}
