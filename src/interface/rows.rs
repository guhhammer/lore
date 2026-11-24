//! Turns a list of commands into the rows the screen shows: a header per
//! section, then that section's commands.

use crate::commands::Command;
use crate::interface::section::Section;

/// One line of the list.
pub enum Row {
    /// A section letter, like `A` or `#`; the cursor skips it.
    Header(Section),
    /// A heading in words (the beacon's "trending this month"); skipped too.
    Title(String),
    Command(Command),
}

impl Row {
    /// Whether the cursor can stop here (commands yes, headers no).
    pub fn is_selectable(&self) -> bool {
        matches!(self, Self::Command(_))
    }
}

/// The whole list, with no search: already-sorted commands grouped under
/// their A-Z headers (`#` first), order inside a section kept.
pub fn build(mut commands: Vec<Command>) -> Vec<Row> {
    // by section first: `{` sorts after `z` by name, but belongs under `#`;
    // sort_by_key is stable, so names keep their order within a section
    commands.sort_by_key(|c| Section::of(&c.name));
    let mut rows = Vec::new();
    let mut current = None; // the section of the last header pushed
    for command in commands {
        let section = Section::of(&command.name);
        if current != Some(section) {
            rows.push(Row::Header(section)); // a new section starts here
            current = Some(section);
        }
        rows.push(Row::Command(command));
    }
    rows
}

/// Search results: no headers, in the order given (best match first).
pub fn ranked(commands: Vec<Command>) -> Vec<Row> {
    commands.into_iter().map(Row::Command).collect()
}
