//! Tests for the search rules (src/search.rs).
//! Each test name says the rule it checks.

use lore_tui::search::first_section;
use lore_tui::section::Section;

#[test]
fn a_single_letter_asks_for_its_section_first_whatever_its_case() {
    assert_eq!(first_section("h"), Some(Section::Letter('H')));
    assert_eq!(first_section("H"), Some(Section::Letter('H')));
}

#[test]
fn a_single_digit_or_symbol_asks_for_the_symbol_section_first() {
    assert_eq!(first_section("7"), Some(Section::Symbol));
    assert_eq!(first_section("["), Some(Section::Symbol));
}

#[test]
fn an_empty_or_longer_query_asks_for_no_section_first() {
    assert_eq!(first_section(""), None);
    assert_eq!(first_section("ht"), None);
}
