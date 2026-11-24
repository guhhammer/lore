//! Hardcoded values: man and the reader's language.

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
