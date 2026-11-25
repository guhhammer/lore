//! A command's whole page, as lines to draw, top to bottom: the name and
//! summary, VERSION, HOW TO USE, NOTES, LINKS (with the safety note under
//! them) and MANUAL. `facts` gathers what has to be looked up, once per
//! command; `render` lays it out for a width, again on every resize.

use std::path::PathBuf;

use super::content::{self, Content};
use super::danger;
use super::links;
use super::package::{self, Package};
use super::recipes;
use crate::commands::{self, Command, Kind};
use crate::configs::{
    DANGER_EXAMPLE_NOTE, DANGER_MARK, DANGER_WARNING, EXAMPLE_INDENT, EXAMPLE_PROMPT,
    FIELD_INSTALLED, FIELD_LABEL_WIDTH, FIELD_LOCATION, FIELD_NEWEST, FIELD_PACKAGE, FIELD_UPDATE,
    FILES_HINT, FILES_MAX, FILES_PATH_STARTS, HEADING_FILES, HEADING_INTERNET, HEADING_LINKS,
    HEADING_MANUAL, HEADING_NOTES, HEADING_RELATED, HEADING_STEPS, HEADING_SUBCOMMANDS,
    HEADING_TASKS, HEADING_USAGE, HEADING_USES, HEADING_VERSION, INTERNET_CHECKING, INTERNET_CUT,
    INTERNET_DISABLED, INTERNET_DOT, INTERNET_FAILED, INTERNET_GUTTER, INTERNET_LOADING,
    INTERNET_MAX_LINES, INTERNET_NOTHING, INTERNET_OFFLINE, INTERNET_REMINDER, INTERNET_SOURCE,
    IS_ALIAS, IS_BUILTIN, IS_FUNCTION, IS_KEYWORD, IS_RECIPE, IS_SUBCOMMAND, IS_TOOL, LINK_HINT,
    LINK_WARNING, NEWEST_SAME, NEWEST_UPDATE, NO_EXAMPLES, NO_PAGE, NO_PAGE_REASONS, NO_SUMMARY,
    PAGE_INDENT, REASON_INDENT, RELATED_HINT, STEPS_HINT, SUBCOMMANDS_HINT, TASKS_HINT, USES_HINT,
    VERSION_ALIAS, VERSION_FUNCTION, VERSION_NOT_FOUND, VERSION_SHELL, VERSION_UNPACKAGED,
};
use crate::utils::i18n::tr;
use crate::utils::internet::{self, Answer, Status};
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
    /// The internet's dot and state: green when online, red when not.
    Online,
    Offline,
    /// Text from the internet, shown as it came: never a link, never copied.
    Raw,
    /// A language page's file, coloured like `bat`: its comments, keywords,
    /// strings and numbers, and the line numbers beside it.
    Comment,
    Keyword,
    Literal,
    Number,
    Gutter,
}

/// One line of the page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageLine {
    pub text: String,
    pub style: Style,
    /// When not empty, the line in parts of their own style (a language
    /// page's code); `text` is then those parts joined, and `style` says
    /// only what the line is (a heading the jumper stops at, or not).
    pub spans: Vec<(String, Style)>,
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
    if command.kind.is_lores() && !matches!(command.kind, Kind::Docset | Kind::DocEntry) {
        // lore's own page: nothing to look up on the machine
        return Facts {
            content: recipes::content_of(command),
            ..Facts::default()
        };
    }
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
    if command.kind == Kind::Recipe {
        recipe(&mut page, command, facts);
        return page.lines;
    }
    if command.kind == Kind::Tool {
        tool(&mut page, command, facts);
        return page.lines;
    }
    if command.kind == Kind::Language {
        return crate::info::languages::render(&command.name, width);
    }
    if matches!(command.kind, Kind::Docset | Kind::DocEntry) {
        return docs_page(command, width);
    }
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
        Kind::Alias => Some(tr(IS_ALIAS).replace("{runs}", command.runs.as_deref().unwrap_or(""))),
        Kind::Builtin => Some(tr(IS_BUILTIN).to_owned()),
        Kind::Keyword => Some(tr(IS_KEYWORD).to_owned()),
        Kind::Function => Some(tr(IS_FUNCTION).to_owned()),
        Kind::Subcommand => Some(tr(IS_SUBCOMMAND).replace("{program}", command.program())),
        Kind::Program
        | Kind::Recipe
        | Kind::Tool
        | Kind::Language
        | Kind::Docset
        | Kind::DocEntry => None,
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
        page.wrapped(&tr(NO_SUMMARY).replace("{name}", name), Style::Dim, "");
    }

    if danger::command(content) {
        page.blank();
        page.wrapped(tr(DANGER_WARNING), Style::Danger, "");
    }

    page.heading(tr(HEADING_VERSION));
    version(&mut page, command, facts);

    page.heading(tr(HEADING_USAGE));
    if content.usage.is_empty() {
        page.wrapped(
            &tr(NO_EXAMPLES).replace("{name}", name),
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
        page.wrapped(tr(DANGER_EXAMPLE_NOTE), Style::Dim, PAGE_INDENT);
    }

    let related = related(command, facts, man);
    if !related.is_empty() {
        page.heading(tr(HEADING_RELATED));
        page.wrapped(tr(RELATED_HINT), Style::Dim, PAGE_INDENT);
        page.wrapped(&related.join(", "), Style::Text, PAGE_INDENT);
    }

    if !facts.subcommands.is_empty() {
        page.heading(tr(HEADING_SUBCOMMANDS));
        page.wrapped(
            &tr(SUBCOMMANDS_HINT).replace("{first}", &facts.subcommands[0]),
            Style::Dim,
            PAGE_INDENT,
        );
        page.wrapped(&facts.subcommands.join(", "), Style::Text, PAGE_INDENT);
    }

    if !content.notes.is_empty() {
        page.heading(tr(HEADING_NOTES));
        for note in &content.notes {
            page.wrapped(note, Style::Text, PAGE_INDENT);
        }
    }

    let files = man.map(files_in_man).unwrap_or_default();
    if !files.is_empty() {
        page.heading(tr(HEADING_FILES));
        page.wrapped(tr(FILES_HINT), Style::Dim, PAGE_INDENT);
        for path in &files {
            page.add(&format!("{PAGE_INDENT}{path}"), Style::Code); // whole, like a URL
        }
    }

    page.heading(tr(HEADING_LINKS));
    page.wrapped(tr(LINK_HINT), Style::Dim, PAGE_INDENT);
    for (label, url) in content.links.iter().cloned().chain(links::searches(name)) {
        page.blank();
        page.wrapped(&label, Style::Text, PAGE_INDENT);
        page.add(&format!("{PAGE_INDENT}{url}"), Style::Url); // whole, or the terminal opens half
    }
    page.blank();
    page.wrapped(tr(LINK_WARNING), Style::Warning, PAGE_INDENT);

    page.heading(tr(HEADING_MANUAL));
    match man {
        Some(lines) => lines.iter().for_each(|l| page.add(l, Style::Text)),
        None => {
            page.wrapped(tr(NO_PAGE), Style::Dim, PAGE_INDENT);
            for reason in NO_PAGE_REASONS.iter().map(|r| tr(r)) {
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
            Kind::Alias => tr(VERSION_ALIAS),
            Kind::Function => tr(VERSION_FUNCTION),
            _ => tr(VERSION_SHELL),
        };
        page.wrapped(why, Style::Text, PAGE_INDENT);
        return;
    }
    match &facts.package {
        Some(package) => {
            page.field(
                tr(FIELD_PACKAGE),
                &format!("{} ({})", package.name, package.manager),
            );
            if let Some(installed) = &package.installed {
                page.field(tr(FIELD_INSTALLED), installed);
            }
            match (&package.installed, &package.newest) {
                (Some(installed), Some(newest)) if installed == newest => {
                    page.field(
                        tr(FIELD_NEWEST),
                        &tr(NEWEST_SAME).replace("{newest}", newest),
                    );
                }
                (_, Some(newest)) => page.field(
                    tr(FIELD_NEWEST),
                    &tr(NEWEST_UPDATE).replace("{newest}", newest),
                ),
                _ => {}
            }
            page.field(tr(FIELD_UPDATE), &package.update);
        }
        None if facts.location.is_some() => {
            page.wrapped(tr(VERSION_UNPACKAGED), Style::Text, PAGE_INDENT)
        }
        None => page.wrapped(tr(VERSION_NOT_FOUND), Style::Dim, PAGE_INDENT),
    }
    if let Some(location) = &facts.location {
        page.field(tr(FIELD_LOCATION), &location.display().to_string());
    }
}

/// A docs topic page: a set's cover, or one of its pages (`runs` says
/// which: the slug, or `slug<TAB>path<TAB>type`).
fn docs_page(command: &Command, width: usize) -> Vec<PageLine> {
    use crate::info::docs;
    let runs = command.runs.clone().unwrap_or_default();
    let mut parts = runs.split('\t');
    let slug = parts.next().unwrap_or_default();
    let Some(set) = docs::catalog().into_iter().find(|s| s.slug == slug) else {
        return Vec::new();
    };
    match (command.kind, parts.next()) {
        (Kind::DocEntry, Some(path)) => {
            let entry = docs::Entry {
                name: command.name.clone(),
                path: path.to_owned(),
                kind: parts.next().unwrap_or_default().to_owned(),
            };
            docs::entry_page(&set, &entry, width)
        }
        _ => docs::set_page(&set, width),
    }
}

/// Whether `line` is a section's heading (INTERNET's too, coloured by the
/// connection): where the jumper (Space) stops on a page.
pub fn starts_a_section(line: &PageLine) -> bool {
    match line.style {
        Style::Heading => true,
        Style::Online | Style::Offline => line.text.starts_with(tr(HEADING_INTERNET)),
        _ => false,
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
            spans: Vec::new(),
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
        // as wide as the widest label, in the reader's language
        let widest = [
            FIELD_PACKAGE,
            FIELD_INSTALLED,
            FIELD_NEWEST,
            FIELD_UPDATE,
            FIELD_LOCATION,
        ]
        .iter()
        .map(|label| text::width(tr(label)) + 1)
        .max()
        .unwrap_or(0)
        .max(FIELD_LABEL_WIDTH);
        let label = format!("{PAGE_INDENT}{}", text::pad(label, widest));
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

/// The INTERNET section, put before MANUAL when the page is drawn: the dot
/// and the state, the reminder that this is anyone's text, then the cheat
/// sheet's lines as they came, behind a bar, cut at `INTERNET_MAX_LINES`.
pub fn internet_lines(
    name: &str,
    status: Status,
    answer: Option<&Answer>,
    width: usize,
) -> Vec<PageLine> {
    let mut page = Page {
        lines: Vec::new(),
        width,
    };
    let online = status == Status::Online;
    let style = if online {
        Style::Online
    } else {
        Style::Offline
    };
    page.blank();
    page.add(&format!("{} {INTERNET_DOT}", tr(HEADING_INTERNET)), style);
    let state = match status {
        Status::Online => tr(INTERNET_SOURCE).replace("{name}", name),
        Status::Offline => tr(INTERNET_OFFLINE).to_owned(),
        Status::Off => tr(INTERNET_DISABLED).to_owned(),
        Status::Checking => tr(INTERNET_CHECKING).to_owned(),
    };
    let state_style = if status == Status::Checking {
        Style::Dim
    } else {
        style
    };
    page.wrapped(&format!("{INTERNET_DOT} {state}"), state_style, PAGE_INDENT);
    if status == Status::Off {
        return page.lines; // turned off: nothing fetched is shown either
    }
    page.wrapped(tr(INTERNET_REMINDER), Style::Warning, PAGE_INDENT);
    match answer {
        Some(Answer::Lines(lines)) => {
            page.blank();
            let room = width.saturating_sub(text::width(INTERNET_GUTTER)).max(10);
            for (shown, line) in lines.iter().enumerate() {
                if shown >= INTERNET_MAX_LINES {
                    page.wrapped(
                        &tr(INTERNET_CUT).replace("{url}", &internet::url(name)),
                        Style::Dim,
                        PAGE_INDENT,
                    );
                    break;
                }
                for piece in wrap(line, room) {
                    page.add(&format!("{INTERNET_GUTTER}{piece}"), Style::Raw);
                }
            }
        }
        Some(Answer::Loading) => page.wrapped(tr(INTERNET_LOADING), Style::Dim, PAGE_INDENT),
        Some(Answer::NotFound) => {
            page.wrapped(
                &tr(INTERNET_NOTHING).replace("{name}", name),
                Style::Dim,
                PAGE_INDENT,
            );
        }
        Some(Answer::Failed) => page.wrapped(tr(INTERNET_FAILED), Style::Dim, PAGE_INDENT),
        None => {}
    }
    page.lines
}

/// A recipe's page: its title and categories, what it does, its STEPS in
/// order (numbered; risky ones marked), the commands it USES, its notes.
/// No version, links or manual: it is lore's, not a program's.
fn recipe(page: &mut Page, command: &Command, facts: &Facts) {
    let empty = Content::default();
    let content = facts.content.as_ref().unwrap_or(&empty);
    page.add(crate::utils::i18n::shown(command), Style::Title);
    if !content.categories.is_empty() {
        page.add(
            &format!("[{}]", content.categories.join(" ")),
            Style::Categories,
        );
    }
    page.blank();
    page.wrapped(tr(IS_RECIPE), Style::Dim, "");
    page.blank();
    for paragraph in &content.summary {
        page.wrapped(paragraph, Style::Text, "");
    }
    page.heading(tr(HEADING_STEPS));
    page.wrapped(tr(STEPS_HINT), Style::Dim, PAGE_INDENT);
    for (i, (typed, what)) in content.usage.iter().enumerate() {
        page.blank();
        page.wrapped(&format!("{}. {what}", i + 1), Style::Text, PAGE_INDENT);
        let (mark, style) = if danger::example(typed) {
            (DANGER_MARK, Style::Danger)
        } else {
            ("", Style::Code)
        };
        page.wrapped(
            &format!("{mark}{EXAMPLE_PROMPT}{typed}"),
            style,
            EXAMPLE_INDENT,
        );
    }
    let mut uses = recipes::uses(content);
    for name in &content.see_also {
        if !uses.contains(name) {
            uses.push(name.clone());
        }
    }
    if !uses.is_empty() {
        page.heading(tr(HEADING_USES));
        page.wrapped(tr(USES_HINT), Style::Dim, PAGE_INDENT);
        page.wrapped(&uses.join(", "), Style::Text, PAGE_INDENT);
    }
    if !content.notes.is_empty() {
        page.heading(tr(HEADING_NOTES));
        for note in &content.notes {
            page.wrapped(note, Style::Text, PAGE_INDENT);
        }
    }
}

/// A tool page: its title and categories, what the tool is, its TASKS in
/// groups (each `### ` title a heading the jumper stops at; risky tasks
/// marked), its notes and links. No version or manual: it is lore's.
fn tool(page: &mut Page, command: &Command, facts: &Facts) {
    let empty = Content::default();
    let content = facts.content.as_ref().unwrap_or(&empty);
    page.add(crate::utils::i18n::shown(command), Style::Title);
    if !content.categories.is_empty() {
        page.add(
            &format!("[{}]", content.categories.join(" ")),
            Style::Categories,
        );
    }
    page.blank();
    page.wrapped(tr(IS_TOOL), Style::Dim, "");
    page.blank();
    for (i, paragraph) in content.summary.iter().enumerate() {
        if i > 0 {
            page.blank();
        }
        page.wrapped(paragraph, Style::Text, "");
    }
    page.heading(tr(HEADING_TASKS));
    page.wrapped(tr(TASKS_HINT), Style::Dim, PAGE_INDENT);
    for (i, (typed, what)) in content.usage.iter().enumerate() {
        for (_, group) in content.groups.iter().filter(|(at, _)| *at == i) {
            page.heading(group);
        }
        page.blank();
        page.wrapped(what, Style::Text, PAGE_INDENT);
        let (mark, style) = if danger::example(typed) {
            (DANGER_MARK, Style::Danger)
        } else {
            ("", Style::Code)
        };
        page.wrapped(
            &format!("{mark}{EXAMPLE_PROMPT}{typed}"),
            style,
            EXAMPLE_INDENT,
        );
    }
    if !content.notes.is_empty() {
        page.heading(tr(HEADING_NOTES));
        for note in &content.notes {
            page.wrapped(note, Style::Text, PAGE_INDENT);
        }
    }
    if !content.see_also.is_empty() {
        page.heading(tr(HEADING_RELATED));
        page.wrapped(&content.see_also.join(", "), Style::Text, PAGE_INDENT);
    }
    if !content.links.is_empty() {
        page.heading(tr(HEADING_LINKS));
        page.wrapped(tr(LINK_HINT), Style::Dim, PAGE_INDENT);
        for (label, url) in &content.links {
            page.blank();
            page.wrapped(label, Style::Text, PAGE_INDENT);
            page.add(&format!("{PAGE_INDENT}{url}"), Style::Url); // whole, or the terminal opens half
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

/// The paths named under a man page's FILES heading (or any heading with
/// FILES in it: "CONFIGURATION FILES", "FILES AND DIRECTORIES"): each once,
/// in order, at most `FILES_MAX`.
pub fn files_in_man(lines: &[String]) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    let mut inside = false;
    for line in lines {
        let heading = !line.starts_with(' ') && !line.trim().is_empty();
        if heading {
            inside = line.split_whitespace().any(|word| word == HEADING_FILES);
            continue;
        }
        if !inside {
            continue;
        }
        for token in line.split_whitespace() {
            // "(/etc/foo.conf)," → /etc/foo.conf
            let path = token
                .trim_start_matches(['(', '[', '"', '\'', '`'])
                .trim_end_matches([')', ']', '"', '\'', '`', ',', ';', ':', '.']);
            let is_path = FILES_PATH_STARTS.iter().any(|start| path.starts_with(start))
                && path.len() > 1
                && !path.starts_with("//") // a URL's tail, not a path
                && !found.iter().any(|f| f == path);
            if is_path {
                found.push(path.to_owned());
            }
        }
    }
    found.truncate(FILES_MAX);
    found
}
