//! Tests for recognising the user's shell and its lists (src/commands/shell.rs).
//! Each test name says the rule it checks.

use lore_tui::commands::shell::Shell;

/// Just the names of a list of commands.
fn names(commands: Vec<lore_tui::commands::Command>) -> Vec<String> {
    commands.into_iter().map(|c| c.name).collect()
}

#[test]
fn the_shell_is_recognised_by_the_last_part_of_its_path() {
    assert_eq!(Shell::from_path("/bin/zsh"), Shell::Zsh);
    assert_eq!(Shell::from_path("/opt/homebrew/bin/fish"), Shell::Fish);
    assert_eq!(Shell::from_path("/usr/bin/bash"), Shell::Bash);
}

#[test]
fn an_unknown_shell_is_treated_as_bash() {
    assert_eq!(Shell::from_path("/bin/dash"), Shell::Bash);
    assert_eq!(Shell::from_path(""), Shell::Bash);
}

#[test]
fn each_shell_brings_its_own_builtins_and_keywords() {
    assert!(names(Shell::Bash.builtins()).contains(&"shopt".to_string()));
    assert!(names(Shell::Zsh.builtins()).contains(&"setopt".to_string()));
    assert!(names(Shell::Zsh.keywords()).contains(&"foreach".to_string()));
    assert!(names(Shell::Fish.builtins()).contains(&"set_color".to_string()));
    assert!(!names(Shell::Fish.builtins()).contains(&"shopt".to_string()));
}

#[test]
fn every_shell_reads_its_users_files_after_the_system_ones() {
    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish] {
        let files = shell.alias_files();
        let first_user = files.iter().position(|f| f.starts_with("~/")).unwrap();
        assert!(
            files[first_user..].iter().all(|f| f.starts_with("~/")),
            "{shell:?}"
        );
    }
}
