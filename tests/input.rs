//! Tests for what each key and event does in each mode (src/interface/input.rs).
//! Each test name says the rule it checks.

use lore_tui::app::Mode;
use lore_tui::interface::input::{Action, action_for, action_for_event};
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
fn esc_quits_while_browsing_leaves_the_search_and_closes_the_page() {
    assert_eq!(
        action_for(key(KeyCode::Esc), Mode::Browse),
        Some(Action::Quit)
    );
    assert_eq!(
        action_for(key(KeyCode::Esc), Mode::Search),
        Some(Action::LeaveSearch)
    );
    assert_eq!(
        action_for(key(KeyCode::Esc), Mode::Info),
        Some(Action::CloseInfo)
    );
}

#[test]
fn enter_opens_the_page_from_the_list_and_the_search_bar_and_right_from_the_search_bar() {
    assert_eq!(
        action_for(key(KeyCode::Enter), Mode::Browse),
        Some(Action::OpenInfo)
    );
    for code in [KeyCode::Right, KeyCode::Enter] {
        assert_eq!(action_for(key(code), Mode::Search), Some(Action::OpenInfo));
    }
}

#[test]
fn left_and_right_in_the_list_switch_the_topic() {
    assert_eq!(
        action_for(key(KeyCode::Right), Mode::Browse),
        Some(Action::NextTopic)
    );
    assert_eq!(
        action_for(key(KeyCode::Left), Mode::Browse),
        Some(Action::PreviousTopic)
    );
}

#[test]
fn left_closes_the_page_and_page_keys_scroll_it() {
    assert_eq!(
        action_for(key(KeyCode::Left), Mode::Info),
        Some(Action::CloseInfo)
    );
    assert_eq!(
        action_for(key(KeyCode::PageUp), Mode::Info),
        Some(Action::PageUp)
    );
    assert_eq!(
        action_for(key(KeyCode::PageDown), Mode::Info),
        Some(Action::PageDown)
    );
}

#[test]
fn arrows_scroll_the_page_while_reading_and_do_not_move_the_list() {
    assert_eq!(
        action_for(key(KeyCode::Up), Mode::Info),
        Some(Action::LineUp)
    );
    assert_eq!(
        action_for(key(KeyCode::Down), Mode::Info),
        Some(Action::LineDown)
    );
}

#[test]
fn slash_searches_from_a_page_and_letters_do_nothing_there() {
    assert_eq!(
        action_for(key(KeyCode::Char('/')), Mode::Info),
        Some(Action::OpenSearch)
    );
    assert_eq!(action_for(key(KeyCode::Char('q')), Mode::Info), None);
}

#[test]
fn ctrl_c_quits_in_every_mode() {
    assert_eq!(action_for(ctrl_c(), Mode::Browse), Some(Action::Quit));
    assert_eq!(action_for(ctrl_c(), Mode::Search), Some(Action::Quit));
    assert_eq!(action_for(ctrl_c(), Mode::Info), Some(Action::Quit));
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
fn arrows_move_the_list_while_browsing_and_searching() {
    for mode in [Mode::Browse, Mode::Search] {
        assert_eq!(action_for(key(KeyCode::Up), mode), Some(Action::Up));
        assert_eq!(action_for(key(KeyCode::Down), mode), Some(Action::Down));
    }
}

#[test]
fn a_resize_is_an_action_in_every_mode_so_the_screen_redraws_at_once() {
    for mode in [Mode::Browse, Mode::Search, Mode::Info] {
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
