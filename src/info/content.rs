//! A command's written page: a short Markdown file, `<command>.md`. The
//! default pages are built into lore (from the repository's content/ folder,
//! by build.rs), so no folder is needed to use them; a page in one of
//! `USER_CONTENT_DIRS` adds or replaces pages; `UPDATED_CONTENT_DIRS`
//! (`lore --update-pages`) and `SYSTEM_CONTENT_DIRS` (other programs' pages)
//! only add them. Pages from folders must pass `info::check`.
//!
//! The format (content/README.md has it for writers):
//!
//! ```text
//! # ls
//!
//! What it is and does, in a few sentences.
//!
//! ## Usage
//! - `ls -la` : every file, hidden ones too, with details
//!
//! ## Notes
//! Anything important: what can go wrong, what it cannot undo.
//!
//! ## Links
//! - GNU manual: https://www.gnu.org/software/coreutils/ls
//! ```

use std::collections::HashSet;
use std::fs;
use std::io::Read;

use crate::compile_config::{
    CHECK_INVISIBLE, FORBIDDEN_FILE_CHARS, LOCALE_VARS, NO_TRANSLATION_LOCALES, PAGE_MAX_BYTES,
    READING_LANGUAGE, SYSTEM_CONTENT_DIRS, UPDATED_CONTENT_DIRS, USER_CONTENT_DIRS,
};
use crate::info::check;
use crate::interface::paths;

// EMBEDDED: the built-in pages, (name, byte range in EMBEDDED_TEXT), sorted by
// name; EMBEDDED_I18N: their translations, (language, name, range), sorted by
// language then name; EMBEDDED_TEXT: every page, unpacked on first use
include!(concat!(env!("OUT_DIR"), "/content.rs"));

/// One command's page, parsed.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Content {
    /// The text before the first `##`: paragraphs, blank-line separated.
    pub summary: Vec<String>,
    /// (what to type, what it does).
    pub usage: Vec<(String, String)>,
    /// The `## Notes` lines, as written.
    pub notes: Vec<String>,
    /// (label, URL); only `https://` links are kept.
    pub links: Vec<(String, String)>,
    /// `## See also`: related commands, by name.
    pub see_also: Vec<String>,
    /// `## Categories`: the areas it belongs to (`files, archives`);
    /// `danger` marks a command that can destroy data or stop the system.
    pub categories: Vec<String>,
}

/// The file name (without `.md`) of `name`'s page: a subcommand's spaces
/// become dashes, as tldr names them (`git commit` → `git-commit`), and the
/// characters Windows forbids in file names become `%XX` (`:` → `%3A.md`),
/// so the repository checks out, and packages, everywhere.
pub fn page_key(name: &str) -> String {
    let mut key = String::with_capacity(name.len());
    for c in name.chars() {
        match c {
            ' ' => key.push('-'),
            c if FORBIDDEN_FILE_CHARS.contains(&c) => key.push_str(&format!("%{:02X}", c as u32)),
            _ => key.push(c),
        }
    }
    key
}

/// `name`'s page: from the first folder that has one, else built in.
pub fn find(name: &str) -> Option<Content> {
    let name = &page_key(name);
    if name.is_empty() || name.contains('/') || name.starts_with('.') {
        return None; // a page name must stay inside the folder
    }
    // yours, then lore's own, then one --update-pages fetched, then a page
    // another program installed
    let text = folder_page(USER_CONTENT_DIRS, None, name)
        .or_else(|| built_in(name).map(str::to_owned))
        .or_else(|| folder_page(UPDATED_CONTENT_DIRS, None, name))
        .or_else(|| folder_page(SYSTEM_CONTENT_DIRS, None, name))?;
    let mut content = parse(&text);
    // in the reader's language when there is a translation: its summary and
    // examples replace the English ones; the rest stays. Translations come
    // in the same order.
    let translated = languages().iter().find_map(|lang| {
        folder_page(USER_CONTENT_DIRS, Some(lang), name)
            .or_else(|| translation(lang, name).map(str::to_owned))
            .or_else(|| folder_page(UPDATED_CONTENT_DIRS, Some(lang), name))
            .or_else(|| folder_page(SYSTEM_CONTENT_DIRS, Some(lang), name))
    });
    if let Some(translated) = translated {
        let translated = parse(&translated);
        if !translated.summary.is_empty() {
            content.summary = translated.summary;
        }
        if !translated.usage.is_empty() {
            content.usage = translated.usage;
        }
        for link in translated.links {
            if !content.links.contains(&link) {
                content.links.push(link);
            }
        }
    }
    Some(content)
}

/// `name`'s page (or its translation into `lang`, under `i18n/<lang>/`)
/// from the first of `dirs` that has one that passes the check; one that
/// fails is passed over, as if it were not there.
fn folder_page(dirs: &[&str], lang: Option<&str>, name: &str) -> Option<String> {
    let home = paths::home();
    dirs.iter()
        .flat_map(|dir| paths::expand(dir, home.as_deref()))
        .find_map(|dir| {
            let dir = match lang {
                Some(lang) => dir.join("i18n").join(lang),
                None => dir,
            };
            let file = fs::File::open(dir.join(format!("{name}.md"))).ok()?;
            let mut bytes = Vec::new();
            // one byte more than allowed, so a file too big is seen as such
            file.take(PAGE_MAX_BYTES as u64 + 1)
                .read_to_end(&mut bytes)
                .ok()?;
            if !check::passes(&check::check(name, &bytes)) {
                return None;
            }
            String::from_utf8(bytes).ok()
        })
}

/// A character a page may show: not a control character (an escape
/// sequence would be obeyed by the terminal), not one that hides or
/// reorders text.
pub fn shown(c: char) -> bool {
    !c.is_control()
        && !CHECK_INVISIBLE
            .iter()
            .any(|(from, to)| (*from..=*to).contains(&c))
}

/// The built-in pages, packed by build.rs with zstd, as one text. A pack
/// that does not unpack (it cannot happen with a build that passed its
/// tests) leaves lore with no built-in pages rather than stopping it.
pub fn unpack(packed: &[u8]) -> String {
    let mut text = String::new();
    if let Ok(mut decoder) = ruzstd::decoding::StreamingDecoder::new(packed)
        && decoder.read_to_string(&mut text).is_err()
    {
        text.clear();
    }
    text
}

/// The text at `range` in the built-in pages; empty if it is not there.
fn embedded((start, end): (usize, usize)) -> &'static str {
    EMBEDDED_TEXT.get(start..end).unwrap_or_default()
}

/// The page built into lore for `name`, if any.
pub fn built_in(name: &str) -> Option<&'static str> {
    // sorted by build.rs, so a binary search: this runs for every command
    EMBEDDED
        .binary_search_by(|(n, _)| (*n).cmp(name))
        .ok()
        .map(|i| embedded(EMBEDDED[i].1))
}

/// The built-in translation of `name`'s page into `lang` (`pt_BR`), if any.
pub fn translation(lang: &str, name: &str) -> Option<&'static str> {
    EMBEDDED_I18N
        .binary_search_by(|(l, n, _)| (*l, *n).cmp(&(lang, name)))
        .ok()
        .map(|i| embedded(EMBEDDED_I18N[i].2))
}

/// The reader's languages, most specific first, from `LC_ALL`,
/// `LC_MESSAGES` or `LANG` (the first one set): `pt_BR.UTF-8` gives
/// `["pt_BR", "pt"]`. English, `C` and `POSIX` give none: the pages are
/// English already.
pub fn languages() -> Vec<String> {
    // Pages in, on the about screen, wins over LANG
    if let Some(code) = READING_LANGUAGE
        .read()
        .ok()
        .and_then(|chosen| chosen.clone())
    {
        return languages_of(&code);
    }
    let value = LOCALE_VARS
        .iter()
        .find_map(|var| std::env::var(var).ok().filter(|v| !v.is_empty()))
        .unwrap_or_default();
    languages_of(&value)
}

/// How many languages the built-in translations are in (the about screen).
pub fn translated_languages() -> usize {
    let mut seen: Vec<&str> = EMBEDDED_I18N.iter().map(|(lang, _, _)| *lang).collect();
    seen.dedup(); // the table is sorted by language
    seen.len()
}

/// The language codes the built-in translations are in, sorted.
pub fn translated_codes() -> Vec<&'static str> {
    let mut seen: Vec<&str> = EMBEDDED_I18N.iter().map(|(lang, _, _)| *lang).collect();
    seen.dedup();
    seen
}

/// `languages` for one locale value; separate so it can be tested.
pub fn languages_of(locale: &str) -> Vec<String> {
    // "pt_BR.UTF-8@euro" → "pt_BR"
    let code = locale.split(['.', '@']).next().unwrap_or_default();
    let base = code.split('_').next().unwrap_or_default();
    if base.is_empty()
        || NO_TRANSLATION_LOCALES.contains(&base)
        || NO_TRANSLATION_LOCALES.contains(&code)
    {
        return Vec::new();
    }
    let mut found = vec![code.to_owned()];
    if base != code {
        found.push(base.to_owned());
    }
    found
}

/// Every name with a page: built in, or in a content folder. Each
/// folder is listed once, not checked per command.
pub fn names() -> HashSet<String> {
    let home = paths::home();
    let mut names: HashSet<String> = built_in_names().map(str::to_owned).collect();
    for dir in USER_CONTENT_DIRS
        .iter()
        .chain(UPDATED_CONTENT_DIRS)
        .chain(SYSTEM_CONTENT_DIRS)
        .flat_map(|dir| paths::expand(dir, home.as_deref()))
    {
        let Ok(entries) = fs::read_dir(dir) else {
            continue; // most of these folders do not exist, and need not
        };
        for entry in entries.flatten() {
            let file = entry.file_name();
            if let Some(name) = file.to_str().and_then(|f| f.strip_suffix(".md"))
                && name != "README"
            {
                names.insert(name.to_owned());
            }
        }
    }
    names
}

/// The titles of the built-in pages that are subcommands: those whose `#`
/// title has a space (`# git commit` in git-commit.md).
pub fn subcommand_titles() -> impl Iterator<Item = &'static str> {
    EMBEDDED.iter().filter_map(|&(name, range)| {
        let title = embedded(range).lines().next()?.strip_prefix("# ")?.trim();
        (title.contains(' ') && page_key(title) == name).then_some(title)
    })
}

/// The subcommands of `parent` that have a built-in page, sorted.
pub fn subcommands_of(parent: &str) -> Vec<&'static str> {
    let mut found: Vec<&str> = subcommand_titles()
        .filter(|title| title.split(' ').next() == Some(parent))
        .collect();
    found.sort_unstable();
    found
}

/// The names of every built-in page.
pub fn built_in_names() -> impl Iterator<Item = &'static str> {
    EMBEDDED.iter().map(|(name, _)| *name)
}

/// Which part of the page a line belongs to.
enum Part {
    Summary,
    Usage,
    Notes,
    Links,
    SeeAlso,
    Categories,
    /// A `##` section lore does not know: skipped.
    Other,
}

/// A page's text, parsed. Never fails: what it does not understand, it skips.
pub fn parse(text: &str) -> Content {
    let mut content = Content::default();
    let mut part = Part::Summary;
    let mut paragraph = String::new();
    for line in text.lines() {
        // control and invisible characters out: a page can come from anyone
        let line: String = line.chars().filter(|&c| shown(c)).collect();
        let line = line.trim_end();
        if let Some(heading) = line.strip_prefix("## ") {
            flush(&mut content.summary, &mut paragraph);
            part = match heading.trim().to_lowercase().as_str() {
                // a recipe's steps are its examples
                "usage" | "steps" => Part::Usage,
                "notes" => Part::Notes,
                "links" => Part::Links,
                "see also" => Part::SeeAlso,
                "categories" => Part::Categories,
                _ => Part::Other,
            };
            continue;
        }
        match part {
            Part::Summary if line.starts_with("# ") => {} // the title: the name, shown anyway
            Part::Summary if line.trim().is_empty() => flush(&mut content.summary, &mut paragraph),
            Part::Summary => {
                // lines of one paragraph join with a space, as Markdown does
                if !paragraph.is_empty() {
                    paragraph.push(' ');
                }
                paragraph.push_str(line.trim());
            }
            Part::Usage => content.usage.extend(usage(line)),
            Part::Notes if !line.trim().is_empty() => content.notes.push(line.trim().to_owned()),
            Part::Links => content.links.extend(link(line)),
            Part::SeeAlso => {
                if let Some(name) = line.trim().strip_prefix("- ") {
                    content
                        .see_also
                        .push(name.trim().trim_matches('`').to_owned());
                }
            }
            Part::Categories => content.categories.extend(
                line.split(',')
                    .map(|c| c.trim().to_lowercase())
                    .filter(|c| !c.is_empty()),
            ),
            Part::Notes | Part::Other => {}
        }
    }
    flush(&mut content.summary, &mut paragraph);
    content
}

/// Ends the paragraph being read, if there is one.
fn flush(summary: &mut Vec<String>, paragraph: &mut String) {
    if !paragraph.is_empty() {
        summary.push(std::mem::take(paragraph));
    }
}

/// "- `ls -la` : what it does" → (ls -la, what it does).
fn usage(line: &str) -> Option<(String, String)> {
    let rest = line.trim().strip_prefix("- ")?.trim();
    let rest = rest.strip_prefix('`')?;
    let (typed, after) = rest.split_once('`')?;
    let what = after.trim().trim_start_matches(':').trim();
    Some((typed.to_owned(), what.to_owned()))
}

/// "- label: https://…" → (label, URL). Anything but https is dropped: a
/// page can come from anyone, and lore only ever links to web pages.
fn link(line: &str) -> Option<(String, String)> {
    let rest = line.trim().strip_prefix("- ")?.trim();
    let start = rest.find("https://")?;
    let url = rest[start..].split_whitespace().next()?;
    let label = rest[..start].trim().trim_end_matches(':').trim();
    let label = if label.is_empty() { url } else { label };
    Some((label.to_owned(), url.to_owned()))
}
