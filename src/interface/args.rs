//! The command line: `lore` opens the list, `lore NAME` opens NAME's page
//! straight away, like `man NAME`.

/// What lore was asked to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Start {
    /// The whole list.
    List,
    /// One command's page; closing it quits.
    Page(String),
    Help,
    Version,
}

/// `lore --help`.
pub const HELP: &str = "\
lore: every command you can type, and what each one does. Offline.

Usage:
  lore               the list of every command, A to Z
  lore NAME          NAME's page straight away (like `man NAME`)
  lore git commit    a subcommand's page
  lore --help        this text
  lore --version     lore's version

In the list: arrows move, / searches (start with @ for only the commands
that have a lore page), right arrow or Enter opens a page, Esc quits.
On a page: arrows and PgUp/PgDn scroll, / finds another command, left
arrow or Esc close it.
`man lore` has the rest.";

/// The arguments after the program's name, read. The error is the message
/// to show.
pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Start, String> {
    let args: Vec<String> = args.into_iter().collect();
    match args.as_slice() {
        [] => Ok(Start::List),
        [flag] if flag == "-h" || flag == "--help" => Ok(Start::Help),
        [flag] if flag == "-V" || flag == "--version" => Ok(Start::Version),
        // `--` ends the options: `lore -- -x` is the page of `-x`
        [dashes, name] if dashes == "--" => Ok(Start::Page(name.clone())),
        [option] if option.starts_with('-') => {
            Err(format!("unknown option {option}; see lore --help"))
        }
        [name] => Ok(Start::Page(name.clone())),
        // `lore git commit`: a subcommand's page
        names if names.iter().all(|n| !n.starts_with('-')) => Ok(Start::Page(names.join(" "))),
        _ => Err("options go alone: lore NAME, or see lore --help".to_owned()),
    }
}
