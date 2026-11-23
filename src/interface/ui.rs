//! Draws the screen, top to bottom: search bar, the body, key bar. The body
//! is the list, or, with a page open, a narrow list beside the page. Reads
//! the state; changes only the list's scroll, the loaded page and its scroll.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, Paragraph};

use crate::app::{App, Mode};
use crate::commands::{Command, Kind};
use crate::compile_config::{
    DANGER, INFO_LIST_PERCENT, KEY_BAR_HEIGHT, KEY_BAR_NAME, KEYS_BROWSE, KEYS_PAGE,
    KEYS_PAGE_DIRECT, KEYS_SEARCH, LIST_HEADER_INDENT, LIST_INDENT, LIST_INDENT_NARROW, LIST_TITLE,
    LIST_TITLE_SEARCHING, NO_MATCH, SEARCH_BAR_HEIGHT, SEARCH_HINT, SEARCH_KEPT_HINT, SEARCH_TITLE,
    STYLE_CATEGORY, STYLE_CODE, STYLE_DANGER, STYLE_DIM, STYLE_HEADING, STYLE_KEY, STYLE_SEARCHING,
    STYLE_SECTION, STYLE_SELECTED, STYLE_SNIPPET, STYLE_URL, STYLE_WARNING,
};
use crate::info::page::Style as PageStyle;
use crate::interface::rows::Row;
use crate::utils::text;

/// Draws the whole screen; called once per loop in lib.rs.
pub fn draw(frame: &mut Frame, app: &mut App) {
    // the search bar on top, the keys at the bottom, the body gets the rest
    let [search, body, keys] = Layout::vertical([
        Constraint::Length(SEARCH_BAR_HEIGHT),
        Constraint::Min(0),
        Constraint::Length(KEY_BAR_HEIGHT),
    ])
    .areas(frame.area());
    draw_search(frame, app, search);
    if app.page_shown() {
        let [list, page] = Layout::horizontal([
            Constraint::Percentage(INFO_LIST_PERCENT),
            Constraint::Min(0),
        ])
        .areas(body);
        draw_list(frame, app, list, true);
        draw_page(frame, app, page);
    } else {
        draw_list(frame, app, body, false);
    }
    draw_keys(frame, app.mode, app.direct, keys);
}

/// The search bar: its text (kept after leaving it), a cursor while typing,
/// and a hint when it is empty.
fn draw_search(frame: &mut Frame, app: &App, area: Rect) {
    let searching = app.mode == Mode::Search;
    let block = Block::bordered()
        .title(SEARCH_TITLE)
        .border_style(if searching {
            STYLE_SEARCHING // keys go to the search bar
        } else {
            Style::new()
        });
    let text = match (searching, app.query.is_empty()) {
        (true, _) => Line::from(app.query.as_str()),
        (false, true) => Line::from(Span::styled(SEARCH_HINT, STYLE_DIM)),
        // left with text in it: the filter still applies, `/` goes back to it
        (false, false) => Line::from(vec![
            Span::raw(app.query.as_str()),
            Span::styled(SEARCH_KEPT_HINT, STYLE_DIM),
        ]),
    };
    frame.render_widget(Paragraph::new(text).block(block), area);
    if searching {
        // cursor right after the text; +1 skips the left border and the top one
        let typed = u16::try_from(text::width(&app.query)).unwrap_or(u16::MAX);
        frame.set_cursor_position(Position::new(area.x + 1 + typed, area.y + 1));
    }
}

/// The list of rows, or a note when nothing matches. `narrow`: beside a page,
/// with names only and no title; names that do not fit end in `...`.
fn draw_list(frame: &mut Frame, app: &mut App, area: Rect, narrow: bool) {
    let total = app.commands.len();
    let block = if narrow {
        Block::bordered() // a title would not fit in a tenth of the screen
    } else if app.query.is_empty() {
        Block::bordered().title(LIST_TITLE.replace("{total}", &total.to_string()))
    } else {
        Block::bordered().title(
            LIST_TITLE_SEARCHING
                .replace("{shown}", &app.shown_count().to_string())
                .replace("{total}", &total.to_string()),
        )
    };
    if app.rows.is_empty() {
        let none = Line::from(Span::styled(
            NO_MATCH.replace("{query}", &app.query),
            STYLE_DIM,
        ));
        frame.render_widget(Paragraph::new(none).block(block), area);
        return;
    }
    let width = usize::from(area.width.saturating_sub(2)); // inside the borders
    let items: Vec<ListItem> = app
        .rows
        .iter()
        .map(|row| match row {
            Row::Header(section) => {
                let indent = if narrow { "" } else { LIST_HEADER_INDENT };
                let header = format!("{indent}{section}");
                ListItem::new(Line::from(Span::styled(
                    text::fit(&header, width),
                    STYLE_SECTION,
                )))
            }
            Row::Command(command) => {
                let categories = app
                    .docs
                    .get(&command.name)
                    .map(|d| d.categories.as_slice())
                    .unwrap_or_default();
                let snippet = app.snippets.get(&command.name).map(String::as_str);
                ListItem::new(command_line(command, snippet, categories, width, narrow))
            }
        })
        .collect();
    let list = List::new(items)
        .block(block)
        .highlight_style(STYLE_SELECTED);
    // stateful: ratatui reads the selection and scrolls to keep it visible
    frame.render_stateful_widget(list, area, &mut app.list);
}

/// One command's row, cut to `width`: the name, then (not when `narrow`,
/// where the name is all there is) the page line that matched the search,
/// if any, then its `[categories]`, what an alias runs and what it hides.
fn command_line(
    command: &Command,
    snippet: Option<&str>,
    categories: &[String],
    width: usize,
    narrow: bool,
) -> Line<'static> {
    let indent = if narrow {
        LIST_INDENT_NARROW
    } else {
        LIST_INDENT
    };
    let mut parts = vec![(format!("{indent}{}", command.name), Style::new())];
    if !narrow {
        if let Some(snippet) = snippet {
            parts.push((format!("  {snippet}"), STYLE_SNIPPET));
        }
        if !categories.is_empty() {
            // `danger` in red, so it is seen before the command is run
            parts.push(("  [".to_owned(), STYLE_CATEGORY));
            for (i, category) in categories.iter().enumerate() {
                let style = if category == DANGER {
                    STYLE_DANGER
                } else {
                    STYLE_CATEGORY
                };
                let space = if i == 0 { "" } else { " " };
                parts.push((format!("{space}{category}"), style));
            }
            parts.push(("]".to_owned(), STYLE_CATEGORY));
        }
        let dim = STYLE_DIM;
        if let Some(runs) = &command.runs {
            parts.push((format!("  = {runs}"), dim));
        }
        if let Some(hidden) = command.hides {
            let what = match hidden {
                Kind::Alias => "another alias",
                Kind::Keyword => "a keyword",
                Kind::Function => "a function",
                Kind::Builtin => "a builtin",
                Kind::Program => "a program",
                Kind::Subcommand => "a subcommand",
            };
            parts.push((format!("  (hides {what})"), dim));
        }
    }
    // the parts in order until the width runs out; the one that overflows
    // ends in `...` and the rest are dropped
    let mut left = width;
    let mut spans = Vec::new();
    for (part, style) in parts {
        let length = text::width(&part);
        if length <= left {
            left -= length;
            spans.push(Span::styled(part, style));
        } else {
            spans.push(Span::styled(text::fit(&part, left), style));
            break;
        }
    }
    Line::from(spans)
}

/// The selected command's page (info::page). It is laid out for this
/// panel's width already, so every line is one row: no wrapping here, which
/// also keeps each URL whole for the terminal to open.
fn draw_page(frame: &mut Frame, app: &mut App, area: Rect) {
    let width = area.width.saturating_sub(2); // inside the borders
    let height = usize::from(area.height.saturating_sub(2));
    // while searching from a page, it stays the one being read: the chosen
    // command's page loads when it is opened, not on every key typed
    if app.mode == Mode::Info {
        app.load_page(width);
    } else {
        app.relayout_page(width); // the same page, fitted to a new width
    }
    let Some(page) = &app.page else {
        return; // nothing selected: open_info does not allow it, but be safe
    };
    let rows = page.lines.len();
    app.scroll = app.scroll.min(rows.saturating_sub(height));
    let lines: Vec<Line> = page
        .lines
        .iter()
        .map(|line| Line::styled(line.text.as_str(), style_of(line.style)))
        .collect();
    let title = format!(
        " {}  {}/{} ",
        page.command.name,
        app.scroll + 1,
        rows.max(1)
    );
    let paragraph = Paragraph::new(lines)
        .block(Block::bordered().title(title))
        .scroll((u16::try_from(app.scroll).unwrap_or(u16::MAX), 0));
    frame.render_widget(paragraph, area);
}

/// How each kind of page line looks. Plain for now; restyled later.
fn style_of(style: PageStyle) -> Style {
    match style {
        PageStyle::Title | PageStyle::Heading => STYLE_HEADING,
        PageStyle::Code => STYLE_CODE,
        PageStyle::Dim => STYLE_DIM,
        PageStyle::Url => STYLE_URL,
        PageStyle::Warning => STYLE_WARNING,
        PageStyle::Text => Style::new(),
        PageStyle::Categories => STYLE_CATEGORY,
        PageStyle::Danger => STYLE_DANGER,
    }
}

/// The bottom line, nano-style: each key highlighted, then what it does.
fn draw_keys(frame: &mut Frame, mode: Mode, direct: bool, area: Rect) {
    let keys = match mode {
        Mode::Browse => KEYS_BROWSE,
        Mode::Search => KEYS_SEARCH,
        Mode::Info if direct => KEYS_PAGE_DIRECT, // opened as `lore NAME`
        Mode::Info => KEYS_PAGE,
    };
    // the app's name first, on every line of keys; styled later
    let mut spans = vec![Span::raw(format!("{KEY_BAR_NAME}   "))];
    for (key, label) in keys {
        spans.push(Span::styled(*key, STYLE_KEY));
        spans.push(Span::raw(format!(" {label}   ")));
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}
