//! Tests for the app's state: moving, searching, leaving the search, the
//! page (src/app.rs). Each test name says the rule it checks.

use lore_tui::app::{App, Mode};
use lore_tui::commands::{Command, Kind};
use lore_tui::interface::rows::Row;

/// An app over these names, all as programs.
fn app(names: &[&str]) -> App {
    App::new(
        names
            .iter()
            .map(|&n| Command::new(n, Kind::Program))
            .collect(),
    )
}

/// The selected command's name; fails if a header is selected.
fn selected_name(app: &App) -> &str {
    match &app.rows[app.list.selected().unwrap()] {
        Row::Command(command) => &command.name,
        Row::Header(_) => panic!("a header is selected"),
    }
}

/// The rows as text: headers as `[A]`, commands by name.
fn labels(app: &App) -> Vec<String> {
    app.rows
        .iter()
        .map(|row| match row {
            Row::Header(section) => format!("[{section}]"),
            Row::Command(command) => command.name.clone(),
        })
        .collect()
}

/// Opens the search and types `text` into it.
fn typed(app: &mut App, text: &str) {
    app.open_search();
    text.chars().for_each(|c| app.type_char(c));
}

#[test]
fn the_first_command_is_selected_at_start_not_its_header() {
    assert_eq!(selected_name(&app(&["awk", "bash"])), "awk");
}

#[test]
fn moving_down_and_up_skips_the_section_headers() {
    let mut app = app(&["awk", "bash"]);
    app.down();
    assert_eq!(selected_name(&app), "bash");
    app.up();
    assert_eq!(selected_name(&app), "awk");
}

#[test]
fn moving_past_either_end_stays_on_the_end() {
    let mut app = app(&["awk", "bash"]);
    app.up();
    assert_eq!(selected_name(&app), "awk");
    app.down();
    app.down();
    assert_eq!(selected_name(&app), "bash");
}

#[test]
fn typing_keeps_only_the_matching_names_under_their_headers() {
    let mut app = app(&["apt", "awk", "bash", "gawk"]);
    typed(&mut app, "AWK");
    assert_eq!(labels(&app), ["[A]", "awk", "[G]", "gawk"]);
    assert_eq!(selected_name(&app), "awk");
    assert_eq!(app.shown_count(), 2);
}

#[test]
fn one_letter_puts_its_own_section_first_then_the_other_names_containing_it() {
    let mut app = app(&["bash", "chmod", "ssh", "htop"]);
    typed(&mut app, "h");
    assert_eq!(
        labels(&app),
        ["[H]", "htop", "[B]", "bash", "[C]", "chmod", "[S]", "ssh"]
    );
    assert_eq!(selected_name(&app), "htop");
}

#[test]
fn two_letters_keep_the_usual_alphabetical_sections() {
    let mut app = app(&["bash", "chmod", "ssh", "htop"]);
    typed(&mut app, "sh");
    assert_eq!(labels(&app), ["[B]", "bash", "[S]", "ssh"]);
}

#[test]
fn erasing_widens_the_search_again() {
    let mut app = app(&["awk", "ash"]);
    typed(&mut app, "aw");
    assert_eq!(app.shown_count(), 1);
    app.erase();
    assert_eq!(app.shown_count(), 2);
}

#[test]
fn a_search_with_no_match_shows_no_rows_and_selects_nothing() {
    let mut app = app(&["awk"]);
    typed(&mut app, "zzz");
    assert!(app.rows.is_empty());
    assert_eq!(app.list.selected(), None);
    app.down();
    app.up();
}

#[test]
fn leaving_the_search_keeps_its_text_and_its_filter() {
    let mut app = app(&["awk", "bash"]);
    typed(&mut app, "ba");
    app.leave_search();
    assert_eq!(app.mode, Mode::Browse);
    assert_eq!(app.query, "ba");
    assert_eq!(app.shown_count(), 1);
}

#[test]
fn opening_the_search_again_continues_the_text_left_in_it() {
    let mut app = app(&["bash", "basename"]);
    typed(&mut app, "bas");
    app.leave_search();
    typed(&mut app, "e");
    assert_eq!(app.query, "base");
    assert_eq!(labels(&app), ["[B]", "basename"]);
}

#[test]
fn the_page_opens_on_a_command_and_closes_back_to_the_list() {
    let mut app = app(&["awk"]);
    app.open_info();
    assert_eq!(app.mode, Mode::Info);
    app.close_info();
    assert_eq!(app.mode, Mode::Browse);
}

#[test]
fn the_page_opens_from_the_search_bar_and_the_search_text_stays() {
    let mut app = app(&["awk", "bash"]);
    typed(&mut app, "aw");
    app.open_info();
    assert_eq!(app.mode, Mode::Info);
    app.close_info();
    assert_eq!(app.query, "aw");
}

#[test]
fn the_page_does_not_open_when_nothing_matches() {
    let mut app = app(&["awk"]);
    typed(&mut app, "zzz");
    app.open_info();
    assert_eq!(app.mode, Mode::Search);
}

#[test]
fn scrolling_up_past_the_top_stops_at_the_top() {
    let mut app = app(&["awk"]);
    app.scroll_by(5);
    app.scroll_by(-20);
    assert_eq!(app.scroll, 0);
}
