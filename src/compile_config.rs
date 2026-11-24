//! Hardcoded values, fixed at compile time. To change one, edit it here and
//! rebuild (`cli/make`).
//!
//! Every path here is optional: a missing file or directory is skipped, never
//! an error, so one list serves Linux and macOS users of bash, zsh and fish.
//! Path syntax: `~/` is the user's home (skipped if `$HOME` is unset), and a
//! `*` stands for any name within one path component (`/opt/*/bin`).

use ratatui::style::{Color, Modifier};

// ── texts ───────────────────────────────────────────────────────────────────

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

On a page, c copies one of its examples to the clipboard, through the
terminal, so over ssh too (OSC 52: most terminals, tmux with set-clipboard).

+ shows what lore is, where it keeps things and what it found here.

Topics: the bar under the search bar holds commands, recipes, beacon
(what people use most and what is rising, from Debian's and Homebrew's
public numbers) and your own: bookmarks (b), the pages you opened last and
the commands your shell history ran most (a dim dot marks the ones it never
ran). Kept in ~/.local/share/lore/; `history = off` in the config file
stops reading the history.

In the list: up and down move, left and right switch topics, Enter opens a
page, Tab and Shift+Tab jump between headings, / searches (start with @
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
    ("↑↓ PgUp/PgDn", "Scroll"),
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

// ── shell completion (src/interface/complete.rs, completions/) ──────────────

/// `lore --complete WORDS` (the scripts ask it) and `lore --completions
/// SHELL` (prints a script).
pub const COMPLETE_FLAG: &str = "--complete";
pub const COMPLETIONS_FLAG: &str = "--completions";

/// What can start the command line, offered after `lore -` (and `explain`
/// after `lore e`).
pub const COMPLETE_OPTIONS: &[(&str, &str)] = &[
    ("--print", "a page as plain text, for pipes and scripts"),
    ("-p", "a page as plain text, for pipes and scripts"),
    ("--help", "how to use lore"),
    ("-h", "how to use lore"),
    ("--version", "lore's version"),
    ("-V", "lore's version"),
    (
        "--completions",
        "print the completion script for bash, zsh or fish",
    ),
    ("explain", "each word of a command line, from the man pages"),
    (
        "--update-pages",
        "download newer pages (the only time lore asks for them)",
    ),
];

/// Candidates get a description (the page's first sentence) only when
/// there are at most this many, and it is cut to this many columns.
pub const COMPLETE_DESCRIBE_MAX: usize = 300;
pub const COMPLETE_DESCRIBE_WIDTH: usize = 60;

/// The completion scripts, by shell, for `lore --completions SHELL`.
pub const COMPLETION_SCRIPTS: &[(&str, &str)] = &[
    ("bash", include_str!("../completions/lore.bash")),
    ("zsh", include_str!("../completions/_lore")),
    ("fish", include_str!("../completions/lore.fish")),
];
pub const ERROR_COMPLETIONS_SHELL: &str = "--completions needs a shell: bash, zsh or fish";

// ── the internet (src/utils/internet.rs) ────────────────────────────────────

/// `internet = off` in the settings file: no request at all.
pub const SETTING_INTERNET: &str = "internet";

/// Where a command's cheat sheet comes from: cheat.sh, community-written,
/// as plain text (`?T`: no colours).
pub const INTERNET_HOST: &str = "cht.sh";
pub const INTERNET_URL: &str = "https://cht.sh/{name}?T";
/// Who asks: lore, by name. cheat.sh sends plain text only to the clients
/// it knows, curl among them, so the name says "like curl".
pub const INTERNET_USER_AGENT: &str = "lore/{version} (plain text; like curl)";
/// What the site answers for a name it does not know.
pub const INTERNET_NOT_FOUND: &str = "Unknown topic";

/// How long a connection may take; the whole request four times that.
pub const INTERNET_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(3);
/// How often the screen looks for news from the background while waiting
/// for a key.
pub const WAKE_EVERY: std::time::Duration = std::time::Duration::from_millis(250);
/// How often the connection is checked again, for the dot.
pub const INTERNET_CHECK_EVERY: std::time::Duration = std::time::Duration::from_secs(30);
/// The most of an answer lore reads, and shows.
pub const INTERNET_MAX_BYTES: usize = 256 * 1024;
pub const INTERNET_MAX_LINES: usize = 150;

/// Answers kept for a day in `$XDG_CACHE_HOME/lore/internet` (or
/// `~/.cache/lore/internet`), so a page opened again makes no request.
pub const CACHE_HOME_VAR: &str = "XDG_CACHE_HOME";
pub const CACHE_HOME_FALLBACK: &str = ".cache";
pub const INTERNET_CACHE_DIR: &str = "lore/internet";
pub const INTERNET_CACHE_HOURS: u64 = 24;

/// The dot, and its word, on every screen and the about screen: always
/// green or red, whatever the palette.
pub const INTERNET_DOT: &str = "●";
pub const INTERNET_WORD: &str = "internet";
pub const INTERNET_ON: Color = Color::Green;
pub const INTERNET_OFF: Color = Color::Red;

/// The INTERNET section of a page, before MANUAL.
pub const HEADING_INTERNET: &str = "INTERNET";
pub const INTERNET_SOURCE: &str = "online: the cheat sheet for {name} from cht.sh";
pub const INTERNET_REMINDER: &str = "Raw text from the internet, written by anyone: it can be changed or altered \
at any time without any check, and lore does not check it either. Read it as text; copy from it \
carefully, and never paste a command you have not understood. lore will not copy it for you.";
pub const INTERNET_LOADING: &str = "Fetching it…";
pub const INTERNET_NOTHING: &str = "cht.sh has nothing for {name}.";
pub const INTERNET_FAILED: &str =
    "It could not be fetched (slow or refused); it is tried again next time lore starts.";
pub const INTERNET_OFFLINE: &str = "offline: no connection, so nothing was fetched";
pub const INTERNET_DISABLED: &str = "off: turned off on the about screen (+), so lore asks nothing";
pub const INTERNET_CHECKING: &str = "checking the connection…";
pub const INTERNET_CUT: &str = "… more at {url}";
/// The margin before each raw line: a bar, so the quote is plain to see.
pub const INTERNET_GUTTER: &str = "  │ ";

// ── lore explain ────────────────────────────────────────────────────────────

/// `lore explain LINE`: the word that starts it.
pub const EXPLAIN_WORD: &str = "explain";

/// What the words of a command line mean when the man page does not say.
pub const EXPLAIN_NO_SUMMARY: &str = "no description found on this machine";
pub const EXPLAIN_NO_MANUAL: &str = "an option (no manual on this machine to say which)";
pub const EXPLAIN_NOT_IN_MANUAL: &str = "an option its manual does not list";
pub const EXPLAIN_VALUE_OF: &str = "the value given to {flag}";
pub const EXPLAIN_FILE_OF: &str = "the file for {op}";
pub const EXPLAIN_ARGUMENT: &str = "an argument: what it works on (a file, a name, a text)";
pub const EXPLAIN_SETTING: &str = "sets {name} for this command only";
pub const EXPLAIN_OPTIONS_END: &str = "the end of the options: what follows is taken as is";

/// The longest a flag's description gets, in characters (then `...`).
pub const EXPLAIN_MAX_CHARS: usize = 160;

/// Columns of the word column at most; a longer word gets its own line.
pub const EXPLAIN_WORD_WIDTH: usize = 18;

/// Programs that run another command after their own options: that
/// command is explained after them.
pub const EXPLAIN_WRAPPERS: &[&str] = &[
    "sudo", "doas", "env", "nice", "nohup", "time", "timeout", "xargs", "exec", "command", "watch",
    "strace", "ionice", "chroot", "flock", "stdbuf",
];

/// Programs whose first argument can be their flags without a dash
/// (`tar xzvf a.tgz`, `ps aux`).
pub const EXPLAIN_BUNDLED: &[&str] = &["tar", "ps", "ar"];

/// Between commands: what each joins.
pub const EXPLAIN_OPERATORS: &[(&str, &str)] = &[
    (
        "|",
        "sends the output of the command before into the next one",
    ),
    ("||", "runs the next command only if the one before failed"),
    ("&&", "runs the next command only if the one before worked"),
    (";", "then runs the next command"),
    ("&", "runs the command before in the background"),
];

/// Redirections: (as written, what it does, whether a file follows).
pub const EXPLAIN_REDIRECTS: &[(&str, &str, bool)] = &[
    (">", "writes the output to a file, replacing it", true),
    ("1>", "writes the output to a file, replacing it", true),
    (">>", "adds the output to the end of a file", true),
    ("<", "reads the input from a file", true),
    ("2>", "writes the error messages to a file", true),
    ("2>>", "adds the error messages to the end of a file", true),
    ("&>", "writes the output and the errors to a file", true),
    (
        "2>&1",
        "sends the error messages where the output goes",
        false,
    ),
    (">&2", "sends the output where the error messages go", false),
    (
        "1>&2",
        "sends the output where the error messages go",
        false,
    ),
];

/// Columns a man page is laid out at for reading its options: wide, so an
/// option's description is not wrapped onto many lines.
pub const EXPLAIN_MAN_WIDTH: u16 = 200;

/// `lore --print NAME` into a pipe or a file, where there is no terminal to
/// take the width from: columns to lay the page out at.
pub const PRINT_WIDTH: u16 = 80;

// ── page texts ──────────────────────────────────────────────────────────────

/// The page's section headings, in the order they appear.
pub const HEADING_VERSION: &str = "VERSION";
pub const HEADING_USAGE: &str = "HOW TO USE";
pub const HEADING_RELATED: &str = "RELATED";
pub const HEADING_SUBCOMMANDS: &str = "SUBCOMMANDS";
pub const HEADING_NOTES: &str = "NOTES";
pub const HEADING_FILES: &str = "FILES";
pub const HEADING_STEPS: &str = "STEPS";
pub const HEADING_USES: &str = "USES";

/// Words a recipe's step can start with that are not the command it uses:
/// wrappers and the shell's own words.
pub const RECIPE_SKIPPED_WORDS: &[&str] = &[
    "sudo", "doas", "env", "time", "nohup", "exec", "while", "until", "for", "do", "done", "if",
    "then", "else", "elif", "fi", "case", "esac", "in", "function", "select", "!", "[", "[[", "((",
    "local", "declare", "export", "readonly",
];

/// On a recipe's page: what it is, and the hints under its headings.
pub const IS_RECIPE: &str = "A recipe: a task done in steps, with the commands for each.";
pub const STEPS_HINT: &str = "In order; c copies a step's command. Read each before you run it.";
pub const USES_HINT: &str = "The commands it uses, each with its own page (/ to find one).";
pub const HEADING_LINKS: &str = "LINKS";
pub const HEADING_MANUAL: &str = "MANUAL";

/// What a name is, under its title, when it is not a program.
pub const IS_ALIAS: &str = "Your alias: typing it runs `{runs}`.";
pub const IS_BUILTIN: &str = "Built into your shell.";
pub const IS_KEYWORD: &str = "A word of your shell's language.";
pub const IS_FUNCTION: &str = "A shell function you defined.";
pub const IS_SUBCOMMAND: &str = "A subcommand of {program}.";

/// When a program has no page and `whatis` knows nothing of it.
pub const NO_SUMMARY: &str = "No summary yet for {name}.";
/// When a page has no examples.
pub const NO_EXAMPLES: &str =
    "No examples yet for {name}. Anyone can add them: see content/README.md in lore's repository.";
/// Under the examples, when one of them is marked dangerous.
pub const DANGER_EXAMPLE_NOTE: &str =
    "! marks an example that can destroy data or stop the system: check what it acts on first.";
/// Above the related commands and the subcommands.
pub const RELATED_HINT: &str = "Open one with / or `lore NAME`.";
pub const SUBCOMMANDS_HINT: &str =
    "Each has its own page: find it in the list, or run `lore {first}`.";

/// VERSION when there is no package to read it from.
pub const VERSION_ALIAS: &str =
    "An alias has no version of its own: it is the one of the command it runs.";
pub const VERSION_FUNCTION: &str = "A function you wrote has no version.";
pub const VERSION_SHELL: &str = "Part of your shell, so it has the shell's version (`bash --version`, `zsh --version`, `fish --version`).";
pub const VERSION_UNPACKAGED: &str = "Not from a package manager lore knows: installed by hand, by a language's tool (cargo, pip, npm…) or by a script. Its version: try `--version`.";
pub const VERSION_NOT_FOUND: &str =
    "Not installed on this machine, or not in a folder lore looks in.";

/// VERSION's fields, and the newest version's two cases.
pub const FIELD_PACKAGE: &str = "Package";
pub const FIELD_INSTALLED: &str = "Installed";
pub const FIELD_NEWEST: &str = "Newest";
pub const FIELD_UPDATE: &str = "Update";
pub const FIELD_LOCATION: &str = "Location";
pub const NEWEST_SAME: &str = "{newest}, up to date as of your last package list update";
pub const NEWEST_UPDATE: &str = "{newest}: an update is available";

/// How wide a field's label is, so the values line up.
pub const FIELD_LABEL_WIDTH: usize = 11;

/// A page's indents: of a section's text, of what an example does, of the
/// reasons there is no manual.
pub const PAGE_INDENT: &str = "  ";
pub const EXAMPLE_INDENT: &str = "      ";
pub const REASON_INDENT: &str = "    ";

/// Before an example, and before one that can destroy data.
pub const EXAMPLE_PROMPT: &str = "$ ";
pub const DANGER_MARK: &str = "! ";

// ── screen ──────────────────────────────────────────────────────────────────

/// The list's share of the width, in percent, while a page is open; the page
/// gets the rest.
pub const INFO_LIST_PERCENT: u16 = 10;

/// Lines that PgUp and PgDn scroll the page by.
pub const PAGE_STEP: isize = 20;

/// The list's indents: of a command, of a letter header; beside a page,
/// where a tenth of the width is all there is, less.
pub const LIST_INDENT: &str = "   ";
pub const LIST_HEADER_INDENT: &str = " ";
pub const LIST_INDENT_NARROW: &str = " ";

/// The header of names that start with a digit or a symbol.
pub const SYMBOL_SECTION: &str = "#";

/// Lines the bordered search bar takes; the key bar takes one.
pub const SEARCH_BAR_HEIGHT: u16 = 3;
pub const KEY_BAR_HEIGHT: u16 = 1;

/// The ending that marks text cut short.
pub const ELLIPSIS: &str = "...";

/// Bold: headings, letter headers, `danger`.
pub const MODIFIER_HEADING: Modifier = Modifier::BOLD;
pub const MODIFIER_SECTION: Modifier = Modifier::BOLD;
pub const MODIFIER_DANGER: Modifier = Modifier::BOLD;

// ── the name, drawn ─────────────────────────────────────────────────────────

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

/// The about screen's settings, changed there (↑↓ choose, Enter changes)
/// and saved in the settings file at once: their heading, and each one's
/// name, in the order shown (`Setting::all()` in src/app.rs).
pub const ABOUT_SETTINGS_HEADING: &str = "SETTINGS";
pub const ABOUT_SETTINGS_HINT: &str = "↑↓ choose, Enter changes; saved for the next runs";
pub const ABOUT_SETTING_NAMES: [&str; 3] = ["Colours", "Internet", "History"];
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
pub const ABOUT_LABEL_WIDTH: usize = 10;

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
pub const COPY_TITLE: &str = " copy an example ";
pub const COPIED: &str = "copied: {text}";
pub const NOTHING_TO_COPY: &str = "this page has no examples to copy";

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

// ── settings file (src/settings.rs) ─────────────────────────────────────────

/// `$XDG_CONFIG_HOME/lore/config`; without it, `~/.config/lore/config`.
pub const CONFIG_HOME_VAR: &str = "XDG_CONFIG_HOME";
pub const CONFIG_HOME_FALLBACK: &str = ".config";
pub const CONFIG_FILE: &str = "lore/config";

/// The first lines of a settings file lore makes.
pub const CONFIG_HEADER: &str = "\
# lore's settings. lore writes this file when a setting is changed in the
# app (the + screen, or p for colours); it can be edited by hand too. One `key = value` a line.
";

/// The palette's name in the settings file (`palette = nord`).
pub const SETTING_PALETTE: &str = "palette";

/// `history = off` in the settings file: lore does not read the shell's
/// history (no most-used group, no never-used marks).
pub const SETTING_HISTORY: &str = "history";
pub const SETTING_OFF: &str = "off";
pub const SETTING_ON: &str = "on";

/// Lists lore keeps between runs, one name a line, in
/// `$XDG_DATA_HOME/lore/`, or `~/.local/share/lore/` when that is unset.
pub const DATA_HOME_VAR: &str = "XDG_DATA_HOME";
pub const DATA_HOME_FALLBACK: &str = ".local/share";
pub const DATA_DIR: &str = "lore";
pub const BOOKMARKS_FILE: &str = "bookmarks";
pub const RECENT_FILE: &str = "recent";

// ── your own use ────────────────────────────────────────────────────────────

/// The key that adds or removes a bookmark, in the list and on a page.
pub const BOOKMARK_KEY: char = 'b';

/// How many recently opened pages are kept, and how many commands the most
/// used topic lists.
pub const RECENT_MAX: usize = 10;
pub const MOST_USED_MAX: usize = 50;

/// The topics, in the bar under the search bar (← → switch them), in the
/// order shown (`Topic::all()` in src/app.rs). A topic with nothing in it
/// (no bookmarks yet, no history) is not shown; commands always is.
pub const TOPIC_NAMES: [&str; 6] = [
    "commands",
    "recipes",
    "beacon",
    "most used",
    "recent",
    "bookmarks",
];
/// The bar's height, and the room between two topics.
pub const TOPIC_BAR_HEIGHT: u16 = 1;
pub const TOPIC_GAP: &str = "  ";
/// After the topics, what switches them.
pub const TOPIC_HINT: &str = "  ← → topics";

/// Before the title of a bookmarked command's page.
pub const BOOKMARK_MARK: &str = "★ ";

/// After the name of a command your shell history never ran (only when
/// there is a history to go by).
pub const UNUSED_MARK: &str = " ·";

/// The shell history files read for the most-used group, besides
/// `$HISTFILE`: bash, zsh (two usual names), fish.
pub const HISTORY_FILES: &[&str] = &[
    "~/.bash_history",
    "~/.zsh_history",
    "~/.histfile",
    "~/.local/share/fish/fish_history",
];

/// Words in front of the command that ran: `sudo apt install` ran apt.
pub const HISTORY_WRAPPERS: &[&str] = &[
    "sudo", "doas", "time", "nohup", "exec", "command", "builtin", "env", "nice", "then", "do",
    "else", "!", "(", "{",
];

/// The colour palettes are files: `palettes/<name>.palette` in the
/// repository, built in by build.rs, and yours in this folder of the
/// settings folder (`~/.config/lore/palettes/`). Format: palettes/README.md.
pub const PALETTE_DIR: &str = "palettes";
pub const PALETTE_FILE_END: &str = ".palette";

/// The colour names a palette file can use besides `#rrggbb`: the
/// terminal's 16 ANSI colours, which follow its own theme, and `reset`, the
/// terminal's default (build.rs checks the files against these names too).
pub const COLOUR_NAMES: &[(&str, Color)] = &[
    ("reset", Color::Reset),
    ("black", Color::Black),
    ("red", Color::Red),
    ("green", Color::Green),
    ("yellow", Color::Yellow),
    ("blue", Color::Blue),
    ("magenta", Color::Magenta),
    ("cyan", Color::Cyan),
    ("gray", Color::Gray),
    ("darkgray", Color::DarkGray),
    ("lightred", Color::LightRed),
    ("lightgreen", Color::LightGreen),
    ("lightyellow", Color::LightYellow),
    ("lightblue", Color::LightBlue),
    ("lightmagenta", Color::LightMagenta),
    ("lightcyan", Color::LightCyan),
    ("white", Color::White),
];

// ── content pages ───────────────────────────────────────────────────────────

/// Optional folders of extra pages (`<command>.md`, translations in
/// `i18n/<lang>/<command>.md`). None has to exist: lore carries its own
/// pages. Every page found in them must pass `info::check`, or it is not
/// shown.
///
/// Yours come first and may replace a page built into lore.
pub const USER_CONTENT_DIRS: &[&str] = &["~/.local/share/lore/content"];
/// Where other programs install a page for their command (a package's
/// `/usr/share/lore/content/NAME.md`): used only for a command lore has no
/// page of its own for, so no package can rewrite another command's page.
pub const SYSTEM_CONTENT_DIRS: &[&str] =
    &["/usr/local/share/lore/content", "/usr/share/lore/content"];

/// Pages `lore --update-pages` downloaded: used only for a command with no
/// page of yours or lore's own, like `SYSTEM_CONTENT_DIRS`. The first one
/// is where they are written; lore replaces the whole folder each time.
pub const UPDATED_CONTENT_DIRS: &[&str] = &["~/.local/share/lore/updates"];

// ── newer pages: `lore --update-pages` (src/info/updates.rs) ────────────────

pub const UPDATE_PAGES_FLAG: &str = "--update-pages";
/// First: the pages lore's repository converts each week, one zstd file of
/// records (`RECORD_MARK` + path + newline + the page).
pub const UPDATE_BUNDLE_URL: &str =
    "https://github.com/guhhammer/lore/releases/download/content/pages.zst";
/// A record's start: a control character, which no page that passes the
/// check can hold, so a page cannot fake one.
pub const RECORD_MARK: char = '\u{1e}';
/// Else: tldr-pages' own release, one zip per language, converted here.
pub const UPDATE_TLDR_URL: &str =
    "https://github.com/tldr-pages/tldr/releases/latest/download/tldr-pages.{lang}.zip";
/// tldr's folders this system uses, the first winning a name: the
/// system's own, then the common ones.
pub const TLDR_PLATFORMS_LINUX: &[&str] = &["linux", "common"];
pub const TLDR_PLATFORMS_MACOS: &[&str] = &["osx", "common"];
/// The link every converted page gets, besides tldr's "More information".
pub const TLDR_PAGE_LINK: &str = "https://tldr.inbrowser.app/pages/{platform}/{name}";
pub const TLDR_MORE_INFORMATION: &str = "More information";
/// The most examples a converted page keeps (the built-in pages keep 8 too).
pub const TLDR_MAX_EXAMPLES: usize = 8;
/// The most a download may weigh, and the most a bundle may unpack to.
pub const UPDATE_MAX_BYTES: usize = 64 * 1024 * 1024;
/// What `lore --update-pages` says.
pub const UPDATE_TRYING: &str = "lore: fetching {url}";
pub const UPDATE_FALLBACK: &str = "lore: not there ({why}); converting tldr-pages instead";
pub const UPDATE_DONE: &str =
    "lore: {pages} new pages and {translations} translations in {dir}, from {source}";
pub const UPDATE_SOURCE_BUNDLE: &str = "lore's weekly pages";
pub const UPDATE_SOURCE_TLDR: &str = "tldr-pages (CC BY 4.0)";
pub const ERROR_UPDATE_FAILED: &str = "could not get newer pages: {why}";
pub const ERROR_UPDATE_NO_HOME: &str = "no home folder to keep the pages in";
pub const ERROR_UPDATE_EMPTY: &str = "nothing usable came back";

// ── the beacon topic (src/info/beacon.rs) ───────────────────────────────────

/// The numbers built into lore: beacon/beacon.tsv, from
/// scripts/beacon/snapshot.py (popcon and Homebrew).
pub const BEACON_SNAPSHOT: &str = include_str!("../beacon/beacon.tsv");
/// A newer copy, fetched in the background when Internet is on, and kept
/// in `$XDG_CACHE_HOME/lore/beacon.tsv` for `BEACON_CACHE_HOURS`.
pub const BEACON_URL: &str =
    "https://github.com/guhhammer/lore/releases/download/content/beacon.tsv";
pub const BEACON_CACHE_FILE: &str = "lore/beacon.tsv";
pub const BEACON_CACHE_HOURS: u64 = 24 * 7;
pub const BEACON_MAX_BYTES: usize = 4 * 1024 * 1024;
/// Without lore's copy, the sources themselves: popcon's ranking (read only
/// this far: the most installed come first) and Homebrew's installs asked
/// for by name. A package or formula then counts for the command of the
/// same name only.
pub const BEACON_POPCON_URL: &str = "https://popcon.debian.org/by_inst";
pub const BEACON_POPCON_MAX_BYTES: usize = 3 * 1024 * 1024;
pub const BEACON_BREW_URL: &str =
    "https://formulae.brew.sh/api/analytics/install-on-request/{days}d.json";
/// A package on this share of the most installed one's machines is the base
/// system, not a choice: left out (scripts/beacon/snapshot.py does the same).
pub const BEACON_BASE_SHARE: f64 = 0.9;
/// How many commands a table made here keeps, the most used first (as the
/// script's).
pub const BEACON_KEEP: usize = 1500;
/// Trending: installs in the last 30 days against the year's monthly mean,
/// at least this much higher, from at least `BEACON_TREND_MIN` installs.
pub const BEACON_TREND_MIN_RISE: f64 = 1.3;
pub const BEACON_TREND_MIN: u64 = 2000;
/// How many the beacon lists: trending, popular, and per category.
pub const BEACON_TRENDING_MAX: usize = 20;
pub const BEACON_POPULAR_MAX: usize = 50;
pub const BEACON_PER_CATEGORY: usize = 5;
pub const BEACON_CATEGORIES_MAX: usize = 12;
/// How many of the most popular the categories are made from.
pub const BEACON_CATEGORY_POOL: usize = 400;
/// The beacon's headings, and what follows each name.
pub const BEACON_TRENDING: &str = "trending this month";
pub const BEACON_POPULAR: &str = "most used";
pub const BEACON_FOR: &str = "most used for {category}";
pub const BEACON_RISE: &str = "↑ {rise}× this month";
pub const BEACON_RANK: &str = "#{rank}";
pub const BEACON_NOT_HERE: &str = "not installed";

// ── the check of pages lore did not write (src/info/check.rs) ───────────────

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

/// The sections lore shows (and a recipe's `Steps`).
pub const CHECK_SECTIONS: &[&str] = &["usage", "steps", "notes", "links", "see also", "categories"];

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

// ── search ──────────────────────────────────────────────────────────────────

/// Starts a search that keeps only the commands with a lore page.
pub const PAGES_ONLY: char = '@';

/// A search needs this many characters (spaces not counted) before pages
/// are searched too: fewer would match nearly every page.
pub const CONTENT_MIN_CHARS: usize = 3;

/// What starts a word inside a name: "commit" in `git commit`, "get" in
/// `apt-get`.
pub const NAME_WORD_SEPARATORS: [char; 5] = [' ', '-', '_', '.', ':'];

/// Points for where on a page a search word is: its categories (whole, or
/// part of one), summary, examples' descriptions, examples; the word as
/// typed counts `SCORE_TYPED` times one counting as it; the whole search
/// found as typed adds `SCORE_PHRASE`.
pub const SCORE_CATEGORY: u32 = 6;
pub const SCORE_CATEGORY_PART: u32 = 4;
pub const SCORE_SUMMARY: u32 = 3;
pub const SCORE_DESCRIPTION: u32 = 2;
pub const SCORE_EXAMPLE: u32 = 1;
pub const SCORE_TYPED: u32 = 2;
pub const SCORE_PHRASE: u32 = 5;

/// A word this long or longer ending in `s` also finds its singular.
pub const PLURAL_MIN_CHARS: usize = 4;

/// Characters of context kept each side of a search result's matched line.
pub const SNIPPET_CONTEXT: usize = 12;

/// Words that mean the same in a search by content: typing one finds pages
/// that use any of its group. The pages are written in tldr's and the man
/// pages' words ("directory", "remove"); a learner types their own
/// ("folder", "delete").
#[rustfmt::skip]
pub const SYNONYMS: &[&[&str]] = &[
    &["folder", "directory", "dir"],
    &["delete", "remove", "erase", "rm"],
    &["copy", "duplicate", "cp"],
    &["move", "rename", "mv"],
    &["find", "search", "locate", "look for"],
    &["show", "display", "print", "list", "view"],
    &["compress", "zip", "archive", "pack"],
    &["extract", "unzip", "unpack", "decompress", "uncompress"],
    &["download", "fetch", "get"],
    &["upload", "send", "transfer"],
    &["stop", "kill", "terminate", "end"],
    &["start", "launch", "run", "execute"],
    &["disk", "drive", "storage"],
    &["size", "space", "usage"],
    &["edit", "modify", "change"],
    &["install", "add"],
    &["uninstall", "remove", "purge"],
    &["password", "passphrase", "credentials"],
    &["internet", "network", "online"],
    &["picture", "image", "photo"],
    &["permission", "permissions", "access", "mode"],
    &["user", "account"],
    &["process", "program", "task"],
];

// ── man and the reader's language ───────────────────────────────────────────

/// `man` gives up below this width.
pub const MAN_MIN_WIDTH: u16 = 20;

/// The width `lore NAME` asks `man` at to learn whether a page exists.
pub const MAN_PROBE_WIDTH: u16 = 80;

/// Locales whose reader gets the pages as written (English).
pub const NO_TRANSLATION_LOCALES: [&str; 3] = ["en", "C", "POSIX"];

/// Where the reader's language is read from, the first one set winning.
pub const LOCALE_VARS: [&str; 3] = ["LC_ALL", "LC_MESSAGES", "LANG"];

/// Characters Windows forbids in file names: a page's file name has them
/// as `%XX`, so the repository checks out and packages everywhere.
pub const FORBIDDEN_FILE_CHARS: [char; 8] = ['<', '>', ':', '"', '\\', '|', '?', '*'];

// ── packages ────────────────────────────────────────────────────────────────

/// How each package manager updates a package (`{name}`), shown on its page.
pub const UPDATE_DPKG: &str = "sudo apt update && sudo apt install --only-upgrade {name}";
pub const UPDATE_RPM: &str = "sudo dnf upgrade {name}";
/// Arch upgrades as a whole, never one package.
pub const UPDATE_PACMAN: &str = "sudo pacman -Syu";
pub const UPDATE_HOMEBREW: &str = "brew update && brew upgrade {name}";

// ── danger ──────────────────────────────────────────────────────────────────

/// Examples marked dangerous on a page: (program, what in the rest of the
/// line makes it so; "" = always). A program ending in `*` is a prefix
/// (`mkfs*` covers mkfs.ext4). A leading `sudo` is skipped. Whole commands
/// are marked by their page's `danger` category instead.
#[rustfmt::skip]
pub const DANGEROUS_EXAMPLES: &[(&str, &str)] = &[
    // delete or overwrite, with no undo
    ("rm", ""), ("shred", ""), ("wipefs", ""), ("srm", ""), ("truncate", ""),
    ("dd", "of="), ("mkfs*", ""), ("mke2fs", ""), ("mkswap", ""),
    ("fdisk", ""), ("sfdisk", ""), ("sgdisk", ""), ("gdisk", ""), ("parted", ""),
    ("find", "-delete"), ("find", "-exec rm"), ("rsync", "--delete"),
    ("git", "reset --hard"), ("git", "clean -f"), ("git", "push --force"), ("git", "push -f"),
    ("git", "branch -D"), ("git", "checkout -- "), ("git", "restore "),
    ("chmod", "-R"), ("chown", "-R"), ("crontab", "-r"), ("userdel", ""), ("deluser", ""),
    ("docker", "prune"), ("docker", " rm"), ("kubectl", "delete"), ("terraform", "destroy"),
    ("iptables", "-F"), ("mv", "-f"), ("cp", "-f"),
    // stop the machine or processes
    ("reboot", ""), ("shutdown", ""), ("poweroff", ""), ("halt", ""),
    ("kill", "-9"), ("killall", ""), ("pkill", ""),
    // Windows
    ("format", ""), ("diskpart", ""), ("del", ""), ("rd", "/s"), ("rmdir", "/s"),
];

/// Words before a command that run it as another user: skipped when
/// judging an example (`sudo rm …` is judged as `rm …`).
pub const RUN_AS: [&str; 2] = ["sudo", "doas"];

/// Anywhere in an example, these make it dangerous whatever the program:
/// writing straight onto a disk device.
pub const DANGEROUS_ANYWHERE: &[&str] = &["of=/dev/sd", "of=/dev/nvme", "> /dev/sd", "> /dev/nvme"];

// ── programs ────────────────────────────────────────────────────────────────

/// Always scanned for programs, besides `$PATH`, which is often cut down
/// (desktop launchers, `sudo`, fresh logins). A normal Debian user's `$PATH`
/// has no sbin, which holds the commands for running the machine (reboot,
/// fdisk, useradd…): exactly what is needed offline.
#[rustfmt::skip]
pub const PROGRAM_DIRS: &[&str] = &[
    // Linux and macOS
    "/usr/local/sbin", "/usr/local/bin", "/usr/sbin", "/usr/bin", "/sbin", "/bin",
    "/usr/games", "/usr/local/games",
    // Snap, Flatpak (one command per installed app)
    "/snap/bin", "/var/lib/flatpak/exports/bin", "~/.local/share/flatpak/exports/bin",
    // Nix and NixOS
    "~/.nix-profile/bin", "/nix/var/nix/profiles/default/bin", "/run/current-system/sw/bin",
    // Homebrew: Linux, macOS on Apple silicon (Intel Macs use /usr/local/bin)
    "/home/linuxbrew/.linuxbrew/bin", "/home/linuxbrew/.linuxbrew/sbin",
    "/opt/homebrew/bin", "/opt/homebrew/sbin",
    // MacPorts, and macOS's own extra system directories
    "/opt/local/bin", "/opt/local/sbin",
    "/System/Cryptexes/App/usr/bin", "/Library/Apple/usr/bin",
    // the user's own and language tools
    "~/bin", "~/.local/bin", "~/.cargo/bin", "~/go/bin",
    // software installed under /opt
    "/opt/*/bin",
];

/// Files listing one program directory per line. macOS builds its default
/// `$PATH` from these; on Linux they usually do not exist.
pub const PATH_LIST_FILES: &[&str] = &["/etc/paths", "/etc/paths.d/*"];

// ── bash ────────────────────────────────────────────────────────────────────

/// Where bash aliases are defined: system-wide first, then the user's, so the
/// user's win (a later definition wins). `/etc/bashrc` is the name on macOS
/// and Fedora, `/etc/bash.bashrc` on Debian.
#[rustfmt::skip]
pub const BASH_ALIAS_FILES: &[&str] = &[
    "/etc/profile", "/etc/bash.bashrc", "/etc/bashrc", "/etc/profile.d/*.sh",
    "~/.profile", "~/.bash_profile", "~/.bash_login", "~/.bash_aliases", "~/.bashrc",
];

/// Bash 5.2's builtins (`compgen -b`). They are not files, so no directory
/// has them; listing them here avoids starting a bash just to ask.
#[rustfmt::skip]
pub const BASH_BUILTINS: &[&str] = &[
    ".", ":", "[", "alias", "bg", "bind", "break", "builtin", "caller", "cd", "command",
    "compgen", "complete", "compopt", "continue", "declare", "dirs", "disown", "echo", "enable",
    "eval", "exec", "exit", "export", "false", "fc", "fg", "getopts", "hash", "help", "history",
    "jobs", "kill", "let", "local", "logout", "mapfile", "popd", "printf", "pushd", "pwd", "read",
    "readarray", "readonly", "return", "set", "shift", "shopt", "source", "suspend", "test",
    "times", "trap", "true", "type", "typeset", "ulimit", "umask", "unalias", "unset", "wait",
];

/// Bash 5.2's keywords (`compgen -k`).
#[rustfmt::skip]
pub const BASH_KEYWORDS: &[&str] = &[
    "if", "then", "else", "elif", "fi", "case", "esac", "for", "select", "while", "until", "do",
    "done", "in", "function", "time", "{", "}", "!", "[[", "]]", "coproc",
];

// ── zsh (the default shell on macOS) ────────────────────────────────────────

/// Where zsh aliases are defined, system-wide first. `/etc/zsh/` is Debian's
/// layout, `/etc/` macOS's. oh-my-zsh: only the user's custom files; its
/// plugins' aliases depend on the `plugins=(…)` line and are not read.
#[rustfmt::skip]
pub const ZSH_ALIAS_FILES: &[&str] = &[
    "/etc/zshenv", "/etc/zsh/zshenv", "/etc/zprofile", "/etc/zsh/zprofile",
    "/etc/zshrc", "/etc/zsh/zshrc",
    "~/.zshenv", "~/.zprofile", "~/.zshrc", "~/.zsh_aliases", "~/.oh-my-zsh/custom/*.zsh",
];

/// Zsh 5.9's builtins, from the zshbuiltins manual, including those of its
/// standard modules (compctl, zle, zstyle…), which most setups load.
#[rustfmt::skip]
pub const ZSH_BUILTINS: &[&str] = &[
    "-", ".", ":", "[", "alias", "autoload", "bg", "bindkey", "break", "builtin", "bye", "cap",
    "cd", "chdir", "clone", "command", "comparguments", "compcall", "compctl", "compdescribe",
    "compfiles", "compgroups", "compquote", "comptags", "comptry", "compvalues", "continue",
    "declare", "dirs", "disable", "disown", "echo", "echotc", "echoti", "emulate", "enable",
    "eval", "exec", "exit", "export", "false", "fc", "fg", "float", "functions", "getcap",
    "getln", "getopts", "hash", "history", "integer", "jobs", "kill", "let", "limit", "local",
    "logout", "noglob", "popd", "print", "printf", "pushd", "pushln", "pwd", "r", "read",
    "readonly", "rehash", "return", "sched", "set", "setcap", "setopt", "shift", "source",
    "stat", "suspend", "test", "times", "trap", "true", "ttyctl", "type", "typeset", "ulimit",
    "umask", "unalias", "unfunction", "unhash", "unlimit", "unset", "unsetopt", "vared", "wait",
    "whence", "where", "which", "zcompile", "zformat", "zftp", "zle", "zmodload", "zparseopts",
    "zprof", "zpty", "zregexparse", "zsocket", "zstyle", "ztcp",
];

/// Zsh 5.9's reserved words (the zshmisc manual, "Reserved Words").
#[rustfmt::skip]
pub const ZSH_KEYWORDS: &[&str] = &[
    "do", "done", "esac", "then", "elif", "else", "fi", "for", "case", "if", "while", "function",
    "repeat", "time", "until", "select", "coproc", "nocorrect", "foreach", "end", "!", "[[", "]]",
    "{", "}", "declare", "export", "float", "integer", "local", "readonly", "typeset",
];

// ── fish ────────────────────────────────────────────────────────────────────

/// Where fish aliases (`alias name 'value'`) are defined, system-wide first.
#[rustfmt::skip]
pub const FISH_ALIAS_FILES: &[&str] = &[
    "/etc/fish/config.fish", "/etc/fish/conf.d/*.fish",
    "~/.config/fish/config.fish", "~/.config/fish/conf.d/*.fish",
];

/// Fish functions: one per file, named like the file (`mkcd.fish` → `mkcd`).
pub const FISH_FUNCTION_FILES: &[&str] = &["~/.config/fish/functions/*.fish"];

/// Fish's builtins. The fish manual does not mark which commands are
/// builtins, so this is fish 3's `builtin -n` from memory, minus the keywords
/// below: worth checking against `builtin -n` on a machine with fish.
#[rustfmt::skip]
pub const FISH_BUILTINS: &[&str] = &[
    ".", ":", "[", "_", "abbr", "argparse", "bg", "bind", "block", "breakpoint", "cd",
    "commandline", "complete", "contains", "count", "disown", "echo", "emit", "eval", "exit",
    "false", "fg", "functions", "history", "jobs", "math", "path", "printf", "pwd", "random",
    "read", "realpath", "set", "set_color", "source", "status", "string", "test", "true", "type",
    "ulimit", "wait",
];

/// Fish's keywords (the fish manual, "Keywords").
#[rustfmt::skip]
pub const FISH_KEYWORDS: &[&str] = &[
    "if", "else", "for", "while", "break", "continue", "function", "return", "begin", "end",
    "and", "or", "not", "switch", "case", "command", "builtin", "time", "exec",
];
