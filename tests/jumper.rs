//! Tests for the jumper, Tab and Shift+Tab: the next or previous letter in
//! the list, section on a page, round at either end (src/app.rs).

use lore_tui::app::{App, Mode};
use lore_tui::commands::{Command, Kind};
use lore_tui::compile_config::HEADING_INTERNET;
use lore_tui::info::page::{PageLine, Style, starts_a_section};
use lore_tui::interface::input::{Action, action_for};
use lore_tui::interface::rows::Row;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn app(names: &[&str]) -> App {
    App::new(
        names
            .iter()
            .map(|&n| Command::new(n, Kind::Program))
            .collect(),
    )
}

fn selected_name(app: &App) -> String {
    match &app.rows[app.list.selected().unwrap()] {
        Row::Command(command) => command.name.clone(),
        Row::Header(_) | Row::Title(_) => panic!("a header is selected"),
    }
}

#[test]
fn tab_jumps_forward_and_shift_tab_back_in_the_list_and_on_a_page() {
    assert_eq!(
        action_for(key(KeyCode::Tab), Mode::Browse),
        Some(Action::NextLetter)
    );
    assert_eq!(
        action_for(key(KeyCode::BackTab), Mode::Browse),
        Some(Action::PreviousLetter)
    );
    assert_eq!(
        action_for(key(KeyCode::Tab), Mode::Info),
        Some(Action::NextSection)
    );
    assert_eq!(
        action_for(key(KeyCode::BackTab), Mode::Info),
        Some(Action::PreviousSection)
    );
    assert_eq!(
        action_for(key(KeyCode::Char(' ')), Mode::Search),
        Some(Action::Type(' ')),
        "a space is typed in the search"
    );
}

#[test]
fn tab_in_the_list_goes_a_to_b_to_c_and_back_to_a() {
    let mut app = app(&["awk", "apt", "bash", "cat", "cut"]);
    assert_eq!(selected_name(&app), "awk");
    let mut seen = Vec::new();
    for _ in 0..4 {
        app.next_letter();
        seen.push(selected_name(&app));
    }
    assert_eq!(seen, ["bash", "cat", "awk", "bash"]);
}

#[test]
fn the_letter_jumped_to_shows_its_header_on_top() {
    let mut app = app(&["awk", "bash", "cat"]);
    app.next_letter();
    let header = app.list.offset();
    assert!(matches!(app.rows[header], Row::Header(_)));
    assert_eq!(app.list.selected(), Some(header + 1));
}

#[test]
fn tab_on_a_page_goes_section_by_section_then_back_to_the_top() {
    let mut app = app(&["ls"]);
    app.sections = vec![0, 5, 12, 40];
    app.scroll_max = 30;
    let mut seen = Vec::new();
    for _ in 0..5 {
        app.next_section();
        seen.push(app.scroll);
    }
    // 40 cannot be reached (the page ends at 30): the end, then the top
    assert_eq!(seen, [5, 12, 30, 0, 5]);
}

#[test]
fn the_sections_are_the_headings_and_internets_coloured_one() {
    let line = |text: &str, style| PageLine {
        text: text.to_owned(),
        style,
        spans: Vec::new(),
    };
    assert!(starts_a_section(&line("HOW TO USE", Style::Heading)));
    assert!(starts_a_section(&line(
        &format!("{HEADING_INTERNET} ●"),
        Style::Online
    )));
    assert!(!starts_a_section(&line("● offline", Style::Offline)));
    assert!(!starts_a_section(&line("  tar -xf a.tar", Style::Code)));
}

#[test]
fn shift_tab_goes_back_a_letter_and_from_the_first_round_to_the_last() {
    let mut app = app(&["awk", "bash", "cat", "cut"]);
    let mut seen = Vec::new();
    for _ in 0..3 {
        app.previous_letter();
        seen.push(selected_name(&app));
    }
    assert_eq!(seen, ["cat", "bash", "awk"]);
    app.next_letter();
    app.next_letter();
    app.down(); // cut, the second under C
    app.previous_letter();
    assert_eq!(
        selected_name(&app),
        "bash",
        "from inside a letter, the one before"
    );
}

#[test]
fn shift_tab_on_a_page_goes_back_a_section_and_from_the_top_to_the_last() {
    let mut app = app(&["ls"]);
    app.sections = vec![0, 5, 12, 40];
    app.scroll_max = 30;
    app.scroll = 12;
    let mut seen = Vec::new();
    for _ in 0..4 {
        app.previous_section();
        seen.push(app.scroll);
    }
    // 40 is past the end of the page, so the last reachable is 30
    assert_eq!(seen, [5, 0, 30, 12]);
}
