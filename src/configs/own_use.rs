//! Hardcoded values: your own use.

use ratatui::style::Color;

/// The key that adds or removes a bookmark, in the list and on a page.
pub const BOOKMARK_KEY: char = 'b';

/// How many recently opened pages are kept, and how many commands the most
/// used topic lists.
pub const RECENT_MAX: usize = 10;
pub const MOST_USED_MAX: usize = 50;

/// The topics, in the bar under the search bar (← → switch them), in the
/// order shown (`Topic::all()` in src/app/types.rs). A topic with nothing in it
/// (no bookmarks yet, no history) is not shown; commands always is.
pub const TOPIC_NAMES: [&str; 9] = [
    "commands",
    "recipes",
    "languages",
    "docs",
    "tools",
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
