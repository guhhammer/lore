//! Draws the screen, top to bottom: search bar, the body, key bar. The body
//! is the list, or, with a page open, a narrow list beside the page. Reads
//! the state; changes only the list's scroll, the loaded page and its scroll.

use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Position, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, List, ListItem, ListState, Paragraph};

use crate::app::{App, Mode, Setting, copy_label};
use crate::commands::{Command, Kind};
use crate::configs::{
    ABOUT_COLOURS, ABOUT_COMMANDS, ABOUT_FAST_SCROLL, ABOUT_FIELDS, ABOUT_HISTORY_EMPTY,
    ABOUT_HISTORY_OFF, ABOUT_HISTORY_ON, ABOUT_INTERNET_CHECKING, ABOUT_INTERNET_OFF,
    ABOUT_INTERNET_OFFLINE, ABOUT_INTERNET_ON, ABOUT_KEPT, ABOUT_LABEL_WIDTH, ABOUT_LICENCE,
    ABOUT_MANUAL, ABOUT_PAGES, ABOUT_SETTING_MARK, ABOUT_SETTING_NAMES, ABOUT_SETTINGS_HEADING,
    ABOUT_SETTINGS_HINT, ABOUT_SOURCE, ABOUT_TAGLINE, ABOUT_TITLE, BOOKMARK_MARK, BOOKMARKS_FILE,
    COPY_PICKER_MARGIN, COPY_TITLE, DANGER, HEADING_MANUAL, INFO_LIST_PERCENT, INTERNET_DOT,
    INTERNET_OFF, INTERNET_ON, INTERNET_WORD, KEY_BAR_HEIGHT, KEY_BAR_ROWS_MAX, KEY_GAP,
    KEYS_ABOUT, KEYS_BROWSE, KEYS_COPY, KEYS_PAGE, KEYS_PALETTE, KEYS_SEARCH, LANGUAGE_CATEGORY,
    LIST_HEADER_INDENT, LIST_INDENT, LIST_INDENT_NARROW, LIST_TITLE, LIST_TITLE_SEARCHING, LOGO,
    LOGO_GAP, LORE_ORANGE, MIN_HEIGHT, MIN_WIDTH, NO_MATCH, PALETTE_DIR, PALETTE_NOT_SAVED,
    PALETTE_PICKER_MARGIN, PALETTE_TITLE, SEARCH_BAR_HEIGHT, SEARCH_HINT, SEARCH_KEPT_HINT,
    SEARCH_TITLE, TOO_SMALL, TOPIC_BAR_HEIGHT, TOPIC_GAP, TOPIC_HINT, UNUSED_MARK,
};
use crate::configs::{
    ABOUT_FAVOURITE, ABOUT_READING, ABOUT_READING_SYSTEM, CHOOSE_MARGIN, CHOOSE_TITLE_FAVOURITE,
    CHOOSE_TITLE_READING, FAVOURITE_NONE, KEYS_CHOOSE, READING_ENGLISH,
};
use crate::configs::{
    ABOUT_FOLLOW, ABOUT_FOLLOW_HEADING, ABOUT_FOLLOW_HINT, ABOUT_RESET, ABOUT_RESET_ARMED,
    TOOLS_EMPTY,
};
use crate::info::content;
use crate::info::page::{self, PageLine, Style as PageStyle};
use crate::interface::palette::Palette;
use crate::interface::paths;
use crate::interface::rows::Row;
use crate::settings;
use crate::utils::internet::Status;
use crate::utils::text;

/// Draws the whole screen; called once per loop in lib.rs.
pub fn draw(frame: &mut Frame, app: &mut App) {
    // the palette's background and text under everything drawn after
    frame.render_widget(Block::new().style(app.palette().base()), frame.area());
    let area = frame.area();
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        draw_too_small(frame, app.palette(), area);
        return;
    }
    let key_rows = match &app.note {
        Some(_) => Vec::new(),
        None => key_rows(keys_of(app.mode), usize::from(area.width)),
    };
    let key_height = u16::try_from(key_rows.len())
        .unwrap_or(u16::MAX)
        .max(KEY_BAR_HEIGHT);
    // the search bar on top, the keys at the bottom, the body gets the rest
    // the logo, 4 rows, beside the search bar, which sits on its last 3
    let logo_height = u16::try_from(LOGO.len()).unwrap_or(u16::MAX);
    let [top, topics, body, keys] = Layout::vertical([
        Constraint::Length(logo_height.max(SEARCH_BAR_HEIGHT)),
        Constraint::Length(TOPIC_BAR_HEIGHT),
        Constraint::Min(0),
        Constraint::Length(key_height),
    ])
    .areas(area);
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
    draw_topics(frame, app, topics);
    if app.about_shown() {
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
        None => draw_keys(frame, &key_rows, app.palette(), keys),
    }
    if app.mode == Mode::Palette {
        draw_picker(frame, app, body);
    }
    if app.mode == Mode::Copy {
        draw_copy(frame, app, body);
    }
    if app.mode == Mode::Choose {
        draw_choice(frame, app, body);
    }
}

/// A setting's picker (Pages in, Favourite), over the about screen: its
/// choices in a list as tall as fits, the chosen one selected.
fn draw_choice(frame: &mut Frame, app: &App, area: Rect) {
    let Some(choice) = &app.choice else {
        return;
    };
    let title = match choice.setting {
        Setting::Reading => CHOOSE_TITLE_READING,
        _ => CHOOSE_TITLE_FAVOURITE,
    };
    let longest = choice
        .items
        .iter()
        .map(|i| text::width(i))
        .chain([text::width(title)])
        .max()
        .unwrap_or(0);
    let width = u16::try_from(longest)
        .unwrap_or(u16::MAX)
        .saturating_add(CHOOSE_MARGIN)
        .min(area.width);
    let height = u16::try_from(choice.items.len() + 2)
        .unwrap_or(u16::MAX)
        .min(area.height);
    let [middle] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    let [popup] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(middle);
    let items: Vec<ListItem> = choice
        .items
        .iter()
        .map(|item| ListItem::new(format!(" {item}")))
        .collect();
    let list = List::new(items)
        .block(
            Block::bordered()
                .title(title)
                .border_style(app.palette().active()),
        )
        .style(app.palette().base())
        .highlight_style(app.palette().selected());
    let mut state = ListState::default().with_selected(Some(choice.at));
    frame.render_widget(Clear, popup);
    frame.render_stateful_widget(list, popup, &mut state);
}

/// The about screen: what lore is, and what it has and keeps here (the
/// logo is already on top, as on every screen).
fn draw_about(frame: &mut Frame, app: &mut App, area: Rect) {
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
        tilde(config),
        tilde(palettes_dir),
        format!(
            "{}  ({})",
            tilde(kept),
            ABOUT_KEPT
                .replace("{bookmarks}", &count(app.bookmarks.len()))
                .replace("{recent}", &count(app.recent.len()))
        ),
        ABOUT_LICENCE.to_owned(),
        ABOUT_SOURCE.to_owned(),
        ABOUT_MANUAL.to_owned(),
    ];
    let mut chosen_line = 0;
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

    // the author's profiles, to follow
    lines.push(Line::raw(""));
    lines.push(Line::from(vec![
        Span::styled(format!("  {ABOUT_FOLLOW_HEADING}"), app.palette().heading()),
        Span::styled(format!("   {ABOUT_FOLLOW_HINT}"), muted),
    ]));
    for (site, link) in ABOUT_FOLLOW {
        lines.push(Line::from(vec![
            Span::styled(format!("  {site:<ABOUT_LABEL_WIDTH$}"), muted),
            Span::styled(*link, app.palette().accent()),
        ]));
    }

    // the settings, changed here: the chosen one marked and highlighted
    lines.push(Line::raw(""));
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {ABOUT_SETTINGS_HEADING}"),
            app.palette().heading(),
        ),
        Span::styled(format!("   {ABOUT_SETTINGS_HINT}"), muted),
    ]));
    let pad = " ".repeat(text::width(ABOUT_SETTING_MARK));
    for (i, (setting, name)) in Setting::all().iter().zip(ABOUT_SETTING_NAMES).enumerate() {
        let (value, style) = match setting {
            Setting::Colours => (
                ABOUT_COLOURS
                    .replace("{name}", &app.palette().name)
                    .replace("{count}", &count(app.palettes.len())),
                Style::new(),
            ),
            Setting::Internet => {
                let (state, style) = match app.internet.status() {
                    Status::Online => (ABOUT_INTERNET_ON, Style::new().fg(INTERNET_ON)),
                    Status::Offline => (ABOUT_INTERNET_OFFLINE, Style::new().fg(INTERNET_OFF)),
                    Status::Off => (ABOUT_INTERNET_OFF, Style::new().fg(INTERNET_OFF)),
                    Status::Checking => (ABOUT_INTERNET_CHECKING, muted),
                };
                (format!("{INTERNET_DOT} {state}"), style)
            }
            Setting::Reading => (
                match &app.reading {
                    Some(code) => ABOUT_READING
                        .replace("{code}", code)
                        .replace("{count}", &count(crate::reading_choices().len())),
                    None => ABOUT_READING_SYSTEM.replace(
                        "{code}",
                        &crate::info::content::languages()
                            .first()
                            .cloned()
                            .unwrap_or_else(|| READING_ENGLISH.to_owned()),
                    ),
                },
                Style::new(),
            ),
            Setting::Favourite => (
                ABOUT_FAVOURITE
                    .replace("{name}", app.favourite.as_deref().unwrap_or(FAVOURITE_NONE)),
                Style::new(),
            ),
            Setting::Reset => (
                if app.reset_armed {
                    ABOUT_RESET_ARMED
                } else {
                    ABOUT_RESET
                }
                .to_owned(),
                if app.reset_armed {
                    app.palette().danger()
                } else {
                    muted
                },
            ),
            Setting::FastScroll => (
                ABOUT_FAST_SCROLL.replace("{lines}", &app.fast_scroll.to_string()),
                Style::new(),
            ),
            Setting::History => (
                match (app.history, app.uses.len()) {
                    (false, _) => ABOUT_HISTORY_OFF.to_owned(),
                    (true, 0) => ABOUT_HISTORY_EMPTY.to_owned(),
                    (true, seen) => ABOUT_HISTORY_ON.replace("{seen}", &count(seen)),
                },
                Style::new(),
            ),
        };
        let chosen = i == app.setting;
        if chosen {
            chosen_line = lines.len(); // kept in view below
        }
        let (mark, label_style) = if chosen {
            (ABOUT_SETTING_MARK, app.palette().selected())
        } else {
            (pad.as_str(), muted)
        };
        lines.push(Line::from(vec![
            Span::raw(format!("  {mark}")),
            Span::styled(format!("{name:<ABOUT_LABEL_WIDTH$}"), label_style),
            Span::raw(" "),
            Span::styled(value, style),
        ]));
    }
    // a small window: the screen scrolls (PgUp/PgDn, w/s), and to the
    // chosen setting when ↑↓ choose another
    let height = usize::from(area.height.saturating_sub(2));
    let most = lines.len().saturating_sub(height);
    let mut scroll = app.about_scroll.min(most);
    if std::mem::take(&mut app.about_follow) {
        if chosen_line < scroll {
            scroll = chosen_line;
        } else if height > 0 && chosen_line >= scroll + height {
            scroll = chosen_line + 1 - height;
        }
    }
    app.about_scroll = scroll;
    let block = Block::bordered().title(ABOUT_TITLE);
    let paragraph = Paragraph::new(lines)
        .block(block)
        .scroll((u16::try_from(scroll).unwrap_or(u16::MAX), 0));
    frame.render_widget(paragraph, area);
}

/// The page's examples, over the page, to copy one: each as typed, cut to
/// fit, the chosen one selected.
fn draw_copy(frame: &mut Frame, app: &App, area: Rect) {
    let labels: Vec<String> = app.copies.iter().map(|c| copy_label(c)).collect();
    let room = area.width.saturating_sub(2 * COPY_PICKER_MARGIN);
    let inside = usize::from(room.saturating_sub(4)); // borders and margins
    let longest = labels.iter().map(|e| text::width(e) + 4).max().unwrap_or(0);
    let width = u16::try_from(longest).unwrap_or(u16::MAX).min(room);
    // + borders; a long list scrolls inside the body
    let height = u16::try_from(labels.len() + 2)
        .unwrap_or(u16::MAX)
        .min(area.height);
    let [middle] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    let [popup] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(middle);
    let items: Vec<ListItem> = labels
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
/// ` ● internet `: green when online, red when not (or off), dim while the
/// first check runs. On every screen, on the search bar's right.
fn internet_dot(app: &App) -> Line<'static> {
    let style = match app.internet.status() {
        Status::Online => Style::new().fg(INTERNET_ON),
        Status::Offline | Status::Off => Style::new().fg(INTERNET_OFF),
        Status::Checking => app.palette().muted(),
    };
    Line::styled(format!(" {INTERNET_DOT} {INTERNET_WORD} "), style)
}

fn draw_search(frame: &mut Frame, app: &App, area: Rect) {
    let searching = app.mode == Mode::Search;
    let block = Block::bordered()
        .title(SEARCH_TITLE)
        .title_top(internet_dot(app).right_aligned())
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

/// The topics under the search bar, the one shown highlighted; then what
/// switches them. Never in the way: one line, cut if the screen is narrow.
fn draw_topics(frame: &mut Frame, app: &mut App, area: Rect) {
    let palette = app.palette();
    let mut spans = vec![Span::raw(" ")];
    let mut chosen = (0, 0); // where the chosen topic starts and ends, in columns
    let mut at = 1;
    for (i, topic) in app.topics.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw(TOPIC_GAP));
            at += text::width(TOPIC_GAP);
        }
        let name = format!(" {} ", topic.name());
        if *topic == app.topic {
            chosen = (at, at + text::width(&name));
        }
        at += text::width(&name);
        let style = if *topic == app.topic {
            palette.selected().add_modifier(Modifier::BOLD)
        } else {
            palette.muted()
        };
        spans.push(Span::styled(name, style));
    }
    if app.topics.len() > 1 {
        spans.push(Span::styled(TOPIC_HINT, palette.muted()));
        at += text::width(TOPIC_HINT);
    }
    // a bar wider than the window slides, only as far as it takes to keep
    // the chosen topic in sight (a column of room on each side)
    let width = usize::from(area.width);
    let mut scroll = app.topic_scroll.min(at.saturating_sub(width));
    if chosen.0 < scroll + 1 {
        scroll = chosen.0.saturating_sub(1);
    } else if chosen.1 + 1 > scroll + width {
        scroll = (chosen.1 + 1).saturating_sub(width);
    }
    app.topic_scroll = scroll;
    let scroll = u16::try_from(scroll).unwrap_or(u16::MAX);
    frame.render_widget(Paragraph::new(Line::from(spans)).scroll((0, scroll)), area);
}

/// The list of rows, or a note when nothing matches. `narrow`: beside a page,
/// with names only and no title; names that do not fit end in `...`.
fn draw_list(frame: &mut Frame, app: &mut App, area: Rect, narrow: bool) {
    let total = app.commands.len();
    let block = if narrow {
        Block::bordered() // a title would not fit in a tenth of the screen
    } else if app.query.is_empty() || app.topic == crate::app::Topic::Docs {
        // the docs topic searches only itself: its own title, its count
        Block::bordered().title(
            LIST_TITLE
                .replace("{topic}", app.topic.name())
                .replace("{total}", &app.shown_count().to_string()),
        )
    } else {
        Block::bordered().title(
            LIST_TITLE_SEARCHING
                .replace("{shown}", &app.shown_count().to_string())
                .replace("{total}", &total.to_string()),
        )
    };
    if app.rows.is_empty() {
        // the tools topic, empty for now, says so
        let note = if app.query.is_empty() && app.topic == crate::app::Topic::Tools {
            TOOLS_EMPTY.to_owned()
        } else {
            NO_MATCH.replace("{query}", &app.query)
        };
        let none = Line::from(Span::styled(note, app.palette().muted()));
        frame.render_widget(Paragraph::new(none).block(block), area);
        return;
    }
    let width = usize::from(area.width.saturating_sub(2)); // inside the borders
    let items: Vec<ListItem> = app
        .rows
        .iter()
        .map(|row| match row {
            Row::Header(_) | Row::Title(_) => {
                let indent = if narrow { "" } else { LIST_HEADER_INDENT };
                let header = match row {
                    Row::Title(title) => format!("{indent}{title}"),
                    Row::Header(section) => format!("{indent}{section}"),
                    Row::Command(_) => String::new(),
                };
                ListItem::new(Line::from(Span::styled(
                    text::fit(&header, width),
                    app.palette().section(),
                )))
            }
            Row::Command(command) => {
                // a docs row is not the command of the same name
                let categories = app
                    .docs
                    .get(&command.name)
                    .filter(|_| !matches!(command.kind, Kind::Docset | Kind::DocEntry))
                    .map(|d| d.categories.as_slice())
                    .unwrap_or_default();
                let snippet = app.snippets.get(&command.name).map(String::as_str);
                ListItem::new(command_line(
                    command,
                    snippet,
                    categories,
                    width,
                    narrow,
                    app.is_unused(&command.name) && !command.kind.is_lores(),
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
                // danger in red; a language page's own category in the
                // palette's third colour, so languages stand out in a search
                let style = if category == DANGER {
                    palette.danger()
                } else if category == LANGUAGE_CATEGORY {
                    palette.accent()
                } else {
                    palette.category()
                };
                let space = if i == 0 { "" } else { " " };
                parts.push((format!("{space}{category}"), style));
            }
            parts.push(("]".to_owned(), palette.category()));
        }
        let dim = palette.muted();
        // what an alias runs (docs rows keep their set's name there, unshown)
        if let Some(runs) = command
            .runs
            .as_ref()
            .filter(|_| command.kind == Kind::Alias)
        {
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
                Kind::Recipe => "a recipe",
                Kind::Tool => "a tool page",
                Kind::Language => "a language",
                Kind::Docset | Kind::DocEntry => "documentation",
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
    // the INTERNET section goes before MANUAL (a recipe has neither): made
    // here, at every draw, since its answer arrives in the background
    let mut all: Vec<&PageLine> = page.lines.iter().collect();
    let internet;
    if let Some(manual) = page
        .lines
        .iter()
        .rposition(|l| l.style == PageStyle::Heading && l.text == HEADING_MANUAL)
    {
        let name = &page.command.name;
        internet = page::internet_lines(
            name,
            app.internet.status(),
            app.internet.answer(name).as_ref(),
            usize::from(width),
        );
        let at = manual.saturating_sub(1); // before the blank line over MANUAL
        all.splice(at..at, internet.iter());
    }
    let rows = all.len();
    // where each section starts, for the jumper (Space)
    let sections = all
        .iter()
        .enumerate()
        .filter(|(_, l)| page::starts_a_section(l))
        .map(|(i, _)| i)
        .collect();
    app.scroll = app.scroll.min(rows.saturating_sub(height));
    let lines: Vec<Line> = all
        .iter()
        .map(|line| {
            if line.spans.is_empty() {
                Line::styled(line.text.as_str(), style_of(line.style, app.palette()))
            } else {
                // a language page's code: each part in its own colour
                Line::from(
                    line.spans
                        .iter()
                        .map(|(text, style)| {
                            Span::styled(text.as_str(), style_of(*style, app.palette()))
                        })
                        .collect::<Vec<_>>(),
                )
            }
        })
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
    app.sections = sections;
    app.scroll_max = rows.saturating_sub(height);
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
        // green or red whatever the palette, as the dot on top
        PageStyle::Online => Style::new().fg(INTERNET_ON),
        PageStyle::Offline => Style::new().fg(INTERNET_OFF),
        PageStyle::Raw => Style::new(),
        PageStyle::Comment | PageStyle::Gutter => palette.muted(),
        PageStyle::Keyword => palette.heading(),
        PageStyle::Literal => palette.code(),
        PageStyle::Number => palette.category(),
    }
}

/// The keys of a mode, for the key bar.
fn keys_of(mode: Mode) -> &'static [(&'static str, &'static str)] {
    match mode {
        Mode::Browse => KEYS_BROWSE,
        Mode::Search => KEYS_SEARCH,
        Mode::Info => KEYS_PAGE,
        Mode::Palette => KEYS_PALETTE,
        Mode::Choose => KEYS_CHOOSE,
        Mode::Copy => KEYS_COPY,
        Mode::About => KEYS_ABOUT,
    }
}

/// The key bar's rows for a window `width` wide: as many keys on a row as
/// fit, then the next row, at most `KEY_BAR_ROWS_MAX`; keys past the last
/// row are left out (the window is too narrow for them).
pub fn key_rows<'a>(keys: &'a [(&'a str, &'a str)], width: usize) -> Vec<Vec<(&'a str, &'a str)>> {
    let mut rows: Vec<Vec<(&str, &str)>> = vec![Vec::new()];
    let mut used = 0;
    for &(key, label) in keys {
        let room = text::width(key) + 1 + text::width(label);
        let gap = if used == 0 { 0 } else { text::width(KEY_GAP) };
        if used > 0 && used + gap + room > width {
            if rows.len() == KEY_BAR_ROWS_MAX {
                break;
            }
            rows.push(Vec::new());
            used = 0;
        }
        used += if used == 0 { room } else { gap + room };
        if let Some(row) = rows.last_mut() {
            row.push((key, label));
        }
    }
    rows
}

/// The bottom rows, nano-style: each key highlighted, then what it does.
fn draw_keys(frame: &mut Frame, rows: &[Vec<(&str, &str)>], palette: &Palette, area: Rect) {
    let lines: Vec<Line> = rows
        .iter()
        .map(|row| {
            let mut spans = Vec::new();
            for (i, (key, label)) in row.iter().enumerate() {
                if i > 0 {
                    spans.push(Span::raw(KEY_GAP));
                }
                spans.push(Span::styled(*key, palette.key()));
                spans.push(Span::raw(format!(" {label}")));
            }
            Line::from(spans)
        })
        .collect();
    frame.render_widget(Paragraph::new(lines), area);
}

/// What a window too small for lore shows: its size and the size needed.
fn draw_too_small(frame: &mut Frame, palette: &Palette, area: Rect) {
    let note = TOO_SMALL
        .replace("{width}", &area.width.to_string())
        .replace("{height}", &area.height.to_string())
        .replace("{min_width}", &MIN_WIDTH.to_string())
        .replace("{min_height}", &MIN_HEIGHT.to_string());
    let lines: Vec<Line> = note
        .lines()
        .map(|l| Line::styled(l.to_owned(), palette.accent()))
        .collect();
    let height = u16::try_from(lines.len())
        .unwrap_or(u16::MAX)
        .min(area.height);
    let [middle] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    frame.render_widget(Paragraph::new(lines).centered(), middle);
}
