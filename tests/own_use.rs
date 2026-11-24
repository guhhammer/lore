//! Tests for your own use: the bookmarks, recent and most used topics, and
//! the never-used mark (src/app.rs).

use std::collections::HashMap;

use lore_tui::app::{App, Mode, Topic};
use lore_tui::commands::{Command, Kind};
use lore_tui::compile_config::{BOOKMARK_KEY, RECENT_MAX};
use lore_tui::interface::input::{Action, action_for};
use lore_tui::interface::rows::Row;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn app(names: &[&str]) -> App {
    App::new(
        names
            .iter()
            .map(|&n| Command::new(n, Kind::Program))
            .collect(),
    )
}

/// The rows as text: headers as `[A]`, commands by name.
fn labels(app: &App) -> Vec<String> {
    app.rows
        .iter()
        .map(|row| match row {
            Row::Header(section) => format!("[{section}]"),
            Row::Title(title) => format!("[{title}]"),
            Row::Command(command) => command.name.clone(),
        })
        .collect()
}

fn uses(list: &[(&str, u32)]) -> HashMap<String, u32> {
    list.iter().map(|(n, u)| ((*n).to_owned(), *u)).collect()
}

fn select(app: &mut App, name: &str) {
    let row = app
        .rows
        .iter()
        .rposition(|r| matches!(r, Row::Command(c) if c.name == name));
    app.list.select(row);
}

#[test]
fn with_no_bookmarks_recent_or_history_none_of_your_own_topics_show() {
    let app = app(&["ls", "cat"]);
    assert_eq!(app.topics, [Topic::Commands, Topic::Docs, Topic::Tools]);
    assert_eq!(labels(&app), ["[C]", "cat", "[L]", "ls"]);
}

#[test]
fn bookmarks_recent_and_most_used_are_topics_of_their_own() {
    let mut app = app(&["ls", "cat", "tar"]);
    app.set_personal(
        vec!["tar".into()],
        vec!["cat".into()],
        uses(&[("ls", 9), ("cat", 2)]),
    );
    assert_eq!(
        app.topics,
        [
            Topic::Commands,
            Topic::Docs,
            Topic::Tools,
            Topic::MostUsed,
            Topic::Recent,
            Topic::Bookmarks
        ]
    );
    assert_eq!(labels(&app), ["[C]", "cat", "[L]", "ls", "[T]", "tar"]);
    app.step_topic(3);
    assert_eq!(labels(&app), ["ls", "cat"], "most used, most first");
    app.step_topic(1);
    assert_eq!(labels(&app), ["cat"]);
    app.step_topic(1);
    assert_eq!(labels(&app), ["tar"]);
}

#[test]
fn names_kept_that_are_not_on_this_machine_are_left_out() {
    let mut app = app(&["ls"]);
    app.set_personal(
        vec!["gone".into()],
        vec!["gone".into()],
        uses(&[("gone", 3)]),
    );
    assert_eq!(
        app.topics,
        [Topic::Commands, Topic::Docs, Topic::Tools],
        "nothing to show in them"
    );
    assert_eq!(labels(&app), ["[L]", "ls"]);
}

#[test]
fn a_search_looks_in_every_topic() {
    let mut app = app(&["ls", "lsof"]);
    app.set_personal(vec!["ls".into()], Vec::new(), HashMap::new());
    app.show_topic(Topic::Bookmarks); // only ls
    app.open_search();
    app.type_char('l');
    assert_eq!(labels(&app), ["ls", "lsof"]);
}

#[test]
fn in_a_search_the_commands_you_run_most_go_first_within_a_kind_of_match() {
    let mut app = app(&["lsblk", "lsof", "ls"]);
    app.set_personal(Vec::new(), Vec::new(), uses(&[("lsof", 5), ("lsblk", 1)]));
    app.open_search();
    app.type_char('l');
    app.type_char('s');
    // the exact name still comes first; then the prefixes, most used first
    assert_eq!(labels(&app), ["ls", "lsof", "lsblk"]);
}

#[test]
fn b_bookmarks_and_b_again_takes_the_bookmark_away() {
    let mut app = app(&["ls", "cat"]);
    select(&mut app, "ls");
    assert!(app.toggle_bookmark());
    assert_eq!(app.bookmarks, ["ls"]);
    assert_eq!(app.selected().unwrap().name, "ls", "the selection stays");
    app.toggle_bookmark();
    assert!(app.bookmarks.is_empty());
}

#[test]
fn b_is_the_bookmark_key_in_the_list_and_on_a_page_but_types_in_the_search() {
    let b = KeyEvent::new(KeyCode::Char(BOOKMARK_KEY), KeyModifiers::NONE);
    assert_eq!(action_for(b, Mode::Browse), Some(Action::ToggleBookmark));
    assert_eq!(action_for(b, Mode::Info), Some(Action::ToggleBookmark));
    assert_eq!(
        action_for(b, Mode::Search),
        Some(Action::Type(BOOKMARK_KEY))
    );
}

#[test]
fn an_opened_page_goes_first_among_the_recent_ones_once() {
    let mut app = app(&["ls", "cat"]);
    for name in ["ls", "cat", "ls"] {
        select(&mut app, name);
        app.open_info();
        assert_eq!(
            app.selected().unwrap().name,
            name,
            "the page opened is the one selected"
        );
        app.close_info();
    }
    assert_eq!(app.recent, ["ls", "cat"]);
}

#[test]
fn the_recent_pages_are_kept_to_a_limit() {
    let names: Vec<String> = (0..RECENT_MAX + 3).map(|i| format!("c{i}")).collect();
    let refs: Vec<&str> = names.iter().map(String::as_str).collect();
    let mut app = app(&refs);
    for name in &names {
        select(&mut app, name);
        app.open_info();
        app.close_info();
    }
    assert_eq!(app.recent.len(), RECENT_MAX);
}

#[test]
fn a_command_is_marked_unused_only_when_there_is_a_history_to_go_by() {
    let mut app = app(&["ls", "cat"]);
    assert!(!app.is_unused("cat"));
    app.set_personal(Vec::new(), Vec::new(), uses(&[("ls", 1)]));
    assert!(app.is_unused("cat"));
    assert!(!app.is_unused("ls"));
}
