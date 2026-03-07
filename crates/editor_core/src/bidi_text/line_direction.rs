use crate::bidi_text::algorithm::types::Direction;
use crate::bidi_text::math_detection::starts_with_typst_math;
use unicode_bidi::{BidiClass, bidi_class};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineDirection {
    Ltr,
    Rtl,
    Math,
}

#[derive(Default)]
pub struct LineDirectionDetector;

impl LineDirectionDetector {
    pub fn detect(&self, line: &str, fallback: LineDirection) -> LineDirection {
        if starts_with_typst_math(line) {
            return LineDirection::Math;
        }

        for ch in line.chars() {
            if ch.is_whitespace() {
                continue;
            }
            match bidi_class(ch) {
                BidiClass::R | BidiClass::AL => return LineDirection::Rtl,
                BidiClass::L => return LineDirection::Ltr,
                _ => {}
            }
        }
        fallback
    }
}

pub fn detect_line_directions(lines: &[String], default: Direction) -> Vec<LineDirection> {
    let detector = LineDirectionDetector;
    let mut last_non_empty = if matches!(default, Direction::Rtl) {
        LineDirection::Rtl
    } else {
        LineDirection::Ltr
    };
    let mut out = Vec::with_capacity(lines.len());
    for line in lines {
        let fallback = if line.trim().is_empty() {
            last_non_empty
        } else {
            match default {
                Direction::Ltr => LineDirection::Ltr,
                Direction::Rtl => LineDirection::Rtl,
            }
        };
        let dir = detector.detect(line, fallback);
        if !line.trim().is_empty() {
            last_non_empty = dir;
        }
        out.push(dir);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_math_as_ltr() {
        let detector = LineDirectionDetector;
        assert_eq!(detector.detect("  $שלום$", LineDirection::Rtl), LineDirection::Math);
    }

    #[test]
    fn empty_line_inherits_previous_direction() {
        let dirs = detect_line_directions(
            &["שלום".to_string(), "".to_string(), "abc".to_string()],
            Direction::Ltr,
        );
        assert_eq!(dirs[0], LineDirection::Rtl);
        assert_eq!(dirs[1], LineDirection::Rtl);
        assert_eq!(dirs[2], LineDirection::Ltr);
    }
}
