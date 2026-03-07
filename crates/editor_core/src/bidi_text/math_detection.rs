pub fn starts_with_typst_math(line: &str) -> bool {
    line.trim_start().starts_with('$')
}
