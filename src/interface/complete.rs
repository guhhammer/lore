//! Shell completion: what can come next after `lore …`, for the scripts in
//! completions/ (bash, zsh, fish), which ask `lore --complete WORDS` (the
//! words after `lore`, the last one the word being typed). lore answers
//! itself, so the shells and lore never disagree.
//!
//! - `lore gi` → names starting with `gi`: programs, builtins, aliases, the
//!   first words of recipes, and `explain`;
//! - `lore git ` → the next word of every name that starts with `git`
//!   (`git commit`), and so on for recipes (`lore free ` → `disk`);
//! - `lore -` → the options; `lore --print` completes names like `lore`;
//! - `lore explain tar -` → tar's flags, from its man page.
//!
//! Nothing to offer: the scripts complete file names instead.

use crate::configs::{COMPLETE_OPTIONS, EXPLAIN_WORD, SEARCH_WORD};
use crate::info::explain::ManOption;

/// One candidate: the word, and what it is (empty when not known).
pub type Candidate = (String, String);

/// What lore knows, for completing: every name in the list, a one-line
/// description of a name, a program's options.
pub struct Sources<'a> {
    pub names: &'a [String],
    pub describe: &'a dyn Fn(&str) -> String,
    pub options_of: &'a dyn Fn(&str) -> Vec<ManOption>,
}

/// The candidates for the last of `words`, sorted, each once.
pub fn candidates(words: &[String], sources: &Sources) -> Vec<Candidate> {
    let (current, before) = match words.split_last() {
        Some((current, before)) => (current.as_str(), before),
        None => ("", &[][..]),
    };
    let mut found: Vec<Candidate> = match before {
        [] if current.starts_with('-') => COMPLETE_OPTIONS
            .iter()
            .filter(|(option, _)| option.starts_with(current))
            .map(|(option, what)| ((*option).to_owned(), (*what).to_owned()))
            .collect(),
        [] => {
            let mut words = next_words(&[], current, sources);
            for word in [EXPLAIN_WORD, SEARCH_WORD] {
                if word.starts_with(current) {
                    words.push((word.to_owned(), word_line(word)));
                }
            }
            words
        }
        [first, rest @ ..] if first == "-p" || first == "--print" => {
            next_words(rest, current, sources)
        }
        [first] if first == EXPLAIN_WORD => next_words(&[], current, sources),
        [first, program, ..] if first == EXPLAIN_WORD && current.starts_with('-') => {
            (sources.options_of)(program)
                .into_iter()
                .flat_map(|option| {
                    let text = option.text.clone();
                    option
                        .names
                        .into_iter()
                        .map(move |name| (name, text.clone()))
                })
                .filter(|(name, _)| name.starts_with(current))
                .collect()
        }
        // after `search`, free words: nothing to offer
        [first, ..] if first.starts_with('-') || first == EXPLAIN_WORD || first == SEARCH_WORD => {
            Vec::new()
        }
        before => next_words(before, current, sources),
    };
    // one of each word, the one described kept (`search` the word, not
    // a program of that name with no description)
    found.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.len().cmp(&a.1.len())));
    found.dedup_by(|a, b| a.0 == b.0);
    found
}

/// A first word's description (`explain`, `search`), from the options' list.
fn word_line(word: &str) -> String {
    COMPLETE_OPTIONS
        .iter()
        .find(|(option, _)| *option == word)
        .map(|(_, what)| (*what).to_owned())
        .unwrap_or_default()
}

/// The word after `before` in every name that starts with `before` and
/// whose next word starts with `current`: `[git]` + `co` → `commit`,
/// `config`… Described when there are few enough to read.
fn next_words(before: &[String], current: &str, sources: &Sources) -> Vec<Candidate> {
    let mut words: Vec<(String, String)> = Vec::new(); // (word, the whole name)
    for name in sources.names {
        let mut parts = name.split(' ');
        if !before.iter().all(|b| parts.next() == Some(b.as_str())) {
            continue;
        }
        if let Some(next) = parts.next().filter(|n| n.starts_with(current)) {
            // the whole name only when the word ends it: `git` itself, not `git commit`
            let whole = if parts.next().is_none() {
                name.clone()
            } else {
                String::new()
            };
            words.push((next.to_owned(), whole));
        }
    }
    words.sort();
    words.dedup_by(|later, kept| {
        let same = later.0 == kept.0;
        if same && kept.1.is_empty() {
            kept.1 = std::mem::take(&mut later.1); // keep the one that is a name itself
        }
        same
    });
    let describe = words.len() <= crate::configs::COMPLETE_DESCRIBE_MAX;
    words
        .into_iter()
        .map(|(word, whole)| {
            let what = if describe && !whole.is_empty() {
                (sources.describe)(&whole)
            } else {
                String::new()
            };
            (word, what)
        })
        .collect()
}
