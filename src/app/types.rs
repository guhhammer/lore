//! The app's small types: its modes, topics and settings, and the
//! pickers' state.

use crate::configs::TOPIC_NAMES;

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
    /// The help over the screen (`?`): every key. ↑↓ scroll it, any other
    /// key closes it.
    Help,
    /// A page's first link alone in a box (`o`), to Ctrl+click; any key
    /// closes it.
    Link,
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
    /// Tools: a cheat sheet each (docker, ssh keys, postgresql), from tools/.
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
        crate::utils::i18n::tr(TOPIC_NAMES[at])
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
