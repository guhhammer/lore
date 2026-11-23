//! The user's aliases, read as plain text from their shell's startup files
//! (`Shell::alias_files`). Reading, not running the shell: running it would
//! load the whole startup (ble.sh, oh-my-zsh…) on every launch. The cost:
//! aliases in files those files load from elsewhere are missed.

use std::path::PathBuf;

use super::Command;
use super::shell::Shell;
use crate::interface::paths;

/// Every alias of `shell` in its files, as commands.
pub fn scan(shell: Shell) -> Vec<Command> {
    let home = paths::home();
    let files: Vec<PathBuf> = shell
        .alias_files()
        .iter()
        .flat_map(|pattern| paths::expand(pattern, home.as_deref()))
        .collect();
    read(&files, shell)
}

/// Every alias in these files, in order, so a later definition wins. Never
/// fails: an unreadable file is skipped, and bytes that are not UTF-8 become
/// `�` instead of losing the whole file.
pub fn read(files: &[PathBuf], shell: Shell) -> Vec<Command> {
    let text = files
        .iter()
        .filter_map(|file| paths::read_lossy(file))
        .collect::<Vec<_>>()
        .join("\n");
    let pairs = match shell {
        Shell::Bash | Shell::Zsh => parse(&text),
        Shell::Fish => parse_fish(&text),
    };
    pairs
        .into_iter()
        .map(|(name, value)| Command::alias(name, value))
        .collect()
}

/// Every bash/zsh `alias name=value` in the text as (name, value). A name
/// defined twice keeps its last value. Comments are skipped.
pub fn parse(text: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for line in text.lines() {
        // indented lines count too: aliases often sit inside an `if`
        let Some(rest) = line.trim_start().strip_prefix("alias ") else {
            continue; // not an alias line (this also skips `# alias …`)
        };
        let Some(rest) = skip_flags(rest) else {
            continue; // zsh `alias -s`: a file-suffix alias, not a command
        };
        for pair in definitions(rest) {
            keep(&mut found, pair);
        }
    }
    found
}

/// Every fish `alias name 'value'` (or `alias name=value`) in the text.
pub fn parse_fish(text: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for line in text.lines() {
        let Some(rest) = line.trim_start().strip_prefix("alias ") else {
            continue;
        };
        let Some(rest) = skip_flags(rest) else {
            continue;
        };
        let rest = rest.trim();
        let name_end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        if rest[..name_end].contains('=') {
            // `alias name=value`: same as bash
            definitions(rest)
                .into_iter()
                .for_each(|pair| keep(&mut found, pair));
        } else if name_end < rest.len() {
            // `alias name value`: the value is the rest, one level of quotes off
            let value = rest[name_end..].trim();
            let value = unquote(value).unwrap_or(value);
            keep(
                &mut found,
                (rest[..name_end].to_string(), value.to_string()),
            );
        }
    }
    found
}

/// Adds a (name, value), replacing the value if the name is already there.
fn keep(found: &mut Vec<(String, String)>, (name, value): (String, String)) {
    match found.iter_mut().find(|(n, _)| *n == name) {
        Some(existing) => existing.1 = value, // redefined: last one wins
        None => found.push((name, value)),
    }
}

/// The text after `alias`'s options (`-g`, `--save`…). `None` for zsh's `-s`
/// (and `--`-less forms like `-gs`), which defines suffix aliases.
fn skip_flags(mut rest: &str) -> Option<&str> {
    loop {
        rest = rest.trim_start();
        if !rest.starts_with('-') {
            return Some(rest);
        }
        let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        let flag = &rest[..end];
        if !flag.starts_with("--") && flag.contains('s') {
            return None;
        }
        rest = &rest[end..];
    }
}

/// The text inside one pair of matching quotes around the whole value.
fn unquote(value: &str) -> Option<&str> {
    let quote = value.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    value.strip_prefix(quote)?.strip_suffix(quote)
}

/// The `name=value` pairs of a bash/zsh alias line, until it ends or a `#`.
/// Values may be 'single' quoted, "double" quoted, or bare.
fn definitions(mut rest: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    loop {
        rest = rest.trim_start();
        if rest.is_empty() || rest.starts_with('#') {
            return pairs;
        }
        let Some((name, after)) = rest.split_once('=') else {
            return pairs; // `alias -p` or `alias name`: prints, defines nothing
        };
        if name.is_empty() || name.contains(char::is_whitespace) {
            return pairs; // not a valid name=value
        }
        let (value, remaining) = match after.chars().next() {
            // quoted: the value runs to the matching quote (or the line's end)
            Some(quote @ ('\'' | '"')) => match after[1..].find(quote) {
                Some(end) => (&after[1..1 + end], &after[2 + end..]),
                None => (&after[1..], ""),
            },
            // bare: the value runs to the next space
            _ => {
                let end = after.find(char::is_whitespace).unwrap_or(after.len());
                (&after[..end], &after[end..])
            }
        };
        pairs.push((name.to_string(), value.to_string()));
        rest = remaining; // another alias may follow on the same line
    }
}
