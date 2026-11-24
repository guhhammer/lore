//! Tests for the beacon: its table, its rankings, the sources it can be
//! made from, and its topic (src/info/beacon.rs, src/app.rs).

use std::collections::HashSet;

use lore_tui::app::{App, Topic};
use lore_tui::commands::{Command, Kind};
use lore_tui::compile_config::{BEACON_NOT_HERE, BEACON_POPULAR, BEACON_SNAPSHOT, BEACON_TRENDING};
use lore_tui::info::beacon::{self, Entry};
use lore_tui::interface::rows::Row;

fn entry(name: &str, debian: u64, brew_year: u64, brew_month: u64) -> Entry {
    Entry {
        name: name.to_owned(),
        debian,
        brew_year,
        brew_month,
    }
}

#[test]
fn the_table_reads_four_columns_and_skips_comments_and_odd_lines() {
    let text = "# a comment\ngit\t10\t20\t3\nbroken line\n../x\t1\t1\t1\njq\t1\t2\tnope\n";
    assert_eq!(beacon::parse(text), [entry("git", 10, 20, 3)]);
}

#[test]
fn the_table_written_reads_back_the_same() {
    let entries = vec![entry("git", 10, 20, 3), entry("jq", 0, 5, 1)];
    assert_eq!(beacon::parse(&beacon::to_tsv(&entries)), entries);
}

#[test]
fn the_built_in_table_has_commands_with_a_page() {
    let entries = beacon::parse(BEACON_SNAPSHOT);
    assert!(entries.len() > 100, "{} entries", entries.len());
    let pages: HashSet<&str> = lore_tui::info::content::built_in_names().collect();
    assert!(entries.iter().all(|e| pages.contains(e.name.as_str())));
}

#[test]
fn most_used_weighs_debian_and_homebrew_alike() {
    // each has all of one source: a tie, broken by name, though the
    // numbers differ a thousandfold
    let entries = vec![
        entry("b", 100_000, 0, 0),
        entry("a", 0, 100, 0),
        entry("c", 0, 0, 0),
    ];
    let order: Vec<&str> = beacon::popular(&entries)
        .iter()
        .map(|e| e.name.as_str())
        .collect();
    assert_eq!(order, ["a", "b", "c"]);
}

#[test]
fn trending_is_a_month_above_the_years_mean_and_big_enough() {
    let rising = entry("rising", 0, 120_000, 30_000); // 3× a mean month
    let steady = entry("steady", 0, 120_000, 10_000); // a mean month
    let tiny = entry("tiny", 0, 12, 10); // risen, but too few
    assert_eq!(beacon::rise(&rising), Some(3.0));
    assert_eq!(beacon::rise(&steady), None);
    assert_eq!(beacon::rise(&tiny), None);
    let entries = [steady, tiny, rising.clone()];
    let trending = beacon::trending(&entries);
    assert_eq!(trending.len(), 1);
    assert_eq!(trending[0].0, &rising);
}

#[test]
fn popcon_leaves_the_base_system_out() {
    let text = "#rank name inst vote old recent no-files (maintainer)\n\
                1 dpkg 1000 900 0 0 0 (x)\n\
                2 git 400 300 0 0 0 (y)\n";
    let votes = beacon::popcon_votes(text);
    assert_eq!(votes.get("git"), Some(&300));
    assert!(!votes.contains_key("dpkg"), "on nearly every machine");
}

#[test]
fn homebrew_counts_drop_commas_and_versions() {
    let json = r#"{"items":[{"number":1,"formula":"python@3.13","count":"1,000","percent":"1"},
                  {"number":2,"formula":"python@3.12","count":"500","percent":"1"},
                  {"number":3, "formula": "jq", "count": "7"}]}"#;
    let counts = beacon::brew_counts(json);
    assert_eq!(counts.get("python"), Some(&1500));
    assert_eq!(counts.get("jq"), Some(&7));
}

#[test]
fn a_table_made_from_the_sources_keeps_only_commands_with_a_page() {
    let popcon = "1 dpkg 1000 900 0 0 0 (x)\n2 git 400 300 0 0 0 (y)\n3 libfoo 300 200 0 0 0 (z)\n";
    let year = r#"[{"formula":"git","count":"50"},{"formula":"jq","count":"40"}]"#;
    let month = r#"[{"formula":"jq","count":"9"}]"#;
    let entries = beacon::combine(popcon, year, month, &|name| name != "libfoo");
    let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["git", "jq"]);
    assert_eq!(entries[1], entry("jq", 0, 40, 9));
}

fn app_with_a_beacon() -> App {
    let mut app = App::with_pages(
        vec![Command::new("git", Kind::Program)],
        HashSet::from(["git".to_owned(), "jq".to_owned()]),
    );
    app.set_beacon(vec![
        entry("git", 300, 100_000, 8_000),
        entry("jq", 100, 60_000, 20_000), // 4× a mean month
        entry("nopage", 900, 900, 900),
    ]);
    app
}

#[test]
fn the_beacon_is_a_topic_once_it_has_numbers() {
    let mut app = App::new(vec![Command::new("git", Kind::Program)]);
    assert!(!app.topics.contains(&Topic::Beacon));
    app.set_beacon(vec![entry("git", 1, 1, 1)]);
    assert!(app.topics.contains(&Topic::Beacon));
}

#[test]
fn the_beacon_lists_trending_then_most_used_under_their_headings() {
    let mut app = app_with_a_beacon();
    app.show_topic(Topic::Beacon);
    assert_eq!(app.topic, Topic::Beacon);
    let shown: Vec<String> = app
        .rows
        .iter()
        .map(|row| match row {
            Row::Title(title) => format!("[{title}]"),
            Row::Header(section) => format!("[{section}]"),
            Row::Command(command) => command.name.clone(),
        })
        .collect();
    let trending = format!("[{BEACON_TRENDING}]");
    let popular = format!("[{BEACON_POPULAR}]");
    assert_eq!(
        shown,
        [trending.as_str(), "jq", popular.as_str(), "git", "jq"]
    );
}

#[test]
fn a_command_not_installed_is_listed_for_its_page_and_says_so() {
    let mut app = app_with_a_beacon();
    app.show_topic(Topic::Beacon);
    assert!(app.snippets["jq"].contains(BEACON_NOT_HERE));
    assert!(app.snippets["jq"].contains("4.0×"));
    assert!(!app.snippets["git"].contains(BEACON_NOT_HERE));
    assert!(
        !app.snippets.contains_key("nopage"),
        "no page, not installed: left out"
    );
}

#[test]
fn a_command_not_installed_is_never_marked_unused() {
    let mut app = app_with_a_beacon();
    app.set_personal(Vec::new(), Vec::new(), [("ls".to_owned(), 1)].into());
    assert!(app.is_unused("git"));
    assert!(!app.is_unused("jq"));
}
