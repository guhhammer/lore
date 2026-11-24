//! Hardcoded values: texts.

/// `lore --help`.
pub const HELP: &str = "\
lore: every command you can type, and what each one does. Offline.

Usage:
  lore               the list of every command, A to Z
  lore NAME          NAME's page straight away (like `man NAME`)
  lore git commit    a subcommand's page
  lore --print NAME  NAME's page as plain text, for pipes and scripts (-p)
  lore explain LINE  each word of a command line, from the man pages:
                     lore explain \"tar -xzvf a.tgz\"
  lore --completions SHELL   the completion script for bash, zsh or fish
  lore check FILE    check a page file another program ships (see man lore)
  lore --update-pages  download newer pages, for commands lore has no page
                     for (lore's weekly pages, else tldr-pages); only then
                     does lore fetch pages
  lore --help        this text
  lore --version     lore's version

Colours: press + then Enter on Colours (or p there) to pick a palette; it is
saved in ~/.config/lore/config ($XDG_CONFIG_HOME/lore/config) for the next
runs. LORE_PALETTE=NAME lore uses another one for one run. Your own
palettes go in ~/.config/lore/palettes/NAME.palette (`man lore`, COLOURS).

On a page, c copies one of its examples (a language's or a doc's code:
a piece of it) to the clipboard, only the text, through the
terminal, so over ssh too (OSC 52: most terminals, tmux with set-clipboard).

+ shows what lore is, where it keeps things and what it found here.

Topics: the bar under the search bar holds commands, recipes, beacon
(what people use most and what is rising, from Debian's and Homebrew's
public numbers) and your own: bookmarks (b), the pages you opened last and
the commands your shell history ran most (a dim dot marks the ones it never
ran). Kept in ~/.local/share/lore/; `history = off` in the config file
stops reading the history.

Docs: d downloads the selected documentation set to read offline, x
deletes it; Enter on a downloaded one lists its pages, Esc goes back.

In the list: up and down move (PgUp/PgDn by 20, w/s by the Fast scroll
lines set on +), left and right switch topics, Enter opens a page, Tab and
Shift+Tab jump between headings, / searches (start with @
for only the commands that have a lore page), Esc quits.
On a page: arrows and PgUp/PgDn scroll, Tab jumps between sections, / finds
another command, left arrow or Esc close it.
`man lore` has the rest.";

/// Under FILES: where the paths come from.
pub const FILES_HINT: &str = "From its manual: the configuration, logs and data it uses.";

/// The most paths FILES lists; a manual can name dozens.
pub const FILES_MAX: usize = 20;

/// How a path starts in a manual's FILES section: absolute, or under the
/// home or an XDG folder (`~/.ssh/config`, `$HOME/.bashrc`).
pub const FILES_PATH_STARTS: &[&str] = &[
    "/",
    "~/",
    "$HOME/",
    "${HOME}/",
    "$XDG_CONFIG_HOME/",
    "${XDG_CONFIG_HOME}/",
    "$XDG_DATA_HOME/",
    "${XDG_DATA_HOME}/",
    "$XDG_STATE_HOME/",
    "${XDG_STATE_HOME}/",
];

/// How to open a link; the terminal does it, not lore.
pub const LINK_HINT: &str = "Ctrl+click a link to open it in your browser.";

/// Always under the links: the content can come from anyone.
pub const LINK_WARNING: &str = "Check every link before you open it: anyone can add pages to \
lore. lore will never ask you to log in anywhere, or for any account.";

/// The category that marks a whole command.
pub const DANGER: &str = "danger";

/// Under the summary of a command whose page is in the `danger` category.
pub const DANGER_WARNING: &str = "! Can destroy data or stop the system, with no undo. Read each \
example, and check what it acts on, before you run it.";

/// Said when a command has no manual page; `NO_PAGE_REASONS` follow it.
pub const NO_PAGE: &str = "No manual page for this command. Probable reasons:";

/// Why there may be no manual page. One general list for now, since `man`
/// does not say which one it is.
pub const NO_PAGE_REASONS: &[&str] = &[
    "it is part of the shell (a builtin or a keyword): try `help <name>` in bash, or the shell's own manual (`man bash`, `man zshbuiltins`)",
    "it is one of your aliases: its page is the one of the command it runs",
    "the program ships no manual page, or its documentation package is not installed (on Debian and Ubuntu some come as a separate `-doc` package)",
    "manual pages were removed to save space (minimal and container images; on Ubuntu, `unminimize` brings them back)",
    "`man` itself is not installed",
];

/// The key bar, per mode: (key, what it does).
pub const KEYS_BROWSE: &[(&str, &str)] = &[
    ("↑↓←→", "Move"),
    ("Enter", "Page"),
    ("/", "Search"),
    ("Tab/ShiftTab", "Jump"),
    ("PgUp/PgDn w/s", "Fast"),
    ("b", "Bookmark"),
    ("+", "About"),
    ("Esc/^C", "Quit"),
];
pub const KEYS_SEARCH: &[(&str, &str)] = &[
    ("type", "Filter"),
    ("Bksp", "Erase"),
    ("↑↓", "Move"),
    ("→/Enter", "Page"),
    ("@", "Lore pages"),
    ("Esc", "Leave search"),
    ("^C", "Quit"),
];
pub const KEYS_PAGE: &[(&str, &str)] = &[
    ("↑↓ PgUp/PgDn w/s", "Scroll"),
    ("Tab/ShiftTab", "Jump"),
    ("/", "Search"),
    ("c", "Copy"),
    ("b", "Bookmark"),
    ("←/Esc", "Back"),
    ("^C", "Quit"),
];
/// The list of a page's examples to copy.
pub const KEYS_COPY: &[(&str, &str)] = &[
    ("↑↓", "Choose"),
    ("Enter", "Copy"),
    ("Esc", "Cancel"),
    ("^C", "Quit"),
];
/// The palette picker.
pub const KEYS_PALETTE: &[(&str, &str)] = &[
    ("↑↓", "Try"),
    ("Enter", "Keep"),
    ("Esc", "Cancel"),
    ("^C", "Quit"),
];

/// The search bar: its title, the hint when empty, the hint after leaving it.
pub const SEARCH_TITLE: &str = " search ";
pub const SEARCH_HINT: &str = "press / to search";
pub const SEARCH_KEPT_HINT: &str = "   (/ to edit)";

/// The list's title: `{total}` commands, or `{shown}` of them while searching.
pub const LIST_TITLE: &str = " {topic} ({total}) ";
/// A search looks in every topic.
pub const LIST_TITLE_SEARCHING: &str = " every topic: {shown} of {total} ";

/// When nothing matches the search `{query}`.
pub const NO_MATCH: &str = " no command matches \"{query}\"";

/// Errors printed in the shell (after `lore: `).
pub const ERROR_NO_TERMINAL: &str =
    "needs a terminal; run it directly, not through a pipe (over ssh: ssh -t)";
pub const ERROR_NOTHING_NAMED: &str = "no command or page named {name}";
pub const ERROR_UNKNOWN_OPTION: &str = "unknown option {option}; see lore --help";
pub const ERROR_OPTIONS_ALONE: &str = "options go alone: lore NAME, or see lore --help";
pub const ERROR_PRINT_NAME: &str = "--print needs a name: lore --print NAME";

pub const ERROR_EXPLAIN_LINE: &str =
    "explain needs a command line: lore explain \"tar -xzvf a.tgz\"";
