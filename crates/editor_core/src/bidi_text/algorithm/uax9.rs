use crate::bidi_text::algorithm::types::{BidiInfo, BidiRun, Direction};
use unicode_bidi::{BidiInfo as UnicodeBidiInfo, LTR_LEVEL, RTL_LEVEL};

pub fn analyze_bidi(text: &str, default_direction: Direction) -> BidiInfo {
    if text.is_empty() {
        return BidiInfo {
            levels: Vec::new(),
            paragraph_level: matches!(default_direction, Direction::Rtl) as u8,
            runs: Vec::new(),
        };
    }

    let default_level = if matches!(default_direction, Direction::Rtl) {
        Some(RTL_LEVEL)
    } else {
        Some(LTR_LEVEL)
    };
    let bidi = UnicodeBidiInfo::new(text, default_level);
    let para = bidi.paragraphs.first().cloned().unwrap_or(unicode_bidi::ParagraphInfo {
        range: 0..text.len(),
        level: default_level.unwrap_or(LTR_LEVEL),
    });
    let paragraph_level = para.level.number();
    let (levels, runs) = bidi.visual_runs(&para, para.range.clone());

    let bidi_runs = runs
        .into_iter()
        .map(|run| {
            let level = levels.get(run.start).map(|lvl| lvl.number()).unwrap_or(paragraph_level);
            BidiRun {
                start: run.start,
                end: run.end,
                level,
                direction: if level % 2 == 0 {
                    Direction::Ltr
                } else {
                    Direction::Rtl
                },
            }
        })
        .collect();

    BidiInfo {
        levels: levels.into_iter().map(|lvl| lvl.number()).collect(),
        paragraph_level,
        runs: bidi_runs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analyzes_mixed_direction_runs() {
        let text = "abc שלום";
        let info = analyze_bidi(text, Direction::Ltr);
        assert!(!info.runs.is_empty());
        assert!(info.levels.len() >= text.len());
    }
}
