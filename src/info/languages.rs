//! The languages topic: learnxinyminutes.com's pages (languages/, built in
//! by build.rs), one per programming language, tool or format. Each is a
//! short introduction and one long, commented file that walks through the
//! language; lore shows it the way `bat` shows a file, with line numbers
//! and colours, instead of in sections. In the reader's language when the
//! site has a translation.
//!
//! A page starts with a header between `---` lines:
//!
//! ```text
//! ---
//! name: Python
//! contributors:
//!     - ["Louie Dinh", "http://pythonpracticeprojects.com"]
//! filename: learnpython.py
//! ---
//! ```

use std::collections::HashSet;

use crate::commands::{Command, Kind};
use crate::compile_config::{
    LANGUAGE_AND_MORE, LANGUAGE_BAR, LANGUAGE_CATEGORY, LANGUAGE_CONTRIBUTORS_MAX, LANGUAGE_FILE,
    LANGUAGE_GUTTER_MIN, LANGUAGE_LINK_HEADING, LANGUAGE_RULE, LANGUAGE_RULE_BOTTOM,
    LANGUAGE_RULE_MIDDLE, LANGUAGE_RULE_TOP, LANGUAGE_SOURCE, LANGUAGE_SUFFIX, LANGUAGE_TAB,
    LANGUAGE_URL,
};
use crate::info::content::{self, Content, unpack};
use crate::info::page::{PageLine, Style};
use crate::utils::highlight::{self, Syntax};
use crate::utils::text;

// LANGUAGE_PAGES: (file name, range in LANGUAGE_TEXT), sorted; LANGUAGE_I18N:
// (language, file name, range), sorted; LANGUAGE_TEXT: unpacked on first use
include!(concat!(env!("OUT_DIR"), "/languages.rs"));

fn text_at((start, end): (usize, usize)) -> &'static str {
    LANGUAGE_TEXT.get(start..end).unwrap_or_default()
}

/// A page's header (between the `---` lines) and its body.
pub fn split(page: &str) -> (&str, &str) {
    let Some(rest) = page.strip_prefix("---") else {
        return ("", page);
    };
    match rest.find("\n---") {
        Some(at) => {
            let body = &rest[at + 4..];
            (&rest[..at], body.strip_prefix('\n').unwrap_or(body))
        }
        None => ("", page),
    }
}

/// A header's `key: value`, quotes and spaces taken off.
pub fn field<'a>(header: &'a str, key: &str) -> Option<&'a str> {
    header.lines().find_map(|line| {
        let value = line.strip_prefix(key)?.strip_prefix(':')?;
        let value = value.trim().trim_matches('"').trim();
        (!value.is_empty()).then_some(value)
    })
}

/// The people a header names (`- ["Name", "url"]`), contributors then
/// translators.
pub fn people(header: &str) -> Vec<&str> {
    header
        .lines()
        .filter_map(|line| line.trim().strip_prefix("- [\"")?.split('"').next())
        .filter(|name| !name.is_empty())
        .collect()
}

/// The language codes the site's translations are in, sorted.
pub fn translated_codes() -> Vec<&'static str> {
    let mut seen: Vec<&str> = LANGUAGE_I18N.iter().map(|(code, _, _)| *code).collect();
    seen.dedup();
    seen
}

/// Every language page, as (file name, name shown).
fn all() -> Vec<(&'static str, String)> {
    LANGUAGE_PAGES
        .iter()
        .filter_map(|&(file, range)| {
            let (header, _) = split(text_at(range));
            Some((file, field(header, "name")?.to_owned()))
        })
        .collect()
}

/// Every language, as entries of the list. A name a command here already
/// has (`bc`, `R`) gets `LANGUAGE_SUFFIX`, so both are listed.
pub fn commands(taken: &HashSet<String>) -> Vec<Command> {
    all()
        .into_iter()
        .map(|(_, name)| {
            let name = if taken.contains(&name) {
                format!("{name}{LANGUAGE_SUFFIX}")
            } else {
                name
            };
            Command::new(name, Kind::Language)
        })
        .collect()
}

/// The file of the language shown as `name`.
fn file_of(name: &str) -> Option<&'static str> {
    let name = name.strip_suffix(LANGUAGE_SUFFIX).unwrap_or(name);
    all()
        .into_iter()
        .find(|(_, shown)| shown == name)
        .map(|(file, _)| file)
}

/// The page of `file` in the reader's language when there is one, else
/// in English.
fn page_of(file: &str) -> Option<&'static str> {
    let english = LANGUAGE_PAGES
        .binary_search_by(|(f, _)| (*f).cmp(file))
        .ok()
        .map(|i| text_at(LANGUAGE_PAGES[i].1))?;
    let translated = content::languages().iter().find_map(|lang| {
        LANGUAGE_I18N
            .binary_search_by(|(l, f, _)| (*l, *f).cmp(&(lang.as_str(), file)))
            .ok()
            .map(|i| text_at(LANGUAGE_I18N[i].2))
    });
    Some(translated.unwrap_or(english))
}

/// What the list and the search know of a language: its first paragraph,
/// and the `language` category.
pub fn content(name: &str) -> Option<Content> {
    let (_, body) = split(page_of(file_of(name)?)?);
    let summary: Vec<&str> = body
        .lines()
        .skip_while(|line| line.trim().is_empty())
        .take_while(|line| !line.trim().is_empty() && !line.starts_with("```"))
        .map(str::trim)
        .collect();
    Some(Content {
        summary: vec![summary.join(" ")],
        categories: vec![LANGUAGE_CATEGORY.to_owned()],
        ..Content::default()
    })
}

/// The page for `name`, `width` columns wide: the title, where it comes
/// from, the introduction as text, each code block as a file with line
/// numbers and colours, then the page on the web.
pub fn render(name: &str, width: usize) -> Vec<PageLine> {
    let mut lines = Vec::new();
    let Some(file) = file_of(name) else {
        return lines;
    };
    let Some(page) = page_of(file) else {
        return lines;
    };
    let (header, body) = split(page);
    let plain = |text: &str, style| PageLine {
        text: text.to_owned(),
        style,
        spans: Vec::new(),
    };
    lines.push(plain(name, Style::Title));
    lines.push(plain(&format!("[{LANGUAGE_CATEGORY}]"), Style::Categories));
    let people = people(header);
    let mut who = people
        .iter()
        .take(LANGUAGE_CONTRIBUTORS_MAX)
        .copied()
        .collect::<Vec<_>>()
        .join(", ");
    if people.len() > LANGUAGE_CONTRIBUTORS_MAX {
        who.push_str(LANGUAGE_AND_MORE);
    }
    for line in text::wrap(&LANGUAGE_SOURCE.replace("{who}", &who), width) {
        lines.push(plain(&line, Style::Dim));
    }
    lines.push(plain("", Style::Text));

    // the file's name, for the first block's frame (`learnpython.py`)
    let english_header = split(page_of_english(file)).0;
    let filename = field(header, "filename").or_else(|| field(english_header, "filename"));
    let mut fence: Option<(usize, String)> = None; // (backticks, tag) while in a block
    let mut block: Vec<String> = Vec::new();
    let mut blocks = 0;
    // the blank lines before the introduction: one is drawn already
    for raw in body.trim_start_matches(['\n', '\r']).lines() {
        let trimmed = raw.trim_start();
        let ticks = trimmed.chars().take_while(|&c| c == '`').count();
        match &fence {
            None if ticks >= 3 => {
                fence = Some((ticks, trimmed[ticks..].trim().to_owned()));
            }
            Some((open, tag)) if ticks >= *open && trimmed[ticks..].trim().is_empty() => {
                let shown = if blocks == 0 { filename } else { None };
                code_block(&mut lines, &block, tag, shown.or(filename), shown, width);
                blocks += 1;
                block.clear();
                fence = None;
            }
            Some(_) => block.push(raw.replace('\t', LANGUAGE_TAB)),
            None => prose(&mut lines, raw, width),
        }
    }
    if let Some((_, tag)) = &fence {
        // a block left open: shown as far as it goes
        code_block(&mut lines, &block, tag, filename, None, width);
    }
    lines.push(plain("", Style::Text));
    lines.push(plain(LANGUAGE_LINK_HEADING, Style::Heading));
    let stem = file.strip_suffix(".md").unwrap_or(file);
    lines.push(plain(&LANGUAGE_URL.replace("{name}", stem), Style::Url));
    lines
}

/// The English page of `file` (its header names the file).
fn page_of_english(file: &str) -> &'static str {
    LANGUAGE_PAGES
        .binary_search_by(|(f, _)| (*f).cmp(file))
        .map(|i| text_at(LANGUAGE_PAGES[i].1))
        .unwrap_or_default()
}

/// One line of the introduction: a `#` heading is a heading the jumper
/// stops at; the rest is text, wrapped.
fn prose(lines: &mut Vec<PageLine>, raw: &str, width: usize) {
    let line = |text: String, style| PageLine {
        text,
        style,
        spans: Vec::new(),
    };
    let heading = raw.trim_start_matches('#');
    if raw.starts_with('#') && heading.starts_with(' ') {
        lines.push(line(heading.trim().to_owned(), Style::Heading));
    } else if raw.trim().is_empty() {
        lines.push(line(String::new(), Style::Text));
    } else {
        for part in text::wrap(raw, width) {
            lines.push(line(part, Style::Text));
        }
    }
}

/// A code block as `bat` shows a file: a rule, the file's name, a rule,
/// each line numbered and coloured (wrapped under its number when too
/// long), a closing rule. A comment line under a rule of `###` or `---`
/// is a heading the jumper stops at.
fn code_block(
    lines: &mut Vec<PageLine>,
    code: &[String],
    tag: &str,
    syntax_name: Option<&str>,
    shown_name: Option<&str>,
    width: usize,
) {
    let extension = syntax_name.and_then(|f| f.rsplit('.').next());
    let syntax: Syntax = highlight::by_name(tag)
        .or_else(|| extension.and_then(highlight::by_name))
        .unwrap_or_else(|| highlight::guess(code));
    let gutter = code.len().to_string().len().max(LANGUAGE_GUTTER_MIN);
    let room = width
        .saturating_sub(gutter + LANGUAGE_BAR.chars().count())
        .max(10);
    let rule = |joint: char| {
        let left: String = std::iter::repeat_n(LANGUAGE_RULE, gutter + 1).collect();
        let right: String =
            std::iter::repeat_n(LANGUAGE_RULE, width.saturating_sub(gutter + 2)).collect();
        PageLine {
            text: format!("{left}{joint}{right}"),
            style: Style::Gutter,
            spans: Vec::new(),
        }
    };
    lines.push(rule(LANGUAGE_RULE_TOP));
    let name = shown_name
        .map(str::to_owned)
        .or_else(|| (!tag.is_empty()).then(|| tag.to_owned()));
    if let Some(name) = name {
        let blank = " ".repeat(gutter);
        lines.push(PageLine {
            text: format!(
                "{blank}{LANGUAGE_BAR}{}",
                LANGUAGE_FILE.replace("{file}", &name)
            ),
            style: Style::Text,
            spans: vec![
                (format!("{blank}{LANGUAGE_BAR}"), Style::Gutter),
                (LANGUAGE_FILE.replace("{file}", &name), Style::Text),
            ],
        });
        lines.push(rule(LANGUAGE_RULE_MIDDLE));
    }
    let mut open = None;
    let mut after_rule = false;
    for (number, code_line) in code.iter().enumerate() {
        let parts = highlight::line(code_line, &syntax, &mut open);
        let comment = parts
            .iter()
            .all(|(text, style)| *style == Style::Comment || text.trim().is_empty());
        let core = code_line
            .trim()
            .trim_start_matches(|c: char| !c.is_alphanumeric());
        let is_rule = comment && code_line.trim().chars().count() >= 3 && core.is_empty();
        let heading = comment && after_rule && !core.is_empty();
        after_rule = is_rule || (after_rule && !heading && code_line.trim().is_empty());
        // the line cut into rows of `room` columns, its number on the first
        let mut rows: Vec<Vec<(String, Style)>> = vec![Vec::new()];
        let mut used = 0;
        for (text_part, style) in parts {
            let mut rest = text_part.as_str();
            while !rest.is_empty() {
                if used == room {
                    rows.push(Vec::new());
                    used = 0;
                }
                let (piece, after) = text::take_columns(rest, room - used);
                let piece = if piece.is_empty() {
                    // one character wider than what is left: on the next row
                    rows.push(Vec::new());
                    used = 0;
                    continue;
                } else {
                    piece
                };
                used += text::width(piece);
                if let Some(row) = rows.last_mut() {
                    row.push((piece.to_owned(), style));
                }
                rest = after;
            }
        }
        for (i, row) in rows.into_iter().enumerate() {
            let left = if i == 0 {
                format!("{:>gutter$}{LANGUAGE_BAR}", number + 1)
            } else {
                format!("{}{LANGUAGE_BAR}", " ".repeat(gutter))
            };
            let mut spans = vec![(left, Style::Gutter)];
            spans.extend(row);
            let text: String = spans.iter().map(|(t, _)| t.as_str()).collect();
            let style = if heading && i == 0 {
                Style::Heading
            } else {
                Style::Code
            };
            lines.push(PageLine { text, style, spans });
        }
    }
    lines.push(rule(LANGUAGE_RULE_BOTTOM));
}
