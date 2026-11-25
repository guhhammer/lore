//! Tests for how the screen fits a small window (src/interface/ui.rs): the
//! key bar on two rows at most, the topic bar sliding, the minimum size.

use lore_tui::app::{App, Topic};
use lore_tui::commands::{Command, Kind};
use lore_tui::configs::{KEYS_BROWSE, MIN_HEIGHT, MIN_WIDTH};
use lore_tui::interface::ui::{draw, key_rows};

fn screen(app: &mut App, width: u16, height: u16) -> String {
    let mut terminal =
        ratatui::Terminal::new(ratatui::backend::TestBackend::new(width, height)).unwrap();
    terminal.draw(|frame| draw(frame, app)).unwrap();
    let buffer = terminal.backend().buffer().clone();
    buffer
        .content()
        .chunks(usize::from(buffer.area.width))
        .map(|row| row.iter().map(|cell| cell.symbol()).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn app() -> App {
    App::new(vec![
        Command::new("ls", Kind::Program),
        Command::new("Python", Kind::Language),
    ])
}

#[test]
fn the_key_bar_is_one_row_when_it_fits_and_two_at_most_when_not() {
    assert_eq!(key_rows(KEYS_BROWSE, 400).len(), 1);
    let narrow = key_rows(KEYS_BROWSE, 60);
    assert_eq!(narrow.len(), 2);
    assert_eq!(key_rows(KEYS_BROWSE, 10).len(), 2, "never a third row");
    // nothing dropped while two rows hold everything
    let all = key_rows(KEYS_BROWSE, 90);
    if all.len() <= 2 {
        assert_eq!(all.iter().map(Vec::len).sum::<usize>(), KEYS_BROWSE.len());
    }
}

#[test]
fn on_a_narrow_window_the_keys_wrap_onto_a_second_row() {
    let mut app = app();
    let shown = screen(&mut app, 60, 24);
    let rows: Vec<&str> = shown.lines().collect();
    let last_two = format!("{}\n{}", rows[rows.len() - 2], rows[rows.len() - 1]);
    assert!(last_two.contains("Move"), "{last_two}");
    assert!(
        last_two.contains("Quit") || last_two.contains("Search"),
        "{last_two}"
    );
}

#[test]
fn a_window_smaller_than_the_minimum_says_what_it_needs() {
    let mut app = app();
    let shown = screen(&mut app, MIN_WIDTH - 1, MIN_HEIGHT);
    assert!(
        shown.contains(&format!("{MIN_WIDTH}×{MIN_HEIGHT}")),
        "{shown}"
    );
    let shown = screen(&mut app, MIN_WIDTH, MIN_HEIGHT);
    assert!(!shown.contains("needs at least"), "{shown}");
}

#[test]
fn the_topic_bar_slides_to_keep_the_chosen_topic_in_sight() {
    let mut app = app();
    // own lists give the bar more topics than a narrow window holds
    app.bookmarks = vec!["ls".to_owned()];
    app.recent = vec!["ls".to_owned()];
    app.refresh_rows();
    screen(&mut app, MIN_WIDTH, 24);
    assert_eq!(app.topic_scroll, 0, "the first topic needs no slide");
    app.show_topic(Topic::Bookmarks);
    let shown = screen(&mut app, MIN_WIDTH, 24);
    assert!(app.topic_scroll > 0, "{shown}");
    assert!(shown.contains(" bookmarks "), "{shown}");
    app.show_topic(Topic::Commands);
    let shown = screen(&mut app, MIN_WIDTH, 24);
    assert_eq!(app.topic_scroll, 0);
    assert!(shown.contains(" commands "), "{shown}");
}

#[test]
fn a_box_over_wide_characters_keeps_its_border_whole() {
    // names in two-column characters, starting one column apart, so one of
    // them always straddles the help box's left edge
    let wide = "压缩文件夹".repeat(8);
    let mut app = App::new(vec![
        Command::new(&wide, Kind::Program),
        Command::new(format!("a{wide}"), Kind::Program),
    ]);
    app.open_help();
    let mut terminal = ratatui::Terminal::new(ratatui::backend::TestBackend::new(100, 30)).unwrap();
    terminal.draw(|frame| draw(frame, &mut app)).unwrap();
    let buffer = terminal.backend().buffer().clone();
    let width = usize::from(buffer.area.width);
    let rows: Vec<&[ratatui::buffer::Cell]> = buffer.content().chunks(width).collect();
    // the help box's corners: its top row is the one with its title
    let (top, left) = rows
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            let line: Vec<&str> = row.iter().map(|c| c.symbol()).collect();
            let at = line.join("").find("keys")?;
            let x = line[..at].iter().rposition(|s| *s == "┌")?;
            Some((y, x))
        })
        .expect("the help box is drawn");
    let right = (left + 1..width)
        .find(|&x| rows[top][x].symbol() == "┐")
        .unwrap();
    let bottom = (top + 1..rows.len())
        .find(|&y| rows[y][left].symbol() == "└")
        .expect("the help box's bottom corner");
    for row in &rows[top..=bottom] {
        assert!(
            ["┌", "│", "└"].contains(&row[left].symbol()),
            "the box's left border is whole"
        );
        for x in [left - 1, right + 1] {
            assert_eq!(
                unicode_width::UnicodeWidthStr::width(row[x].symbol()),
                1,
                "a character wider than a column beside the box spills over its border"
            );
        }
    }
}

#[test]
fn an_about_value_too_long_for_the_window_ends_in_dots_and_links_stay_whole() {
    let mut app = app();
    app.open_about();
    let shown = screen(&mut app, 60, 40);
    // the reset hint is longer than 60 columns: cut, with the sign
    let reset = shown
        .lines()
        .find(|l| l.contains("Reset all"))
        .expect("the reset setting is on screen");
    assert!(
        reset.trim_end_matches('│').trim_end().ends_with("..."),
        "{reset}"
    );
    // a link is never cut: the terminal opens it whole
    assert!(
        shown.contains("https://github.com/guhhammer/lore"),
        "{shown}"
    );
}
