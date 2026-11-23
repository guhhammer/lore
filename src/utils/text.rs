//! Fitting text into a column: cut with `...`, or wrapped onto more lines.

/// The ending that marks text cut short.
pub const ELLIPSIS: &str = "...";

/// `text` cut to at most `width` characters, ending in `...` when it was cut.
/// A column narrower than the ellipsis gets as many dots as fit.
pub fn fit(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        return text.to_owned();
    }
    let dots = ELLIPSIS.len();
    if width <= dots {
        return ".".repeat(width);
    }
    let kept: String = text.chars().take(width - dots).collect();
    kept + ELLIPSIS
}

/// `text` broken into lines of at most `width` characters, at spaces; a word
/// longer than the width is split. An empty text gives one empty line.
pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        let mut word: Vec<char> = word.chars().collect();
        // a word too long for any line: fill lines with pieces of it
        while word.len() > width {
            if !line.is_empty() {
                lines.push(std::mem::take(&mut line));
            }
            lines.push(word.drain(..width).collect());
        }
        let word: String = word.into_iter().collect();
        let needed = if line.is_empty() { 0 } else { 1 } + word.chars().count();
        if line.chars().count() + needed > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(&word);
    }
    if !line.is_empty() || lines.is_empty() {
        lines.push(line);
    }
    lines
}
