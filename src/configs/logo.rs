//! Hardcoded values: the name, drawn.

use ratatui::style::Color;

/// lore's orange (guhhammer.dev's `--color-flare-500`): the logo and the
/// about screen's banner are always in it, whatever the palette.
pub const LORE_ORANGE: Color = Color::Rgb(0xe8, 0x79, 0x3a);

/// "lore" in Press Start 2P, the 8-bit font, from its own 8×8 pixels: two
/// pixel rows a line, in half blocks. On every screen, left of the search
/// bar.
pub const LOGO: [&str; 4] = [
    "  ▀██",
    "   ██   ▄█▀▀▀█▄  ██▄▀▀▀ ▄█▀▀▀█▄",
    "   ██   ██   ██  ██     ██▀▀▀▀▀",
    " ▀▀▀▀▀▀  ▀▀▀▀▀   ▀▀      ▀▀▀▀▀",
];

/// Columns between the logo and the search bar.
pub const LOGO_GAP: u16 = 2;

/// The key that opens the about screen, from the list or a page.
pub const ABOUT_KEY: char = '+';

/// The about screen: its title, the line under the banner, its fields.
pub const ABOUT_TITLE: &str = " about ";
pub const ABOUT_TAGLINE: &str =
    "lore {version}: every command you can type, and what each one does. Offline.";
pub const ABOUT_COMMANDS: &str = "{total} on this machine, {pages} with a lore page";
pub const ABOUT_PAGES: &str = "{pages} built in, translations in {languages} languages";
pub const ABOUT_KEPT: &str = "{bookmarks} bookmarks, {recent} recent pages";
pub const ABOUT_LICENCE: &str = "MIT; examples and translations from tldr-pages, CC BY 4.0";
pub const ABOUT_SOURCE: &str = "https://github.com/guhhammer/lore";
pub const ABOUT_MANUAL: &str = "man lore, lore --help";
pub const ABOUT_FIELDS: [&str; 8] = [
    "Commands", "Pages", "Config", "Palettes", "Kept", "Licence", "Source", "Manual",
];
/// The author's profiles, to follow: a heading, a call to follow, and each
/// one's name and link (opened by the terminal's Ctrl+click).
pub const ABOUT_FOLLOW_HEADING: &str = "FOLLOW";
pub const ABOUT_FOLLOW_HINT: &str = "like lore? follow guhhammer for what comes next (Ctrl+click)";
pub const ABOUT_FOLLOW: &[(&str, &str)] = &[
    ("GitHub", "https://github.com/guhhammer"),
    ("LinkedIn", "https://www.linkedin.com/in/guhhammer/"),
    ("X", "https://x.com/guhhammer"),
];

/// The about screen's settings, changed there (↑↓ choose, Enter changes)
/// and saved in the settings file at once: their heading, and each one's
/// name, in the order shown (`Setting::all()` in src/app/types.rs).
pub const ABOUT_SETTINGS_HEADING: &str = "SETTINGS";
pub const ABOUT_SETTINGS_HINT: &str = "↑↓ choose, Enter changes; saved for the next runs";
pub const ABOUT_SETTING_NAMES: [&str; 7] = [
    "Colours",
    "Internet",
    "History",
    "Fast scroll",
    "Pages in",
    "Favourite",
    "Reset all",
];
/// Reset all: every setting back to its default, and what lore kept for
/// you erased (bookmarks, recent pages, downloaded docs and pages, the
/// caches). Your own pages and palettes stay. Enter asks, Enter again does it.
pub const ABOUT_RESET: &str =
    "Enter: settings back to default, bookmarks, recent, downloads and caches erased";
pub const ABOUT_RESET_ARMED: &str =
    "Enter again to erase it all for good; ↑↓ or Esc keep everything";
pub const RESET_DONE: &str =
    "reset: every setting is back to its default, and lore's saved things are erased";
pub const RESET_FAILED: &str = "reset, but some could not be erased: {why}";
/// Pages in: the language pages are read in (translations of the command
/// and language pages), `system` following `LANG`; saved as
/// `reading = pt_BR`. Enter opens a picker of the languages there are.
pub const SETTING_READING: &str = "reading";
pub const READING_SYSTEM: &str = "system";
pub const READING_ENGLISH: &str = "en";
pub const ABOUT_READING: &str = "{code}  (Enter picks one of {count})";
pub const ABOUT_READING_SYSTEM: &str = "system: from LANG ({code})";
/// Favourite: the programming language pinned on top of the languages
/// topic; saved as `favourite = Python`. Enter opens a picker of them.
pub const SETTING_FAVOURITE: &str = "favourite";
pub const FAVOURITE_NONE: &str = "none";
pub const ABOUT_FAVOURITE: &str = "{name}  (Enter picks one; first in the languages topic)";
pub const LANGUAGE_FAVOURITE_TITLE: &str = "favourite";
/// The picker of those two settings, over the about screen; a letter
/// jumps to the next choice starting with it.
pub const CHOOSE_TITLE_READING: &str = " pages in ";
pub const CHOOSE_TITLE_FAVOURITE: &str = " favourite language ";
pub const CHOOSE_MARGIN: u16 = 6;
/// How far PgUp and PgDn move in it.
pub const CHOOSE_PAGE: isize = 10;
pub const KEYS_CHOOSE: &[(&str, &str)] = &[
    ("↑↓ PgUp/PgDn", "Choose"),
    ("a-z", "Jump"),
    ("Enter", "Keep"),
    ("Esc", "Cancel"),
    ("^C", "Quit"),
];
/// The language chosen in Pages in, for this run (None: `LANG` decides).
/// Set at start and by the picker; `info::content::languages` reads it.
pub static READING_LANGUAGE: std::sync::RwLock<Option<String>> = std::sync::RwLock::new(None);
/// Fast scroll: how many lines `w` and `s` move, in the list and on a
/// page. Enter on the about screen goes to the next of these; saved as
/// `scroll = 10`.
pub const FAST_SCROLL_CHOICES: [usize; 5] = [5, 10, 15, 20, 30];
pub const FAST_SCROLL_DEFAULT: usize = 10;
pub const SETTING_FAST_SCROLL: &str = "scroll";
pub const FAST_UP_KEY: char = 'w';
pub const FAST_DOWN_KEY: char = 's';
pub const ABOUT_FAST_SCROLL: &str = "{lines} lines a press of w or s  (Enter: 5, 10, 15, 20 or 30)";
/// The mark before the chosen setting, and the room it takes on the others.
pub const ABOUT_SETTING_MARK: &str = "▸ ";
pub const ABOUT_COLOURS: &str = "{name}  ({count} palettes: Enter picks one; p does too)";
pub const ABOUT_INTERNET_ON: &str = "on, online: pages show cht.sh's cheat sheets";
pub const ABOUT_INTERNET_OFFLINE: &str = "on, offline: no connection right now";
pub const ABOUT_INTERNET_OFF: &str = "off: lore makes no request";
pub const ABOUT_INTERNET_CHECKING: &str = "on, checking…";
pub const ABOUT_HISTORY_ON: &str =
    "on: {seen} commands seen in your shell's history (only read, here)";
pub const ABOUT_HISTORY_EMPTY: &str = "on: no shell history found";
pub const ABOUT_HISTORY_OFF: &str = "off: your shell's history is not read";
/// The key bar, when a setting could not be saved (it still holds for this run).
pub const SETTING_NOT_SAVED: &str = "not saved for the next runs: {why}";
/// Width of the field names' column on the about screen.
pub const ABOUT_LABEL_WIDTH: usize = 12;

/// The about screen's keys.
pub const KEYS_ABOUT: &[(&str, &str)] = &[
    ("↑↓", "Choose"),
    ("Enter", "Change"),
    ("+/←/Esc", "Back"),
    ("^C", "Quit"),
];

/// The key that opens the list of a page's examples to copy.
pub const COPY_KEY: char = 'c';

/// That list's title, and what the key bar says after a copy (until the
/// next key) or when a page has nothing to copy.
pub const COPY_TITLE: &str = " copy ";
pub const COPIED: &str = "copied: {text}";
pub const NOTHING_TO_COPY: &str = "this page has nothing to copy";

/// After a piece's first line in the copy list: how many lines follow.
pub const COPY_MORE_LINES: &str = "  (+{n} lines)";
pub const COPY_ONE_MORE_LINE: &str = "  (+1 line)";

/// Columns the copy list leaves free on each side of the page.
pub const COPY_PICKER_MARGIN: u16 = 2;

/// The key that opens the palette picker, on the about screen (`+`) only.
pub const PALETTE_KEY: char = 'p';

/// The picker's title, and its title when the choice could not be saved.
pub const PALETTE_TITLE: &str = " colours ";
pub const PALETTE_NOT_SAVED: &str = " not saved: {why} ";

/// Columns the picker adds around the longest name: borders and margins.
pub const PALETTE_PICKER_MARGIN: u16 = 6;

/// The environment variable that names a palette for one run
/// (`LORE_PALETTE=nord lore`), over the one saved in the settings file.
pub const PALETTE_VAR: &str = "LORE_PALETTE";
