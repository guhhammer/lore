//! Tests for the jumper, Space: the next letter in the list, the next
//! section on a page, back to the top after the last (src/app.rs).

use lore_tui::app::{App, Mode};
use lore_tui::commands::{Command, Kind};
use lore_tui::compile_config::{HEADING_INTERNET, JUMP_KEY};
use lore_tui::info::page::{PageLine, Style, starts_a_section};
use lore_tui::interface::input::{Action, action_for};
use lore_tui::interface::rows::Row;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn space() -> KeyEvent {
    KeyEvent::new(KeyCode::Char(JUMP_KEY), KeyModifiers::NONE)
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
        Row::Header(_) => panic!("a header is selected"),
    }
}

#[test]
fn space_jumps_in_the_list_and_on_a_page_but_types_in_the_search() {
    assert_eq!(action_for(space(), Mode::Browse), Some(Action::NextLetter));
    assert_eq!(action_for(space(), Mode::Info), Some(Action::NextSection));
    assert_eq!(
        action_for(space(), Mode::Search),
        Some(Action::Type(JUMP_KEY))
    );
}

#[test]
fn space_in_the_list_goes_a_to_b_to_c_and_back_to_a() {
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
fn space_on_a_page_goes_section_by_section_then_back_to_the_top() {
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
    };
    assert!(starts_a_section(&line("HOW TO USE", Style::Heading)));
    assert!(starts_a_section(&line(
        &format!("{HEADING_INTERNET} ●"),
        Style::Online
    )));
    assert!(!starts_a_section(&line("● offline", Style::Offline)));
    assert!(!starts_a_section(&line("  tar -xf a.tar", Style::Code)));
}
