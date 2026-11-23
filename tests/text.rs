//! Tests for cutting text to a column (src/utils/text.rs).

use lore_tui::utils::text::fit;

#[test]
fn text_that_fits_is_left_as_it_is() {
    assert_eq!(fit("ls", 10), "ls");
    assert_eq!(fit("abcdef", 6), "abcdef");
}

#[test]
fn text_too_long_is_cut_and_ends_in_three_dots_within_the_width() {
    assert_eq!(fit("systemctl", 7), "syst...");
    assert_eq!(fit("systemctl", 7).chars().count(), 7);
}

#[test]
fn a_column_narrower_than_the_dots_gets_only_dots() {
    assert_eq!(fit("systemctl", 2), "..");
    assert_eq!(fit("systemctl", 0), "");
}

#[test]
fn characters_beyond_ascii_are_counted_as_one_each() {
    assert_eq!(fit("ééééé", 4), "é...");
}

use lore_tui::utils::text::wrap;

#[test]
fn wrapping_breaks_at_spaces_within_the_width() {
    assert_eq!(wrap("one two three four", 9), ["one two", "three", "four"]);
}

#[test]
fn a_word_longer_than_the_width_is_split() {
    assert_eq!(wrap("abcdefgh", 3), ["abc", "def", "gh"]);
}

#[test]
fn an_empty_text_wraps_to_one_empty_line() {
    assert_eq!(wrap("", 10), [""]);
}
