//! Tests for the about screen and the logo (src/app.rs, src/interface/input.rs).

use lore_tui::app::{App, Mode};
use lore_tui::commands::{Command, Kind};
use lore_tui::compile_config::{ABOUT_KEY, LOGO, LORE_ORANGE};
use lore_tui::interface::input::{Action, action_for};
use lore_tui::utils::text;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::Color;

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn plus_opens_the_about_screen_from_the_list_and_a_page_but_types_in_the_search() {
    let plus = key(KeyCode::Char(ABOUT_KEY));
    assert_eq!(action_for(plus, Mode::Browse), Some(Action::OpenAbout));
    assert_eq!(action_for(plus, Mode::Info), Some(Action::OpenAbout));
    assert_eq!(
        action_for(plus, Mode::Search),
        Some(Action::Type(ABOUT_KEY))
    );
}

#[test]
fn plus_left_or_esc_leave_the_about_screen_and_other_keys_do_nothing() {
    for code in [KeyCode::Char(ABOUT_KEY), KeyCode::Left, KeyCode::Esc] {
        assert_eq!(action_for(key(code), Mode::About), Some(Action::CloseAbout));
    }
    assert_eq!(action_for(key(KeyCode::Down), Mode::About), None);
}

#[test]
fn the_about_screen_goes_back_to_where_it_was_opened() {
    let mut app = App::new(vec![Command::new("ls", Kind::Program)]);
    app.open_info();
    app.open_about();
    assert_eq!(app.mode, Mode::About);
    assert!(!app.page_shown(), "the about screen takes the whole body");
    app.close_about();
    assert_eq!(app.mode, Mode::Info);
}

#[test]
fn the_logo_is_four_rows_that_fit_beside_the_search_bar() {
    assert_eq!(LOGO.len(), 4);
    assert!(LOGO.iter().all(|line| text::width(line) <= 32));
}

#[test]
fn the_logo_is_in_lores_orange() {
    assert_eq!(LORE_ORANGE, Color::Rgb(0xe8, 0x79, 0x3a));
}

#[test]
fn the_built_in_translations_are_in_41_languages() {
    assert_eq!(lore_tui::info::content::translated_languages(), 41);
}
