//! The command line: `lore` opens the list, `lore NAME` opens NAME's page
//! straight away, like `man NAME`, and `lore --print NAME` writes it out.

use crate::configs::{
    CHECK_WORD, COMPLETE_FLAG, COMPLETIONS_FLAG, ERROR_CHECK_FILES, ERROR_COMPLETIONS_SHELL,
    ERROR_EXPLAIN_LINE, ERROR_OPTIONS_ALONE, ERROR_PRINT_NAME, ERROR_UNKNOWN_OPTION, EXPLAIN_WORD,
    UPDATE_PAGES_FLAG,
};

/// What lore was asked to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Start {
    /// The whole list.
    List,
    /// One command's page; closing it quits.
    Page(String),
    /// One command's page as plain text on stdout (`--print NAME`).
    Print(String),
    /// Each word of a command line and what it means (`explain LINE`).
    Explain(String),
    /// What can come next on the command line, for the completion scripts
    /// (`--complete WORDS`, the last one being typed).
    Complete(Vec<String>),
    /// A shell's completion script (`--completions bash`).
    Completions(String),
    /// Check page files another program ships (`check FILE…`).
    Check(Vec<String>),
    /// Download newer pages, the only time lore asks for them
    /// (`--update-pages`).
    UpdatePages,
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
        [flag] if flag == UPDATE_PAGES_FLAG => Ok(Start::UpdatePages),
        [flag, words @ ..] if flag == COMPLETE_FLAG => Ok(Start::Complete(words.to_vec())),
        [flag, shell] if flag == COMPLETIONS_FLAG => Ok(Start::Completions(shell.clone())),
        [flag] if flag == COMPLETIONS_FLAG => Err(ERROR_COMPLETIONS_SHELL.to_owned()),
        [word, files @ ..] if word == CHECK_WORD => match files {
            [] => Err(ERROR_CHECK_FILES.to_owned()),
            files => Ok(Start::Check(files.to_vec())),
        },
        // `lore explain "tar -xzvf a.tgz"`, or the words unquoted: all of
        // them are the line, dashes included
        [word, line @ ..] if word == EXPLAIN_WORD => match line {
            [] => Err(ERROR_EXPLAIN_LINE.to_owned()),
            line => Ok(Start::Explain(line.join(" "))),
        },
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
