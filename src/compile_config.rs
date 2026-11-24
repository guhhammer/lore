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
  lore --help        this text
  lore --version     lore's version

Colours: press p in the list or on a page to pick a palette; the choice is
saved in ~/.config/lore/config ($XDG_CONFIG_HOME/lore/config) for the next
runs. LORE_PALETTE=NAME lore uses another one for one run. Your own
palettes go in ~/.config/lore/palettes/NAME.palette (`man lore`, COLOURS).

On a page, c copies one of its examples to the clipboard, through the
terminal, so over ssh too (OSC 52: most terminals, tmux with set-clipboard).

+ shows what lore is, where it keeps things and what it found here.

Your own use: b bookmarks a command; with the search empty, the list starts
with your bookmarks, the pages you opened last and the commands your shell
history ran most (a dim dot marks the ones it never ran). Kept in
~/.local/share/lore/; `history = off` in the config file stops reading it.

In the list: arrows move, / searches (start with @ for only the commands
that have a lore page), right arrow or Enter opens a page, Esc quits.
On a page: arrows and PgUp/PgDn scroll, / finds another command, left
arrow or Esc close it.
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
    ("↑↓", "Move"),
    ("→/Enter", "Page"),
    ("/", "Search"),
    ("b", "Bookmark"),
    ("p", "Colours"),
    ("+", "About"),
    ("Esc", "Quit"),
    ("^C", "Quit"),
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
    ("/", "Search"),
    ("c", "Copy"),
    ("b", "Bookmark"),
    ("p", "Colours"),
    ("←/Esc", "Back"),
    ("^C", "Quit"),
];
/// A page opened as `lore NAME`: closing it quits.
pub const KEYS_PAGE_DIRECT: &[(&str, &str)] = &[
    ("↑↓ PgUp/PgDn", "Scroll"),
    ("/", "Search"),
    ("c", "Copy"),
    ("b", "Bookmark"),
    ("p", "Colours"),
    ("←/Esc", "Quit"),
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
pub const LIST_TITLE: &str = " commands ({total}) ";
pub const LIST_TITLE_SEARCHING: &str = " commands ({shown} of {total}) ";

/// When nothing matches the search `{query}`.
pub const NO_MATCH: &str = " no command matches \"{query}\"";

/// Errors printed in the shell (after `lore: `).
pub const ERROR_NO_TERMINAL: &str =
    "needs a terminal; run it directly, not through a pipe (over ssh: ssh -t)";
pub const ERROR_NOTHING_NAMED: &str = "no command or page named {name}";
pub const ERROR_UNKNOWN_OPTION: &str = "unknown option {option}; see lore --help";
pub const ERROR_OPTIONS_ALONE: &str = "options go alone: lore NAME, or see lore --help";
pub const ERROR_PRINT_NAME: &str = "--print needs a name: lore --print NAME";

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
pub const ABOUT_PALETTE: &str = "{name} ({count} palettes; p picks one)";
pub const ABOUT_KEPT: &str = "{bookmarks} bookmarks, {recent} recent pages";
pub const ABOUT_HISTORY_ON: &str = "{seen} commands seen in your shell's history (only read, here)";
pub const ABOUT_HISTORY_OFF: &str = "not read (no history, or `history = off`)";
pub const ABOUT_LICENCE: &str = "MIT; examples and translations from tldr-pages, CC BY 4.0";
pub const ABOUT_SOURCE: &str = "https://github.com/guhhammer/lore";
pub const ABOUT_MANUAL: &str = "man lore, lore --help";
pub const ABOUT_FIELDS: [&str; 10] = [
    "Commands", "Pages", "Palette", "Settings", "Palettes", "Kept", "History", "Licence", "Source",
    "Manual",
];
/// Width of the field names' column on the about screen.
pub const ABOUT_LABEL_WIDTH: usize = 10;

/// The about screen's keys.
pub const KEYS_ABOUT: &[(&str, &str)] = &[("+/←/Esc", "Back"), ("^C", "Quit")];

/// The key that opens the list of a page's examples to copy.
pub const COPY_KEY: char = 'c';

/// That list's title, and what the key bar says after a copy (until the
/// next key) or when a page has nothing to copy.
pub const COPY_TITLE: &str = " copy an example ";
pub const COPIED: &str = "copied: {text}";
pub const NOTHING_TO_COPY: &str = "this page has no examples to copy";

/// Columns the copy list leaves free on each side of the page.
pub const COPY_PICKER_MARGIN: u16 = 2;

/// The key that opens the palette picker, in the list and on a page.
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
# app (p: colours); it can be edited by hand too. One `key = value` a line.
";

/// The palette's name in the settings file (`palette = nord`).
pub const SETTING_PALETTE: &str = "palette";

/// `history = off` in the settings file: lore does not read the shell's
/// history (no most-used group, no never-used marks).
pub const SETTING_HISTORY: &str = "history";
pub const SETTING_OFF: &str = "off";

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

/// How many recently opened pages, and most-used commands, head the list.
pub const RECENT_MAX: usize = 10;
pub const MOST_USED_MAX: usize = 10;

/// The headers of the groups above A to Z, when the search is empty.
pub const GROUP_BOOKMARKS: &str = "★ bookmarks";
pub const GROUP_RECENT: &str = "recent";
pub const GROUP_MOST_USED: &str = "most used";

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

/// Optional folders of extra pages (`<command>.md`), checked in this order
/// before the pages built into lore; the first page found wins. None has to
/// exist: lore carries its own pages and needs no folder.
pub const CONTENT_DIRS: &[&str] = &[
    "~/.local/share/lore/content",
    "/usr/local/share/lore/content",
    "/usr/share/lore/content",
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
