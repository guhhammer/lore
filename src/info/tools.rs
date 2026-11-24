//! Tools: a cheat sheet per tool ("docker", "ssh keys"), its everyday tasks
//! in groups, from `tools/*.md`, built in by build.rs. Same format as a
//! recipe, with `## Tasks` and `### Group` titles inside it; each is an
//! entry of the tools topic, by its title.

use std::collections::HashSet;

use crate::commands::{Command, Kind};
use crate::configs::TOOL_SUFFIX;
use crate::info::content::{self, Content};

// BUILT_IN_TOOLS: the text of every tools/*.md
include!(concat!(env!("OUT_DIR"), "/tools.rs"));

/// A tool page's title: its first line, without `# `.
fn title(text: &str) -> &str {
    text.lines()
        .next()
        .unwrap_or_default()
        .trim_start_matches("# ")
        .trim()
}

/// Every tool page, as entries of the list. One named like a command here
/// (`docker`) is shown as `docker (tool)`, so both can be told apart.
pub fn commands(taken: &HashSet<String>) -> Vec<Command> {
    BUILT_IN_TOOLS
        .iter()
        .map(|text| {
            let name = title(text);
            let shown = if taken.contains(name) {
                format!("{name}{TOOL_SUFFIX}")
            } else {
                name.to_owned()
            };
            Command::new(shown, Kind::Tool)
        })
        .collect()
}

/// The tool page shown as `name`, read.
pub fn find(name: &str) -> Option<Content> {
    let name = name.strip_suffix(TOOL_SUFFIX).unwrap_or(name);
    BUILT_IN_TOOLS
        .iter()
        .find(|text| title(text) == name)
        .map(|text| content::parse(text))
}
