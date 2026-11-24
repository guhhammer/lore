//! How often you have run each command, from your shell's history files:
//! bash's, zsh's and fish's, whichever exist. Read only, never written, and
//! nothing leaves the machine; `history = off` in the settings file stops it.

use std::collections::HashMap;
use std::fs;

use crate::compile_config::{HISTORY_FILES, HISTORY_WRAPPERS};
use crate::interface::paths;

/// Uses per command name, from every history file found (`$HISTFILE` too).
/// A subcommand counts as itself and as its program (`git commit` → both).
pub fn read() -> HashMap<String, u32> {
    let home = paths::home();
    let mut files: Vec<_> = HISTORY_FILES
        .iter()
        .flat_map(|pattern| paths::expand(pattern, home.as_deref()))
        .collect();
    if let Some(own) = std::env::var_os("HISTFILE") {
        files.push(own.into());
    }
    files.sort();
    files.dedup(); // $HISTFILE is usually one of the usual files
    let mut uses = HashMap::new();
    for file in files {
        if let Ok(bytes) = fs::read(&file) {
            count(&String::from_utf8_lossy(&bytes), &mut uses);
        }
    }
    uses
}

/// Adds the uses in one history file's text to `uses`.
pub fn count(text: &str, uses: &mut HashMap<String, u32>) {
    for line in text.lines() {
        for name in commands_in(line) {
            *uses.entry(name).or_insert(0) += 1;
        }
    }
}

/// The commands one history line ran. Understands zsh's `: 1700000000:0;cmd`,
/// fish's `- cmd: cmd`, and plain bash lines; skips bash's `#1700000000`
/// times and fish's other fields. Each part of a pipe or a list counts, past
/// `sudo`, `time` and the like and past `VAR=value` settings.
pub fn commands_in(line: &str) -> Vec<String> {
    let line = if let Some(rest) = line.strip_prefix("- cmd: ") {
        rest // fish
    } else if line.starts_with(": ") && line.contains(';') {
        line.split_once(';').map_or("", |(_, rest)| rest) // zsh, with times
    } else if line.starts_with('#') || line.starts_with(char::is_whitespace) {
        "" // bash's times, fish's `when:`, a continued line
    } else {
        line
    };
    let mut found = Vec::new();
    for part in line.split(['|', ';', '&']) {
        let mut words = part
            .split_whitespace()
            .skip_while(|w| HISTORY_WRAPPERS.contains(w) || is_setting(w));
        let Some(first) = words.next() else {
            continue;
        };
        // `/usr/bin/ls` and `./script.sh` run ls and script.sh
        let name = first.rsplit('/').next().unwrap_or(first);
        if name.is_empty() || name.starts_with('-') {
            continue;
        }
        found.push(name.to_owned());
        if let Some(second) = words.next().filter(|w| !w.starts_with('-')) {
            found.push(format!("{name} {second}")); // `git commit`
        }
    }
    found
}

/// `LANG=C`, `FOO=1`: a setting before the command, not the command.
fn is_setting(word: &str) -> bool {
    word.split_once('=').is_some_and(|(name, _)| {
        !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    })
}
