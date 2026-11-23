//! Tests for grouping commands under section headers (src/interface/rows.rs).
//! Each test name says the rule it checks.

use lore_tui::commands::{Command, Kind};
use lore_tui::interface::rows::{self, Row};
use lore_tui::interface::section::Section;

/// The rows as text: headers as `[A]`, commands by name.
fn labels(rows: &[Row]) -> Vec<String> {
    rows.iter()
        .map(|row| match row {
            Row::Header(section) => format!("[{section}]"),
            Row::Command(command) => command.name.clone(),
        })
        .collect()
}

#[test]
fn each_section_gets_one_header_before_its_commands_with_symbols_first() {
    let commands = ["awk", "7z", "apt", "bash"]
        .map(|n| Command::new(n, Kind::Program))
        .to_vec();
    assert_eq!(
        labels(&rows::build(commands, None)),
        ["[#]", "7z", "[A]", "awk", "apt", "[B]", "bash"]
    );
}

#[test]
fn the_section_asked_for_first_goes_on_top_and_the_rest_keep_their_order() {
    let commands = ["awk", "7z", "bash", "chmod"]
        .map(|n| Command::new(n, Kind::Program))
        .to_vec();
    assert_eq!(
        labels(&rows::build(commands, Some(Section::Letter('C')))),
        ["[C]", "chmod", "[#]", "7z", "[A]", "awk", "[B]", "bash"]
    );
}
