//! Fitting text into a column: cut with `...`, or wrapped onto more lines.
//! Measured in terminal columns, not characters: a Chinese, Japanese or
//! Korean character takes two.

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::configs::ELLIPSIS;

/// How many columns `text` takes on screen.
pub fn width(text: &str) -> usize {
    text.width()
}

/// `text` cut to at most `columns`, ending in `...` when it was cut. A
/// column narrower than the ellipsis gets as many dots as fit.
pub fn fit(text: &str, columns: usize) -> String {
    if text.width() <= columns {
        return text.to_owned();
    }
    let dots = ELLIPSIS.len();
    if columns <= dots {
        return ".".repeat(columns);
    }
    take_columns(text, columns - dots).0.to_owned() + ELLIPSIS
}

/// The longest start of `text` that fits in `columns`, and the rest.
pub fn take_columns(text: &str, columns: usize) -> (&str, &str) {
    let mut used = 0;
    for (at, c) in text.char_indices() {
        used += c.width().unwrap_or(0);
        if used > columns {
            return text.split_at(at);
        }
    }
    (text, "")
}

/// `text` broken into lines of at most `columns`, at spaces; a word wider
/// than a line is split (Chinese and Japanese have no spaces, so a sentence
/// is one long word). An empty text gives one empty line.
pub fn wrap(text: &str, columns: usize) -> Vec<String> {
    let columns = columns.max(2); // room for one double-width character
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        let mut word = word;
        // a word too wide for any line: fill lines with pieces of it
        while word.width() > columns {
            if !line.is_empty() {
                lines.push(std::mem::take(&mut line));
            }
            let (piece, rest) = take_columns(word, columns);
            lines.push(piece.to_owned());
            word = rest;
        }
        let needed = usize::from(!line.is_empty()) + word.width();
        if line.width() + needed > columns {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() || lines.is_empty() {
        lines.push(line);
    }
    lines
}

/// `text` then spaces up to `columns` columns (none when it is wider):
/// labels lined up, in any script.
pub fn pad(text: &str, columns: usize) -> String {
    format!("{text}{}", " ".repeat(columns.saturating_sub(width(text))))
}
