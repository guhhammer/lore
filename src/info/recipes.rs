//! Recipes: short task pages that chain commands ("free disk space"), from
//! `recipes/*.md`, built in by build.rs. Same format as a command's page,
//! with `## Steps` for `## Usage`; each is an entry of the list, under the
//! `recipes` header, by its title.

use crate::commands::{Command, Kind};
use crate::compile_config::RECIPE_SKIPPED_WORDS;
use crate::info::content::{self, Content};

// BUILT_IN_RECIPES: the text of every recipes/*.md
include!(concat!(env!("OUT_DIR"), "/recipes.rs"));

/// A recipe's title: its first line, without `# `.
fn title(text: &str) -> &str {
    text.lines()
        .next()
        .unwrap_or_default()
        .trim_start_matches("# ")
        .trim()
}

/// Every recipe, as entries of the list.
pub fn commands() -> Vec<Command> {
    BUILT_IN_RECIPES
        .iter()
        .map(|text| Command::new(title(text), Kind::Recipe))
        .collect()
}

/// The recipe called `name`, read.
pub fn find(name: &str) -> Option<Content> {
    BUILT_IN_RECIPES
        .iter()
        .find(|text| title(text) == name)
        .map(|text| content::parse(text))
}

/// What there is to show for `command`: its recipe, or its page.
pub fn content_of(command: &Command) -> Option<Content> {
    match command.kind {
        Kind::Recipe => find(&command.name),
        Kind::Language => crate::info::languages::content(&command.name),
        _ => content::find(&command.name),
    }
}

/// The commands a recipe's steps run, each once, in order: the first word
/// of each part of a step (`sudo ss -tulpn | grep` → ss, grep), past
/// `sudo` and the shell's own words (`while`, `do`, `done`), settings
/// (`IFS=`) and anything that is not a name (`[`, `$x`, `2`).
pub fn uses(content: &Content) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    for (typed, _) in &content.usage {
        for part in typed.split(['|', ';', '&', '(', ')', '{', '}']) {
            let first = part
                .split_whitespace()
                .find(|w| !RECIPE_SKIPPED_WORDS.contains(w) && !w.contains('='));
            let is_name = first.is_some_and(|w| {
                w.starts_with(|c: char| c.is_ascii_alphabetic())
                    && w.chars()
                        .all(|c| c.is_ascii_alphanumeric() || "-_.+".contains(c))
            });
            if let Some(name) = first.filter(|n| is_name && !found.iter().any(|f| f == n)) {
                found.push(name.to_owned());
            }
        }
    }
    found
}
