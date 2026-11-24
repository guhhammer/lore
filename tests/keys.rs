//! Tests for the quick keys: g/G, n/N, y, o, 1–9, B, r, ? and Ctrl+L
//! (src/app/keys.rs, src/interface/input.rs, the help and link boxes).

use std::collections::HashMap;

use lore_tui::app::{App, Mode, Topic};
use lore_tui::commands::{Command, Kind};
use lore_tui::configs::{HELP_TITLE, LANGUAGE_NO_COPY, LINK_TITLE};
use lore_tui::interface::input::{Action, action_for};
use lore_tui::interface::ui::draw;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn key(c: char) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)
}

fn screen(app: &mut App) -> String {
    let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(100, 30)).unwrap();
    terminal.draw(|frame| draw(frame, app)).unwrap();
    let buffer = terminal.backend().buffer().clone();
    buffer
        .content()
        .chunks(usize::from(buffer.area.width))
        .map(|row| row.iter().map(|cell| cell.symbol()).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

/// The curl tool page open, drawn once (which lays it out).
fn on_curl() -> App {
    let mut app = App::new(vec![
        Command::new("curl", Kind::Tool),
        Command::new("ls", Kind::Program),
    ]);
    app.show_topic(Topic::Tools);
    app.open_info();
    screen(&mut app);
    app
}

#[test]
fn the_quick_keys_act_in_the_list_and_on_a_page_but_type_in_the_search() {
    for (c, action) in [
        ('g', Action::Top),
        ('G', Action::Bottom),
        ('?', Action::OpenHelp),
        ('B', Action::JumpBookmarks),
        ('r', Action::JumpRecent),
        ('3', Action::JumpTopic(3)),
    ] {
        assert_eq!(action_for(key(c), Mode::Browse), Some(action), "{c}");
        assert_eq!(action_for(key(c), Mode::Info), Some(action), "{c}");
        assert_eq!(action_for(key(c), Mode::Search), Some(Action::Type(c)));
    }
    for (c, action) in [
        ('n', Action::FindNext),
        ('N', Action::FindPrevious),
        ('y', Action::QuickCopy),
        ('o', Action::OpenLink),
    ] {
        assert_eq!(action_for(key(c), Mode::Info), Some(action), "{c}");
        assert_eq!(
            action_for(key(c), Mode::Browse),
            None,
            "{c} is a page's key"
        );
    }
    let ctrl_l = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::CONTROL);
    for mode in [Mode::Browse, Mode::Search, Mode::Info, Mode::About] {
        assert_eq!(action_for(ctrl_l, mode), Some(Action::Repaint));
    }
}

#[test]
fn the_help_scrolls_with_the_arrows_and_any_other_key_closes_it() {
    assert_eq!(
        action_for(KeyEvent::from(KeyCode::Down), Mode::Help),
        Some(Action::ScrollHelp(1))
    );
    for mode in [Mode::Help, Mode::Link] {
        assert_eq!(action_for(key('x'), mode), Some(Action::CloseOverlay));
        assert_eq!(
            action_for(KeyEvent::from(KeyCode::Esc), mode),
            Some(Action::CloseOverlay)
        );
    }
    let mut app = on_curl();
    app.open_help();
    assert_eq!(app.mode, Mode::Help);
    let shown = screen(&mut app);
    assert!(shown.contains(HELP_TITLE.trim()), "{shown}");
    assert!(shown.contains("this help"), "{shown}");
    app.close_overlay();
    assert_eq!(app.mode, Mode::Info, "back to the page it was opened on");
}

#[test]
fn g_and_capital_g_go_to_the_top_and_the_bottom_of_the_list_and_the_page() {
    let mut app = App::new(vec![
        Command::new("awk", Kind::Program),
        Command::new("ls", Kind::Program),
        Command::new("zip", Kind::Program),
    ]);
    app.go_bottom();
    assert_eq!(app.selected().unwrap().name, "zip");
    app.go_top();
    assert_eq!(app.selected().unwrap().name, "awk");

    let mut app = on_curl();
    app.go_bottom();
    assert_eq!(app.scroll, app.scroll_max);
    assert!(app.scroll > 0, "curl's page is longer than the window");
    app.go_top();
    assert_eq!(app.scroll, 0);
}

#[test]
fn n_and_capital_n_jump_between_the_page_lines_with_the_search_text() {
    let mut app = on_curl();
    app.find(true);
    assert!(app.note.as_deref().unwrap().contains("search a word"));

    app.query = "download".to_owned();
    screen(&mut app); // the draw finds the lines
    assert!(app.matches.len() >= 2, "{:?}", app.matches);
    let (first, second) = (app.matches[0], app.matches[1]);
    app.find(true);
    assert_eq!(app.scroll, first);
    assert!(app.note.as_deref().unwrap().contains("1 of"));
    app.find(true);
    assert_eq!(app.scroll, second);
    app.find(false);
    assert_eq!(app.scroll, first);

    app.query = "zzqqxx".to_owned();
    screen(&mut app);
    app.find(true);
    assert!(app.note.as_deref().unwrap().contains("not on this page"));
}

#[test]
fn y_gives_the_first_example_at_once_but_nothing_on_a_language_page() {
    let mut app = on_curl();
    let first = app.copyables()[0].clone();
    assert_eq!(app.quick_copy(), Some(first));
    assert_eq!(app.mode, Mode::Info, "no list opens");

    let mut app = App::new(vec![Command::new("Python", Kind::Language)]);
    app.show_topic(Topic::Languages);
    app.mode = Mode::Info;
    screen(&mut app);
    assert_eq!(app.quick_copy(), None);
    assert_eq!(app.note.as_deref(), Some(LANGUAGE_NO_COPY));
}

#[test]
fn o_shows_the_page_first_link_alone_in_a_box() {
    let mut app = on_curl();
    let url = app.first_link().unwrap();
    assert!(url.starts_with("https://"));
    app.open_link();
    assert_eq!(app.mode, Mode::Link);
    let shown = screen(&mut app);
    assert!(shown.contains(LINK_TITLE.trim()));
    assert!(shown.contains(&url), "the URL is drawn whole: {shown}");
    app.close_overlay();
    assert_eq!(app.mode, Mode::Info);
}

#[test]
fn digits_b_and_r_jump_to_a_topic_and_an_empty_one_says_so() {
    let mut app = on_curl();
    app.jump_to(Topic::Bookmarks);
    assert!(app.note.as_deref().unwrap().contains("bookmarks"));
    assert_eq!(app.mode, Mode::Info, "nothing to go to: the page stays");

    app.set_personal(
        vec!["ls".to_owned()],
        vec!["curl".to_owned()],
        HashMap::new(),
    );
    app.refresh_rows();
    app.jump_to(Topic::Bookmarks);
    assert_eq!((app.mode, app.topic), (Mode::Browse, Topic::Bookmarks));
    app.jump_to(Topic::Recent);
    assert_eq!(app.topic, Topic::Recent);
    app.jump_topic(1);
    assert_eq!(app.topic, app.topics[0]);
    app.jump_topic(9);
    assert_eq!(app.topic, app.topics[0], "past the last topic: nothing");
}
