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
