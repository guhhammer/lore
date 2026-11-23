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
use crate::utils::manual;
use crate::utils::text::wrap;

/// Always under the links: the content can come from anyone.
pub const LINK_WARNING: &str = "Check every link before you open it: anyone can add pages to \
lore. lore will never ask you to log in anywhere, or for any account.";

/// Under the summary of a command whose page is in the `danger` category.
pub const DANGER_WARNING: &str = "! Can destroy data or stop the system, with no undo. Read each \
example, and check what it acts on, before you run it.";

/// How to open a link; the terminal does it, not lore.
pub const LINK_HINT: &str = "Ctrl+click a link to open it in your browser.";

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
    let program_name = command.name.split(' ').next().unwrap_or_default();
    let location = is_program(command)
        .then(|| commands::locate(program_name))
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
    let name = &command.name;
    page.add(name, Style::Title);
    let categories = facts
        .content
        .as_ref()
        .map(|c| c.categories.as_slice())
        .unwrap_or_default();
    if !categories.is_empty() {
        page.add(&format!("[{}]", categories.join(" ")), Style::Categories);
    }
    page.blank();

    // what it is
    match command.kind {
        Kind::Alias => page.wrapped(
            &format!(
                "Your alias: typing it runs `{}`.",
                command.runs.as_deref().unwrap_or("")
            ),
            Style::Text,
            "",
        ),
        Kind::Builtin => page.wrapped("Built into your shell.", Style::Text, ""),
        Kind::Keyword => page.wrapped("A word of your shell's language.", Style::Text, ""),
        Kind::Function => page.wrapped("A shell function you defined.", Style::Text, ""),
        Kind::Subcommand => {
            let program = command.name.split(' ').next().unwrap_or_default();
            page.wrapped(&format!("A subcommand of {program}."), Style::Text, "");
        }
        Kind::Program => {}
    }
    let summary = facts
        .content
        .as_ref()
        .map(|c| c.summary.as_slice())
        .unwrap_or_default();
    if !summary.is_empty() {
        for (i, paragraph) in summary.iter().enumerate() {
            if i > 0 {
                page.blank();
            }
            page.wrapped(paragraph, Style::Text, "");
        }
    } else if let Some(line) = &facts.whatis {
        page.wrapped(&capitalized(line), Style::Text, "");
    } else if is_program(command) {
        page.wrapped(&format!("No summary yet for {name}."), Style::Dim, "");
    }

    if facts.content.as_ref().is_some_and(danger::command) {
        page.blank();
        page.wrapped(DANGER_WARNING, Style::Danger, "");
    }

    page.heading("VERSION");
    version(&mut page, command, facts);

    page.heading("HOW TO USE");
    let usage = facts
        .content
        .as_ref()
        .map(|c| c.usage.as_slice())
        .unwrap_or_default();
    if usage.is_empty() {
        page.wrapped(
            &format!("No examples yet for {name}. Anyone can add them: see content/README.md in lore's repository."),
            Style::Dim,
            "  ",
        );
    }
    for (typed, what) in usage {
        if danger::example(typed) {
            page.wrapped(&format!("! $ {typed}"), Style::Danger, "  ");
        } else {
            page.wrapped(&format!("$ {typed}"), Style::Code, "  ");
        }
        page.wrapped(what, Style::Text, "      ");
    }
    if usage.iter().any(|(typed, _)| danger::example(typed)) {
        page.wrapped(
            "! marks an example that can destroy data or stop the system: check what it acts on first.",
            Style::Dim,
            "  ",
        );
    }

    let related = related(command, facts, man);
    if !related.is_empty() {
        page.heading("RELATED");
        page.wrapped("Open one with / or `lore NAME`.", Style::Dim, "  ");
        page.wrapped(&related.join(", "), Style::Text, "  ");
    }

    if !facts.subcommands.is_empty() {
        page.heading("SUBCOMMANDS");
        page.wrapped(
            &format!(
                "Each has its own page: find it in the list, or run `lore {}`.",
                facts.subcommands[0]
            ),
            Style::Dim,
            "  ",
        );
        page.wrapped(&facts.subcommands.join(", "), Style::Text, "  ");
    }

    let notes = facts
        .content
        .as_ref()
        .map(|c| c.notes.as_slice())
        .unwrap_or_default();
    if !notes.is_empty() {
        page.heading("NOTES");
        for note in notes {
            page.wrapped(note, Style::Text, "  ");
        }
    }

    page.heading("LINKS");
    page.wrapped(LINK_HINT, Style::Dim, "  ");
    let own = facts
        .content
        .as_ref()
        .map(|c| c.links.clone())
        .unwrap_or_default();
    for (label, url) in own.into_iter().chain(links::searches(name)) {
        page.blank();
        page.wrapped(&label, Style::Text, "  ");
        page.add(&format!("  {url}"), Style::Url); // whole, or the terminal opens half
    }
    page.blank();
    page.wrapped(LINK_WARNING, Style::Warning, "  ");

    page.heading("MANUAL");
    match man {
        Some(lines) => lines.iter().for_each(|l| page.add(l, Style::Text)),
        None => {
            page.wrapped(manual::NO_PAGE, Style::Dim, "  ");
            for reason in manual::NO_PAGE_REASONS {
                page.wrapped(&format!("- {reason}"), Style::Dim, "    ");
            }
        }
    }
    page.lines
}

/// The VERSION section: what the package manager knows, or why there is none.
fn version(page: &mut Page, command: &Command, facts: &Facts) {
    if !is_program(command) {
        let why = match command.kind {
            Kind::Alias => {
                "An alias has no version of its own: it is the one of the command it runs."
            }
            Kind::Function => "A function you wrote has no version.",
            _ => {
                "Part of your shell, so it has the shell's version (`bash --version`, `zsh --version`, `fish --version`)."
            }
        };
        page.wrapped(why, Style::Text, "  ");
        return;
    }
    match &facts.package {
        Some(package) => {
            page.field("Package", &format!("{} ({})", package.name, package.manager));
            if let Some(installed) = &package.installed {
                page.field("Installed", installed);
            }
            match (&package.installed, &package.newest) {
                (Some(installed), Some(newest)) if installed == newest => {
                    page.field("Newest", &format!("{newest}, up to date as of your last package list update"));
                }
                (_, Some(newest)) => page.field("Newest", &format!("{newest}: an update is available")),
                _ => {}
            }
            page.field("Update", &package.update);
        }
        None if facts.location.is_some() => page.wrapped(
            "Not from a package manager lore knows: installed by hand, by a language's tool (cargo, pip, npm…) or by a script. Its version: try `--version`.",
            Style::Text,
            "  ",
        ),
        None => page.wrapped("Not installed on this machine, or not in a folder lore looks in.", Style::Dim, "  "),
    }
    if let Some(location) = &facts.location {
        page.field("Location", &location.display().to_string());
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
        let room = self.width.saturating_sub(indent.chars().count());
        for line in wrap(text, room) {
            self.add(&format!("{indent}{line}"), style);
        }
    }

    /// "  Label      value", the values lined up, wrapped under themselves.
    fn field(&mut self, label: &str, value: &str) {
        let indent = "  ";
        let label = format!("{indent}{label:<11}");
        let room = self.width.saturating_sub(label.chars().count());
        for (i, line) in wrap(value, room).into_iter().enumerate() {
            let lead = if i == 0 {
                label.clone()
            } else {
                " ".repeat(label.chars().count())
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
        .unwrap_or_default();
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
