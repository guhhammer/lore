//! Tests for merging commands from all sources into one list
//! (src/commands/mod.rs). Each test name says the rule it checks.

use lore_tui::commands::{self, Command, Kind};

/// Just the names, in order.
fn names(commands: &[Command]) -> Vec<&str> {
    commands.iter().map(|c| c.name.as_str()).collect()
}

#[test]
fn merging_sorts_names_ignoring_case() {
    let merged = commands::merge(vec![
        Command::new("bash", Kind::Program),
        Command::new("Xorg", Kind::Program),
        Command::new("awk", Kind::Program),
    ]);
    assert_eq!(names(&merged), ["awk", "bash", "Xorg"]);
}

#[test]
fn a_name_from_two_sources_is_listed_once_as_what_bash_would_run() {
    let merged = commands::merge(vec![
        Command::new("echo", Kind::Program),
        Command::new("echo", Kind::Builtin),
        Command::new("time", Kind::Program),
        Command::new("time", Kind::Keyword),
    ]);
    let kept: Vec<_> = merged
        .iter()
        .map(|c| (c.name.as_str(), c.kind, c.hides))
        .collect();
    assert_eq!(
        kept,
        [
            ("echo", Kind::Builtin, Some(Kind::Program)),
            ("time", Kind::Keyword, Some(Kind::Program))
        ]
    );
}

#[test]
fn an_alias_wins_over_the_program_of_the_same_name_and_says_it_hides_it() {
    let merged = commands::merge(vec![
        Command::new("ss", Kind::Program),
        Command::alias("ss", "flatpak run com.spotify.Client"),
    ]);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].kind, Kind::Alias);
    assert_eq!(
        merged[0].runs.as_deref(),
        Some("flatpak run com.spotify.Client")
    );
    assert_eq!(merged[0].hides, Some(Kind::Program));
}

#[test]
fn a_name_from_one_source_hides_nothing() {
    let merged = commands::merge(vec![Command::new("awk", Kind::Program)]);
    assert_eq!(merged[0].hides, None);
}

#[test]
fn the_collected_list_has_every_name_once_in_order_with_the_shell_and_sbin_ones() {
    let all = commands::collect();
    let mut sorted_unique = all.clone();
    sorted_unique
        .sort_by(|a, b| (a.name.to_lowercase(), &a.name).cmp(&(b.name.to_lowercase(), &b.name)));
    sorted_unique.dedup_by(|a, b| a.name == b.name);
    assert_eq!(all, sorted_unique);
    // cd and if are bash's own; reboot and fdisk are in /usr/sbin, which a
    // normal user's $PATH leaves out.
    for name in ["cd", "export", "if", "[[", "sudo", "su", "reboot", "fdisk"] {
        assert!(all.iter().any(|c| c.name == name), "{name} is missing");
    }
}
