//! Hardcoded values: settings file (src/settings.rs).

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
