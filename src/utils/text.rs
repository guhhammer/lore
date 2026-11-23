//! Fitting text into a column: what does not fit ends in `...`.

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
