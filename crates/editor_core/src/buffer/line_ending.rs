use crate::document::LineEnding;

pub fn detect_line_ending(text: &str) -> LineEnding {
    if text.contains("\r\n") {
        LineEnding::CrLf
    } else if text.contains('\r') {
        LineEnding::Cr
    } else {
        LineEnding::Lf
    }
}

pub fn normalize_line_endings(text: &str, line_ending: LineEnding) -> String {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    if matches!(line_ending, LineEnding::Lf) {
        normalized
    } else {
        normalized.replace('\n', line_ending.as_str())
    }
}
