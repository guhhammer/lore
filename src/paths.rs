//! Turns the path patterns of compile_config.rs into real paths: `~/` → the
//! home directory, `*` → every matching name in that directory.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// The user's home directory, if `$HOME` is set and not empty.
pub fn home() -> Option<PathBuf> {
    env::var_os("HOME")
        .filter(|h| !h.is_empty())
        .map(PathBuf::from)
}

/// The paths a pattern stands for, sorted. A path without `*` is returned
/// even if it does not exist (the caller skips it); with a `*`, only paths
/// that exist. A `~/` pattern with no home gives nothing.
pub fn expand(pattern: &str, home: Option<&Path>) -> Vec<PathBuf> {
    let full = match pattern.strip_prefix("~/") {
        Some(rest) => match home {
            Some(home) => home.join(rest),
            None => return Vec::new(),
        },
        None => PathBuf::from(pattern),
    };
    // walk the path one component at a time; a `*` component fans out
    let mut found = vec![PathBuf::new()];
    for part in full.iter() {
        let part = part.to_string_lossy();
        match part.split_once('*') {
            None => found.iter_mut().for_each(|p| p.push(&*part)),
            Some((prefix, suffix)) => {
                found = found
                    .iter()
                    .flat_map(|dir| matching(dir, prefix, suffix))
                    .collect();
            }
        }
    }
    if pattern.contains('*') {
        // `/opt/*/bin`: keep only the matches that really have a `bin`
        found.retain(|p| p.exists());
    }
    found.sort();
    found
}

/// The entries of `dir` whose names start with `prefix` and end with
/// `suffix`; none if `dir` cannot be read.
fn matching(dir: &Path, prefix: &str, suffix: &str) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    entries
        .flatten()
        .filter(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            name.len() >= prefix.len() + suffix.len()
                && name.starts_with(prefix)
                && name.ends_with(suffix)
        })
        .map(|entry| entry.path())
        .collect()
}

/// A file's text, with bytes that are not UTF-8 turned into `�`; `None` if
/// it cannot be read.
pub fn read_lossy(path: &Path) -> Option<String> {
    fs::read(path)
        .ok()
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
}
