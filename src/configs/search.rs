//! Hardcoded values: search.

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
