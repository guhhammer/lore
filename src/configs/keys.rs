//! The quick keys: top and bottom, finding on a page, copying at once,
//! the link box, jumping to a topic, and the help over everything (`?`).

/// `g` and `G`: the top and the bottom of the list or the page.
pub const TOP_KEY: char = 'g';
pub const BOTTOM_KEY: char = 'G';
/// `n` and `N` on a page: the next and the previous line holding the
/// search bar's text (type it with `/`, leave with Esc: the text stays).
pub const FIND_NEXT_KEY: char = 'n';
pub const FIND_PREVIOUS_KEY: char = 'N';
pub const FIND_NO_TEXT: &str = "nothing to find: search a word with / first, then n jumps to it";
pub const FIND_NOT_HERE: &str = "\"{text}\" is not on this page";
pub const FIND_AT: &str = "\"{text}\": {at} of {count}";
/// `y` on a page: copies the first example (a doc's first code block)
/// straight away, else the page's name.
pub const QUICK_COPY_KEY: char = 'y';
/// `o` on a page: its first link alone in a box, to Ctrl+click.
pub const LINK_KEY: char = 'o';
pub const LINK_TITLE: &str = " link ";
pub const LINK_NONE: &str = "this page has no link";
/// `B` and `r`: the bookmarks and the recent pages, from the list or a page.
pub const BOOKMARKS_KEY: char = 'B';
pub const RECENT_KEY: char = 'r';
pub const TOPIC_EMPTY: &str = "nothing in {topic} yet";
/// `?`: the help over the screen, every key.
pub const HELP_KEY: char = '?';
pub const HELP_TITLE: &str = " keys ";
/// Columns the help and the link box leave free on each side.
pub const OVERLAY_MARGIN: u16 = 2;

/// The help's lines: (keys, what they do); a key of "" is a heading.
pub const HELP_KEYS: &[(&str, &str)] = &[
    ("", "Everywhere"),
    ("?", "this help"),
    ("^L", "draw the screen again"),
    ("^C", "quit"),
    ("", "The list"),
    ("↑↓", "move"),
    ("PgUp/PgDn w/s", "move faster"),
    ("g / G", "top / bottom"),
    ("← →", "topic before / after"),
    ("1–9", "go to that topic in the bar"),
    ("B / r", "bookmarks / recent pages"),
    ("Tab/ShiftTab", "next / previous letter"),
    ("Enter", "open the page"),
    ("/", "search (Esc keeps the text)"),
    ("b", "bookmark, or take it out"),
    ("d / x", "docs: download / delete a set"),
    ("+", "about and settings"),
    ("Esc", "quit (or leave a docs set)"),
    ("", "A page"),
    ("↑↓ PgUp/PgDn w/s", "scroll"),
    ("g / G", "top / bottom"),
    ("Tab/ShiftTab", "next / previous section"),
    ("/", "search other pages, this one kept"),
    ("n / N", "next / previous line with the search's text"),
    ("c", "choose something to copy"),
    ("y", "copy the first example (else the name) at once"),
    ("o", "the first link, big, to Ctrl+click"),
    ("b", "bookmark, or take it out"),
    ("B / r", "bookmarks / recent pages"),
    ("←/Esc", "back to the list"),
];

/// The key bars of the help and of the link box.
pub const KEYS_HELP: &[(&str, &str)] = &[
    ("↑↓ PgUp/PgDn", "Scroll"),
    ("any key", "Close"),
    ("^C", "Quit"),
];
pub const KEYS_LINK: &[(&str, &str)] =
    &[("Ctrl+click", "Open"), ("any key", "Close"), ("^C", "Quit")];
