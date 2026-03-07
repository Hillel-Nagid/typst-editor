pub fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

pub fn find_prev_word_start(text: &str, offset: usize) -> usize {
    let chars: Vec<char> = text.chars().collect();
    let mut i = offset.min(chars.len());
    while i > 0 && !is_word_char(chars[i - 1]) {
        i -= 1;
    }
    while i > 0 && is_word_char(chars[i - 1]) {
        i -= 1;
    }
    i
}

pub fn find_next_word_start(text: &str, offset: usize) -> usize {
    let chars: Vec<char> = text.chars().collect();
    let mut i = offset.min(chars.len());
    while i < chars.len() && is_word_char(chars[i]) {
        i += 1;
    }
    while i < chars.len() && !is_word_char(chars[i]) {
        i += 1;
    }
    i
}
