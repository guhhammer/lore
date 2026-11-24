//! Draws the screen, top to bottom: search bar, the body, key bar. The body
//! is the list, or, with a page open, a narrow list beside the page. Reads
//! the state; changes only the list's scroll, the loaded page and its scroll.

use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Position, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, List, ListItem, ListState, Paragraph};

use crate::app::{App, Mode};
use crate::commands::{Command, Kind};
use crate::compile_config::{
    ABOUT_COMMANDS, ABOUT_FIELDS, ABOUT_HISTORY_OFF, ABOUT_HISTORY_ON, ABOUT_KEPT,
    ABOUT_LABEL_WIDTH, ABOUT_LICENCE, ABOUT_MANUAL, ABOUT_PAGES, ABOUT_PALETTE, ABOUT_SOURCE,
    ABOUT_TAGLINE, ABOUT_TITLE, BOOKMARK_MARK, BOOKMARKS_FILE, COPY_PICKER_MARGIN, COPY_TITLE,
    DANGER, INFO_LIST_PERCENT, KEY_BAR_HEIGHT, KEYS_ABOUT, KEYS_BROWSE, KEYS_COPY, KEYS_PAGE,
    KEYS_PAGE_DIRECT, KEYS_PALETTE, KEYS_SEARCH, LIST_HEADER_INDENT, LIST_INDENT,
    LIST_INDENT_NARROW, LIST_TITLE, LIST_TITLE_SEARCHING, LOGO, LOGO_GAP, LORE_ORANGE, NO_MATCH,
    PALETTE_DIR, PALETTE_NOT_SAVED, PALETTE_PICKER_MARGIN, PALETTE_TITLE, SEARCH_BAR_HEIGHT,
    SEARCH_HINT, SEARCH_KEPT_HINT, SEARCH_TITLE, UNUSED_MARK,
};
use crate::info::content;
use crate::info::page::Style as PageStyle;
use crate::interface::palette::Palette;
use crate::interface::paths;
use crate::interface::rows::Row;
use crate::settings;
use crate::utils::text;

/// Draws the whole screen; called once per loop in lib.rs.
pub fn draw(frame: &mut Frame, app: &mut App) {
    // the palette's background and text under everything drawn after
    frame.render_widget(Block::new().style(app.palette().base()), frame.area());
    // the search bar on top, the keys at the bottom, the body gets the rest
    // the logo, 4 rows, beside the search bar, which sits on its last 3
    let logo_height = u16::try_from(LOGO.len()).unwrap_or(u16::MAX);
    let [top, body, keys] = Layout::vertical([
        Constraint::Length(logo_height.max(SEARCH_BAR_HEIGHT)),
        Constraint::Min(0),
        Constraint::Length(KEY_BAR_HEIGHT),
    ])
    .areas(frame.area());
    let logo_width = LOGO.iter().map(|l| text::width(l)).max().unwrap_or(0);
    let logo_width = u16::try_from(logo_width).unwrap_or(u16::MAX);
    let [logo, _, search] = Layout::horizontal([
        Constraint::Length(logo_width),
        Constraint::Length(LOGO_GAP),
        Constraint::Min(0),
    ])
    .areas(top);
    let [_, search] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(SEARCH_BAR_HEIGHT)]).areas(search);
    let orange = Style::new().fg(LORE_ORANGE);
    let logo_lines: Vec<Line> = LOGO.iter().map(|l| Line::styled(*l, orange)).collect();
    frame.render_widget(Paragraph::new(logo_lines), logo);
    draw_search(frame, app, search);
    if app.mode == Mode::About {
        draw_about(frame, app, body);
    } else if app.page_shown() {
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
    match &app.note {
        // `copied: …`, in place of the keys until the next key
        Some(note) => frame.render_widget(
            Paragraph::new(Line::styled(note.as_str(), app.palette().accent())),
            keys,
        ),
        None => draw_keys(frame, app.mode, app.direct, app.palette(), keys),
    }
    if app.mode == Mode::Palette {
        draw_picker(frame, app, body);
    }
    if app.mode == Mode::Copy {
        draw_copy(frame, app, body);
    }
}

/// The about screen: what lore is, and what it has and keeps here (the
/// logo is already on top, as on every screen).
fn draw_about(frame: &mut Frame, app: &App, area: Rect) {
    let muted = app.palette().muted();
    let home = paths::home();
    // `~/.config/lore/config`: shorter, and what the user would type
    let tilde = |path: Option<std::path::PathBuf>| {
        path.map_or_else(String::new, |path| {
            match home.as_deref().and_then(|h| path.strip_prefix(h).ok()) {
                Some(rest) => format!("~/{}", rest.display()),
                None => path.display().to_string(),
            }
        })
    };
    let config = settings::path();
    let palettes_dir = config
        .as_deref()
        .and_then(|c| c.parent())
        .map(|d| d.join(PALETTE_DIR));
    let kept =
        settings::list_path(BOOKMARKS_FILE).and_then(|p| p.parent().map(|d| d.to_path_buf()));
    let count = |n: usize| n.to_string();
    let values = [
        ABOUT_COMMANDS
            .replace("{total}", &count(app.commands.len()))
            .replace("{pages}", &count(app.docs.len())),
        ABOUT_PAGES
            .replace("{pages}", &count(app.pages.len()))
            .replace("{languages}", &count(content::translated_languages())),
        ABOUT_PALETTE
            .replace("{name}", &app.palette().name)
            .replace("{count}", &count(app.palettes.len())),
        tilde(config),
        tilde(palettes_dir),
        format!(
            "{}  ({})",
            tilde(kept),
            ABOUT_KEPT
                .replace("{bookmarks}", &count(app.bookmarks.len()))
                .replace("{recent}", &count(app.recent.len()))
        ),
        if app.uses.is_empty() {
            ABOUT_HISTORY_OFF.to_owned()
        } else {
            ABOUT_HISTORY_ON.replace("{seen}", &count(app.uses.len()))
        },
        ABOUT_LICENCE.to_owned(),
        ABOUT_SOURCE.to_owned(),
        ABOUT_MANUAL.to_owned(),
    ];
    let mut lines: Vec<Line> = vec![Line::raw("")];
    lines.push(Line::raw(format!(
        "  {}",
        ABOUT_TAGLINE.replace("{version}", env!("CARGO_PKG_VERSION"))
    )));
    lines.push(Line::raw(""));
    for (label, value) in ABOUT_FIELDS.iter().zip(values) {
        let style = if value.starts_with("https://") {
            app.palette().accent()
        } else {
            Style::new()
        };
        lines.push(Line::from(vec![
            Span::styled(format!("  {label:<ABOUT_LABEL_WIDTH$}"), muted),
            Span::styled(value, style),
        ]));
    }
    let block = Block::bordered().title(ABOUT_TITLE);
    frame.render_widget(Paragraph::new(lines).block(block), area);
}

/// The page's examples, over the page, to copy one: each as typed, cut to
/// fit, the chosen one selected.
fn draw_copy(frame: &mut Frame, app: &App, area: Rect) {
    let examples = app.examples();
    let room = area.width.saturating_sub(2 * COPY_PICKER_MARGIN);
    let inside = usize::from(room.saturating_sub(4)); // borders and margins
    let longest = examples
        .iter()
        .map(|e| text::width(e) + 4)
        .max()
        .unwrap_or(0);
    let width = u16::try_from(longest).unwrap_or(u16::MAX).min(room);
    let height = u16::try_from(examples.len() + 2).unwrap_or(u16::MAX); // + borders
    let [middle] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    let [popup] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(middle);
    let items: Vec<ListItem> = examples
        .iter()
        .map(|e| {
            ListItem::new(Span::styled(
                text::fit(&format!(" {e}"), inside),
                app.palette().code(),
            ))
        })
        .collect();
    let list = List::new(items)
        .block(
            Block::bordered()
                .title(COPY_TITLE)
                .border_style(app.palette().active()),
        )
        .style(app.palette().base())
        .highlight_style(app.palette().selected());
    let mut state = ListState::default().with_selected(Some(app.example));
    frame.render_widget(Clear, popup);
    frame.render_stateful_widget(list, popup, &mut state);
}

/// The palette picker, in the middle of the body: every palette's name, the
/// one being tried selected. The screen behind is already in its colours.
fn draw_picker(frame: &mut Frame, app: &App, area: Rect) {
    let longest = app
        .palettes
        .iter()
        .map(|p| text::width(&p.name))
        .max()
        .unwrap_or(0);
    let width = u16::try_from(longest)
        .unwrap_or(u16::MAX)
        .saturating_add(PALETTE_PICKER_MARGIN);
    let height = u16::try_from(app.palettes.len() + 2).unwrap_or(u16::MAX); // + borders
    let [middle] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    let [popup] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(middle);
    let title = match app.picker.as_ref().and_then(|p| p.note.as_deref()) {
        Some(why) => Line::styled(
            PALETTE_NOT_SAVED.replace("{why}", why),
            app.palette().danger(),
        ),
        None => Line::from(PALETTE_TITLE),
    };
    let items: Vec<ListItem> = app
        .palettes
        .iter()
        .map(|p| ListItem::new(format!(" {}", p.name)))
        .collect();
    let list = List::new(items)
        .block(
            Block::bordered()
                .title(title)
                .border_style(app.palette().active()),
        )
        .style(app.palette().base())
        .highlight_style(app.palette().selected());
    let mut state = ListState::default().with_selected(Some(app.chosen));
    frame.render_widget(Clear, popup); // the list or page under it, blanked
    frame.render_stateful_widget(list, popup, &mut state);
}

/// The search bar: its text (kept after leaving it), a cursor while typing,
/// and a hint when it is empty.
fn draw_search(frame: &mut Frame, app: &App, area: Rect) {
    let searching = app.mode == Mode::Search;
    let block = Block::bordered()
        .title(SEARCH_TITLE)
        .border_style(if searching {
            app.palette().active() // keys go to the search bar
        } else {
            Style::new()
        });
    let text = match (searching, app.query.is_empty()) {
        (true, _) => Line::from(app.query.as_str()),
        (false, true) => Line::from(Span::styled(SEARCH_HINT, app.palette().muted())),
        // left with text in it: the filter still applies, `/` goes back to it
        (false, false) => Line::from(vec![
            Span::raw(app.query.as_str()),
            Span::styled(SEARCH_KEPT_HINT, app.palette().muted()),
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
            app.palette().muted(),
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
                    app.palette().section(),
                )))
            }
            Row::Command(command) => {
                let categories = app
                    .docs
                    .get(&command.name)
                    .map(|d| d.categories.as_slice())
                    .unwrap_or_default();
                let snippet = app.snippets.get(&command.name).map(String::as_str);
                ListItem::new(command_line(
                    command,
                    snippet,
                    categories,
                    width,
                    narrow,
                    app.is_unused(&command.name),
                    app.palette(),
                ))
            }
        })
        .collect();
    let list = List::new(items)
        .block(block)
        .highlight_style(app.palette().selected());
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
    unused: bool,
    palette: &Palette,
) -> Line<'static> {
    let indent = if narrow {
        LIST_INDENT_NARROW
    } else {
        LIST_INDENT
    };
    let mut parts = vec![(format!("{indent}{}", command.name), Style::new())];
    if !narrow {
        if unused {
            parts.push((UNUSED_MARK.to_owned(), palette.muted())); // your history never ran it
        }
        if let Some(snippet) = snippet {
            parts.push((format!("  {snippet}"), palette.accent()));
        }
        if !categories.is_empty() {
            // `danger` in red, so it is seen before the command is run
            parts.push(("  [".to_owned(), palette.category()));
            for (i, category) in categories.iter().enumerate() {
                let style = if category == DANGER {
                    palette.danger()
                } else {
                    palette.category()
                };
                let space = if i == 0 { "" } else { " " };
                parts.push((format!("{space}{category}"), style));
            }
            parts.push(("]".to_owned(), palette.category()));
        }
        let dim = palette.muted();
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
        .map(|line| Line::styled(line.text.as_str(), style_of(line.style, app.palette())))
        .collect();
    let mark = if app.is_bookmarked(&page.command.name) {
        BOOKMARK_MARK
    } else {
        ""
    };
    let title = format!(
        " {mark}{}  {}/{} ",
        page.command.name,
        app.scroll + 1,
        rows.max(1)
    );
    let paragraph = Paragraph::new(lines)
        .block(Block::bordered().title(title))
        .scroll((u16::try_from(app.scroll).unwrap_or(u16::MAX), 0));
    frame.render_widget(paragraph, area);
}

/// How each kind of page line looks, in the palette's colours.
fn style_of(style: PageStyle, palette: &Palette) -> Style {
    match style {
        PageStyle::Title | PageStyle::Heading => palette.heading(),
        PageStyle::Code => palette.code(),
        PageStyle::Dim => palette.muted(),
        PageStyle::Url => palette.accent(),
        PageStyle::Warning => palette.warning(),
        PageStyle::Text => Style::new(),
        PageStyle::Categories => palette.category(),
        PageStyle::Danger => palette.danger(),
    }
}

/// The bottom line, nano-style: each key highlighted, then what it does.
fn draw_keys(frame: &mut Frame, mode: Mode, direct: bool, palette: &Palette, area: Rect) {
    let keys = match mode {
        Mode::Browse => KEYS_BROWSE,
        Mode::Search => KEYS_SEARCH,
        Mode::Info if direct => KEYS_PAGE_DIRECT, // opened as `lore NAME`
        Mode::Info => KEYS_PAGE,
        Mode::Palette => KEYS_PALETTE,
        Mode::Copy => KEYS_COPY,
        Mode::About => KEYS_ABOUT,
    };
    let mut spans = Vec::new();
    for (key, label) in keys {
        spans.push(Span::styled(*key, palette.key()));
        spans.push(Span::raw(format!(" {label}   ")));
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}
