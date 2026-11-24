//! lore's settings file, kept between runs: `$XDG_CONFIG_HOME/lore/config`,
//! or `~/.config/lore/config` when that is unset. Plain text, one
//! `key = value` per line, `#` starts a comment; lore writes it when a
//! setting is changed in the app (the about screen, `+`: colours, internet, history).
//!
//! Lists lore keeps for you (bookmarks, recent pages) are data, not
//! settings: one name a line in `$XDG_DATA_HOME/lore/`, or
//! `~/.local/share/lore/`.
//!
//! A missing or unreadable file is no error: every setting has a default,
//! and a list starts empty.

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::compile_config::{
    CONFIG_FILE, CONFIG_HEADER, CONFIG_HOME_FALLBACK, CONFIG_HOME_VAR, DATA_DIR,
    DATA_HOME_FALLBACK, DATA_HOME_VAR,
};
use crate::interface::paths;

/// Where the settings file is, from `$XDG_CONFIG_HOME` and `$HOME`; none
/// when neither is usable.
pub fn path() -> Option<PathBuf> {
    path_from(
        std::env::var_os(CONFIG_HOME_VAR).as_deref(),
        paths::home().as_deref(),
    )
}

/// `path`, with the two variables given. The XDG spec ignores a relative
/// `$XDG_CONFIG_HOME`, so lore does too.
pub fn path_from(config_home: Option<&OsStr>, home: Option<&Path>) -> Option<PathBuf> {
    let base = config_home
        .map(Path::new)
        .filter(|dir| dir.is_absolute())
        .map(Path::to_path_buf)
        .or_else(|| home.map(|home| home.join(CONFIG_HOME_FALLBACK)))?;
    Some(base.join(CONFIG_FILE))
}

/// A setting's value from the file's text; the last line for `key` wins.
pub fn get(text: &str, key: &str) -> Option<String> {
    text.lines()
        .filter_map(entry)
        .rfind(|(k, _)| *k == key)
        .map(|(_, value)| value.to_owned())
}

/// The file's text with `key` set to `value`: its first line replaced and
/// any later ones dropped, or a line added at the end. Every other line,
/// comments included, stays as it was.
pub fn set(text: &str, key: &str, value: &str) -> String {
    let line = format!("{key} = {value}");
    let mut done = false;
    let mut out: Vec<&str> = Vec::new();
    for current in text.lines() {
        if entry(current).is_some_and(|(k, _)| k == key) {
            if !done {
                out.push(&line);
                done = true;
            }
        } else {
            out.push(current);
        }
    }
    if !done {
        out.push(&line);
    }
    out.join("\n") + "\n"
}

/// A `key = value` line, both trimmed; none for blanks and comments.
fn entry(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();
    if line.starts_with('#') {
        return None;
    }
    let (key, value) = line.split_once('=')?;
    Some((key.trim(), value.trim()))
}

/// A setting from the file, if the file and the setting are there.
pub fn read(key: &str) -> Option<String> {
    get(&fs::read_to_string(path()?).ok()?, key)
}

/// Saves a setting, making the folder and the file if needed; returns the
/// file's path. Fails when there is no home to save in or it cannot be
/// written; the caller says so, and the setting still holds for this run.
pub fn write(key: &str, value: &str) -> io::Result<PathBuf> {
    let path = path().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no home folder"))?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let text = fs::read_to_string(&path).unwrap_or_else(|_| CONFIG_HEADER.to_owned());
    fs::write(&path, set(&text, key, value))?;
    Ok(path)
}

/// Where the list `file` is kept, from `$XDG_DATA_HOME` and `$HOME`.
pub fn list_path(file: &str) -> Option<PathBuf> {
    list_path_from(
        std::env::var_os(DATA_HOME_VAR).as_deref(),
        paths::home().as_deref(),
        file,
    )
}

/// `list_path`, with the two variables given; a relative
/// `$XDG_DATA_HOME` is ignored, as for the settings file.
pub fn list_path_from(
    data_home: Option<&OsStr>,
    home: Option<&Path>,
    file: &str,
) -> Option<PathBuf> {
    let base = data_home
        .map(Path::new)
        .filter(|dir| dir.is_absolute())
        .map(Path::to_path_buf)
        .or_else(|| home.map(|home| home.join(DATA_HOME_FALLBACK)))?;
    Some(base.join(DATA_DIR).join(file))
}

/// A kept list's names, in order; empty when there is no file.
pub fn read_list(file: &str) -> Vec<String> {
    list_path(file)
        .and_then(|path| fs::read_to_string(path).ok())
        .map(|text| parse_list(&text))
        .unwrap_or_default()
}

/// The names in a list file's text: one a line, blanks skipped.
pub fn parse_list(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

/// Keeps a list, making its folder if needed.
pub fn write_list(file: &str, names: &[String]) -> io::Result<()> {
    let path =
        list_path(file).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no home folder"))?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut text = names.join("\n");
    text.push('\n');
    fs::write(path, text)
}
