//! Tests for the INTERNET section (src/utils/internet.rs and its page part).
//! None of them reaches the network.

use lore_tui::configs::{HEADING_INTERNET, INTERNET_MAX_LINES, INTERNET_REMINDER};
use lore_tui::info::page::{Style, internet_lines};
use lore_tui::utils::internet::{Answer, Internet, Status, answer_of, clean, url};

#[test]
fn the_address_is_the_pages_name_percent_encoded() {
    assert_eq!(url("tar"), "https://cht.sh/tar?T");
    assert_eq!(url("git commit"), "https://cht.sh/git-commit?T");
    assert_eq!(url("a&b"), "https://cht.sh/a%26b?T");
}

#[test]
fn escape_sequences_control_and_invisible_characters_are_taken_out() {
    assert_eq!(clean("\x1b[31mred\x1b[0m text"), "red text");
    assert_eq!(clean("bell\x07 and \u{202E}reversed"), "bell and reversed");
    assert_eq!(clean("a\tb"), "a    b");
}

#[test]
fn an_unknown_name_or_a_web_page_is_not_shown_as_a_cheat_sheet() {
    assert_eq!(answer_of("Unknown topic.\nDo you mean"), Answer::NotFound);
    assert_eq!(answer_of(""), Answer::NotFound);
    assert_eq!(answer_of("<html><script>x</script>"), Answer::Failed);
    assert_eq!(
        answer_of("# tar\ntar -xf a.tar\n"),
        Answer::Lines(vec!["# tar".into(), "tar -xf a.tar".into()])
    );
}

#[test]
fn with_the_internet_off_nothing_is_asked() {
    let internet = Internet::off();
    internet.request("tar");
    assert_eq!(internet.status(), Status::Off);
    assert_eq!(internet.answer("tar"), None);
}

#[test]
fn an_internet_turned_off_is_off_and_says_so_in_the_section_without_its_text() {
    let internet = Internet::off();
    internet.set_enabled(false);
    assert!(!internet.enabled());
    assert_eq!(internet.status(), Status::Off);
    assert!(internet.take_changed(), "the screen is drawn again");
    let shown = internet_lines(
        "tar",
        Status::Off,
        Some(&Answer::Lines(vec!["tar -xf a.tar".into()])),
        80,
    );
    assert!(
        !texts(&shown).iter().any(|t| t.contains("tar -xf")),
        "nothing fetched is shown while it is off"
    );
}

fn texts(lines: &[lore_tui::info::page::PageLine]) -> Vec<String> {
    lines.iter().map(|l| l.text.clone()).collect()
}

#[test]
fn the_section_has_its_dot_coloured_by_the_connection_and_the_reminder() {
    let online = internet_lines("tar", Status::Online, None, 80);
    assert!(
        online
            .iter()
            .any(|l| l.text.starts_with(HEADING_INTERNET) && l.style == Style::Online)
    );
    let offline = internet_lines("tar", Status::Offline, None, 80);
    assert!(
        offline
            .iter()
            .any(|l| l.text.starts_with(HEADING_INTERNET) && l.style == Style::Offline)
    );
    let reminder = INTERNET_REMINDER
        .split_whitespace()
        .take(4)
        .collect::<Vec<_>>()
        .join(" ");
    assert!(texts(&online).join(" ").contains(&reminder));
}

#[test]
fn the_cheat_sheet_is_raw_text_behind_a_bar_and_cut_at_a_limit() {
    let many: Vec<String> = (0..INTERNET_MAX_LINES + 20)
        .map(|i| format!("line {i}"))
        .collect();
    let lines = internet_lines("tar", Status::Online, Some(&Answer::Lines(many)), 80);
    let raw: Vec<_> = lines.iter().filter(|l| l.style == Style::Raw).collect();
    assert_eq!(raw.len(), INTERNET_MAX_LINES);
    assert!(raw.iter().all(|l| l.text.contains('│')));
    assert!(
        lines
            .last()
            .unwrap()
            .text
            .contains("more at https://cht.sh/tar")
    );
}
