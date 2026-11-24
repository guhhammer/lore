//! The app's state and the moves on it. No drawing, no keys: those are in
//! interface/ui.rs and interface/input.rs.

use std::collections::{HashMap, HashSet};

use ratatui::widgets::ListState;

use crate::commands::{self, Command, Kind};
use crate::compile_config::{COPIED, MAN_PROBE_WIDTH, MOST_USED_MAX, NOTHING_TO_COPY, RECENT_MAX};
use crate::info::content;
use crate::info::page::{self, Facts, PageLine};
use crate::interface::palette::{self, Palette};
use crate::interface::rows::{self, Row};
use crate::interface::search;
use crate::interface::section::Section;
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
    /// The palette picker over the screen: ↑↓ try a palette, Enter keeps
    /// it, Esc goes back to the one before.
    Palette,
    /// The list of the page's examples over it: ↑↓ choose, Enter copies,
    /// Esc closes. Opened from a page only.
    Copy,
    /// The about screen (`+`): what lore is, where it keeps things.
    About,
}

/// The open palette picker: where it was opened from, and what to go back
/// to on Esc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Picker {
    /// Browse or Info: the mode to return to.
    pub from: Mode,
    /// Where the palette from before is in `palettes`.
    pub before: usize,
    /// Why the palette could not be saved, after a failed Enter.
    pub note: Option<String>,
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
    /// A line for the key bar in place of the keys, until the next key
    /// (`copied: tar -cf …`).
    pub note: Option<String>,
    /// The mode the about screen was opened from, to go back to.
    pub about_from: Mode,
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
            palettes: palette::built_in(),
            chosen: 0,
            picker: None,
            bookmarks: Vec::new(),
            recent: Vec::new(),
            uses: HashMap::new(),
            example: 0,
            note: None,
            about_from: Mode::Browse,
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
        match self.mode {
            Mode::Info => true,
            Mode::Search => self.beside_page,
            // the picker sits over whatever was on screen
            Mode::Palette => self.picker.as_ref().is_some_and(|p| p.from == Mode::Info),
            Mode::Copy => true,
            Mode::About => false,
            Mode::Browse => false,
        }
    }

    /// Opens the about screen, from the list or a page.
    pub fn open_about(&mut self) {
        if matches!(self.mode, Mode::Browse | Mode::Info) {
            self.about_from = self.mode;
            self.mode = Mode::About;
        }
    }

    /// Back from the about screen to where it was opened.
    pub fn close_about(&mut self) {
        if self.mode == Mode::About {
            self.mode = self.about_from;
        }
    }

    /// The examples of the page on show, as typed (`tar -cf a.tar dir`).
    pub fn examples(&self) -> Vec<&str> {
        self.page
            .as_ref()
            .and_then(|page| page.facts.content.as_ref())
            .map(|content| {
                content
                    .usage
                    .iter()
                    .map(|(typed, _)| typed.as_str())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Opens the list of the page's examples to copy; on a page with none,
    /// says so instead.
    pub fn open_copy(&mut self) {
        if self.mode != Mode::Info {
            return;
        }
        if self.examples().is_empty() {
            self.note = Some(NOTHING_TO_COPY.to_owned());
        } else {
            self.example = 0;
            self.mode = Mode::Copy;
        }
    }

    /// Selects the example `steps` away, wrapping round.
    pub fn step_example(&mut self, steps: isize) {
        let count = self.examples().len().max(1);
        let forward = steps.rem_euclid(isize::try_from(count).unwrap_or(isize::MAX));
        self.example = (self.example + usize::try_from(forward).unwrap_or(0)) % count;
    }

    /// The example selected in the copy list.
    pub fn chosen_example(&self) -> Option<String> {
        self.examples().get(self.example).map(|e| (*e).to_owned())
    }

    /// Back to the page; `copied` is the example sent to the clipboard, for
    /// the key bar to say so.
    pub fn close_copy(&mut self, copied: Option<&str>) {
        if self.mode == Mode::Copy {
            self.mode = Mode::Info;
        }
        self.note = copied.map(|text| COPIED.replace("{text}", text));
    }

    /// Opens the palette picker over the list or the page.
    pub fn open_palettes(&mut self) {
        if matches!(self.mode, Mode::Browse | Mode::Info) {
            self.picker = Some(Picker {
                from: self.mode,
                before: self.chosen,
                note: None,
            });
            self.mode = Mode::Palette;
        }
    }

    /// The palette to draw with.
    pub fn palette(&self) -> &Palette {
        &self.palettes[self.chosen.min(self.palettes.len().saturating_sub(1))]
    }

    /// Tries the palette `steps` away in `palettes`, wrapping round.
    pub fn step_palette(&mut self, steps: isize) {
        let count = self.palettes.len().max(1);
        // a step back is a step of count - 1 forward
        let forward = steps.rem_euclid(isize::try_from(count).unwrap_or(isize::MAX));
        let forward = usize::try_from(forward).unwrap_or(0);
        self.chosen = (self.chosen + forward) % count;
        if let Some(picker) = &mut self.picker {
            picker.note = None;
        }
    }

    /// Closes the picker, keeping the palette tried (lib.rs has saved it).
    pub fn close_palettes(&mut self) {
        if let Some(picker) = self.picker.take() {
            self.mode = picker.from;
        }
    }

    /// Closes the picker and puts back the palette from before.
    pub fn cancel_palettes(&mut self) {
        if let Some(picker) = self.picker.take() {
            self.chosen = picker.before;
            self.mode = picker.from;
        }
    }

    /// The palette could not be saved: the picker stays open and says why.
    pub fn palette_failed(&mut self, why: &str) {
        if let Some(picker) = &mut self.picker {
            picker.note = Some(why.to_owned());
        }
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

    /// Opens the selected command's page, and puts it first among the
    /// recent ones; does nothing on an empty list.
    pub fn open_info(&mut self) {
        let Some(name) = self.selected().map(|c| c.name.clone()) else {
            return;
        };
        self.mode = Mode::Info;
        self.scroll = 0;
        self.beside_page = false;
        self.recent.retain(|n| *n != name);
        self.recent.insert(0, name);
        self.recent.truncate(RECENT_MAX);
        self.rebuild_keeping_selection();
    }

    /// Your bookmarks, recent pages and history at start (lib.rs reads them).
    pub fn set_personal(
        &mut self,
        bookmarks: Vec<String>,
        recent: Vec<String>,
        uses: HashMap<String, u32>,
    ) {
        self.bookmarks = bookmarks;
        self.recent = recent;
        self.uses = uses;
        self.refilter();
    }

    /// Bookmarks the command in view (the page's, or the selected one), or
    /// takes its bookmark away. False when there is no command in view.
    pub fn toggle_bookmark(&mut self) -> bool {
        let in_view = match (self.mode, &self.page) {
            (Mode::Info, Some(page)) => Some(page.command.name.clone()),
            _ => self.selected().map(|c| c.name.clone()),
        };
        let Some(name) = in_view else {
            return false;
        };
        if self.is_bookmarked(&name) {
            self.bookmarks.retain(|n| *n != name);
        } else {
            self.bookmarks.push(name);
        }
        self.rebuild_keeping_selection();
        true
    }

    pub fn is_bookmarked(&self, name: &str) -> bool {
        self.bookmarks.iter().any(|n| n == name)
    }

    /// A command your shell history never ran; never true without a history.
    pub fn is_unused(&self, name: &str) -> bool {
        !self.uses.is_empty() && !self.uses.contains_key(name)
    }

    /// The groups above A to Z: bookmarks, recent pages, most used, each
    /// only with commands in `allowed` (on this machine, `@` respected).
    fn groups(&self, allowed: &[&Command]) -> Vec<(Section, Vec<Command>)> {
        let find = |name: &String| {
            allowed
                .iter()
                .find(|c| c.name == *name)
                .map(|c| (*c).clone())
        };
        let mut most_used: Vec<&&Command> = allowed
            .iter()
            .filter(|c| self.uses.contains_key(&c.name))
            .collect();
        most_used.sort_by(|a, b| {
            self.uses[&b.name]
                .cmp(&self.uses[&a.name])
                .then_with(|| a.name.cmp(&b.name))
        });
        vec![
            (
                Section::Bookmarks,
                self.bookmarks.iter().filter_map(find).collect(),
            ),
            (
                Section::Recent,
                self.recent.iter().filter_map(find).collect(),
            ),
            (
                Section::MostUsed,
                most_used
                    .into_iter()
                    .take(MOST_USED_MAX)
                    .map(|c| (*c).clone())
                    .collect(),
            ),
        ]
    }

    /// Builds the rows again (a group changed) with the same command
    /// selected: the row with its name nearest the one selected before, so
    /// the list does not jump and an open page stays the same.
    fn rebuild_keeping_selection(&mut self) {
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
            // no search: your groups, then the whole list, A to Z under its letters
            let groups = self.groups(&allowed);
            self.rows =
                rows::with_groups(groups, rows::build(allowed.into_iter().cloned().collect()));
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
