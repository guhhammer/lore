//! The app's state and the moves on it. No drawing, no keys: those are in
//! interface/ui.rs and interface/input.rs.

mod about;
mod keys;
mod page_view;
mod topics;
mod types;

pub use page_view::copy_label;
pub use types::*;

use std::collections::{HashMap, HashSet};

use ratatui::widgets::ListState;

use crate::commands::{self, Command, Kind};
use crate::configs::{FAST_SCROLL_DEFAULT, LANGUAGE_FAVOURITE_TITLE, MAN_PROBE_WIDTH};
use crate::info::page::{Facts, PageLine};
use crate::info::{beacon, docs};
use crate::info::{content, recipes};
use crate::interface::palette::{self, Palette};
use crate::interface::rows::{self, Row};
use crate::interface::search;
use crate::utils::i18n::tr;
use crate::utils::manual;

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
    /// Searching from a page (`/` while reading): the page stays beside the
    /// list until another one is opened.
    pub beside_page: bool,
    /// What each listed command's page says, for searching by content and
    /// for its categories in the list.
    pub docs: HashMap<String, search::Doc>,
    /// For each search result whose page has a line matching the search,
    /// that line cut around the match: shown after the name.
    pub snippets: HashMap<String, String>,
    /// Every palette there is (built in, then yours); lib.rs loads them.
    pub palettes: Vec<Palette>,
    /// Where the one drawn with is in `palettes`; see `palette()`.
    pub chosen: usize,
    /// The palette picker, while it is open (Mode::Palette).
    pub picker: Option<Picker>,
    /// Your bookmarks, in the order added; lib.rs loads and keeps them.
    pub bookmarks: Vec<String>,
    /// The pages you opened, the last one first, at most `RECENT_MAX`.
    pub recent: Vec<String>,
    /// How often your shell history ran each command; empty when there is
    /// no history (or `history = off`), and then nothing is marked unused.
    pub uses: HashMap<String, u32>,
    /// Which example the copy list has selected (Mode::Copy).
    pub example: usize,
    /// What the copy list offers, taken when it opens: the examples, or a
    /// language's or a doc's code.
    pub copies: Vec<String>,
    /// How far the topic bar is slid left, in columns: it slides when the
    /// chosen topic would be past an edge of a narrow window (ui.rs).
    pub topic_scroll: usize,
    /// A line for the key bar in place of the keys, until the next key
    /// (`copied: tar -cf …`).
    pub note: Option<String>,
    /// The mode the help or the link box was opened from, to go back to.
    pub overlay_from: Mode,
    /// How far the help is scrolled, and the link the box shows.
    pub help_scroll: usize,
    pub link: Option<String>,
    /// The page's lines (as drawn) holding the search bar's text, for `n`
    /// and `N`: ui.rs sets them at each draw, like `sections`.
    pub matches: Vec<usize>,
    /// The mode the about screen was opened from, to go back to.
    pub about_from: Mode,
    /// Where the page's sections start, as drawn (the INTERNET section
    /// counted), and how far it can scroll: ui.rs sets both at each draw,
    /// for the jumper (Space).
    pub sections: Vec<usize>,
    pub scroll_max: usize,
    /// The topic whose list is shown, and the topics that have anything
    /// (commands always): the bar shows those, and ← → move among them.
    pub topic: Topic,
    pub topics: Vec<Topic>,
    /// The setting chosen on the about screen, in `Setting::all()`.
    pub setting: usize,
    /// Whether the shell's history is read (`history = off`: not).
    pub history: bool,
    /// How many lines `w` and `s` move (`scroll = 10`).
    pub fast_scroll: usize,
    /// Pages in: a language code, or None for `system` (LANG).
    pub reading: Option<String>,
    /// The favourite programming language, by its name in the list.
    pub favourite: Option<String>,
    /// The picker open over the about screen (Mode::Choose).
    pub choice: Option<Choice>,
    /// Reset all was pressed once: Enter again resets, anything else keeps.
    pub reset_armed: bool,
    /// How far the about screen is scrolled, on a window too small for it
    /// (ui.rs keeps the chosen setting in view).
    pub about_scroll: usize,
    /// The chosen setting just changed: ui.rs scrolls to show it.
    pub about_follow: bool,
    /// The docs topic: every set there is, the downloads under way, and the
    /// set whose pages are listed (Enter on a downloaded set; Esc leaves).
    pub docsets: Vec<docs::Docset>,
    pub downloads: std::sync::Arc<docs::Downloads>,
    pub docset: Option<(docs::Docset, Vec<docs::Entry>)>,
    /// The beacon's numbers (empty: no beacon topic), and the categories
    /// of the most used among them, read once from their pages.
    pub beacon: Vec<beacon::Entry>,
    pub beacon_categories: HashMap<String, Vec<String>>,
    /// The connection and the cheat sheets fetched (off in tests; lib.rs
    /// starts it unless `internet = off`).
    pub internet: std::sync::Arc<crate::utils::internet::Internet>,
}

/// What each listed command's page says, for the search: in the reader's
/// language where a translation has it (`content::find` and
/// `languages::content` pick it), else in English.
fn docs_of(commands: &[Command], pages: &HashSet<String>) -> HashMap<String, search::Doc> {
    commands
        .iter()
        .filter(|c| pages.contains(&content::page_key(&c.name)))
        .filter_map(|c| Some((c.name.clone(), search::Doc::of(&recipes::content_of(c)?))))
        .collect()
}

impl App {
    /// Starts in Browse mode with the whole list and the first command selected.
    pub fn new(commands: Vec<Command>) -> Self {
        Self::with_pages(commands, content::names())
    }

    /// Like `new`, with the set of names that have a lore page given.
    pub fn with_pages(commands: Vec<Command>, mut pages: HashSet<String>) -> Self {
        // a recipe is a page of its own, for `@` and for searching by content
        pages.extend(
            commands
                .iter()
                .filter(|c| matches!(c.kind, Kind::Recipe | Kind::Tool | Kind::Language))
                .map(|c| content::page_key(&c.name)),
        );
        let docs = docs_of(&commands, &pages);
        let mut app = Self {
            commands,
            query: String::new(),
            mode: Mode::Browse,
            rows: Vec::new(),
            list: ListState::default(),
            page: None,
            scroll: 0,
            pages,
            beside_page: false,
            docs,
            snippets: HashMap::new(),
            palettes: palette::built_in(),
            chosen: 0,
            picker: None,
            bookmarks: Vec::new(),
            recent: Vec::new(),
            uses: HashMap::new(),
            example: 0,
            copies: Vec::new(),
            topic_scroll: 0,
            note: None,
            about_from: Mode::Browse,
            overlay_from: Mode::Browse,
            help_scroll: 0,
            link: None,
            matches: Vec::new(),
            sections: Vec::new(),
            topic: Topic::Commands,
            topics: vec![Topic::Commands],
            scroll_max: 0,
            setting: 0,
            history: true,
            fast_scroll: FAST_SCROLL_DEFAULT,
            reading: None,
            favourite: None,
            choice: None,
            reset_armed: false,
            about_scroll: 0,
            about_follow: false,
            docsets: docs::catalog(),
            downloads: std::sync::Arc::default(),
            docset: None,
            beacon: Vec::new(),
            beacon_categories: HashMap::new(),
            internet: crate::utils::internet::Internet::off(),
        };
        app.refilter(); // builds the rows from the empty query
        app
    }

    /// Searches for `query` across every topic, as typed in the search bar.
    pub fn search(&mut self, query: &str) {
        self.topic = Topic::Commands;
        self.query = query.to_owned();
        self.refilter();
    }

    /// The search's results as plain lines, the best first, at most `max`:
    /// name, the page line that matched (else the summary), categories,
    /// tab apart (`lore search`).
    pub fn result_lines(&self, max: usize) -> Vec<String> {
        self.rows
            .iter()
            .filter_map(|row| match row {
                Row::Command(c) => Some(c),
                _ => None,
            })
            .take(max)
            .map(|c| {
                let content = recipes::content_of(c);
                // the list shows a matched line in [brackets]; plain here
                let snippet = self.snippets.get(&c.name).map(|s| {
                    let s = s.strip_prefix('[').unwrap_or(s);
                    s.strip_suffix(']').unwrap_or(s).to_owned()
                });
                let line = snippet.unwrap_or_else(|| {
                    content
                        .as_ref()
                        .and_then(|p| p.summary.first().cloned())
                        .unwrap_or_default()
                });
                let categories = self
                    .docs
                    .get(&c.name)
                    .map(|d| d.categories.join(" "))
                    .unwrap_or_default();
                format!("{}\t{}\t{}", c.name, line.replace('\t', " "), categories)
            })
            .collect()
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
            Row::Header(_) | Row::Title(_) => None,
        }
    }

    /// Builds the rows again (a download ended) with the same command
    /// selected.
    pub fn refresh_rows(&mut self) {
        self.rebuild_keeping_selection();
    }

    /// Builds the rows again (a group changed) with the same command
    /// selected: the row with its name nearest the one selected before, so
    /// the list does not jump and an open page stays the same.
    pub(super) fn rebuild_keeping_selection(&mut self) {
        let before = self.list.selected();
        let name = self.selected().map(|c| c.name.clone());
        let offset = self.list.offset();
        self.refilter();
        let (Some(before), Some(name)) = (before, name) else {
            return;
        };
        let nearest = self
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| matches!(row, Row::Command(c) if c.name == name))
            .min_by_key(|(i, _)| i.abs_diff(before))
            .map(|(i, _)| i);
        if let Some(i) = nearest {
            self.list.select(Some(i));
            *self.list.offset_mut() = offset;
        }
    }

    /// Tab in the list: selects the first command of the next letter, its
    /// header shown on top; after the last, the first one again. A
    /// search's results, and your own topics, have no letters.
    pub fn next_letter(&mut self) {
        let headers = self.headers();
        let at = self.list.selected().unwrap_or(0);
        let next = headers
            .iter()
            .find(|&&h| h > at)
            .or_else(|| headers.first());
        if let Some(&header) = next {
            self.select_under(header);
        }
    }

    /// Shift+Tab in the list: the letter before the selected one's; from
    /// the first, the last.
    pub fn previous_letter(&mut self) {
        let headers = self.headers();
        let at = self.list.selected().unwrap_or(0);
        // the selected command's own letter, then the one before it
        let own = headers.iter().rposition(|&h| h < at);
        let back = match own {
            Some(i) if i > 0 => headers.get(i - 1),
            _ => headers.last(),
        };
        if let Some(&header) = back {
            self.select_under(header);
        }
    }

    /// Where the list's letter headers are.
    pub(super) fn headers(&self) -> Vec<usize> {
        (0..self.rows.len())
            .filter(|&i| !self.rows[i].is_selectable())
            .collect()
    }

    /// Selects the first command under the header at `header`, with the
    /// header on top of the list.
    pub(super) fn select_under(&mut self, header: usize) {
        if let Some(first) = (header..self.rows.len()).find(|&i| self.rows[i].is_selectable()) {
            self.list.select(Some(first));
            *self.list.offset_mut() = header;
        }
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

    /// Moves the selection `steps` commands down (up when negative),
    /// headers skipped, stopping at either end: PgUp/PgDn and `w`/`s`.
    pub fn move_by(&mut self, steps: isize) {
        for _ in 0..steps.unsigned_abs() {
            if steps < 0 {
                self.up();
            } else {
                self.down();
            }
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

    /// Opens `name`'s page at start (`lore NAME`). Everything works from
    /// there as usual: ← goes back to the list, Esc there quits.
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
        // the topic that holds it: a recipe's, or the commands'
        let kind = self
            .commands
            .iter()
            .find(|c| c.name == name)
            .map(|c| c.kind);
        self.topic = match kind {
            Some(Kind::Recipe) => Topic::Recipes,
            Some(Kind::Tool) => Topic::Tools,
            Some(Kind::Language) => Topic::Languages,
            _ => Topic::Commands,
        };
        self.refilter();
        let Some(row) = self
            .rows
            .iter()
            .position(|r| matches!(r, Row::Command(c) if c.name == name))
        else {
            return false;
        };
        self.list.select(Some(row));
        self.open_info();
        true
    }

    /// Rebuilds `rows` from `query` and selects the first match.
    pub(super) fn refilter(&mut self) {
        let (pages_only, query) = search::split(&self.query);
        let allowed: Vec<&Command> = self
            .commands
            .iter()
            .filter(|c| !pages_only || self.pages.contains(&content::page_key(&c.name)))
            .collect();
        self.snippets.clear();
        // the topics with anything in them; one that emptied (the last
        // bookmark taken away) is left for the commands
        self.topics = Topic::all()
            .into_iter()
            .filter(|t| self.has_any(*t, &allowed))
            .collect();
        if !self.topics.contains(&self.topic) {
            self.topic = Topic::Commands;
        }
        // the docs topic searches among its sets (or an open set's pages)
        if self.topic == Topic::Docs {
            let (rows, snippets) = self.docs_rows(query);
            self.rows = rows;
            self.snippets = snippets;
            self.list = ListState::default();
            self.list
                .select(self.rows.iter().position(Row::is_selectable));
            return;
        }
        if query.trim().is_empty() {
            // no search: the topic's list; A to Z under its letters for the
            // big ones, in its own order for yours
            self.rows = match self.topic {
                Topic::Beacon => {
                    let (rows, snippets) = self.beacon_rows(&allowed);
                    self.snippets = snippets;
                    rows
                }
                Topic::Docs => {
                    let (rows, snippets) = self.docs_rows("");
                    self.snippets = snippets;
                    rows
                }
                Topic::Languages => {
                    let list = self.topic_list(self.topic, &allowed);
                    // the favourite on top, under its own heading
                    let favourite = self
                        .favourite
                        .as_ref()
                        .and_then(|name| list.iter().find(|c| c.name == *name).cloned());
                    let mut rows = Vec::new();
                    if let Some(favourite) = favourite {
                        rows.push(Row::Title(tr(LANGUAGE_FAVOURITE_TITLE).to_owned()));
                        rows.push(Row::Command(favourite));
                    }
                    rows.extend(rows::build(list));
                    rows
                }
                Topic::Recipes => {
                    // A to Z by the names shown, in the reader's language
                    let mut list = self.topic_list(self.topic, &allowed);
                    list.sort_by_cached_key(|c| crate::utils::i18n::shown(c).to_lowercase());
                    rows::build(list)
                }
                Topic::Commands | Topic::Tools => {
                    rows::build(self.topic_list(self.topic, &allowed))
                }
                _ => rows::ranked(self.topic_list(self.topic, &allowed)),
            };
        } else {
            // a search: best first, never alphabetical. Names that match come
            // first (an exact name on top), then pages that match by content.
            let terms = search::Terms::new(query); // None: too short for pages
            let mut by_name: Vec<((u8, usize, usize), &Command)> = Vec::new();
            // (named by a word of the query, score): a command named in the
            // query ("cut files") goes first among the page matches
            let mut by_page: Vec<((bool, u32), &Command)> = Vec::new();
            for command in allowed {
                // a recipe is found by its name in the reader's language too
                let rank = search::name_rank(&command.name, query).or_else(|| {
                    (command.kind == Kind::Recipe)
                        .then(|| search::name_rank(crate::utils::i18n::shown(command), query))
                        .flatten()
                });
                if let Some(rank) = rank {
                    by_name.push((rank, command));
                } else if let (Some(terms), Some(doc)) = (&terms, self.docs.get(&command.name)) {
                    let score = search::content_score(doc, terms);
                    if score > 0 {
                        by_page.push(((terms.names(&command.name), score), command));
                    }
                }
            }
            // within a kind of match, the commands you run most go first
            let used = |c: &Command| self.uses.get(&c.name).copied().unwrap_or(0);
            by_name.sort_by(|(a, x), (b, y)| {
                a.0.cmp(&b.0)
                    .then_with(|| used(y).cmp(&used(x)))
                    .then_with(|| a.cmp(b))
                    .then_with(|| x.name.cmp(&y.name))
            });
            by_page.sort_by(|(a, x), (b, y)| {
                b.cmp(a)
                    .then_with(|| used(y).cmp(&used(x)))
                    .then_with(|| x.name.cmp(&y.name))
            });
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
