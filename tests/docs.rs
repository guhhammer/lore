//! Tests for the docs topic: DevDocs' catalogue, the topic's rows, and
//! HTML shown as text (src/info/docs.rs, src/utils/html.rs).

use lore_tui::app::{App, Topic};
use lore_tui::commands::{Command, Kind};
use lore_tui::configs::{DOCS_ALL, DOCS_MOST, DOCS_MOST_SEARCHED, TOOLS_EMPTY};
use lore_tui::info::docs;
use lore_tui::info::page::Style;
use lore_tui::interface::rows::Row;
use lore_tui::utils::html;

#[test]
fn the_catalogue_is_read_and_odd_lines_are_skipped() {
    let text = "# comment\npython~3.13\tPython\t3.13\t3.13.1\t2048\t17\t2\thttps://python.org\tBy PSF\n\
                ../evil\tX\t\t\t1\t1\t0\t\t\nshort\tline\n";
    let sets = docs::parse_catalog(text);
    assert_eq!(sets.len(), 1);
    assert_eq!(sets[0].title(), "Python 3.13");
    assert_eq!(sets[0].rank, 2);
}

#[test]
fn the_built_in_catalogue_has_every_set_and_the_most_asked_first() {
    let sets = docs::catalog();
    assert!(sets.len() > 500, "{} sets", sets.len());
    let most = docs::most_searched(&sets, DOCS_MOST_SEARCHED);
    assert_eq!(most.len(), DOCS_MOST_SEARCHED);
    assert!(most.windows(2).all(|w| w[0].rank <= w[1].rank));
    let families: std::collections::HashSet<&str> = most
        .iter()
        .map(|s| s.slug.split('~').next().unwrap())
        .collect();
    assert_eq!(families.len(), most.len(), "one version of each");
}

#[test]
fn the_docs_topic_lists_the_most_searched_then_every_set() {
    let mut app = App::new(vec![Command::new("ls", Kind::Program)]);
    app.show_topic(Topic::Docs);
    let titles: Vec<&str> = app
        .rows
        .iter()
        .filter_map(|r| match r {
            Row::Title(t) => Some(t.as_str()),
            _ => None,
        })
        .collect();
    assert!(
        titles.contains(&DOCS_MOST) && titles.contains(&DOCS_ALL),
        "{titles:?}"
    );
    let first = app.selected().unwrap();
    assert_eq!(first.kind, Kind::Docset);
    assert!(first.runs.is_some(), "a set's row carries its slug");
    assert!(app.snippets.contains_key(&first.name));
}

#[test]
fn the_tools_topic_is_there_and_empty_for_now() {
    let mut app = App::new(vec![Command::new("ls", Kind::Program)]);
    assert!(app.topics.contains(&Topic::Tools));
    app.show_topic(Topic::Tools);
    assert!(app.rows.is_empty());
    assert!(!TOOLS_EMPTY.is_empty());
}

#[test]
fn closing_a_set_is_a_no_op_when_none_is_open() {
    let mut app = App::new(vec![Command::new("ls", Kind::Program)]);
    assert!(!app.close_docset(), "Esc then quits");
}

#[test]
fn html_becomes_wrapped_text_headings_bullets_and_code() {
    let page = "<h1>str.split</h1><p>Return a  <code>list</code>\n of the words &amp; more.</p>\
                <ul><li>one</li><li>two</li></ul><pre>&gt;&gt;&gt; 'a b'.split()\n['a', 'b']</pre>\
                <script>alert(1)</script><p>end</p>";
    let lines = html::to_lines(page, 40);
    let shown: Vec<(&str, Style)> = lines.iter().map(|l| (l.text.as_str(), l.style)).collect();
    assert_eq!(
        shown,
        [
            ("str.split", Style::Heading),
            ("", Style::Text),
            ("Return a list of the words & more.", Style::Text),
            ("", Style::Text),
            ("• one", Style::Text),
            ("• two", Style::Text),
            ("", Style::Text),
            ("  >>> 'a b'.split()", Style::Code),
            ("  ['a', 'b']", Style::Code),
            ("", Style::Text),
            ("end", Style::Text),
        ]
    );
}

#[test]
fn entities_are_decoded_and_control_characters_dropped() {
    assert_eq!(
        html::decode("&lt;a&gt; &#39;x&#x27; &copy; &bogus; \u{1b}[31m"),
        "<a> 'x' © &bogus; [31m"
    );
}
