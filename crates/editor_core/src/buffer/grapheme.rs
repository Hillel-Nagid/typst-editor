use unicode_segmentation::UnicodeSegmentation;

fn byte_to_char_offset(text: &str, byte_offset: usize) -> usize {
    text[..byte_offset].chars().count()
}

pub fn char_to_byte_offset(text: &str, char_offset: usize) -> usize {
    if char_offset == 0 {
        return 0;
    }
    text.char_indices()
        .nth(char_offset)
        .map(|(idx, _)| idx)
        .unwrap_or(text.len())
}

pub fn next_grapheme_offset(text: &str, char_offset: usize) -> usize {
    let current_byte = char_to_byte_offset(text, char_offset);
    for (byte_idx, _) in text.grapheme_indices(true) {
        if byte_idx > current_byte {
            return byte_to_char_offset(text, byte_idx);
        }
    }
    text.chars().count()
}

pub fn prev_grapheme_offset(text: &str, char_offset: usize) -> usize {
    let current_byte = char_to_byte_offset(text, char_offset);
    let mut prev = 0;
    for (byte_idx, _) in text.grapheme_indices(true) {
        if byte_idx >= current_byte {
            break;
        }
        prev = byte_idx;
    }
    byte_to_char_offset(text, prev)
}

pub fn grapheme_at_char_offset(text: &str, char_offset: usize) -> Option<String> {
    let target_byte = char_to_byte_offset(text, char_offset);
    for (idx, grapheme) in text.grapheme_indices(true) {
        let end = idx + grapheme.len();
        if target_byte >= idx && target_byte < end {
            return Some(grapheme.to_string());
        }
    }
    None
}
