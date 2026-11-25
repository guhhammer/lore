//! The interface in the reader's language: the screen's texts and the
//! recipes' names, from `i18n/<language>/` (built in by build.rs), in the
//! language Pages in chose (else `LANG`'s). The English text is the key, so
//! a text with no translation, or a language with none, stays English.
//! Only the full-screen app translates (`TRANSLATE_INTERFACE`): what lore
//! prints for scripts stays English.

use std::collections::HashMap;
use std::sync::atomic::Ordering;

use crate::commands::{Command, Kind};
use crate::configs::{INTERFACE_CACHE, RECIPE_CACHE, TRANSLATE_INTERFACE};
use crate::info::content;

// INTERFACE_I18N and RECIPE_I18N: (language, the file's text)
include!(concat!(env!("OUT_DIR"), "/i18n.rs"));

/// One language's translations: English text → translated text.
pub type Catalog = HashMap<String, String>;

/// A translation file read: `English<TAB>translation` lines, `\n`, `\t`
/// and `\\` written as escapes; blank lines and `#` comments skipped.
pub fn parse(text: &str) -> Catalog {
    text.lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .filter_map(|line| line.split_once('\t'))
        .map(|(english, translated)| (unescape(english), unescape(translated)))
        .collect()
}

/// `\n`, `\t` and `\\` as the characters they stand for.
fn unescape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

/// Every language's catalog of one table, read the first time it is needed.
fn catalogs(
    table: &'static [(&'static str, &'static str)],
    cache: &'static std::sync::OnceLock<Vec<(&'static str, Catalog)>>,
) -> &'static [(&'static str, Catalog)] {
    cache.get_or_init(|| {
        table
            .iter()
            .map(|(code, text)| (*code, parse(text)))
            .collect()
    })
}

/// `text` in the first of the reader's languages that has it
/// (`pt_BR`, then `pt`).
fn find(found: &'static [(&'static str, Catalog)], text: &str) -> Option<&'static str> {
    if !TRANSLATE_INTERFACE.load(Ordering::Relaxed) {
        return None;
    }
    content::languages().iter().find_map(|code| {
        found
            .iter()
            .find(|(c, _)| c == code)
            .and_then(|(_, catalog)| catalog.get(text))
            .map(String::as_str)
    })
}

/// A text of the screen, in the reader's language (English when it has no
/// translation, or outside the full-screen app).
pub fn tr(text: &str) -> &str {
    find(catalogs(INTERFACE_I18N, &INTERFACE_CACHE), text).unwrap_or(text)
}

/// A recipe's name as the screen shows it, in the reader's language. The
/// English name stays its name everywhere else (`lore -p`, bookmarks).
pub fn recipe_name(name: &str) -> &str {
    find(catalogs(RECIPE_I18N, &RECIPE_CACHE), name).unwrap_or(name)
}

/// The name an entry is shown by: a recipe's translated, any other as is.
pub fn shown(command: &Command) -> &str {
    match command.kind {
        Kind::Recipe => recipe_name(&command.name),
        _ => &command.name,
    }
}

/// A key bar, each key and what it does in the reader's language.
pub fn keys(bar: &[(&'static str, &'static str)]) -> Vec<(&'static str, &'static str)> {
    bar.iter().map(|(key, what)| (tr(key), tr(what))).collect()
}

/// The languages the interface is translated into, sorted.
pub fn codes() -> Vec<&'static str> {
    INTERFACE_I18N.iter().map(|(code, _)| *code).collect()
}

/// One language's catalogs as written: (interface, recipes). For the tests.
pub fn catalog_of(code: &str) -> (Catalog, Catalog) {
    let of = |table: &[(&str, &str)]| {
        table
            .iter()
            .find(|(c, _)| *c == code)
            .map(|(_, text)| parse(text))
            .unwrap_or_default()
    };
    (of(INTERFACE_I18N), of(RECIPE_I18N))
}
