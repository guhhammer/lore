//! The sections of a topic's list: `#` for names starting with a digit or
//! symbol, then `A` to `Z`.

use std::fmt;

use crate::configs::SYMBOL_SECTION;

/// A section. `Symbol` is declared first, so it sorts before every letter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Section {
    /// Shown as `#`: names like `7z`, `[`, `!`.
    Symbol,
    /// Always upper case: `ls` and `Xorg` go under `L` and `X`.
    Letter(char),
}

impl Section {
    /// The section a name goes under: its first letter upper-cased (`é` and
    /// `я` too, for a recipe's translated name), or `Symbol`.
    pub fn of(name: &str) -> Self {
        match name.chars().next() {
            Some(c) if c.is_ascii_alphabetic() => Self::Letter(c.to_ascii_uppercase()),
            // a letter with a capital: Latin with accents, Greek, Cyrillic
            Some(c) if c.is_lowercase() || c.is_uppercase() => {
                Self::Letter(c.to_uppercase().next().unwrap_or(c))
            }
            _ => Self::Symbol, // digit, symbol, a script with no capitals, or empty
        }
    }
}

/// How a header is printed: `#` or the letter.
impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Symbol => write!(f, "{SYMBOL_SECTION}"),
            Self::Letter(c) => write!(f, "{c}"),
        }
    }
}
