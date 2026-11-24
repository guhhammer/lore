//! The command line: `lore` opens the list, `lore NAME` opens NAME's page
//! straight away, like `man NAME`, and `lore --print NAME` writes it out.

use crate::compile_config::{ERROR_OPTIONS_ALONE, ERROR_PRINT_NAME, ERROR_UNKNOWN_OPTION};

/// What lore was asked to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Start {
    /// The whole list.
    List,
    /// One command's page; closing it quits.
    Page(String),
    /// One command's page as plain text on stdout (`--print NAME`).
    Print(String),
    Help,
    Version,
}

/// The arguments after the program's name, read. The error is the message
/// to show.
pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Start, String> {
    let args: Vec<String> = args.into_iter().collect();
    match args.as_slice() {
        [] => Ok(Start::List),
        [flag] if flag == "-h" || flag == "--help" => Ok(Start::Help),
        [flag] if flag == "-V" || flag == "--version" => Ok(Start::Version),
        // `lore --print git commit`: the same names, printed
        [flag, names @ ..] if flag == "-p" || flag == "--print" => match names {
            [] => Err(ERROR_PRINT_NAME.to_owned()),
            [dashes, name] if dashes == "--" => Ok(Start::Print(name.clone())),
            names if names.iter().all(|n| !n.starts_with('-')) => Ok(Start::Print(names.join(" "))),
            _ => Err(ERROR_OPTIONS_ALONE.to_owned()),
        },
        // `--` ends the options: `lore -- -x` is the page of `-x`
        [dashes, name] if dashes == "--" => Ok(Start::Page(name.clone())),
        [option] if option.starts_with('-') => {
            Err(ERROR_UNKNOWN_OPTION.replace("{option}", option))
        }
        [name] => Ok(Start::Page(name.clone())),
        // `lore git commit`: a subcommand's page
        names if names.iter().all(|n| !n.starts_with('-')) => Ok(Start::Page(names.join(" "))),
        _ => Err(ERROR_OPTIONS_ALONE.to_owned()),
    }
}
