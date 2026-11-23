//! A command's whole page, as lines to draw, top to bottom: the name and
//! summary, VERSION, HOW TO USE, NOTES, LINKS (with the safety note under
//! them) and MANUAL. `facts` gathers what has to be looked up, once per
//! command; `render` lays it out for a width, again on every resize.

use std::path::PathBuf;

use super::content::{self, Content};
use super::danger;
use super::links;
use super::package::{self, Package};
use crate::commands::{self, Command, Kind};
use crate::compile_config::{
    DANGER_EXAMPLE_NOTE, DANGER_MARK, DANGER_WARNING, EXAMPLE_INDENT, EXAMPLE_PROMPT,
    FIELD_INSTALLED, FIELD_LABEL_WIDTH, FIELD_LOCATION, FIELD_NEWEST, FIELD_PACKAGE, FIELD_UPDATE,
    HEADING_LINKS, HEADING_MANUAL, HEADING_NOTES, HEADING_RELATED, HEADING_SUBCOMMANDS,
    HEADING_USAGE, HEADING_VERSION, IS_ALIAS, IS_BUILTIN, IS_FUNCTION, IS_KEYWORD, IS_SUBCOMMAND,
    LINK_HINT, LINK_WARNING, NEWEST_SAME, NEWEST_UPDATE, NO_EXAMPLES, NO_PAGE, NO_PAGE_REASONS,
    NO_SUMMARY, PAGE_INDENT, REASON_INDENT, RELATED_HINT, SUBCOMMANDS_HINT, VERSION_ALIAS,
    VERSION_FUNCTION, VERSION_NOT_FOUND, VERSION_SHELL, VERSION_UNPACKAGED,
};
use crate::utils::manual;
use crate::utils::text::{self, wrap};

/// What a line is, so the drawing can style it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Title,
    Heading,
    Text,
    /// Something to type.
    Code,
    /// Secondary: hints, "nothing here yet".
    Dim,
    /// A whole URL, never wrapped or cut, so the terminal can open it.
    Url,
    Warning,
    /// The `[categories]` under the name.
    Categories,
    /// Can destroy data or stop the system: the warning and the examples.
    Danger,
}

/// One line of the page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageLine {
    pub text: String,
    pub style: Style,
}

/// What is looked up about a command, once: its page, its `whatis` line,
/// where it is and which package it came from.
#[derive(Debug, Clone, Default)]
pub struct Facts {
    pub content: Option<Content>,
    pub whatis: Option<String>,
    pub location: Option<PathBuf>,
    pub package: Option<Package>,
    /// Its subcommands that have a lore page (`git` → `git commit`, …).
    pub subcommands: Vec<String>,
}

/// Looks `command` up: its page, its description and, for a program, its
/// file and package. Runs `whatis` and the package manager; never fails.
pub fn facts(command: &Command) -> Facts {
    // a subcommand's file and package are its program's (`git commit` → git)
    let location = is_program(command)
        .then(|| commands::locate(command.program()))
        .flatten();
    Facts {
        content: content::find(&command.name),
        whatis: manual::whatis(&content::page_key(&command.name)),
        package: location.as_deref().and_then(package::of),
        location,
        subcommands: content::subcommands_of(&command.name)
            .into_iter()
            .map(str::to_owned)
            .collect(),
    }
}

/// Whether `command` is (or hides) a program, or is one's subcommand: what
/// has a file, a package and a version.
fn is_program(command: &Command) -> bool {
    matches!(command.kind, Kind::Program | Kind::Subcommand) || command.hides == Some(Kind::Program)
}

/// The page for `command`, `width` columns wide. `man` is the manual laid
/// out at that width; None when there is none.
pub fn render(
    command: &Command,
    facts: &Facts,
    man: Option<&[String]>,
    width: usize,
) -> Vec<PageLine> {
    let mut page = Page {
        lines: Vec::new(),
        width,
    };
    let empty = Content::default();
    let content = facts.content.as_ref().unwrap_or(&empty); // no page: every part empty
    let name = &command.name;
    page.add(name, Style::Title);
    if !content.categories.is_empty() {
        page.add(
            &format!("[{}]", content.categories.join(" ")),
            Style::Categories,
        );
    }
    page.blank();

    // what it is
    let what = match command.kind {
        Kind::Alias => Some(IS_ALIAS.replace("{runs}", command.runs.as_deref().unwrap_or(""))),
        Kind::Builtin => Some(IS_BUILTIN.to_owned()),
        Kind::Keyword => Some(IS_KEYWORD.to_owned()),
        Kind::Function => Some(IS_FUNCTION.to_owned()),
        Kind::Subcommand => Some(IS_SUBCOMMAND.replace("{program}", command.program())),
        Kind::Program => None,
    };
    if let Some(what) = what {
        page.wrapped(&what, Style::Text, "");
    }
    if !content.summary.is_empty() {
        for (i, paragraph) in content.summary.iter().enumerate() {
            if i > 0 {
                page.blank();
            }
            page.wrapped(paragraph, Style::Text, "");
        }
    } else if let Some(line) = &facts.whatis {
        page.wrapped(&capitalized(line), Style::Text, "");
    } else if is_program(command) {
        page.wrapped(&NO_SUMMARY.replace("{name}", name), Style::Dim, "");
    }

    if danger::command(content) {
        page.blank();
        page.wrapped(DANGER_WARNING, Style::Danger, "");
    }

    page.heading(HEADING_VERSION);
    version(&mut page, command, facts);

    page.heading(HEADING_USAGE);
    if content.usage.is_empty() {
        page.wrapped(
            &NO_EXAMPLES.replace("{name}", name),
            Style::Dim,
            PAGE_INDENT,
        );
    }
    let mut any_dangerous = false;
    for (typed, what) in &content.usage {
        if danger::example(typed) {
            any_dangerous = true;
            page.wrapped(
                &format!("{DANGER_MARK}{EXAMPLE_PROMPT}{typed}"),
                Style::Danger,
                PAGE_INDENT,
            );
        } else {
            page.wrapped(
                &format!("{EXAMPLE_PROMPT}{typed}"),
                Style::Code,
                PAGE_INDENT,
            );
        }
        page.wrapped(what, Style::Text, EXAMPLE_INDENT);
    }
    if any_dangerous {
        page.wrapped(DANGER_EXAMPLE_NOTE, Style::Dim, PAGE_INDENT);
    }

    let related = related(command, facts, man);
    if !related.is_empty() {
        page.heading(HEADING_RELATED);
        page.wrapped(RELATED_HINT, Style::Dim, PAGE_INDENT);
        page.wrapped(&related.join(", "), Style::Text, PAGE_INDENT);
    }

    if !facts.subcommands.is_empty() {
        page.heading(HEADING_SUBCOMMANDS);
        page.wrapped(
            &SUBCOMMANDS_HINT.replace("{first}", &facts.subcommands[0]),
            Style::Dim,
            PAGE_INDENT,
        );
        page.wrapped(&facts.subcommands.join(", "), Style::Text, PAGE_INDENT);
    }

    if !content.notes.is_empty() {
        page.heading(HEADING_NOTES);
        for note in &content.notes {
            page.wrapped(note, Style::Text, PAGE_INDENT);
        }
    }

    page.heading(HEADING_LINKS);
    page.wrapped(LINK_HINT, Style::Dim, PAGE_INDENT);
    for (label, url) in content.links.iter().cloned().chain(links::searches(name)) {
        page.blank();
        page.wrapped(&label, Style::Text, PAGE_INDENT);
        page.add(&format!("{PAGE_INDENT}{url}"), Style::Url); // whole, or the terminal opens half
    }
    page.blank();
    page.wrapped(LINK_WARNING, Style::Warning, PAGE_INDENT);

    page.heading(HEADING_MANUAL);
    match man {
        Some(lines) => lines.iter().for_each(|l| page.add(l, Style::Text)),
        None => {
            page.wrapped(NO_PAGE, Style::Dim, PAGE_INDENT);
            for reason in NO_PAGE_REASONS {
                page.wrapped(&format!("- {reason}"), Style::Dim, REASON_INDENT);
            }
        }
    }
    page.lines
}

/// The VERSION section: what the package manager knows, or why there is none.
fn version(page: &mut Page, command: &Command, facts: &Facts) {
    if !is_program(command) {
        let why = match command.kind {
            Kind::Alias => VERSION_ALIAS,
            Kind::Function => VERSION_FUNCTION,
            _ => VERSION_SHELL,
        };
        page.wrapped(why, Style::Text, PAGE_INDENT);
        return;
    }
    match &facts.package {
        Some(package) => {
            page.field(
                FIELD_PACKAGE,
                &format!("{} ({})", package.name, package.manager),
            );
            if let Some(installed) = &package.installed {
                page.field(FIELD_INSTALLED, installed);
            }
            match (&package.installed, &package.newest) {
                (Some(installed), Some(newest)) if installed == newest => {
                    page.field(FIELD_NEWEST, &NEWEST_SAME.replace("{newest}", newest));
                }
                (_, Some(newest)) => {
                    page.field(FIELD_NEWEST, &NEWEST_UPDATE.replace("{newest}", newest))
                }
                _ => {}
            }
            page.field(FIELD_UPDATE, &package.update);
        }
        None if facts.location.is_some() => {
            page.wrapped(VERSION_UNPACKAGED, Style::Text, PAGE_INDENT)
        }
        None => page.wrapped(VERSION_NOT_FOUND, Style::Dim, PAGE_INDENT),
    }
    if let Some(location) = &facts.location {
        page.field(FIELD_LOCATION, &location.display().to_string());
    }
}

/// "list directory contents" → "List directory contents."
fn capitalized(line: &str) -> String {
    let mut chars = line.chars();
    let mut text: String = chars
        .next()
        .map(|c| c.to_uppercase().collect())
        .unwrap_or_default();
    text.push_str(chars.as_str());
    if !text.ends_with('.') {
        text.push('.');
    }
    text
}

/// The lines being built, and the width they must fit.
struct Page {
    lines: Vec<PageLine>,
    width: usize,
}

impl Page {
    fn add(&mut self, text: &str, style: Style) {
        self.lines.push(PageLine {
            text: text.to_owned(),
            style,
        });
    }

    fn blank(&mut self) {
        self.add("", Style::Text);
    }

    /// A section's heading, after a blank line.
    fn heading(&mut self, title: &str) {
        self.blank();
        self.add(title, Style::Heading);
    }

    /// `text` wrapped to the width, every line starting with `indent`.
    fn wrapped(&mut self, text: &str, style: Style, indent: &str) {
        let room = self.width.saturating_sub(text::width(indent));
        for line in wrap(text, room) {
            self.add(&format!("{indent}{line}"), style);
        }
    }

    /// "  Label      value", the values lined up, wrapped under themselves.
    fn field(&mut self, label: &str, value: &str) {
        let label = format!("{PAGE_INDENT}{label:<FIELD_LABEL_WIDTH$}");
        let room = self.width.saturating_sub(text::width(&label));
        for (i, line) in wrap(value, room).into_iter().enumerate() {
            let lead = if i == 0 {
                label.clone()
            } else {
                " ".repeat(text::width(&label))
            };
            self.add(&format!("{lead}{line}"), Style::Text);
        }
    }
}

/// Commands related to `command`: its page's `See also`, then the man page's
/// SEE ALSO references (`tar(1)` → tar), each once, itself left out.
pub fn related(command: &Command, facts: &Facts, man: Option<&[String]>) -> Vec<String> {
    let mut found: Vec<String> = facts
        .content
        .as_ref()
        .map(|c| c.see_also.clone())
        .unwrap_or_default(); // no page: only the man page's references
    found.extend(man.map(see_also_in_man).unwrap_or_default());
    let mut seen = std::collections::HashSet::new();
    found.retain(|name| *name != command.name && seen.insert(name.clone()));
    found
}

/// The `name(section)` references under a man page's SEE ALSO heading.
pub fn see_also_in_man(lines: &[String]) -> Vec<String> {
    let mut found = Vec::new();
    let mut inside = false;
    for line in lines {
        let heading = !line.starts_with(' ') && !line.trim().is_empty();
        if heading {
            inside = line.trim() == "SEE ALSO";
            continue;
        }
        if !inside {
            continue;
        }
        // "tar(1), gzip(1)": each word written as name(section), with no
        // space before the bracket ("version 3" in a licence is not one)
        for token in line.split(|c: char| c.is_whitespace() || c == ',') {
            let Some((word, section)) = token.split_once('(') else {
                continue;
            };
            if !word.is_empty()
                && section.starts_with(|c: char| c.is_ascii_digit())
                && section.contains(')')
                && word
                    .chars()
                    .all(|c| c.is_alphanumeric() || "-_.+".contains(c))
            {
                found.push(word.to_owned());
            }
        }
    }
    found
}
