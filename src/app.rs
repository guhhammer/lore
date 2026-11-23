//! The app's state and the moves on it. No drawing, no keys: those are in
//! interface/ui.rs and interface/input.rs.

use ratatui::widgets::ListState;

use crate::commands::Command;
use crate::interface::rows::{self, Row};
use crate::interface::search;
use crate::utils::manual;

/// What the keys do right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Arrows move, `/` opens the search, → or Enter opens the page, Esc quits.
    Browse,
    /// Typing filters the list, Esc leaves the search bar (the text stays).
    Search,
    /// The selected command's page beside a narrow list: ↑↓ and PgUp/PgDn
    /// scroll the page, ← or Esc close it (the list moves again after).
    Info,
}

/// The page on show, kept so a redraw does not run `man` again; reloaded when
/// the command or the panel's width changes.
pub struct Page {
    pub name: String,
    pub width: u16,
    /// The manual's lines; None when there is none (see `manual::NO_PAGE`).
    pub lines: Option<Vec<String>>,
}

/// Everything the screen shows.
pub struct App {
    /// Every command found, sorted; never filtered.
    pub commands: Vec<Command>,
    /// The search bar's text.
    pub query: String,
    pub mode: Mode,
    /// The commands matching `query`, under their section headers.
    pub rows: Vec<Row>,
    /// The selected row and the list's scroll position.
    pub list: ListState,
    /// The last page loaded, if any.
    pub page: Option<Page>,
    /// How many lines the page is scrolled down.
    pub scroll: usize,
}

impl App {
    /// Starts in Browse mode with the whole list and the first command selected.
    pub fn new(commands: Vec<Command>) -> Self {
        let mut app = Self {
            commands,
            query: String::new(),
            mode: Mode::Browse,
            rows: Vec::new(),
            list: ListState::default(),
            page: None,
            scroll: 0,
        };
        app.refilter(); // builds the rows from the empty query
        app
    }

    /// How many commands are shown (headers not counted).
    pub fn shown_count(&self) -> usize {
        self.rows.iter().filter(|r| r.is_selectable()).count()
    }

    /// Enters the search bar, its text as it was left.
    pub fn open_search(&mut self) {
        self.mode = Mode::Search;
    }

    /// Leaves the search bar. Its text, and the filter, stay: emptying it is
    /// the user's to do (Backspace), so a search can be read and refined.
    pub fn leave_search(&mut self) {
        self.mode = Mode::Browse;
    }

    /// The selected command, if a command is selected.
    pub fn selected(&self) -> Option<&Command> {
        match self.rows.get(self.list.selected()?)? {
            Row::Command(command) => Some(command),
            Row::Header(_) => None,
        }
    }

    /// Opens the selected command's page; does nothing on an empty list.
    pub fn open_info(&mut self) {
        if self.selected().is_some() {
            self.mode = Mode::Info;
            self.scroll = 0;
        }
    }

    /// Closes the page, back to the list.
    pub fn close_info(&mut self) {
        self.mode = Mode::Browse;
    }

    /// Scrolls the page by `lines` (negative: up); `ui` stops it at the end.
    pub fn scroll_by(&mut self, lines: isize) {
        self.scroll = self.scroll.saturating_add_signed(lines);
    }

    /// Makes `page` the selected command's, `width` columns wide, running
    /// `man` only when the command or the width changed since the last call.
    pub fn load_page(&mut self, width: u16) {
        let Some(name) = self.selected().map(|c| c.name.clone()) else {
            return;
        };
        if let Some(page) = &self.page {
            if page.name == name && page.width == width {
                return; // already loaded
            }
            if page.name != name {
                self.scroll = 0; // a new page starts at its top
            }
        }
        let lines = manual::page(&name, width);
        self.page = Some(Page { name, width, lines });
    }

    /// Adds a character to the search and filters again.
    pub fn type_char(&mut self, c: char) {
        self.query.push(c);
        self.refilter();
    }

    /// Removes the last character of the search and filters again.
    pub fn erase(&mut self) {
        self.query.pop();
        self.refilter();
    }

    /// Selects the next command down, skipping headers; stays put at the end.
    pub fn down(&mut self) {
        let Some(current) = self.list.selected() else {
            return; // nothing selected: the list is empty
        };
        if let Some(i) = (current + 1..self.rows.len()).find(|&i| self.rows[i].is_selectable()) {
            self.list.select(Some(i));
        }
    }

    /// Selects the previous command up, skipping headers. At the first command
    /// it scrolls to the very top instead, so the first header shows.
    pub fn up(&mut self) {
        let Some(current) = self.list.selected() else {
            return; // nothing selected: the list is empty
        };
        match (0..current).rev().find(|&i| self.rows[i].is_selectable()) {
            Some(i) => self.list.select(Some(i)),
            None => *self.list.offset_mut() = 0,
        }
    }

    /// Rebuilds `rows` from `query` and selects the first match.
    fn refilter(&mut self) {
        let matching = self
            .commands
            .iter()
            .filter(|c| search::matches(c, &self.query))
            .cloned()
            .collect();
        self.rows = rows::build(matching, search::first_section(&self.query));
        self.list = ListState::default(); // back to the top of the list
        self.list
            .select(self.rows.iter().position(Row::is_selectable));
    }
}
