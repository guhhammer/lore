//! Tests for the command line (src/interface/args.rs).

use lore_tui::interface::args::{Start, parse};

fn args(list: &[&str]) -> Result<Start, String> {
    parse(list.iter().map(|a| (*a).to_owned()))
}

#[test]
fn no_arguments_opens_the_list() {
    assert_eq!(args(&[]), Ok(Start::List));
}

#[test]
fn a_name_opens_its_page_like_man() {
    assert_eq!(args(&["ls"]), Ok(Start::Page("ls".to_owned())));
}

#[test]
fn help_and_version_have_their_short_and_long_flags() {
    assert_eq!(args(&["-h"]), Ok(Start::Help));
    assert_eq!(args(&["--help"]), Ok(Start::Help));
    assert_eq!(args(&["-V"]), Ok(Start::Version));
    assert_eq!(args(&["--version"]), Ok(Start::Version));
}

#[test]
fn double_dash_makes_a_dash_name_a_name() {
    assert_eq!(args(&["--", "-x"]), Ok(Start::Page("-x".to_owned())));
}

#[test]
fn several_words_name_a_subcommand() {
    assert_eq!(
        args(&["git", "commit"]),
        Ok(Start::Page("git commit".to_owned()))
    );
}

#[test]
fn an_unknown_option_or_an_option_among_names_is_an_error() {
    assert!(args(&["--frobnicate"]).is_err());
    assert!(args(&["git", "--help"]).is_err());
}

#[test]
fn print_writes_a_page_out_with_or_without_a_subcommand() {
    assert_eq!(args(&["--print", "tar"]), Ok(Start::Print("tar".into())));
    assert_eq!(
        args(&["-p", "git", "commit"]),
        Ok(Start::Print("git commit".into()))
    );
    assert_eq!(
        args(&["--print", "--", "-x"]),
        Ok(Start::Print("-x".into()))
    );
}

#[test]
fn print_without_a_name_is_an_error() {
    assert!(args(&["--print"]).is_err());
    assert!(args(&["--print", "-x"]).is_err());
}

#[test]
fn explain_takes_the_whole_line_quoted_or_not() {
    assert_eq!(
        args(&["explain", "tar -xzvf a.tgz"]),
        Ok(Start::Explain("tar -xzvf a.tgz".into()))
    );
    assert_eq!(
        args(&["explain", "tar", "-xzvf", "a"]),
        Ok(Start::Explain("tar -xzvf a".into()))
    );
    assert!(args(&["explain"]).is_err());
}

#[test]
fn complete_takes_every_word_and_completions_takes_a_shell() {
    assert_eq!(
        args(&["--complete", "git", "co"]),
        Ok(Start::Complete(vec!["git".into(), "co".into()]))
    );
    assert_eq!(args(&["--complete"]), Ok(Start::Complete(Vec::new())));
    assert_eq!(
        args(&["--completions", "fish"]),
        Ok(Start::Completions("fish".into()))
    );
    assert!(args(&["--completions"]).is_err());
}

#[test]
fn check_takes_the_files_to_check() {
    assert_eq!(
        args(&["check", "a.md", "b.md"]),
        Ok(Start::Check(vec!["a.md".into(), "b.md".into()]))
    );
    assert!(args(&["check"]).is_err());
}
