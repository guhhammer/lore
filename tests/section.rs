//! Tests for which section a name goes under (src/interface/section.rs).
//! Each test name says the rule it checks.

use lore_tui::interface::section::Section;

#[test]
fn a_name_goes_under_its_first_letter_upper_cased() {
    assert_eq!(Section::of("ls"), Section::Letter('L'));
    assert_eq!(Section::of("Xorg"), Section::Letter('X'));
}

#[test]
fn a_name_that_does_not_start_with_a_letter_goes_under_the_symbol_section() {
    assert_eq!(Section::of("7z"), Section::Symbol);
    assert_eq!(Section::of("["), Section::Symbol);
}

#[test]
fn the_symbol_section_comes_before_every_letter() {
    assert!(Section::Symbol < Section::Letter('A'));
}
