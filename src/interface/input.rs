//! Turns terminal events (keys, resizes) into `Action`s. What a key means
//! depends on the mode; Ctrl+C quits in all of them.

use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::Mode;
use crate::compile_config::{ABOUT_KEY, BOOKMARK_KEY, COPY_KEY, JUMP_KEY, PALETTE_KEY, WAKE_EVERY};

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
    /// Something changed in the background (the connection, a cheat sheet
    /// arrived): draw again.
    Redraw,
    /// Adds the command in view to the bookmarks, or takes it out (`b`).
    ToggleBookmark,
    /// On a page: opens the list of its examples to copy (`c`).
    OpenCopy,
    /// In that list: the example above or below; copy it; close the list.
    PreviousExample,
    NextExample,
    CopyExample,
    CloseCopy,
    /// Opens the about screen (`+`), and goes back from it.
    OpenAbout,
    CloseAbout,
    /// On the about screen: the setting above or below; change it (Enter).
    PreviousSetting,
    NextSetting,
    ChangeSetting,
    /// Space in the list: the first command of the next letter (or group),
    /// back to the top after the last.
    NextLetter,
    /// Space on a page: the next section at the top, back to the top after
    /// the last.
    NextSection,
    /// Opens the palette picker (`p` or Enter on Colours, on the about screen).
    OpenPalettes,
    /// In the picker: try the palette above or below.
    PreviousPalette,
    NextPalette,
    /// In the picker: keep the palette tried (and save it), or go back to
    /// the one before.
    KeepPalette,
    CancelPalette,
}

/// Waits for the next event that means something in `mode`; while
/// waiting, asks `changed` now and then, and draws again when it says so.
pub fn next_action(mode: Mode, changed: &dyn Fn() -> bool) -> io::Result<Action> {
    loop {
        if !event::poll(WAKE_EVERY)? {
            if changed() {
                return Ok(Action::Redraw);
            }
            continue;
        }
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
        // the picker takes the arrows, Enter and Esc while it is open
        (Mode::Palette, KeyCode::Up) => Some(Action::PreviousPalette),
        (Mode::Palette, KeyCode::Down) => Some(Action::NextPalette),
        (Mode::Palette, KeyCode::Enter) => Some(Action::KeepPalette),
        (Mode::Palette, KeyCode::Esc) => Some(Action::CancelPalette),
        (Mode::Palette, _) => None,
        (Mode::About, KeyCode::Esc | KeyCode::Left | KeyCode::Char(ABOUT_KEY)) => {
            Some(Action::CloseAbout)
        }
        (Mode::About, KeyCode::Up) => Some(Action::PreviousSetting),
        (Mode::About, KeyCode::Down) => Some(Action::NextSetting),
        (Mode::About, KeyCode::Enter) => Some(Action::ChangeSetting),
        (Mode::About, KeyCode::Char(PALETTE_KEY)) if plain => Some(Action::OpenPalettes),
        (Mode::About, _) => None,
        (Mode::Browse | Mode::Info, KeyCode::Char(ABOUT_KEY)) if plain => Some(Action::OpenAbout),
        (Mode::Copy, KeyCode::Up) => Some(Action::PreviousExample),
        (Mode::Copy, KeyCode::Down) => Some(Action::NextExample),
        (Mode::Copy, KeyCode::Enter) => Some(Action::CopyExample),
        (Mode::Copy, KeyCode::Esc) => Some(Action::CloseCopy),
        (Mode::Copy, _) => None,
        (Mode::Info, KeyCode::Char(COPY_KEY)) if plain => Some(Action::OpenCopy),
        // the jumper (not in Search, where a space is typed)
        (Mode::Browse, KeyCode::Char(JUMP_KEY)) if plain => Some(Action::NextLetter),
        (Mode::Info, KeyCode::Char(JUMP_KEY)) if plain => Some(Action::NextSection),
        (Mode::Browse | Mode::Info, KeyCode::Char(BOOKMARK_KEY)) if plain => {
            Some(Action::ToggleBookmark)
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
        // search other commands without leaving the page
        (Mode::Info, KeyCode::Char('/')) if plain => Some(Action::OpenSearch),
        (Mode::Info, KeyCode::PageUp) => Some(Action::PageUp),
        (Mode::Info, KeyCode::PageDown) => Some(Action::PageDown),
        (Mode::Search, KeyCode::Esc) => Some(Action::LeaveSearch),
        (Mode::Search, KeyCode::Backspace) => Some(Action::Erase),
        (Mode::Search, KeyCode::Char(c)) if plain => Some(Action::Type(c)),
        _ => None, // any other key does nothing
    }
}
