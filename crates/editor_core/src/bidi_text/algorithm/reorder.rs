use crate::bidi_text::algorithm::types::BidiInfo;

pub fn visual_run_ranges(info: &BidiInfo) -> Vec<std::ops::Range<usize>> {
    info.runs.iter().map(|run| run.start..run.end).collect()
}
