//! Draws the screen, top to bottom: search bar, the body, key bar. The body
//! is the list, or, with a page open, a narrow list beside the page. Reads
//! the state; changes only the list's scroll, the loaded page and its scroll.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, Paragraph};

use crate::app::{App, Mode};
use crate::commands::{Command, Kind};
use crate::compile_config::INFO_LIST_PERCENT;
use crate::info::danger::DANGER;
use crate::info::page::Style as PageStyle;
use crate::interface::rows::Row;
use crate::utils::text;

/// The categories after a name, and on a page under its name.
const CATEGORY: Style = Style::new().fg(Color::Magenta);

/// The page line that matched a search, after the name.
const SNIPPET: Style = Style::new().fg(Color::Cyan);

/// What can destroy data or stop the system.
const DANGER_STYLE: Style = Style::new().fg(Color::Red).add_modifier(Modifier::BOLD);

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
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED)); // the selected row
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
    let indent = if narrow { " " } else { "   " };
    let mut parts = vec![(format!("{indent}{}", command.name), Style::new())];
    if !narrow {
        if let Some(snippet) = snippet {
            parts.push((format!("  {snippet}"), SNIPPET));
        }
        if !categories.is_empty() {
            // `danger` in red, so it is seen before the command is run
            parts.push(("  [".to_owned(), CATEGORY));
            for (i, category) in categories.iter().enumerate() {
                let style = if category == DANGER {
                    DANGER_STYLE
                } else {
                    CATEGORY
                };
                let space = if i == 0 { "" } else { " " };
                parts.push((format!("{space}{category}"), style));
            }
            parts.push(("]".to_owned(), CATEGORY));
        }
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
    let title = format!(" {}  {}/{} ", page.name, app.scroll + 1, rows.max(1));
    let paragraph = Paragraph::new(lines)
        .block(Block::bordered().title(title))
        .scroll((u16::try_from(app.scroll).unwrap_or(u16::MAX), 0));
    frame.render_widget(paragraph, area);
}

/// How each kind of page line looks. Plain for now; restyled later.
fn style_of(style: PageStyle) -> Style {
    match style {
        PageStyle::Title | PageStyle::Heading => Style::new().add_modifier(Modifier::BOLD),
        PageStyle::Code => Style::new().fg(Color::Green),
        PageStyle::Dim => Style::new().add_modifier(Modifier::DIM),
        PageStyle::Url => Style::new().fg(Color::Cyan),
        PageStyle::Warning => Style::new().fg(Color::Yellow),
        PageStyle::Text => Style::new(),
        PageStyle::Categories => CATEGORY,
        PageStyle::Danger => DANGER_STYLE,
    }
}

/// The bottom line, nano-style: each key highlighted, then what it does.
fn draw_keys(frame: &mut Frame, mode: Mode, direct: bool, area: Rect) {
    let keys: &[(&str, &str)] = match mode {
        Mode::Browse => &[
            ("↑↓", "Move"),
            ("→/Enter", "Page"),
            ("/", "Search"),
            ("Esc", "Quit"),
            ("^C", "Quit"),
        ],
        Mode::Search => &[
            ("type", "Filter"),
            ("Bksp", "Erase"),
            ("↑↓", "Move"),
            ("→/Enter", "Page"),
            ("@", "Lore pages"),
            ("Esc", "Leave search"),
            ("^C", "Quit"),
        ],
        // opened as `lore NAME`: closing the page quits
        // opened as `lore NAME`: closing the page quits
        Mode::Info if direct => &[
            ("↑↓ PgUp/PgDn", "Scroll"),
            ("/", "Search"),
            ("←/Esc", "Quit"),
            ("^C", "Quit"),
        ],
        Mode::Info => &[
            ("↑↓ PgUp/PgDn", "Scroll"),
            ("/", "Search"),
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
