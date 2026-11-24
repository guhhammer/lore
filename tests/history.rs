//! Tests for reading your use from the shell's history (src/utils/history.rs).

use std::collections::HashMap;

use lore_tui::utils::history::{commands_in, count};

#[test]
fn a_plain_bash_line_gives_its_command_and_its_subcommand() {
    assert_eq!(commands_in("git commit -m x"), ["git", "git commit"]);
    assert_eq!(commands_in("ls -la"), ["ls"]);
}

#[test]
fn every_part_of_a_pipe_or_a_list_counts() {
    assert_eq!(
        commands_in("ps aux | grep -i x && echo"),
        ["ps", "ps aux", "grep", "echo"]
    );
}

#[test]
fn sudo_time_and_settings_in_front_are_skipped() {
    assert_eq!(commands_in("sudo apt install x"), ["apt", "apt install"]);
    assert_eq!(commands_in("LANG=C time sort"), ["sort"]);
}

#[test]
fn a_path_counts_as_the_program_it_runs() {
    assert_eq!(commands_in("/usr/bin/ls"), ["ls"]);
}

#[test]
fn zsh_and_fish_lines_are_read_and_their_extra_fields_skipped() {
    assert_eq!(commands_in(": 1700000000:0;tar -xf a.tar"), ["tar"]);
    assert_eq!(commands_in("- cmd: cargo build"), ["cargo", "cargo build"]);
    assert!(commands_in("  when: 1700000000").is_empty());
    assert!(commands_in("#1700000000").is_empty());
}

#[test]
fn uses_add_up_over_a_file() {
    let mut uses = HashMap::new();
    count("ls\nls -l\ncd /tmp\n", &mut uses);
    assert_eq!(uses["ls"], 2);
    assert_eq!(uses["cd"], 1);
}
