//! The quick keys: `g`/`G` top and bottom, `n`/`N` find on a page, `y`
//! copy at once, `o` the link box, `1`–`9`, `B`, `r` to a topic, and `?`
//! the help.

use crate::commands::Kind;
use crate::configs::{
    FIND_AT, FIND_NO_TEXT, FIND_NOT_HERE, HEADING_LINKS, LANGUAGE_NO_COPY, LINK_NONE, TOPIC_EMPTY,
};
use crate::info::page::Style;

use super::{App, Mode, Topic};

impl App {
    /// `g`: the first command of the list (its first header in view), or
    /// the top of the page.
    pub fn go_top(&mut self) {
        match self.mode {
            Mode::Info => self.scroll = 0,
            Mode::Browse => {
                if let Some(first) = self.rows.iter().position(|r| r.is_selectable()) {
                    self.list.select(Some(first));
                    *self.list.offset_mut() = 0;
                }
            }
            _ => {}
        }
    }

    /// `G`: the last command of the list, or the end of the page.
    pub fn go_bottom(&mut self) {
        match self.mode {
            Mode::Info => self.scroll = self.scroll_max,
            Mode::Browse => {
                if let Some(last) = self.rows.iter().rposition(|r| r.is_selectable()) {
                    self.list.select(Some(last));
                }
            }
            _ => {}
        }
    }

    /// `n` (forward) and `N` (back) on a page: scrolls to the next line
    /// holding the search bar's text, round at either end, and says which
    /// one of how many it is.
    pub fn find(&mut self, forward: bool) {
        let text = self.query.trim().to_owned();
        if text.is_empty() {
            self.note = Some(FIND_NO_TEXT.to_owned());
            return;
        }
        if self.matches.is_empty() {
            self.note = Some(FIND_NOT_HERE.replace("{text}", &text));
            return;
        }
        let at = if forward {
            self.matches
                .iter()
                .position(|&m| m > self.scroll)
                .unwrap_or(0)
        } else {
            self.matches
                .iter()
                .rposition(|&m| m < self.scroll)
                .unwrap_or(self.matches.len() - 1)
        };
        self.scroll = self.matches[at];
        self.note = Some(
            FIND_AT
                .replace("{text}", &text)
                .replace("{at}", &(at + 1).to_string())
                .replace("{count}", &self.matches.len().to_string()),
        );
    }

    /// `y` on a page: what to copy at once, the first thing `c` would list,
    /// else the page's name. A language's page gives nothing (its code is
    /// typed by hand) and says so.
    pub fn quick_copy(&mut self) -> Option<String> {
        let page = self.page.as_ref().filter(|_| self.mode == Mode::Info)?;
        if page.command.kind == Kind::Language {
            self.note = Some(LANGUAGE_NO_COPY.to_owned());
            return None;
        }
        let name = page.command.name.clone();
        Some(self.copyables().into_iter().next().unwrap_or(name))
    }

    /// The page's first link: the first under its LINKS heading (its own
    /// links come before the searches every page has). A URL in an example
    /// is not a link.
    pub fn first_link(&self) -> Option<String> {
        let page = self.page.as_ref()?;
        let links = page
            .lines
            .iter()
            .position(|l| l.style == Style::Heading && l.text == HEADING_LINKS)?;
        page.lines[links..].iter().find_map(|line| {
            let at = line.text.find("https://")?;
            let url = line.text[at..].split_whitespace().next()?;
            Some(url.to_owned())
        })
    }

    /// `o` on a page: its first link alone in a box, to Ctrl+click.
    pub fn open_link(&mut self) {
        if self.mode != Mode::Info {
            return;
        }
        match self.first_link() {
            Some(url) => {
                self.link = Some(url);
                self.overlay_from = self.mode;
                self.mode = Mode::Link;
            }
            None => self.note = Some(LINK_NONE.to_owned()),
        }
    }

    /// `?`: the help, over the list or the page.
    pub fn open_help(&mut self) {
        if matches!(self.mode, Mode::Browse | Mode::Info) {
            self.overlay_from = self.mode;
            self.help_scroll = 0;
            self.mode = Mode::Help;
        }
    }

    /// Scrolls the help by `lines`; ui.rs stops it at the end.
    pub fn scroll_help(&mut self, lines: isize) {
        self.help_scroll = self.help_scroll.saturating_add_signed(lines);
    }

    /// Closes the help or the link box, back to what was under it.
    pub fn close_overlay(&mut self) {
        if matches!(self.mode, Mode::Help | Mode::Link) {
            self.mode = self.overlay_from;
            self.link = None;
        }
    }

    /// `1`–`9`: the topic at that place in the bar; past the last, nothing.
    pub fn jump_topic(&mut self, place: usize) {
        if let Some(topic) = place.checked_sub(1).and_then(|i| self.topics.get(i)) {
            self.jump_to(*topic);
        }
    }

    /// `B` and `r`: that topic's list, from the list or a page (which
    /// closes); a topic with nothing in it yet says so.
    pub fn jump_to(&mut self, topic: Topic) {
        if !self.topics.contains(&topic) {
            self.note = Some(TOPIC_EMPTY.replace("{topic}", topic.name()));
            return;
        }
        if matches!(self.mode, Mode::Browse | Mode::Info) {
            self.mode = Mode::Browse;
            self.show_topic(topic);
        }
    }
}
