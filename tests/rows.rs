//! Tests for grouping commands under section headers (src/interface/rows.rs).
//! Each test name says the rule it checks.

use lore_tui::commands::{Command, Kind};
use lore_tui::interface::rows::{self, Row};

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
        labels(&rows::build(commands)),
        ["[#]", "7z", "[A]", "awk", "apt", "[B]", "bash"]
    );
}

#[test]
fn search_results_have_no_headers_and_keep_the_order_given() {
    let commands = ["tar", "zip", "gzip"]
        .map(|n| Command::new(n, Kind::Program))
        .to_vec();
    assert_eq!(labels(&rows::ranked(commands)), ["tar", "zip", "gzip"]);
}
