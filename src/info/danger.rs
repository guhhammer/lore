//! What can destroy data or stop the system: a whole command (its page's
//! `danger` category) or one example (`DANGEROUS_EXAMPLES`).

use super::content::Content;
use crate::configs::{DANGER, DANGEROUS_ANYWHERE, DANGEROUS_EXAMPLES, RUN_AS};

/// Whether the page marks its command as dangerous.
pub fn command(content: &Content) -> bool {
    content.categories.iter().any(|c| c == DANGER)
}

/// Whether running `example` can destroy data or stop the system.
pub fn example(example: &str) -> bool {
    if DANGEROUS_ANYWHERE.iter().any(|p| example.contains(p)) {
        return true;
    }
    let mut words = example.split_whitespace();
    let mut program = words.next().unwrap_or_default();
    if RUN_AS.contains(&program) {
        program = words.next().unwrap_or_default(); // the command sudo runs
    }
    let rest = example
        .split_once(program)
        .map(|(_, r)| r)
        .unwrap_or_default();
    DANGEROUS_EXAMPLES.iter().any(|(name, needle)| {
        let named = match name.strip_suffix('*') {
            Some(prefix) => program.starts_with(prefix),
            None => program == *name,
        };
        named && (needle.is_empty() || rest.contains(needle))
    })
}
