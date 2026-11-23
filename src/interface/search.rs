//! The search rules: what matches, and which section goes first. Only names
//! are searched for now; command info will be searched here too.

use crate::commands::Command;
use crate::interface::section::Section;

/// True if the name contains the query, ignoring case; an empty query matches all.
pub fn matches(command: &Command, query: &str) -> bool {
    query.is_empty() || command.name.to_lowercase().contains(&query.to_lowercase())
}

/// For a one-character query, that character's section, to show it first
/// (`h` → the H commands on top). Any other query: `None`, the usual order.
pub fn first_section(query: &str) -> Option<Section> {
    let mut chars = query.chars();
    match (chars.next(), chars.next()) {
        // exactly one character; encode_utf8 turns it into the &str Section::of takes
        (Some(c), None) => Some(Section::of(c.encode_utf8(&mut [0; 4]))),
        _ => None,
    }
}
