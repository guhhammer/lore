//! Tests for the languages topic: learnxinyminutes.com's pages, their
//! header, the bat-like view and its colours (src/info/languages.rs,
//! src/utils/highlight.rs).

use std::collections::HashSet;

use lore_tui::app::{App, Topic};
use lore_tui::commands::{Command, Kind};
use lore_tui::configs::{LANGUAGE_CATEGORY, LANGUAGE_SUFFIX};
use lore_tui::info::languages;
use lore_tui::info::page::Style;
use lore_tui::utils::highlight;

#[test]
fn a_page_splits_into_its_header_and_its_body() {
    let page = "---\nname: \"Common Lisp\"\nfilename: learn.lisp\ncontributors:\n    - [\"Ann\", \"https://a\"]\ntranslators:\n    - [\"Bo\", \"https://b\"]\n---\n\nIntro.\n";
    let (header, body) = languages::split(page);
    assert_eq!(languages::field(header, "name"), Some("Common Lisp"));
    assert_eq!(languages::field(header, "filename"), Some("learn.lisp"));
    assert_eq!(languages::people(header), ["Ann", "Bo"]);
    assert_eq!(body, "\nIntro.\n");
    assert_eq!(languages::split("no header"), ("", "no header"));
}

#[test]
fn every_built_in_language_is_listed_by_its_name() {
    let list = languages::commands(&HashSet::new());
    assert!(list.len() > 150, "{} languages", list.len());
    assert!(list.iter().all(|c| c.kind == Kind::Language));
    assert!(list.iter().any(|c| c.name == "Python"));
    assert!(
        list.iter()
            .all(|c| !c.name.starts_with('"') && c.name == c.name.trim())
    );
}

#[test]
fn a_language_named_like_a_command_here_gets_a_suffix_and_still_opens() {
    let taken = HashSet::from(["bc".to_owned()]);
    let list = languages::commands(&taken);
    let name = format!("bc{LANGUAGE_SUFFIX}");
    assert!(list.iter().any(|c| c.name == name));
    let content = languages::content(&name).unwrap();
    assert_eq!(content.categories, [LANGUAGE_CATEGORY]);
}

#[test]
fn a_language_page_is_a_numbered_file_in_a_frame() {
    let lines = languages::render("Python", 80);
    let texts: Vec<&str> = lines.iter().map(|l| l.text.as_str()).collect();
    assert_eq!(texts[0], "Python");
    assert!(texts.iter().any(|t| t.contains("File: learnpython.py")));
    assert!(texts.iter().any(|t| t.starts_with("   1 │ ")));
    assert!(texts.iter().any(|t| t.contains('┬')) && texts.iter().any(|t| t.contains('┴')));
    assert!(
        lines
            .iter()
            .all(|l| lore_tui::utils::text::width(&l.text) <= 80),
        "fits the width"
    );
    assert!(
        texts
            .last()
            .unwrap()
            .starts_with("https://learnxinyminutes.com/python")
    );
}

#[test]
fn the_files_banner_comments_are_headings_for_the_jumper() {
    let lines = languages::render("Python", 100);
    let headings: Vec<&str> = lines
        .iter()
        .filter(|l| l.style == Style::Heading)
        .map(|l| l.text.as_str())
        .collect();
    assert!(
        headings
            .iter()
            .any(|h| h.contains("1. Primitive Datatypes")),
        "{headings:?}"
    );
}

#[test]
fn a_language_page_is_its_own_topic() {
    let app = App::new(vec![
        Command::new("ls", Kind::Program),
        Command::new("Python", Kind::Language),
    ]);
    assert!(app.topics.contains(&Topic::Languages));
    assert_eq!(app.shown_count(), 1, "the commands topic has ls only");
}

fn styles(code: &str, syntax: &highlight::Syntax) -> Vec<(String, Style)> {
    highlight::line(code, syntax, &mut None)
}

#[test]
fn comments_strings_numbers_and_keywords_get_their_colours() {
    let python = highlight::by_name("py").unwrap();
    assert_eq!(
        styles("if x == \"a # b\": return 42  # done", &python),
        [
            ("if".to_owned(), Style::Keyword),
            (" x == ".to_owned(), Style::Text),
            ("\"a # b\"".to_owned(), Style::Literal),
            (": ".to_owned(), Style::Text),
            ("return".to_owned(), Style::Keyword),
            (" ".to_owned(), Style::Text),
            ("42".to_owned(), Style::Number),
            ("  ".to_owned(), Style::Text),
            ("# done".to_owned(), Style::Comment),
        ]
    );
}

#[test]
fn a_mark_inside_a_word_is_not_a_comment() {
    let bash = highlight::by_name("bash").unwrap();
    assert!(
        styles("echo $#", &bash)
            .iter()
            .all(|(_, s)| *s != Style::Comment)
    );
    assert!(
        styles("x2 = abc123", &bash)
            .iter()
            .all(|(_, s)| *s != Style::Number)
    );
}

#[test]
fn a_block_comment_goes_on_across_lines() {
    let c = highlight::by_name("c").unwrap();
    let mut open = None;
    assert_eq!(
        highlight::line("int a; /* starts", &c, &mut open)
            .last()
            .unwrap()
            .1,
        Style::Comment
    );
    assert!(open.is_some());
    let next = highlight::line("ends */ int b;", &c, &mut open);
    assert_eq!(next[0], ("ends */".to_owned(), Style::Comment));
    assert!(open.is_none());
}

#[test]
fn rust_lifetimes_are_not_strings() {
    let rust = highlight::by_name("rust").unwrap();
    let parts = styles("fn f<'a>(x: &'a str) {}", &rust);
    assert!(parts.iter().all(|(_, s)| *s != Style::Literal), "{parts:?}");
}

#[test]
fn an_unknown_language_is_guessed_from_its_comments() {
    let code: Vec<String> = ["-- one", "x = 1", "-- two"].map(str::to_owned).to_vec();
    assert_eq!(highlight::guess(&code).line, ["--"]);
}

#[test]
fn the_favourite_language_comes_first_under_its_heading() {
    let mut app = App::new(vec![
        Command::new("C", Kind::Language),
        Command::new("Python", Kind::Language),
    ]);
    app.step_topic(1);
    assert_eq!(app.topic, Topic::Languages);
    app.set_favourite(Some("Python".to_owned()));
    let first: Vec<String> = app
        .rows
        .iter()
        .take(2)
        .map(|row| match row {
            lore_tui::interface::rows::Row::Title(t) => format!("[{t}]"),
            lore_tui::interface::rows::Row::Header(h) => format!("[{h}]"),
            lore_tui::interface::rows::Row::Command(c) => c.name.clone(),
        })
        .collect();
    assert_eq!(first, ["[favourite]", "Python"]);
    app.set_favourite(None);
    assert!(!matches!(
        app.rows[0],
        lore_tui::interface::rows::Row::Title(_)
    ));
}
