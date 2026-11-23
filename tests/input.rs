//! Tests for what each key and event does in each mode (src/input.rs).
//! Each test name says the rule it checks.

use lore_tui::app::Mode;
use lore_tui::input::{Action, action_for, action_for_event};
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// A key press with no Ctrl or Alt.
fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

/// Ctrl+C.
fn ctrl_c() -> KeyEvent {
    KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)
}

#[test]
fn slash_opens_the_search_while_browsing() {
    assert_eq!(
        action_for(key(KeyCode::Char('/')), Mode::Browse),
        Some(Action::OpenSearch)
    );
}

#[test]
fn esc_quits_while_browsing_and_closes_the_search_while_searching() {
    assert_eq!(
        action_for(key(KeyCode::Esc), Mode::Browse),
        Some(Action::Quit)
    );
    assert_eq!(
        action_for(key(KeyCode::Esc), Mode::Search),
        Some(Action::CloseSearch)
    );
}

#[test]
fn ctrl_c_quits_in_every_mode() {
    assert_eq!(action_for(ctrl_c(), Mode::Browse), Some(Action::Quit));
    assert_eq!(action_for(ctrl_c(), Mode::Search), Some(Action::Quit));
}

#[test]
fn letters_type_into_the_search_and_do_nothing_while_browsing() {
    assert_eq!(
        action_for(key(KeyCode::Char('q')), Mode::Search),
        Some(Action::Type('q'))
    );
    assert_eq!(action_for(key(KeyCode::Char('q')), Mode::Browse), None);
}

#[test]
fn arrows_move_in_every_mode() {
    for mode in [Mode::Browse, Mode::Search] {
        assert_eq!(action_for(key(KeyCode::Up), mode), Some(Action::Up));
        assert_eq!(action_for(key(KeyCode::Down), mode), Some(Action::Down));
    }
}

#[test]
fn a_resize_is_an_action_in_every_mode_so_the_screen_redraws_at_once() {
    for mode in [Mode::Browse, Mode::Search] {
        assert_eq!(
            action_for_event(Event::Resize(80, 24), mode),
            Some(Action::Resize)
        );
    }
}

#[test]
fn a_key_release_is_ignored_so_one_press_acts_once() {
    let mut release = key(KeyCode::Down);
    release.kind = KeyEventKind::Release;
    assert_eq!(action_for_event(Event::Key(release), Mode::Browse), None);
    assert_eq!(
        action_for_event(Event::Key(key(KeyCode::Down)), Mode::Browse),
        Some(Action::Down)
    );
}
