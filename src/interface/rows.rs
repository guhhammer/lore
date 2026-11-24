//! Turns a list of commands into the rows the screen shows: a header per
//! section, then that section's commands.

use crate::commands::Command;
use crate::interface::section::Section;

/// One line of the list.
pub enum Row {
    /// A section letter, like `A` or `#`; the cursor skips it.
    Header(Section),
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
    commands.sort_by_key(Section::of_command);
    let mut rows = Vec::new();
    let mut current = None; // the section of the last header pushed
    for command in commands {
        let section = Section::of_command(&command);
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

/// `rows` with groups above them (bookmarks, recent, most used), each under
/// its header; an empty group is left out.
pub fn with_groups(groups: Vec<(Section, Vec<Command>)>, rows: Vec<Row>) -> Vec<Row> {
    let mut all = Vec::new();
    for (section, commands) in groups.into_iter().filter(|(_, c)| !c.is_empty()) {
        all.push(Row::Header(section));
        all.extend(commands.into_iter().map(Row::Command));
    }
    all.extend(rows);
    all
}
