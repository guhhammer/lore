//! Tests for shell completion (src/interface/complete.rs, completions/).

use lore_tui::configs::COMPLETION_SCRIPTS;
use lore_tui::info::explain::ManOption;
use lore_tui::interface::complete::{Sources, candidates};

const NAMES: &[&str] = &[
    "git",
    "git commit",
    "git config",
    "gio",
    "free",
    "free disk space",
    "tar",
];

fn complete(words: &[&str]) -> Vec<String> {
    let names: Vec<String> = NAMES.iter().map(|n| (*n).to_owned()).collect();
    let describe = |name: &str| format!("about {name}");
    let options_of = |program: &str| {
        if program == "tar" {
            vec![ManOption {
                names: vec!["-x".into(), "--extract".into()],
                takes_value: false,
                text: "Extract files.".into(),
            }]
        } else {
            Vec::new()
        }
    };
    let words: Vec<String> = words.iter().map(|w| (*w).to_owned()).collect();
    let sources = Sources {
        names: &names,
        describe: &describe,
        options_of: &options_of,
    };
    candidates(&words, &sources)
        .into_iter()
        .map(|(word, _)| word)
        .collect()
}

#[test]
fn the_first_word_completes_names_and_explain() {
    assert_eq!(complete(&["gi"]), ["gio", "git"]);
    assert_eq!(complete(&["ex"]), ["explain"]);
}

#[test]
fn after_a_program_come_its_subcommands_with_a_page() {
    assert_eq!(complete(&["git", "co"]), ["commit", "config"]);
    assert_eq!(complete(&["git", ""]), ["commit", "config"]);
}

#[test]
fn a_recipes_title_completes_word_by_word() {
    assert_eq!(complete(&["free", ""]), ["disk"]);
    assert_eq!(complete(&["free", "disk", ""]), ["space"]);
}

#[test]
fn a_dash_completes_the_options_and_print_completes_names() {
    assert!(complete(&["--pr"]) == ["--print"]);
    assert_eq!(complete(&["--print", "gi"]), ["gio", "git"]);
}

#[test]
fn explain_completes_a_program_then_its_flags_from_the_manual() {
    assert_eq!(complete(&["explain", "ta"]), ["tar"]);
    assert_eq!(complete(&["explain", "tar", "--ex"]), ["--extract"]);
    assert!(
        complete(&["explain", "tar", "file"]).is_empty(),
        "files: the shell's to complete"
    );
}

#[test]
fn a_name_is_described_only_when_it_is_a_whole_name() {
    let names: Vec<String> = NAMES.iter().map(|n| (*n).to_owned()).collect();
    let describe = |name: &str| format!("about {name}");
    let options_of = |_: &str| Vec::new();
    let sources = Sources {
        names: &names,
        describe: &describe,
        options_of: &options_of,
    };
    let found = candidates(&["fr".to_owned()], &sources);
    assert_eq!(found, [("free".to_owned(), "about free".to_owned())]);
}

#[test]
fn there_is_a_script_for_bash_zsh_and_fish_and_each_asks_lore() {
    let shells: Vec<&str> = COMPLETION_SCRIPTS.iter().map(|(s, _)| *s).collect();
    assert_eq!(shells, ["bash", "zsh", "fish"]);
    assert!(
        COMPLETION_SCRIPTS
            .iter()
            .all(|(_, script)| script.contains("lore --complete"))
    );
}
