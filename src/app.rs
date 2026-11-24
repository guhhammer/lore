//! The app's state and the moves on it. No drawing, no keys: those are in
//! interface/ui.rs and interface/input.rs.

use std::collections::{HashMap, HashSet};

use ratatui::widgets::ListState;

use crate::commands::{self, Command, Kind};
use crate::compile_config::{
    BEACON_CATEGORIES_MAX, BEACON_CATEGORY_POOL, BEACON_FOR, BEACON_NOT_HERE, BEACON_PER_CATEGORY,
    BEACON_POPULAR, BEACON_POPULAR_MAX, BEACON_RANK, BEACON_RISE, BEACON_TRENDING,
    BEACON_TRENDING_MAX, COPIED, DANGER, DOCS_ALL, DOCS_DOWNLOADED, DOCS_DOWNLOADING, DOCS_FAILED,
    DOCS_HERE, DOCS_MOST, DOCS_MOST_SEARCHED, DOCS_SIZE, FAST_SCROLL_CHOICES, FAST_SCROLL_DEFAULT,
    LANGUAGE_FAVOURITE_TITLE, MAN_PROBE_WIDTH, MOST_USED_MAX, NOTHING_TO_COPY, RECENT_MAX,
    TOPIC_NAMES,
};
use crate::info::page::{self, Facts, PageLine};
use crate::info::{beacon, docs};
use crate::info::{content, recipes};
use crate::interface::palette::{self, Palette};
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
    /// The palette picker over the screen: ↑↓ try a palette, Enter keeps
    /// it, Esc goes back to the one before.
    Palette,
    /// The list of the page's examples over it: ↑↓ choose, Enter copies,
    /// Esc closes. Opened from a page only.
    Copy,
    /// The about screen (`+`): what lore is, where it keeps things, and
    /// the settings: ↑↓ choose one, Enter changes it.
    About,
    /// A picker over the about screen, for a setting with many choices
    /// (Pages in, Favourite): ↑↓ choose, a letter jumps, Enter keeps, Esc
    /// goes back.
    Choose,
}

/// The open picker of `Mode::Choose`: for which setting, its choices, and
/// the one chosen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    pub setting: Setting,
    pub items: Vec<String>,
    pub at: usize,
}

/// A topic: one list of its own, chosen in the bar under the search bar
/// with ← and →. More kinds of lore will be topics of their own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Topic {
    /// Every command on this machine, A to Z (recipes not among them).
    Commands,
    /// The task pages, A to Z.
    Recipes,
    /// learnxinyminutes.com's pages, one per programming language, A to Z.
    Languages,
    /// Documentation sets (DevDocs): download one to read it offline.
    Docs,
    /// Tools: coming; empty for now.
    Tools,
    /// What people use most and what is rising (`info::beacon`), installed
    /// here or not: trending, most used, then most used per category.
    Beacon,
    /// The commands your shell history runs most, most first.
    MostUsed,
    /// The pages you opened last, the last first.
    Recent,
    /// Your bookmarks, in the order added.
    Bookmarks,
}

impl Topic {
    /// Every topic, in the bar's order (`TOPIC_NAMES` holds their names).
    pub fn all() -> [Self; 9] {
        [
            Self::Commands,
            Self::Recipes,
            Self::Languages,
            Self::Docs,
            Self::Tools,
            Self::Beacon,
            Self::MostUsed,
            Self::Recent,
            Self::Bookmarks,
        ]
    }

    /// The name shown in the bar and the list's title.
    pub fn name(self) -> &'static str {
        let at = Self::all().iter().position(|t| *t == self).unwrap_or(0);
        TOPIC_NAMES[at]
    }
}

/// A setting on the about screen, in the order shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Setting {
    /// The palette: Enter opens the picker over the about screen.
    Colours,
    /// The INTERNET section and the dot: on or off.
    Internet,
    /// Reading the shell's history (most used, never-used marks): on or off.
    History,
    /// How many lines `w` and `s` move: one of `FAST_SCROLL_CHOICES`.
    FastScroll,
    /// The language pages are read in: `system` (LANG) or a code.
    Reading,
    /// The programming language pinned on top of the languages topic.
    Favourite,
    /// Every setting to its default, every saved thing erased: Enter, then
    /// Enter again.
    Reset,
}

impl Setting {
    pub fn all() -> [Self; 7] {
        [
            Self::Colours,
            Self::Internet,
            Self::History,
            Self::FastScroll,
            Self::Reading,
            Self::Favourite,
            Self::Reset,
        ]
    }
}

/// The open palette picker: where it was opened from, and what to go back
/// to on Esc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Picker {
    /// Browse, Info or About: the mode to return to.
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
                .filter(|c| matches!(c.kind, Kind::Recipe | Kind::Language))
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
            note: None,
            about_from: Mode::Browse,
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
            Mode::Palette => self.picker_from(Mode::Info),
            Mode::Copy => true,
            Mode::About | Mode::Choose => false,
            Mode::Browse => false,
        }
    }

    /// Opens the about screen, from the list or a page.
    pub fn open_about(&mut self) {
        if matches!(self.mode, Mode::Browse | Mode::Info) {
            self.about_from = self.mode;
            self.mode = Mode::About;
            self.about_scroll = 0; // from the top
            self.about_follow = false;
        }
    }

    /// True when the about screen is on show: open, or under the picker
    /// opened from it.
    pub fn about_shown(&self) -> bool {
        matches!(self.mode, Mode::About | Mode::Choose) || self.picker_from(Mode::About)
    }

    /// The palette picker is open and was opened from `mode`.
    fn picker_from(&self, mode: Mode) -> bool {
        self.mode == Mode::Palette && self.picker.as_ref().is_some_and(|p| p.from == mode)
    }

    /// Chooses the setting `steps` away on the about screen, wrapping round.
    pub fn step_setting(&mut self, steps: isize) {
        self.reset_armed = false; // moving away from Reset all keeps everything
        self.about_follow = true;
        let count = Setting::all().len();
        let forward = steps.rem_euclid(isize::try_from(count).unwrap_or(isize::MAX));
        self.setting = (self.setting + usize::try_from(forward).unwrap_or(0)) % count;
    }

    /// The setting chosen on the about screen.
    pub fn chosen_setting(&self) -> Setting {
        Setting::all()[self.setting % Setting::all().len()]
    }

    /// Turns reading the shell's history on (with what it holds) or off;
    /// the groups and marks follow at once.
    pub fn set_history(&mut self, on: bool, uses: HashMap<String, u32>) {
        self.history = on;
        self.uses = if on { uses } else { HashMap::new() };
        self.rebuild_keeping_selection();
    }

    /// Asks the internet for the page on show (after it was turned on).
    pub fn ask_internet(&self) {
        if let Some(page) = self
            .page
            .as_ref()
            .filter(|p| p.command.kind != Kind::Recipe)
        {
            self.internet.request(&page.command.name);
        }
    }

    /// Back from the about screen to where it was opened.
    pub fn close_about(&mut self) {
        self.reset_armed = false;
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

    /// Opens the palette picker over the list, the page or the about screen.
    pub fn open_palettes(&mut self) {
        if matches!(self.mode, Mode::Browse | Mode::Info | Mode::About) {
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
            Row::Header(_) | Row::Title(_) => None,
        }
    }

    /// Opens the selected command's page, and puts it first among the
    /// recent ones; does nothing on an empty list.
    pub fn open_info(&mut self) {
        let Some(command) = self.selected().cloned() else {
            return;
        };
        // a downloaded set opens as the list of its pages, not as a page
        if command.kind == Kind::Docset {
            let slug = command.runs.clone().unwrap_or_default();
            if docs::is_downloaded(&slug)
                && let Some(set) = self.docsets.iter().find(|s| s.slug == slug).cloned()
            {
                self.docset = Some((set, docs::entries(&slug)));
                self.query.clear();
                self.mode = Mode::Browse;
                self.refilter();
                return;
            }
        }
        let name = command.name;
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

    /// A command on this machine your shell history never ran; never true
    /// without a history, nor for one not installed (the beacon's), which
    /// says so itself.
    pub fn is_unused(&self, name: &str) -> bool {
        !self.uses.is_empty()
            && !self.uses.contains_key(name)
            && self.commands.iter().any(|c| c.name == name)
    }

    /// The commands of `topic`, among `allowed` (on this machine, `@`
    /// respected), in the topic's own order.
    fn topic_list(&self, topic: Topic, allowed: &[&Command]) -> Vec<Command> {
        let named = |names: &[String]| -> Vec<Command> {
            names
                .iter()
                .filter_map(|name| allowed.iter().find(|c| c.name == *name))
                .map(|c| (*c).clone())
                .collect()
        };
        match topic {
            Topic::Commands => allowed
                .iter()
                .filter(|c| !matches!(c.kind, Kind::Recipe | Kind::Language))
                .map(|c| (*c).clone())
                .collect(),
            Topic::Languages => allowed
                .iter()
                .filter(|c| c.kind == Kind::Language)
                .map(|c| (*c).clone())
                .collect(),
            Topic::Recipes => allowed
                .iter()
                .filter(|c| c.kind == Kind::Recipe)
                .map(|c| (*c).clone())
                .collect(),
            Topic::MostUsed => {
                let mut used: Vec<&&Command> = allowed
                    .iter()
                    .filter(|c| self.uses.contains_key(&c.name))
                    .collect();
                used.sort_by(|a, b| {
                    self.uses[&b.name]
                        .cmp(&self.uses[&a.name])
                        .then_with(|| a.name.cmp(&b.name))
                });
                used.into_iter()
                    .take(MOST_USED_MAX)
                    .map(|c| (*c).clone())
                    .collect()
            }
            Topic::Recent => named(&self.recent),
            Topic::Bookmarks => named(&self.bookmarks),
            // headed rows of their own: `beacon_rows`, `docs_rows`; tools
            // are still to come
            Topic::Beacon | Topic::Docs | Topic::Tools => Vec::new(),
        }
    }

    /// Whether `topic` has anything among `allowed`, without building its
    /// list; commands always counts, so the bar is never empty.
    fn has_any(&self, topic: Topic, allowed: &[&Command]) -> bool {
        let on_machine = |name: &String| allowed.iter().any(|c| c.name == *name);
        match topic {
            Topic::Commands => true,
            Topic::Recipes => allowed.iter().any(|c| c.kind == Kind::Recipe),
            Topic::Languages => allowed.iter().any(|c| c.kind == Kind::Language),
            Topic::Beacon => !self.beacon.is_empty(),
            Topic::Docs | Topic::Tools => true,
            Topic::MostUsed => allowed.iter().any(|c| self.uses.contains_key(&c.name)),
            Topic::Recent => self.recent.iter().any(on_machine),
            Topic::Bookmarks => self.bookmarks.iter().any(on_machine),
        }
    }

    /// Sets the beacon's numbers, and reads the categories of the most used
    /// among them from their pages, once.
    pub fn set_beacon(&mut self, entries: Vec<beacon::Entry>) {
        self.beacon_categories = beacon::popular(&entries)
            .into_iter()
            .take(BEACON_CATEGORY_POOL)
            .filter_map(|e| Some((e.name.clone(), content::find(&e.name)?.categories)))
            .collect();
        self.beacon = entries;
        self.refilter();
    }

    /// The beacon's rows, under their headings, and what follows each name
    /// (its rank, its rise, "not installed"). A command not on this machine
    /// is listed when lore has a page for it, to read about before
    /// installing it.
    fn beacon_rows(&self, allowed: &[&Command]) -> (Vec<Row>, HashMap<String, String>) {
        let listed = |name: &str| -> Option<Command> {
            match allowed.iter().find(|c| c.name == name) {
                Some(command) => Some((*command).clone()),
                None => self
                    .pages
                    .contains(&content::page_key(name))
                    .then(|| Command::new(name, Kind::Program)),
            }
        };
        let popular: Vec<&beacon::Entry> = beacon::popular(&self.beacon)
            .into_iter()
            .filter(|e| listed(&e.name).is_some())
            .collect();
        let mut snippets = HashMap::new();
        for (rank, entry) in popular.iter().enumerate() {
            let mut parts = vec![BEACON_RANK.replace("{rank}", &(rank + 1).to_string())];
            if let Some(rise) = beacon::rise(entry) {
                parts.push(BEACON_RISE.replace("{rise}", &format!("{rise:.1}")));
            }
            if !allowed.iter().any(|c| c.name == entry.name) {
                parts.push(BEACON_NOT_HERE.to_owned());
            }
            snippets.insert(entry.name.clone(), parts.join("  "));
        }
        let mut rows = Vec::new();
        let mut section = |title: String, names: Vec<&str>| {
            let commands: Vec<Command> = names.into_iter().filter_map(listed).collect();
            if !commands.is_empty() {
                rows.push(Row::Title(title));
                rows.extend(commands.into_iter().map(Row::Command));
            }
        };
        let trending = beacon::trending(&self.beacon)
            .into_iter()
            .filter(|(e, _)| listed(&e.name).is_some())
            .take(BEACON_TRENDING_MAX)
            .map(|(e, _)| e.name.as_str())
            .collect();
        section(BEACON_TRENDING.to_owned(), trending);
        let most = popular
            .iter()
            .take(BEACON_POPULAR_MAX)
            .map(|e| e.name.as_str());
        section(BEACON_POPULAR.to_owned(), most.collect());
        // per category, in the order their most used command comes
        let mut groups: Vec<(&str, Vec<&str>)> = Vec::new();
        for entry in &popular {
            let categories = self.beacon_categories.get(&entry.name);
            for category in categories.into_iter().flatten().filter(|c| *c != DANGER) {
                match groups.iter_mut().find(|(c, _)| c == category) {
                    Some((_, names)) if names.len() < BEACON_PER_CATEGORY => {
                        names.push(&entry.name);
                    }
                    Some(_) => {}
                    None => groups.push((category, vec![&entry.name])),
                }
            }
        }
        for (category, names) in groups
            .into_iter()
            .filter(|(_, names)| names.len() == BEACON_PER_CATEGORY)
            .take(BEACON_CATEGORIES_MAX)
        {
            section(BEACON_FOR.replace("{category}", category), names);
        }
        (rows, snippets)
    }

    /// Shows `topic`, if it is in the bar, its list from the top.
    pub fn show_topic(&mut self, topic: Topic) {
        if self.topics.contains(&topic) {
            self.topic = topic;
            self.refilter();
        }
    }

    /// Shows the topic `steps` away in the bar, wrapping round, its list
    /// from the top.
    pub fn step_topic(&mut self, steps: isize) {
        let count = self.topics.len().max(1);
        let at = self
            .topics
            .iter()
            .position(|t| *t == self.topic)
            .unwrap_or(0);
        let forward = steps.rem_euclid(isize::try_from(count).unwrap_or(isize::MAX));
        let next = (at + usize::try_from(forward).unwrap_or(0)) % count;
        self.topic = self.topics.get(next).copied().unwrap_or(Topic::Commands);
        self.refilter();
    }

    /// Leaves the set whose pages are listed, back to the sets; false when
    /// none is open.
    pub fn close_docset(&mut self) -> bool {
        if self.docset.take().is_none() {
            return false;
        }
        self.query.clear();
        self.refilter();
        true
    }

    /// The docs topic's rows: an open set's pages under their types, else
    /// the sets: downloaded, most searched, then every one, each with its
    /// state. With a `query`, only those whose name matches, best first.
    fn docs_rows(&self, query: &str) -> (Vec<Row>, HashMap<String, String>) {
        let mut rows = Vec::new();
        let mut snippets = HashMap::new();
        if let Some((set, entries)) = &self.docset {
            let entry = |e: &docs::Entry| Command {
                runs: Some(format!("{}\t{}\t{}", set.slug, e.path, e.kind)),
                ..Command::new(e.name.clone(), Kind::DocEntry)
            };
            if query.trim().is_empty() {
                rows.push(Row::Title(set.title()));
                let mut kinds: Vec<&str> = entries.iter().map(|e| e.kind.as_str()).collect();
                kinds.sort_unstable();
                kinds.dedup();
                for kind in kinds {
                    rows.push(Row::Title(kind.to_owned()));
                    let mut these: Vec<&docs::Entry> =
                        entries.iter().filter(|e| e.kind == kind).collect();
                    these.sort_by_key(|e| e.name.to_lowercase());
                    rows.extend(these.into_iter().map(|e| Row::Command(entry(e))));
                }
            } else {
                let mut found: Vec<(_, &docs::Entry)> = entries
                    .iter()
                    .filter_map(|e| Some((search::name_rank(&e.name, query)?, e)))
                    .collect();
                found.sort_by(|(a, x), (b, y)| a.cmp(b).then_with(|| x.name.cmp(&y.name)));
                rows.extend(found.into_iter().map(|(_, e)| Row::Command(entry(e))));
            }
            return (rows, snippets);
        }
        let row = |set: &docs::Docset| Command {
            runs: Some(set.slug.clone()),
            ..Command::new(set.title(), Kind::Docset)
        };
        for set in &self.docsets {
            let state = match self.downloads.progress(&set.slug) {
                Some(docs::Progress::Downloading) => DOCS_DOWNLOADING.to_owned(),
                Some(docs::Progress::Failed(why)) => DOCS_FAILED.replace("{why}", &why),
                None if docs::is_downloaded(&set.slug) => DOCS_HERE.to_owned(),
                None => {
                    DOCS_SIZE.replace("{size}", &format!("{:.1}", set.bytes as f64 / 1_048_576.0))
                }
            };
            snippets.insert(set.title(), state);
        }
        if !query.trim().is_empty() {
            let mut found: Vec<(_, &docs::Docset)> = self
                .docsets
                .iter()
                .filter_map(|s| Some((search::name_rank(&s.title(), query)?, s)))
                .collect();
            found.sort_by(|(a, x), (b, y)| a.cmp(b).then_with(|| x.rank.cmp(&y.rank)));
            rows.extend(found.into_iter().map(|(_, s)| Row::Command(row(s))));
            return (rows, snippets);
        }
        let here: Vec<&docs::Docset> = self
            .docsets
            .iter()
            .filter(|s| docs::is_downloaded(&s.slug))
            .collect();
        let mut section = |title: &str, sets: Vec<&docs::Docset>| {
            if !sets.is_empty() {
                rows.push(Row::Title(title.to_owned()));
                rows.extend(sets.into_iter().map(|s| Row::Command(row(s))));
            }
        };
        section(DOCS_DOWNLOADED, here);
        section(
            DOCS_MOST,
            docs::most_searched(&self.docsets, DOCS_MOST_SEARCHED),
        );
        let mut all: Vec<&docs::Docset> = self.docsets.iter().collect();
        all.sort_by_key(|s| s.title().to_lowercase());
        section(DOCS_ALL, all);
        (rows, snippets)
    }

    /// Builds the rows again (a download ended) with the same command
    /// selected.
    pub fn refresh_rows(&mut self) {
        self.rebuild_keeping_selection();
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
    fn headers(&self) -> Vec<usize> {
        (0..self.rows.len())
            .filter(|&i| !self.rows[i].is_selectable())
            .collect()
    }

    /// Selects the first command under the header at `header`, with the
    /// header on top of the list.
    fn select_under(&mut self, header: usize) {
        if let Some(first) = (header..self.rows.len()).find(|&i| self.rows[i].is_selectable()) {
            self.list.select(Some(first));
            *self.list.offset_mut() = header;
        }
    }

    /// Tab on a page: scrolls to the next section's heading; at the last
    /// one (or the end of the page), back to the top.
    pub fn next_section(&mut self) {
        let next = self.sections.iter().copied().find(|&s| s > self.scroll);
        self.scroll = match next {
            Some(line) if self.scroll < self.scroll_max => line.min(self.scroll_max),
            _ => 0,
        };
    }

    /// Shift+Tab on a page: the section before; from the top, the last one.
    pub fn previous_section(&mut self) {
        let back = self.sections.iter().copied().rfind(|&s| s < self.scroll);
        self.scroll = match back {
            Some(line) => line,
            None => self.sections.last().map_or(0, |&s| s.min(self.scroll_max)),
        };
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
                if !command.kind.is_lores() {
                    self.internet.request(&command.name); // in the background
                }
                page::facts(&command)
            }
        };
        // `man git-commit`; a recipe has no manual
        let man = (!command.kind.is_lores())
            .then(|| manual::page(&content::page_key(&command.name), width))
            .flatten();
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

    /// Opens the picker for `setting` over the about screen, `items` its
    /// choices, the one named `current` chosen (else the first).
    pub fn open_choice(&mut self, setting: Setting, items: Vec<String>, current: &str) {
        let at = items.iter().position(|i| i == current).unwrap_or(0);
        self.choice = Some(Choice { setting, items, at });
        self.mode = Mode::Choose;
    }

    /// The choice `steps` away, stopping at either end.
    pub fn step_choice(&mut self, steps: isize) {
        if let Some(choice) = &mut self.choice {
            let last = choice.items.len().saturating_sub(1);
            choice.at = choice.at.saturating_add_signed(steps).min(last);
        }
    }

    /// The next choice starting with `letter` (any case), round after the
    /// last; none: the choice stays.
    pub fn jump_choice(&mut self, letter: char) {
        if let Some(choice) = &mut self.choice {
            let count = choice.items.len();
            let starts = |i: usize| {
                choice.items[i]
                    .chars()
                    .next()
                    .is_some_and(|c| c.to_lowercase().eq(letter.to_lowercase()))
            };
            if let Some(i) = (1..=count)
                .map(|n| (choice.at + n) % count)
                .find(|&i| starts(i))
            {
                choice.at = i;
            }
        }
    }

    /// Closes the picker, back on the about screen; the setting and the
    /// item chosen when `keep`.
    pub fn close_choice(&mut self, keep: bool) -> Option<(Setting, String)> {
        self.mode = Mode::About;
        let choice = self.choice.take()?;
        let item = choice.items.get(choice.at).cloned()?;
        keep.then_some((choice.setting, item))
    }

    /// Reads the pages again for the search, in the language pages are
    /// read in now (Pages in changed), and searches again.
    pub fn reload_docs(&mut self) {
        self.docs = docs_of(&self.commands, &self.pages);
        self.refilter();
    }

    /// Sets the favourite language (None: none), shown first in its topic.
    pub fn set_favourite(&mut self, name: Option<String>) {
        self.favourite = name;
        self.refilter();
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

    /// The fast scroll's next choice, round after the last (Enter on the
    /// about screen); one not among them goes back to the first.
    pub fn next_fast_scroll(&mut self) {
        let at = FAST_SCROLL_CHOICES
            .iter()
            .position(|&n| n == self.fast_scroll);
        self.fast_scroll = match at {
            Some(i) => FAST_SCROLL_CHOICES[(i + 1) % FAST_SCROLL_CHOICES.len()],
            None => FAST_SCROLL_CHOICES[0],
        };
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
    fn refilter(&mut self) {
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
                        rows.push(Row::Title(LANGUAGE_FAVOURITE_TITLE.to_owned()));
                        rows.push(Row::Command(favourite));
                    }
                    rows.extend(rows::build(list));
                    rows
                }
                Topic::Commands | Topic::Recipes => {
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
