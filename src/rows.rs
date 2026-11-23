//! Turns a list of commands into the rows the screen shows: a header per
//! section, then that section's commands.

use crate::commands::Command;
use crate::section::Section;

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

/// Groups already-sorted commands under their headers, in section order;
/// `first`, if given, goes on top. Order inside a section is kept.
pub fn build(mut commands: Vec<Command>, first: Option<Section>) -> Vec<Row> {
    // key (false, _) sorts before (true, _), so `first` lands on top;
    // sort_by_key is stable, so names keep their order within a section
    commands.sort_by_key(|c| {
        let section = Section::of(&c.name);
        (Some(section) != first, section)
    });
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
