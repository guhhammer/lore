//! The app's state and the moves on it. No drawing, no keys: those are in
//! interface/ui.rs and interface/input.rs.

use std::collections::{HashMap, HashSet};

use ratatui::widgets::ListState;

use crate::commands::{self, Command, Kind};
use crate::compile_config::MAN_PROBE_WIDTH;
use crate::info::content;
use crate::info::page::{self, Facts, PageLine};
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

/// The page on show, kept so a redraw does not look things up again: the
/// facts once per command, the layout again when the panel's width changes.
pub struct Page {
    /// The command it is about: kept, so the page can be laid out again
    /// (a resize) even while the search beside it selects others.
    pub command: Command,
    pub width: u16,
    pub facts: Facts,
    /// The page laid out for `width`.
    pub lines: Vec<PageLine>,
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
    /// The names that have a lore page, for the `@` search.
    pub pages: HashSet<String>,
    /// Opened as `lore NAME`: closing the page quits, like man.
    pub direct: bool,
    /// Searching from a page (`/` while reading): the page stays beside the
    /// list until another one is opened.
    pub beside_page: bool,
    /// What each listed command's page says, for searching by content and
    /// for its categories in the list.
    pub docs: HashMap<String, search::Doc>,
    /// For each search result whose page has a line matching the search,
    /// that line cut around the match: shown after the name.
    pub snippets: HashMap<String, String>,
}

impl App {
    /// Starts in Browse mode with the whole list and the first command selected.
    pub fn new(commands: Vec<Command>) -> Self {
        Self::with_pages(commands, content::names())
    }

    /// Like `new`, with the set of names that have a lore page given.
    pub fn with_pages(commands: Vec<Command>, pages: HashSet<String>) -> Self {
        // read once at start: a page per listed command that has one
        let docs = commands
            .iter()
            .filter(|c| pages.contains(&content::page_key(&c.name)))
            .filter_map(|c| Some((c.name.clone(), search::Doc::of(&content::find(&c.name)?))))
            .collect();
        let mut app = Self {
            commands,
            query: String::new(),
            mode: Mode::Browse,
            rows: Vec::new(),
            list: ListState::default(),
            page: None,
            scroll: 0,
            pages,
            direct: false,
            beside_page: false,
            docs,
            snippets: HashMap::new(),
        };
        app.refilter(); // builds the rows from the empty query
        app
    }

    /// How many commands are shown (headers not counted).
    pub fn shown_count(&self) -> usize {
        self.rows.iter().filter(|r| r.is_selectable()).count()
    }

    /// Enters the search bar, its text as it was left. From a page, the
    /// page stays on screen while the list beside it filters.
    pub fn open_search(&mut self) {
        self.beside_page = self.mode == Mode::Info;
        self.mode = Mode::Search;
    }

    /// True while the screen is split: a page open, or a search from one.
    pub fn page_shown(&self) -> bool {
        self.mode == Mode::Info || (self.mode == Mode::Search && self.beside_page)
    }

    /// Leaves the search bar. Its text, and the filter, stay: emptying it is
    /// the user's to do (Backspace), so a search can be read and refined.
    pub fn leave_search(&mut self) {
        self.mode = Mode::Browse;
        self.beside_page = false;
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
            self.beside_page = false;
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

    /// Makes `page` the selected command's, `width` columns wide. Looks the
    /// command up only when it changed, and runs `man` again only when the
    /// command or the width changed.
    pub fn load_page(&mut self, width: u16) {
        let Some(command) = self.selected().cloned() else {
            return;
        };
        self.show_page(command, width);
    }

    /// Lays the page on show out again at `width`, if it is not already:
    /// while searching from a page the selection is the search's, but the
    /// page shown is still the one being read.
    pub fn relayout_page(&mut self, width: u16) {
        if let Some(command) = self.page.as_ref().map(|p| p.command.clone()) {
            self.show_page(command, width);
        }
    }

    /// Makes `page` `command`'s at `width`; see `load_page`.
    fn show_page(&mut self, command: Command, width: u16) {
        let facts = match self.page.take() {
            Some(page) if page.command.name == command.name => {
                if page.width == width {
                    self.page = Some(page); // nothing changed
                    return;
                }
                page.facts // same command, new width: only the layout changes
            }
            _ => {
                self.scroll = 0; // a new page starts at its top
                page::facts(&command)
            }
        };
        let man = manual::page(&content::page_key(&command.name), width); // `man git-commit`
        let lines = page::render(&command, &facts, man.as_deref(), usize::from(width));
        self.page = Some(Page {
            command,
            width,
            facts,
            lines,
        });
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

    /// Opens `name`'s page at start (`lore NAME`), so that closing it quits.
    /// A name lore did not find on this machine is added to the list for the
    /// page to show. False if neither lore nor man has anything on it.
    pub fn open_direct(&mut self, name: &str) -> bool {
        if !self.commands.iter().any(|c| c.name == name) {
            let key = content::page_key(name);
            if !self.pages.contains(&key) && manual::page(&key, MAN_PROBE_WIDTH).is_none() {
                return false;
            }
            let kind = if name.contains(' ') {
                Kind::Subcommand
            } else {
                Kind::Program
            };
            self.commands.push(Command::new(name, kind));
            self.commands = commands::merge(std::mem::take(&mut self.commands));
            self.refilter();
        }
        let Some(row) = self
            .rows
            .iter()
            .position(|r| matches!(r, Row::Command(c) if c.name == name))
        else {
            return false;
        };
        self.list.select(Some(row));
        self.open_info();
        self.direct = true;
        true
    }

    /// Rebuilds `rows` from `query` and selects the first match.
    fn refilter(&mut self) {
        let (pages_only, query) = search::split(&self.query);
        let allowed: Vec<&Command> = self
            .commands
            .iter()
            .filter(|c| !pages_only || self.pages.contains(&content::page_key(&c.name)))
            .collect();
        self.snippets.clear();
        if query.trim().is_empty() {
            // no search: the whole list, A to Z under its letters
            self.rows = rows::build(allowed.into_iter().cloned().collect());
        } else {
            // a search: best first, never alphabetical. Names that match come
            // first (an exact name on top), then pages that match by content.
            let terms = search::Terms::new(query); // None: too short for pages
            let mut by_name: Vec<((u8, usize, usize), &Command)> = Vec::new();
            // (named by a word of the query, score): a command named in the
            // query ("cut files") goes first among the page matches
            let mut by_page: Vec<((bool, u32), &Command)> = Vec::new();
            for command in allowed {
                if let Some(rank) = search::name_rank(&command.name, query) {
                    by_name.push((rank, command));
                } else if let (Some(terms), Some(doc)) = (&terms, self.docs.get(&command.name)) {
                    let score = search::content_score(doc, terms);
                    if score > 0 {
                        by_page.push(((terms.names(&command.name), score), command));
                    }
                }
            }
            by_name.sort_by(|(a, x), (b, y)| a.cmp(b).then_with(|| x.name.cmp(&y.name)));
            by_page.sort_by(|(a, x), (b, y)| b.cmp(a).then_with(|| x.name.cmp(&y.name)));
            let ranked: Vec<Command> = by_name
                .into_iter()
                .map(|(_, c)| c.clone())
                .chain(by_page.into_iter().map(|(_, c)| c.clone()))
                .collect();
            if let Some(terms) = &terms {
                self.snippets = ranked
                    .iter()
                    .filter_map(|c| {
                        let doc = self.docs.get(&c.name)?;
                        Some((c.name.clone(), search::snippet(doc, terms)?))
                    })
                    .collect();
            }
            self.rows = rows::ranked(ranked);
        }
        self.list = ListState::default(); // back to the top of the list
        self.list
            .select(self.rows.iter().position(Row::is_selectable));
    }
}
