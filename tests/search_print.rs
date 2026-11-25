//! Tests for `lore search WORDS` (src/interface/args.rs, App::search and
//! App::result_lines): the search bar's results as plain lines.

use lore_tui::app::App;
use lore_tui::commands::{Command, Kind};
use lore_tui::configs::{ERROR_SEARCH_WORDS, SEARCH_PRINT_MAX};
use lore_tui::interface::args::{Start, parse};

fn args(words: &[&str]) -> Result<Start, String> {
    parse(words.iter().map(|w| (*w).to_owned()))
}

#[test]
fn search_takes_its_words_quoted_or_not_and_needs_one() {
    assert_eq!(
        args(&["search", "compress folder"]),
        Ok(Start::Search("compress folder".to_owned()))
    );
    assert_eq!(
        args(&["search", "compress", "folder"]),
        Ok(Start::Search("compress folder".to_owned()))
    );
    assert_eq!(args(&["search"]), Err(ERROR_SEARCH_WORDS.to_owned()));
}

#[test]
fn the_results_are_name_line_and_categories_tab_apart_the_best_first() {
    let mut app = App::new(vec![
        Command::new("tar", Kind::Program),
        Command::new("curl", Kind::Tool),
        Command::new("ls", Kind::Program),
    ]);
    app.search("tar");
    let lines = app.result_lines(SEARCH_PRINT_MAX);
    let first: Vec<&str> = lines[0].split('\t').collect();
    assert_eq!(first.len(), 3, "{lines:?}");
    assert_eq!(first[0], "tar", "the exact name first");
    assert!(!first[1].is_empty(), "a line to say what it is");
    assert!(!first[1].starts_with('['), "no brackets: {}", first[1]);
    assert!(first[2].contains("archives"), "{}", first[2]);

    app.search("redirects");
    let names: Vec<String> = app
        .result_lines(SEARCH_PRINT_MAX)
        .iter()
        .map(|l| l.split('\t').next().unwrap().to_owned())
        .collect();
    assert!(
        names.contains(&"curl".to_owned()),
        "found by its page: {names:?}"
    );
}

#[test]
fn the_results_stop_at_the_maximum_and_nothing_found_is_empty() {
    let commands = (0..40)
        .map(|i| Command::new(format!("tool{i}"), Kind::Program))
        .collect();
    let mut app = App::new(commands);
    app.search("tool");
    assert_eq!(app.result_lines(5).len(), 5);
    app.search("zzqqxx");
    assert!(app.result_lines(SEARCH_PRINT_MAX).is_empty());
}
