pub fn format_to_length(s: &String, length: usize) -> String {
    if s.len() >= length {
        s.chars().take(length).collect()
    } else {
        let padding = length - s.len();
        let left_padding = padding / 2;
        let right_padding = padding - left_padding;
        format!(
            "{}{}{}",
            " ".repeat(left_padding),
            s,
            " ".repeat(right_padding)
        )
    }
}
