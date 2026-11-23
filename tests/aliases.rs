//! Tests for reading `alias name=value` lines (src/commands/aliases.rs).
//! Each test name says the rule it checks.

use std::path::{Path, PathBuf};

use lore_tui::commands::aliases::{parse, parse_fish, read};
use lore_tui::commands::shell::Shell;

/// Shorthand for `parse`.
fn pairs(text: &str) -> Vec<(String, String)> {
    parse(text)
}

/// One expected (name, value).
fn pair(name: &str, value: &str) -> (String, String) {
    (name.to_string(), value.to_string())
}

#[test]
fn single_double_and_unquoted_values_are_read() {
    let text = "alias c='clear'\nalias ll=\"eza -lah --icons\"\nalias e=exit";
    assert_eq!(
        pairs(text),
        [
            pair("c", "clear"),
            pair("ll", "eza -lah --icons"),
            pair("e", "exit")
        ]
    );
}

#[test]
fn a_value_keeps_its_pipes_and_semicolons() {
    let text = "alias s='df -h | head -1; df -h | grep mapper'";
    assert_eq!(
        pairs(text),
        [pair("s", "df -h | head -1; df -h | grep mapper")]
    );
}

#[test]
fn an_indented_alias_inside_a_block_is_read() {
    assert_eq!(
        pairs("if true; then\n    alias ls='ls --color=auto'\nfi"),
        [pair("ls", "ls --color=auto")]
    );
}

#[test]
fn the_last_definition_of_a_name_wins_in_its_first_place() {
    let text = "alias ls='ls --color=auto'\nalias c='clear'\nalias ls='eza --icons'";
    assert_eq!(pairs(text), [pair("ls", "eza --icons"), pair("c", "clear")]);
}

#[test]
fn several_aliases_on_one_line_are_all_read() {
    assert_eq!(
        pairs("alias a='x y' b=z"),
        [pair("a", "x y"), pair("b", "z")]
    );
}

#[test]
fn commented_out_lines_and_alias_without_a_definition_define_nothing() {
    assert_eq!(
        pairs("# alias old='gone'\n#alias x=y\nalias -p\nalias ls"),
        []
    );
}

#[test]
fn a_comment_after_the_value_is_not_part_of_it() {
    assert_eq!(
        pairs("alias c='clear' # wipe the screen"),
        [pair("c", "clear")]
    );
}

/// A fresh, empty folder for one test, under cargo's temp dir.
fn temp_dir(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// The names `read` finds in these files, for `shell`.
fn names_read(files: &[PathBuf], shell: Shell) -> Vec<String> {
    read(files, shell).into_iter().map(|c| c.name).collect()
}

#[test]
fn files_that_do_not_exist_give_no_aliases_and_no_error() {
    let files = [
        PathBuf::from("/nonexistent/.bashrc"),
        temp_dir("empty").join(".zshrc"),
    ];
    assert!(names_read(&files, Shell::Bash).is_empty());
}

#[test]
fn a_file_with_bytes_that_are_not_utf8_still_gives_its_aliases() {
    let file = temp_dir("non-utf8").join(".bashrc");
    std::fs::write(&file, b"alias ok=true\n\xff\xfe junk\nalias ok2=false\n").unwrap();
    assert_eq!(names_read(&[file], Shell::Bash), ["ok", "ok2"]);
}

#[test]
fn a_later_file_overrides_an_earlier_one_so_the_users_beat_the_systems() {
    let dir = temp_dir("order");
    std::fs::write(dir.join("system"), "alias ll='ls -l'\n").unwrap();
    std::fs::write(dir.join("user"), "alias ll='eza -l'\n").unwrap();
    let found = read(&[dir.join("system"), dir.join("user")], Shell::Bash);
    assert_eq!(found[0].runs.as_deref(), Some("eza -l"));
}

#[test]
fn zsh_global_aliases_are_read_and_suffix_aliases_are_not_commands() {
    let text = "alias -g G='| grep'\nalias -s txt=vim\nalias l='ls'";
    assert_eq!(pairs(text), [pair("G", "| grep"), pair("l", "ls")]);
}

#[test]
fn fish_aliases_are_read_with_a_space_or_an_equals_sign() {
    let text = "alias ll 'eza -l'\nalias gs=\"git status\"\nalias --save c clear";
    assert_eq!(
        parse_fish(text),
        [
            pair("ll", "eza -l"),
            pair("gs", "git status"),
            pair("c", "clear")
        ]
    );
}
