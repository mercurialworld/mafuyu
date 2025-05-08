/// Truncates a string.
///
/// I referenced Python code for this, so this might not be Rust-y.
pub fn truncate_string(mut text: String, limit: Option<usize>, string_end: String) -> String {
    match limit {
        Some(mut l) => {
            l -= string_end.len();

            if text.len() >= l {
                text.truncate(l);
                text.push_str(&string_end);
                text
            } else {
                text
            }
        }
        None => text,
    }
}
