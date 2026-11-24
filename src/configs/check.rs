//! Hardcoded values: the check of pages lore did not write (src/info/check.rs).

/// `lore check FILE…`: checks page files, for the programs that ship one.
pub const CHECK_WORD: &str = "check";
pub const ERROR_CHECK_FILES: &str = "check needs page files: lore check NAME.md";
pub const ERROR_CHECK_FAILED: &str = "{count} file(s) have errors: lore would not show them";
pub const CHECK_OK: &str = "{file}: ok";
pub const CHECK_UNREADABLE: &str = "{file}: cannot be read";

/// The largest page file lore reads: a page is a short text.
pub const PAGE_MAX_BYTES: usize = 64 * 1024;

/// The longest link lore shows whole on a narrow screen.
pub const MAX_LINK_LENGTH: usize = 76;

/// Invisible characters that can hide text or reorder what the terminal
/// shows (bidirectional overrides, "Trojan Source"), as ranges: an error in
/// a page, and taken out of every page shown.
pub const CHECK_INVISIBLE: &[(char, char)] = &[
    ('\u{200B}', '\u{200F}'), // zero-width space and joiners, left/right marks
    ('\u{2028}', '\u{2029}'), // line and paragraph separators
    ('\u{202A}', '\u{202E}'), // bidirectional embeddings and overrides
    ('\u{2060}', '\u{2064}'), // word joiner, invisible operators
    ('\u{2066}', '\u{2069}'), // bidirectional isolates
    ('\u{FEFF}', '\u{FEFF}'), // zero-width no-break space
];

/// What would be code in a page: HTML and script, which no page needs.
pub const CHECK_HTML: &[&str] = &[
    "<script",
    "<iframe",
    "<object",
    "<embed",
    "<style",
    "<?php",
    "javascript:",
    "vbscript:",
    "data:text/html",
    "onerror=",
    "onload=",
];

/// The sections lore shows (and a recipe's `Steps`, a tool's `Tasks`).
pub const CHECK_SECTIONS: &[&str] = &[
    "usage",
    "steps",
    "tasks",
    "notes",
    "links",
    "see also",
    "categories",
];

/// Searches linked on every page, after the page's own links: (label, URL
/// with `{}` where the command's name goes). The places people look for a
/// command's help: Q&A sites, wikis, man page archives, cheat sheets. Short
/// on purpose: a URL wider than the panel is cut, and the terminal would
/// open the cut one. Sites without a search URL of their own go through
/// Google's `site:`.
pub const SEARCH_LINKS: &[(&str, &str)] = &[
    (
        "Stack Overflow",
        "https://stackoverflow.com/search?q={}+command",
    ),
    (
        "Unix & Linux Stack Exchange",
        "https://unix.stackexchange.com/search?q={}",
    ),
    ("Ask Ubuntu", "https://askubuntu.com/search?q={}"),
    ("Super User", "https://superuser.com/search?q={}"),
    ("ArchWiki", "https://wiki.archlinux.org/index.php?search={}"),
    ("explainshell", "https://explainshell.com/explain?cmd={}"),
    ("cheat.sh", "https://cheat.sh/{}"),
    ("Debian man pages", "https://manpages.debian.org/{}"),
    (
        "man7.org (Linux man pages)",
        "https://www.google.com/search?q={}+site:man7.org",
    ),
    (
        "SS64 (bash, macOS, Windows)",
        "https://www.google.com/search?q={}+site:ss64.com",
    ),
    (
        "GeeksforGeeks",
        "https://www.google.com/search?q={}+site:geeksforgeeks.org",
    ),
    ("Google", "https://www.google.com/search?q={}+command"),
];
