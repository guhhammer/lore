//! The sections of the list: `#` for names starting with a digit or symbol,
//! then `A` to `Z`.

use std::fmt;

use crate::compile_config::{GROUP_BOOKMARKS, GROUP_MOST_USED, GROUP_RECENT, SYMBOL_SECTION};

/// A section. `Symbol` is declared first, so it sorts before every letter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Section {
    /// The groups above A to Z, with the search empty; a command in one is
    /// also under its letter.
    Bookmarks,
    Recent,
    MostUsed,
    /// Shown as `#`: names like `7z`, `[`, `!`.
    Symbol,
    /// Always upper case: `ls` and `Xorg` go under `L` and `X`.
    Letter(char),
}

impl Section {
    /// The section a name goes under: its first letter upper-cased, or `Symbol`.
    pub fn of(name: &str) -> Self {
        match name.chars().next() {
            Some(c) if c.is_ascii_alphabetic() => Self::Letter(c.to_ascii_uppercase()),
            _ => Self::Symbol, // digit, symbol, non-ASCII letter, or empty name
        }
    }
}

/// How a header is printed: `#` or the letter.
impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bookmarks => write!(f, "{GROUP_BOOKMARKS}"),
            Self::Recent => write!(f, "{GROUP_RECENT}"),
            Self::MostUsed => write!(f, "{GROUP_MOST_USED}"),
            Self::Symbol => write!(f, "{SYMBOL_SECTION}"),
            Self::Letter(c) => write!(f, "{c}"),
        }
    }
}
