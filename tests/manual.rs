//! Tests for reading manual pages (src/utils/manual.rs).

use lore_tui::utils::manual::{clean, page};

#[test]
fn bold_and_underline_overstrikes_become_plain_text() {
    assert_eq!(clean("N\u{8}NA\u{8}AM\u{8}ME\u{8}E"), "NAME");
    assert_eq!(clean("_\u{8}f_\u{8}i_\u{8}l_\u{8}e"), "file");
}

#[test]
fn colour_codes_are_removed() {
    assert_eq!(clean("\u{1b}[1mls\u{1b}[0m - list"), "ls - list");
}

#[test]
fn a_command_with_no_page_gives_none() {
    assert_eq!(page("lore-surely-has-no-such-page", 80), None);
}

#[test]
fn a_page_is_laid_out_to_the_width_asked_when_man_is_there() {
    let Some(lines) = page("ls", 50) else {
        println!("skipping: no `man` or no page for ls on this machine");
        return;
    };
    assert!(lines.iter().any(|l| l.contains("ls")));
    // man may overrun on an unbreakable word; ordinary text stays inside
    let wide = lines.iter().filter(|l| l.chars().count() > 50).count();
    assert!(
        wide <= lines.len() / 20,
        "{wide} of {} lines wider than 50",
        lines.len()
    );
}

#[test]
fn other_control_characters_are_dropped_but_newlines_and_tabs_stay() {
    assert_eq!(clean("a\u{7}b\r\n\tc"), "ab\n\tc");
}
