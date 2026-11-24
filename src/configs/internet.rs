//! Hardcoded values: the internet (src/utils/internet.rs).

use ratatui::style::Color;

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
