//! Draws the screen, top to bottom: search bar, list, key bar. Reads the
//! state; changes nothing but the list's scroll position.

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, Paragraph};

use crate::app::{App, Mode};
use crate::commands::{Command, Kind};
use crate::rows::Row;

/// Draws the whole screen; called once per loop in lib.rs.
pub fn draw(frame: &mut Frame, app: &mut App) {
    // 3 lines for the bordered search bar, 1 for the keys, the list gets the rest
    let [search, list, keys] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(frame.area());
    draw_search(frame, app, search);
    draw_list(frame, app, list);
    draw_keys(frame, app.mode, keys);
}

/// The search bar: a hint while browsing, the text and a cursor while searching.
fn draw_search(frame: &mut Frame, app: &App, area: Rect) {
    let searching = app.mode == Mode::Search;
    let block = Block::bordered()
        .title(" search ")
        .border_style(if searching {
            Style::new().fg(Color::Yellow) // yellow = keys go to the search bar
        } else {
            Style::new()
        });
    let text = if searching {
        Line::from(app.query.as_str())
    } else {
        Line::from("press / to search").dim()
    };
    frame.render_widget(Paragraph::new(text).block(block), area);
    if searching {
        // cursor right after the text; +1 skips the left border and the top one
        let typed = u16::try_from(app.query.chars().count()).unwrap_or(u16::MAX);
        frame.set_cursor_position(Position::new(area.x + 1 + typed, area.y + 1));
    }
}

/// The list of rows, with the count in its title, or a note when nothing matches.
fn draw_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let total = app.commands.len();
    let title = if app.query.is_empty() {
        format!(" commands ({total}) ")
    } else {
        format!(" commands ({} of {total}) ", app.shown_count())
    };
    let block = Block::bordered().title(title);
    if app.rows.is_empty() {
        let none = Line::from(format!(" no command matches \"{}\"", app.query)).dim();
        frame.render_widget(Paragraph::new(none).block(block), area);
        return;
    }
    let items: Vec<ListItem> = app
        .rows
        .iter()
        .map(|row| match row {
            Row::Header(section) => ListItem::new(Line::from(format!(" {section}")).bold().cyan()),
            Row::Command(command) => ListItem::new(command_line(command)),
        })
        .collect();
    let list = List::new(items)
        .block(block)
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED)); // the selected row
    // stateful: ratatui reads the selection and scrolls to keep it visible
    frame.render_stateful_widget(list, area, &mut app.list);
}

/// One command's row: the name, then (dimmed) what an alias runs and what it hides.
fn command_line(command: &Command) -> Line<'_> {
    let mut spans = vec![Span::raw(format!("   {}", command.name))];
    if let Some(runs) = &command.runs {
        spans.push(Span::raw(format!("  = {runs}")).dim());
    }
    if let Some(hidden) = command.hides {
        let what = match hidden {
            Kind::Alias => "another alias",
            Kind::Keyword => "a keyword",
            Kind::Function => "a function",
            Kind::Builtin => "a builtin",
            Kind::Program => "a program",
        };
        spans.push(Span::raw(format!("  (hides {what})")).dim());
    }
    Line::from(spans)
}

/// The bottom line, nano-style: each key highlighted, then what it does.
fn draw_keys(frame: &mut Frame, mode: Mode, area: Rect) {
    let keys: &[(&str, &str)] = match mode {
        Mode::Browse => &[
            ("↑↓", "Move"),
            ("/", "Search"),
            ("Esc", "Quit"),
            ("^C", "Quit"),
        ],
        Mode::Search => &[
            ("type", "Filter by name"),
            ("↑↓", "Move"),
            ("Esc", "Close search"),
            ("^C", "Quit"),
        ],
    };
    let mut spans = Vec::new();
    for (key, label) in keys {
        spans.push(Span::styled(
            *key,
            Style::new().add_modifier(Modifier::REVERSED),
        ));
        spans.push(Span::raw(format!(" {label}   ")));
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}
