//! Turns terminal events (keys, resizes) into `Action`s. What a key means
//! depends on the mode; Ctrl+C quits in all of them.

use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::Mode;

/// Something the app should do; the loop in lib.rs applies it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Up,
    Down,
    OpenSearch,
    /// Leaves the search bar, keeping its text.
    LeaveSearch,
    /// Opens the selected command's page.
    OpenInfo,
    CloseInfo,
    /// Scrolls the page one line up or down.
    LineUp,
    LineDown,
    /// Scrolls the page a screen up or down.
    PageUp,
    PageDown,
    /// A character typed into the search bar.
    Type(char),
    /// Backspace in the search bar.
    Erase,
    /// The terminal changed size; nothing to do but draw again.
    Resize,
}

/// Waits for the next event that means something in `mode`.
pub fn next_action(mode: Mode) -> io::Result<Action> {
    loop {
        // event::read blocks until the terminal sends something
        if let Some(action) = action_for_event(event::read()?, mode) {
            return Ok(action);
        }
    }
}

/// An event's action, if any. Resizes always count, or the screen would keep
/// its old size until the next key.
pub fn action_for_event(event: Event, mode: Mode) -> Option<Action> {
    match event {
        // Press only: some terminals also send a Release, which would act twice
        Event::Key(key) if key.kind == KeyEventKind::Press => action_for(key, mode),
        Event::Resize(..) => Some(Action::Resize),
        _ => None, // mouse, focus, paste: ignored
    }
}

/// A key's action in `mode`, if any.
pub fn action_for(key: KeyEvent, mode: Mode) -> Option<Action> {
    // "plain" = no Ctrl or Alt, so Ctrl+/ or Alt+x do not type
    let plain = !key
        .modifiers
        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT);
    match (mode, key.code) {
        (_, KeyCode::Char('c')) if key.modifiers.contains(KeyModifiers::CONTROL) => {
            Some(Action::Quit)
        }
        // reading a page, the arrows scroll it; the list moves again once
        // the page is closed
        (Mode::Info, KeyCode::Up) => Some(Action::LineUp),
        (Mode::Info, KeyCode::Down) => Some(Action::LineDown),
        (_, KeyCode::Up) => Some(Action::Up),
        (_, KeyCode::Down) => Some(Action::Down),
        // the page opens from the list and from the search bar alike
        (Mode::Browse | Mode::Search, KeyCode::Right | KeyCode::Enter) => Some(Action::OpenInfo),
        (Mode::Browse, KeyCode::Esc) => Some(Action::Quit),
        (Mode::Browse, KeyCode::Char('/')) if plain => Some(Action::OpenSearch),
        (Mode::Info, KeyCode::Left | KeyCode::Esc) => Some(Action::CloseInfo),
        (Mode::Info, KeyCode::PageUp) => Some(Action::PageUp),
        (Mode::Info, KeyCode::PageDown) => Some(Action::PageDown),
        (Mode::Search, KeyCode::Esc) => Some(Action::LeaveSearch),
        (Mode::Search, KeyCode::Backspace) => Some(Action::Erase),
        (Mode::Search, KeyCode::Char(c)) if plain => Some(Action::Type(c)),
        _ => None, // any other key does nothing
    }
}
