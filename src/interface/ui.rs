//! Draws the screen, top to bottom: search bar, the body, key bar. The body
//! is the list, or, with a page open, a narrow list beside the page. Reads
//! the state; changes only the list's scroll, the loaded page and its scroll.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, Paragraph, Wrap};

use crate::app::{App, Mode};
use crate::commands::{Command, Kind};
use crate::compile_config::INFO_LIST_PERCENT;
use crate::interface::rows::Row;
use crate::utils::manual;
use crate::utils::text;

/// Draws the whole screen; called once per loop in lib.rs.
pub fn draw(frame: &mut Frame, app: &mut App) {
    // 3 lines for the bordered search bar, 1 for the keys, the body gets the rest
    let [search, body, keys] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(frame.area());
    draw_search(frame, app, search);
    if app.mode == Mode::Info {
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
    draw_keys(frame, app.mode, keys);
}

/// The search bar: its text (kept after leaving it), a cursor while typing,
/// and a hint when it is empty.
fn draw_search(frame: &mut Frame, app: &App, area: Rect) {
    let searching = app.mode == Mode::Search;
    let block = Block::bordered()
        .title(" search ")
        .border_style(if searching {
            Style::new().fg(Color::Yellow) // yellow = keys go to the search bar
        } else {
            Style::new()
        });
    let text = match (searching, app.query.is_empty()) {
        (true, _) => Line::from(app.query.as_str()),
        (false, true) => Line::from("press / to search").dim(),
        // left with text in it: the filter still applies, `/` goes back to it
        (false, false) => Line::from(vec![
            Span::raw(app.query.as_str()),
            Span::raw("   (/ to edit)").dim(),
        ]),
    };
    frame.render_widget(Paragraph::new(text).block(block), area);
    if searching {
        // cursor right after the text; +1 skips the left border and the top one
        let typed = u16::try_from(app.query.chars().count()).unwrap_or(u16::MAX);
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
        Block::bordered().title(format!(" commands ({total}) "))
    } else {
        Block::bordered().title(format!(" commands ({} of {total}) ", app.shown_count()))
    };
    if app.rows.is_empty() {
        let none = Line::from(format!(" no command matches \"{}\"", app.query)).dim();
        frame.render_widget(Paragraph::new(none).block(block), area);
        return;
    }
    let width = usize::from(area.width.saturating_sub(2)); // inside the borders
    let items: Vec<ListItem> = app
        .rows
        .iter()
        .map(|row| match row {
            Row::Header(section) => {
                let header = if narrow {
                    format!("{section}")
                } else {
                    format!(" {section}")
                };
                ListItem::new(Line::from(text::fit(&header, width)).bold().cyan())
            }
            Row::Command(command) => ListItem::new(command_line(command, width, narrow)),
        })
        .collect();
    let list = List::new(items)
        .block(block)
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED)); // the selected row
    // stateful: ratatui reads the selection and scrolls to keep it visible
    frame.render_stateful_widget(list, area, &mut app.list);
}

/// One command's row, cut to `width`: the name, then (dimmed, not when
/// `narrow`) what an alias runs and what it hides.
fn command_line(command: &Command, width: usize, narrow: bool) -> Line<'static> {
    let indent = if narrow { " " } else { "   " };
    let mut parts = vec![(format!("{indent}{}", command.name), Style::new())];
    if !narrow {
        let dim = Style::new().add_modifier(Modifier::DIM);
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
            };
            parts.push((format!("  (hides {what})"), dim));
        }
    }
    // the parts in order until the width runs out; the one that overflows
    // ends in `...` and the rest are dropped
    let mut left = width;
    let mut spans = Vec::new();
    for (part, style) in parts {
        let length = part.chars().count();
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

/// The selected command's manual page, or the reasons there is none. The
/// page is laid out by `man` at this panel's width, so it fits as it is;
/// wrapping only catches what `man` could not narrow (wide tables).
fn draw_page(frame: &mut Frame, app: &mut App, area: Rect) {
    let width = area.width.saturating_sub(2); // inside the borders
    let height = usize::from(area.height.saturating_sub(2));
    app.load_page(width);
    let Some(page) = &app.page else {
        return; // nothing selected: open_info does not allow it, but be safe
    };
    let lines: Vec<Line> = match &page.lines {
        Some(lines) => lines.iter().map(|l| Line::from(l.as_str())).collect(),
        None => manual::NO_PAGE.lines().map(Line::from).collect(),
    };
    // rows the lines take once wrapped, so scrolling stops at the end
    let rows: usize = lines
        .iter()
        .map(|l| l.width().div_ceil(usize::from(width.max(1))).max(1))
        .sum();
    app.scroll = app.scroll.min(rows.saturating_sub(height));
    let title = format!(" {}  {}/{} ", page.name, app.scroll + 1, rows.max(1));
    let paragraph = Paragraph::new(lines)
        .block(Block::bordered().title(title))
        .wrap(Wrap { trim: false }) // trim would eat man's indentation
        .scroll((u16::try_from(app.scroll).unwrap_or(u16::MAX), 0));
    frame.render_widget(paragraph, area);
}

/// The bottom line, nano-style: each key highlighted, then what it does.
fn draw_keys(frame: &mut Frame, mode: Mode, area: Rect) {
    let keys: &[(&str, &str)] = match mode {
        Mode::Browse => &[
            ("↑↓", "Move"),
            ("→/Enter", "Manual"),
            ("/", "Search"),
            ("Esc", "Quit"),
            ("^C", "Quit"),
        ],
        Mode::Search => &[
            ("type", "Filter"),
            ("Bksp", "Erase"),
            ("↑↓", "Move"),
            ("→/Enter", "Manual"),
            ("Esc", "Leave search"),
            ("^C", "Quit"),
        ],
        Mode::Info => &[
            ("↑↓ PgUp/PgDn", "Scroll"),
            ("←/Esc", "Back"),
            ("^C", "Quit"),
        ],
    };
    // the app's name first, on every line of keys; styled later
    let mut spans = vec![Span::raw("lore   ")];
    for (key, label) in keys {
        spans.push(Span::styled(
            *key,
            Style::new().add_modifier(Modifier::REVERSED),
        ));
        spans.push(Span::raw(format!(" {label}   ")));
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}
