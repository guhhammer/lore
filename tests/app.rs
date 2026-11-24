//! Tests for the app's state: moving, searching, leaving the search, the
//! page (src/app.rs). Each test name says the rule it checks.

use std::collections::HashSet;

use lore_tui::app::{App, Mode};
use lore_tui::commands::{Command, Kind};
use lore_tui::interface::rows::Row;

/// An app over these names, all as programs.
fn app(names: &[&str]) -> App {
    App::new(
        names
            .iter()
            .map(|&n| Command::new(n, Kind::Program))
            .collect(),
    )
}

/// The selected command's name; fails if a header is selected.
fn selected_name(app: &App) -> &str {
    match &app.rows[app.list.selected().unwrap()] {
        Row::Command(command) => &command.name,
        Row::Header(_) => panic!("a header is selected"),
    }
}

/// The rows as text: headers as `[A]`, commands by name.
fn labels(app: &App) -> Vec<String> {
    app.rows
        .iter()
        .map(|row| match row {
            Row::Header(section) => format!("[{section}]"),
            Row::Command(command) => command.name.clone(),
        })
        .collect()
}

/// Opens the search and types `text` into it.
fn typed(app: &mut App, text: &str) {
    app.open_search();
    text.chars().for_each(|c| app.type_char(c));
}

#[test]
fn the_first_command_is_selected_at_start_not_its_header() {
    assert_eq!(selected_name(&app(&["awk", "bash"])), "awk");
}

#[test]
fn moving_down_and_up_skips_the_section_headers() {
    let mut app = app(&["awk", "bash"]);
    app.down();
    assert_eq!(selected_name(&app), "bash");
    app.up();
    assert_eq!(selected_name(&app), "awk");
}

#[test]
fn moving_past_either_end_stays_on_the_end() {
    let mut app = app(&["awk", "bash"]);
    app.up();
    assert_eq!(selected_name(&app), "awk");
    app.down();
    app.down();
    assert_eq!(selected_name(&app), "bash");
}

#[test]
fn typing_keeps_only_the_matching_names_best_first_without_headers() {
    let mut app = app(&["apt", "awk", "bash", "gawk"]);
    typed(&mut app, "AWK");
    assert_eq!(labels(&app), ["awk", "gawk"]);
    assert_eq!(selected_name(&app), "awk");
    assert_eq!(app.shown_count(), 2);
}

#[test]
fn an_exact_name_is_always_the_first_result() {
    let mut app = app(&["chrm", "rmdir", "rm", "farm"]);
    typed(&mut app, "rm");
    assert_eq!(labels(&app)[0], "rm");
    assert_eq!(selected_name(&app), "rm");
}

#[test]
fn one_letter_ranks_the_names_starting_with_it_first() {
    let mut app = app(&["bash", "chmod", "ssh", "htop"]);
    typed(&mut app, "h");
    assert_eq!(labels(&app), ["htop", "ssh", "bash", "chmod"]);
}

#[test]
fn with_no_search_the_list_is_a_to_z_under_its_letters() {
    let app = app(&["awk", "bash"]);
    assert_eq!(labels(&app), ["[A]", "awk", "[B]", "bash"]);
}

#[test]
fn erasing_widens_the_search_again() {
    let mut app = app(&["awk", "ash"]);
    typed(&mut app, "aw");
    assert_eq!(app.shown_count(), 1);
    app.erase();
    assert_eq!(app.shown_count(), 2);
}

#[test]
fn a_search_with_no_match_shows_no_rows_and_selects_nothing() {
    let mut app = app(&["awk"]);
    typed(&mut app, "zzz");
    assert!(app.rows.is_empty());
    assert_eq!(app.list.selected(), None);
    app.down();
    app.up();
}

#[test]
fn leaving_the_search_keeps_its_text_and_its_filter() {
    let mut app = app(&["awk", "bash"]);
    typed(&mut app, "ba");
    app.leave_search();
    assert_eq!(app.mode, Mode::Browse);
    assert_eq!(app.query, "ba");
    assert_eq!(app.shown_count(), 1);
}

#[test]
fn opening_the_search_again_continues_the_text_left_in_it() {
    let mut app = app(&["bash", "basename"]);
    typed(&mut app, "bas");
    app.leave_search();
    typed(&mut app, "e");
    assert_eq!(app.query, "base");
    assert_eq!(labels(&app), ["basename"]);
}

#[test]
fn the_page_opens_on_a_command_and_closes_back_to_the_list() {
    let mut app = app(&["awk"]);
    app.open_info();
    assert_eq!(app.mode, Mode::Info);
    app.close_info();
    assert_eq!(app.mode, Mode::Browse);
}

#[test]
fn the_page_opens_from_the_search_bar_and_the_search_text_stays() {
    let mut app = app(&["awk", "bash"]);
    typed(&mut app, "aw");
    app.open_info();
    assert_eq!(app.mode, Mode::Info);
    app.close_info();
    assert_eq!(app.query, "aw");
}

#[test]
fn the_page_does_not_open_when_nothing_matches() {
    let mut app = app(&["awk"]);
    typed(&mut app, "zzz");
    app.open_info();
    assert_eq!(app.mode, Mode::Search);
}

#[test]
fn scrolling_up_past_the_top_stops_at_the_top() {
    let mut app = app(&["awk"]);
    app.scroll_by(5);
    app.scroll_by(-20);
    assert_eq!(app.scroll, 0);
}

/// An app over these names, where only `with_pages` have a lore page.
fn app_with_pages(names: &[&str], with_pages: &[&str]) -> App {
    App::with_pages(
        names
            .iter()
            .map(|&n| Command::new(n, Kind::Program))
            .collect(),
        with_pages
            .iter()
            .map(|&n| n.to_owned())
            .collect::<HashSet<_>>(),
    )
}

#[test]
fn an_at_sign_keeps_only_the_commands_with_a_lore_page() {
    let mut app = app_with_pages(&["grep", "groups", "ls"], &["grep", "ls"]);
    typed(&mut app, "@");
    assert_eq!(labels(&app), ["[G]", "grep", "[L]", "ls"]);
    typed(&mut app, "gr");
    assert_eq!(labels(&app), ["grep"]);
}

#[test]
fn a_name_given_at_start_opens_its_page_and_closing_it_goes_back_to_the_list() {
    let mut app = app_with_pages(&["awk", "bash"], &[]);
    assert!(app.open_direct("bash"));
    assert_eq!(app.mode, Mode::Info);
    assert_eq!(selected_name(&app), "bash");
    app.close_info();
    assert_eq!(app.mode, Mode::Browse, "the whole app is there, not a quit");
    assert_eq!(selected_name(&app), "bash");
    let at = app.list.selected();
    app.down();
    assert_ne!(app.list.selected(), at, "the arrows move the list as usual");
}

#[test]
fn a_name_not_on_this_machine_still_opens_when_lore_has_a_page_for_it() {
    let mut app = app_with_pages(&["awk"], &["robocopy"]);
    assert!(app.open_direct("robocopy"));
    assert_eq!(selected_name(&app), "robocopy");
}

#[test]
fn a_name_with_nothing_to_show_does_not_open() {
    let mut app = app_with_pages(&["awk"], &[]);
    assert!(!app.open_direct("lore-surely-has-no-such-command"));
    assert_eq!(app.mode, Mode::Browse);
}

#[test]
fn a_subcommand_given_at_start_opens_its_page_when_lore_has_one() {
    let mut app = app_with_pages(&["git"], &["git-commit"]);
    assert!(app.open_direct("git commit"));
    assert_eq!(selected_name(&app), "git commit");
}

#[test]
fn the_at_search_finds_a_subcommand_by_its_page() {
    let mut app = App::with_pages(
        vec![
            Command::new("git", Kind::Program),
            Command::new("git commit", Kind::Subcommand),
        ],
        ["git-commit".to_owned()].into_iter().collect(),
    );
    typed(&mut app, "@");
    assert_eq!(labels(&app), ["[G]", "git commit"]);
}

#[test]
fn searching_from_a_page_keeps_it_beside_the_list_until_another_opens() {
    let mut app = app(&["awk", "bash", "grep"]);
    app.open_info();
    typed(&mut app, "gr");
    assert!(app.page_shown());
    assert_eq!(selected_name(&app), "grep");
    app.open_info();
    assert_eq!(app.mode, Mode::Info);
    assert!(!app.beside_page);
}

#[test]
fn leaving_a_search_started_on_a_page_goes_to_the_full_list() {
    let mut app = app(&["awk", "bash"]);
    app.open_info();
    app.open_search();
    app.leave_search();
    assert_eq!(app.mode, Mode::Browse);
    assert!(!app.page_shown());
}

#[test]
fn a_search_from_the_list_does_not_split_the_screen() {
    let mut app = app(&["awk"]);
    app.open_search();
    assert!(!app.page_shown());
}

#[test]
fn a_search_finds_commands_by_what_their_pages_say_after_the_names_that_match() {
    // the built-in pages: tar's and zip's talk about compressing
    let mut app = app_with_pages(&["tar", "zip", "ls", "compressor"], &["tar", "zip", "ls"]);
    typed(&mut app, "compress");
    let rows = labels(&app);
    assert_eq!(rows[0], "compressor", "the name match first");
    assert!(rows.contains(&"tar".to_owned()) && rows.contains(&"zip".to_owned()));
    assert!(!rows.contains(&"ls".to_owned()));
}

#[test]
fn a_result_whose_page_has_a_matching_line_shows_it_and_others_do_not() {
    // built-in pages: cut's talks about cutting fields out of files
    let mut app = app_with_pages(&["cut", "ls"], &["cut", "ls"]);
    typed(&mut app, "cut files");
    let snippet = app.snippets.get("cut").expect("cut's page has the words");
    assert!(snippet.starts_with('[') && snippet.to_lowercase().contains("cut"));
    app.erase();
    app.erase();
    app.erase();
    app.erase();
    app.erase();
    app.erase();
    app.erase();
    app.erase();
    app.erase();
    assert!(app.snippets.is_empty(), "no search, no snippets");
}

#[test]
fn a_command_named_by_a_word_of_the_search_comes_first() {
    let mut app = app_with_pages(&["cut", "gzexe", "xclip"], &["cut", "gzexe", "xclip"]);
    typed(&mut app, "cut files");
    assert_eq!(labels(&app)[0], "cut");
}

#[test]
fn a_page_searched_from_is_laid_out_again_at_a_new_width_and_stays_the_same_page() {
    let mut app = app(&["awk", "bash"]);
    app.open_info();
    app.load_page(80);
    typed(&mut app, "bash"); // the search now selects bash
    app.relayout_page(40);
    let page = app.page.as_ref().unwrap();
    assert_eq!(page.command.name, "awk");
    assert_eq!(page.width, 40);
}
