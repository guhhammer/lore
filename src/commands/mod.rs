//! Finds every command. Each source is a file here (aliases, functions,
//! path, shell); `collect` merges them into one sorted list, each name once.

pub mod aliases; // the user's aliases
pub mod functions; // the user's shell functions (fish only)
mod path; // programs in $PATH and the program directories
pub mod shell; // the user's shell, its builtins and keywords

pub use path::locate;
use shell::Shell;

/// What a name is in the shell. The order matters: when one name is several
/// of these, the shell runs the first (alias > keyword > function > builtin >
/// program), and `merge` keeps that one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Alias,
    Keyword,
    Function,
    Builtin,
    Program,
    /// A program's subcommand with a lore page (`git commit`). Its name has
    /// a space, so it never shares a name with anything else.
    Subcommand,
    /// A recipe: a task page that chains commands ("free disk space"),
    /// from recipes/; lore's, not this machine's.
    Recipe,
}

/// One entry of the list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub name: String,
    pub kind: Kind,
    /// For an alias: what it runs.
    pub runs: Option<String>,
    /// What this entry shadows under the same name (the `ss` program behind
    /// an `ss` alias). Set by `merge`.
    pub hides: Option<Kind>,
}

impl Command {
    /// The program a name runs: the name itself, or a subcommand's first
    /// word (`git commit` → `git`), whose file, package and version it has.
    /// Only a subcommand is split: a program's file name may hold a space.
    pub fn program(&self) -> &str {
        match self.kind {
            Kind::Subcommand => self.name.split(' ').next().unwrap_or_default(),
            _ => &self.name,
        }
    }

    /// A command with nothing extra.
    pub fn new(name: impl Into<String>, kind: Kind) -> Self {
        Self {
            name: name.into(),
            kind,
            runs: None,
            hides: None,
        }
    }

    /// An alias and the command line it runs.
    pub fn alias(name: impl Into<String>, runs: impl Into<String>) -> Self {
        Self {
            runs: Some(runs.into()),
            ..Self::new(name, Kind::Alias) // the other fields as `new` sets them
        }
    }
}

/// Every command from every source, for the user's shell, sorted, each name
/// once.
pub fn collect() -> Vec<Command> {
    let shell = Shell::current();
    let mut all = aliases::scan(shell);
    all.extend(functions::scan(shell));
    all.extend(shell.keywords());
    all.extend(shell.builtins());
    all.extend(path::scan());
    let mut all = merge(all);
    all.extend(subcommands(&all));
    merge(all)
}

/// A row for every subcommand lore has a page for (`git commit`) whose
/// program is in the list: a subcommand of a program not installed would
/// only be noise.
fn subcommands(commands: &[Command]) -> Vec<Command> {
    let installed: std::collections::HashSet<&str> =
        commands.iter().map(|c| c.name.as_str()).collect();
    crate::info::content::subcommand_titles()
        .filter(|title| {
            let parent = title.split(' ').next().unwrap_or_default();
            installed.contains(parent)
        })
        .map(|title| Command::new(title, Kind::Subcommand))
        .collect()
}

/// Sorts A to Z ignoring case and keeps one entry per name: the one bash
/// would run, noting in `hides` what it shadows.
pub fn merge(mut commands: Vec<Command>) -> Vec<Command> {
    // by lowercase name, then exact name, then Kind, so equal names sit
    // together with the winner first
    commands.sort_by(|a, b| {
        (a.name.to_lowercase(), &a.name, a.kind).cmp(&(b.name.to_lowercase(), &b.name, b.kind))
    });
    // dedup_by drops `later` when it returns true; `kept` is the entry before it
    commands.dedup_by(|later, kept| {
        let same = later.name == kept.name;
        if same && later.kind != kept.kind {
            kept.hides.get_or_insert(later.kind); // only the first shadowed kind
        }
        same
    });
    commands
}
