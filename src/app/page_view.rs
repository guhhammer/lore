//! The page on show: opening, laying out, scrolling and jumping through it,
//! and copying from it (`c`).

use crate::commands::{Command, Kind};
use crate::configs::{
    COPIED, COPY_MORE_LINES, COPY_ONE_MORE_LINE, LANGUAGE_NO_COPY, NOTHING_TO_COPY, RECENT_MAX,
};
use crate::info::content;
use crate::info::docs;
use crate::info::page::{self};
use crate::utils::i18n::tr;
use crate::utils::manual;

use super::{App, Mode, Page};

impl App {
    /// True while the screen is split: a page open, or a search from one.
    pub fn page_shown(&self) -> bool {
        match self.mode {
            Mode::Info => true,
            Mode::Search => self.beside_page,
            // the picker sits over whatever was on screen
            Mode::Palette => self.picker_from(Mode::Info),
            Mode::Copy => true,
            Mode::Help | Mode::Link => self.overlay_from == Mode::Info,
            Mode::About | Mode::Choose => false,
            Mode::Browse => false,
        }
    }

    /// Asks the internet for the page on show (after it was turned on).
    pub fn ask_internet(&self) {
        if let Some(page) = self.page.as_ref().filter(|p| !p.command.kind.is_lores()) {
            self.internet.request(&page.command.name);
        }
    }

    /// The examples of the page on show, as typed (`tar -cf a.tar dir`).
    pub fn examples(&self) -> Vec<&str> {
        self.page
            .as_ref()
            .and_then(|page| page.facts.content.as_ref())
            .map(|content| {
                content
                    .usage
                    .iter()
                    .map(|(typed, _)| typed.as_str())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// What `c` can copy from the page on show: a doc's code blocks, else
    /// the page's examples, only their text. A language's page has nothing
    /// to copy: its code is there to be typed by hand, which is how it is
    /// learnt.
    pub fn copyables(&self) -> Vec<String> {
        let Some(page) = &self.page else {
            return Vec::new();
        };
        match page.command.kind {
            Kind::Language => Vec::new(),
            Kind::DocEntry => page
                .command
                .runs
                .as_deref()
                .map(crate::info::docs::code_blocks)
                .unwrap_or_default(),
            _ => self.examples().into_iter().map(str::to_owned).collect(),
        }
    }

    /// Opens the list of what the page has to copy; on a page with
    /// nothing, says so instead.
    pub fn open_copy(&mut self) {
        if self.mode != Mode::Info {
            return;
        }
        self.copies = self.copyables();
        let language = self
            .page
            .as_ref()
            .is_some_and(|p| p.command.kind == Kind::Language);
        if language {
            self.note = Some(tr(LANGUAGE_NO_COPY).to_owned());
        } else if self.copies.is_empty() {
            self.note = Some(tr(NOTHING_TO_COPY).to_owned());
        } else {
            self.example = 0;
            self.mode = Mode::Copy;
        }
    }

    /// Selects the piece `steps` away, wrapping round.
    pub fn step_example(&mut self, steps: isize) {
        let count = self.copies.len().max(1);
        let forward = steps.rem_euclid(isize::try_from(count).unwrap_or(isize::MAX));
        self.example = (self.example + usize::try_from(forward).unwrap_or(0)) % count;
    }

    /// The piece selected in the copy list.
    pub fn chosen_example(&self) -> Option<String> {
        self.copies.get(self.example).cloned()
    }

    /// Back to the page; `copied` is the example sent to the clipboard, for
    /// the key bar to say so.
    pub fn close_copy(&mut self, copied: Option<&str>) {
        if self.mode == Mode::Copy {
            self.mode = Mode::Info;
        }
        self.note = copied.map(|text| tr(COPIED).replace("{text}", &copy_label(text)));
    }

    /// Opens the selected command's page, and puts it first among the
    /// recent ones; does nothing on an empty list.
    pub fn open_info(&mut self) {
        let Some(command) = self.selected().cloned() else {
            return;
        };
        // a downloaded set opens as the list of its pages, not as a page
        if command.kind == Kind::Docset {
            let slug = command.runs.clone().unwrap_or_default();
            if docs::is_downloaded(&slug)
                && let Some(set) = self.docsets.iter().find(|s| s.slug == slug).cloned()
            {
                self.docset = Some((set, docs::entries(&slug)));
                self.query.clear();
                self.mode = Mode::Browse;
                self.refilter();
                return;
            }
        }
        let name = command.name;
        self.mode = Mode::Info;
        self.scroll = 0;
        self.beside_page = false;
        self.recent.retain(|n| *n != name);
        self.recent.insert(0, name);
        self.recent.truncate(RECENT_MAX);
        self.rebuild_keeping_selection();
    }

    /// Closes the page, back to the list.
    pub fn close_info(&mut self) {
        self.mode = Mode::Browse;
    }

    /// Tab on a page: scrolls to the next section's heading; at the last
    /// one (or the end of the page), back to the top.
    pub fn next_section(&mut self) {
        let next = self.sections.iter().copied().find(|&s| s > self.scroll);
        self.scroll = match next {
            Some(line) if self.scroll < self.scroll_max => line.min(self.scroll_max),
            _ => 0,
        };
    }

    /// Shift+Tab on a page: the section before; from the top, the last one.
    pub fn previous_section(&mut self) {
        let back = self.sections.iter().copied().rfind(|&s| s < self.scroll);
        self.scroll = match back {
            Some(line) => line,
            None => self.sections.last().map_or(0, |&s| s.min(self.scroll_max)),
        };
    }

    /// Scrolls the page by `lines` (negative: up); `ui` stops it at the end.
    pub fn scroll_by(&mut self, lines: isize) {
        self.scroll = self.scroll.saturating_add_signed(lines);
    }

    /// Makes `page` the selected command's, `width` columns wide. Looks the
    /// command up only when it changed, and runs `man` again only when the
    /// command or the width changed.
    pub fn load_page(&mut self, width: u16) {
        let Some(command) = self.selected().cloned() else {
            return;
        };
        self.show_page(command, width);
    }

    /// Lays the page on show out again at `width`, if it is not already:
    /// while searching from a page the selection is the search's, but the
    /// page shown is still the one being read.
    pub fn relayout_page(&mut self, width: u16) {
        if let Some(command) = self.page.as_ref().map(|p| p.command.clone()) {
            self.show_page(command, width);
        }
    }

    /// Makes `page` `command`'s at `width`; see `load_page`.
    pub(super) fn show_page(&mut self, command: Command, width: u16) {
        let facts = match self.page.take() {
            Some(page) if page.command.name == command.name => {
                if page.width == width {
                    self.page = Some(page); // nothing changed
                    return;
                }
                page.facts // same command, new width: only the layout changes
            }
            _ => {
                self.scroll = 0; // a new page starts at its top
                if !command.kind.is_lores() {
                    self.internet.request(&command.name); // in the background
                }
                page::facts(&command)
            }
        };
        // `man git-commit`; a recipe has no manual
        let man = (!command.kind.is_lores())
            .then(|| manual::page(&content::page_key(&command.name), width))
            .flatten();
        let lines = page::render(&command, &facts, man.as_deref(), usize::from(width));
        self.page = Some(Page {
            command,
            width,
            facts,
            lines,
        });
    }
}

/// How the copy list and the key bar name a piece: its first line, and
/// how many lines follow when there are more.
pub fn copy_label(text: &str) -> String {
    let mut lines = text.lines();
    let first = lines.next().unwrap_or_default().trim_end();
    match lines.count() {
        0 => first.to_owned(),
        1 => format!("{first}{}", tr(COPY_ONE_MORE_LINE)),
        more => format!(
            "{first}{}",
            tr(COPY_MORE_LINES).replace("{n}", &more.to_string())
        ),
    }
}
