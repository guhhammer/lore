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
