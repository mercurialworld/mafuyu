/// Truncates a string.
///
/// I referenced Python code for this, so this might not be Rust-y.
pub fn truncate_string(mut text: String, mut limit: usize, string_end: String) -> String {
    limit -= string_end.len();

    if text.len() >= limit {
        text.truncate(limit);
        text.push_str(&string_end);
        text
    } else {
        text
    }
}
